use std::{future::Future, pin::Pin};

pub use include_dir;
pub use schema_core::CoreError;
use schema_core::{
    commands,
    json_rpc::types::{ApplyMigrationsInput, MarkMigrationAppliedInput, SchemaPushInput},
    EngineState, GenericApi,
};
use thiserror::Error;
use tokio::fs::remove_dir_all;

type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

fn format_error_array(arr: &[String]) -> String {
    arr.join("\n")
}

#[derive(Error, Debug)]
pub enum DbPushError {
    #[error("Failed to reset database: ${0}")]
    ResetFailed(CoreError),
    #[error("Some changes could not be executed:\n {}", format_error_array(.0))]
    UnexecutableChanges(Vec<String>),
    #[error("Data loss may occur:\n {}", format_error_array(.0))]
    PossibleDataLoss(Vec<String>),
    #[error("An error occurred pushing schema to the database: ${0}")]
    Other(#[from] CoreError),
}

pub struct DbPush<'a> {
    datamodel: &'a str,
    url: &'a str,
    force_reset: bool,
    accept_data_loss: bool,
    fut: Option<BoxedFuture<Result<u32, DbPushError>>>,
}

impl<'a> DbPush<'a> {
    pub fn force_reset(mut self) -> Self {
        self.force_reset = true;
        self
    }

    pub fn accept_data_loss(mut self) -> Self {
        self.accept_data_loss = true;
        self
    }
}

impl<'a> Future for DbPush<'a> {
    type Output = Result<u32, DbPushError>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.fut.is_none() {
            let datamodel = self.datamodel.to_string();
            let url = self.url.to_string();
            let force_reset = self.force_reset;
            let accept_data_loss = self.accept_data_loss;

            self.fut = Some(Box::pin(async move {
                let engine_state = EngineState::new(Some(datamodel.clone()), None);

                if force_reset {
                    engine_state
                        .reset()
                        .await
                        .map_err(DbPushError::ResetFailed)?;
                }

                let input = SchemaPushInput {
                    force: accept_data_loss,
                    schema: datamodel,
                };

                let output = engine_state
                    .with_connector_for_url(
                        url.to_string(),
                        Box::new(|connector| Box::pin(commands::schema_push(input, connector))),
                    )
                    .await?;

                if !output.unexecutable.is_empty() && !force_reset {
                    return Err(DbPushError::UnexecutableChanges(output.unexecutable));
                }

                if !output.warnings.is_empty() && !accept_data_loss {
                    return Err(DbPushError::PossibleDataLoss(output.warnings));
                }

                Ok(output.executed_steps)
            }));
        }

        self.fut.as_mut().unwrap().as_mut().poll(cx)
    }
}

pub fn db_push<'a>(datamodel: &'a str, url: &'a str) -> DbPush<'a> {
    DbPush {
        datamodel,
        url,
        force_reset: false,
        accept_data_loss: false,
        fut: None,
    }
}

#[derive(Error, Debug)]
pub enum MigrateDeployError {
    #[error("The temporary file path for the database migrations is invalid.")]
    InvalidDirectory,
    #[error("An error occurred creating the temporary directory for the migrations: {0}")]
    CreateDir(std::io::Error),
    #[error("An error occurred extracting the migrations to the temporary directory: {0}")]
    ExtractMigrations(std::io::Error),
    #[error("An error occurred running the migrations: {0}")]
    Connector(#[from] CoreError),
    #[error("An error occurred removing the temporary directory for the migrations: {0}")]
    RemoveDir(std::io::Error),
}

pub struct MigrateDeploy<'a> {
    datamodel: &'a str,
    migrations: &'static include_dir::Dir<'static>,
    url: &'a str,
    temp_dir: Option<String>,
    fut: Option<BoxedFuture<Result<(), MigrateDeployError>>>,
}

impl<'a> MigrateDeploy<'a> {
    pub fn with_temp_dir(mut self, dir: &str) -> Self {
        self.temp_dir = Some(dir.to_string());
        self
    }
}

pub fn migrate_deploy<'a>(
    datamodel: &'a str,
    migrations: &'static include_dir::Dir<'static>,
    url: &'a str,
) -> MigrateDeploy<'a> {
    MigrateDeploy {
        datamodel,
        migrations,
        url,
        temp_dir: None,
        fut: None,
    }
}

impl<'a> Future for MigrateDeploy<'a> {
    type Output = Result<(), MigrateDeployError>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.fut.is_none() {
            let datamodel = self.datamodel.to_string();
            let url = self.url.to_string();
            let migrations = self.migrations;
            let temp_dir = self.temp_dir.clone();

            self.fut = Some(Box::pin(async move {
                let temp_dir = match temp_dir {
                    Some(d) => d.to_string(),
                    None => tempfile::Builder::new()
                        .prefix("prisma-client-rust-migrations")
                        .tempdir()
                        .map_err(MigrateDeployError::CreateDir)?
                        .into_path()
                        .to_str()
                        .unwrap()
                        .to_string(),
                };

                migrations
                    .extract(&temp_dir)
                    .map_err(MigrateDeployError::ExtractMigrations)?;

                let engine_state = EngineState::new(Some(datamodel.to_string()), None);

                let input = ApplyMigrationsInput {
                    migrations_directory_path: temp_dir.to_string(),
                };

                let output = engine_state
                    .with_connector_for_url(
                        url.to_string(),
                        Box::new(|connector| {
                            Box::pin(commands::apply_migrations(input, connector, None))
                        }),
                    )
                    .await;

                remove_dir_all(&temp_dir)
                    .await
                    .map_err(MigrateDeployError::RemoveDir)?;

                for migration in output?.applied_migration_names {
                    tracing::debug!("Applied migration '{}'", migration);
                }

                // apparently migrate deploy needs some time
                tokio::time::sleep(core::time::Duration::from_millis(1)).await;

                Ok(())
            }));
        }

        self.fut.as_mut().unwrap().as_mut().poll(cx)
    }
}

#[derive(Error, Debug)]
pub enum MigrateResolveError {
    #[error("The temporary file path for the database migrations is invalid.")]
    InvalidDirectory,
    #[error("An error occurred creating the temporary directory for the migrations: {0}")]
    CreateDir(std::io::Error),
    #[error("An error occurred extracting the migrations to the temporary directory: {0}")]
    ExtractMigrations(std::io::Error),
    #[error("An error occurred running the migrations: {0}")]
    Connector(#[from] CoreError),
    #[error("An error occurred removing the temporary directory for the migrations: {0}")]
    RemoveDir(std::io::Error),
}

pub async fn migrate_resolve(
    migration: &str,
    datamodel: &str,
    migrations: &include_dir::Dir<'_>,
    url: &str,
) -> Result<(), MigrateResolveError> {
    let temp_dir = tempfile::Builder::new()
        .prefix("prisma-client-rust-migrations")
        .tempdir()
        .map_err(MigrateResolveError::CreateDir)?
        .into_path();

    let temp_dir_str = match temp_dir.to_str() {
        Some(p) => p.to_string(),
        None => {
            remove_dir_all(&temp_dir)
                .await
                .map_err(MigrateResolveError::RemoveDir)?;

            return Err(MigrateResolveError::InvalidDirectory);
        }
    };

    migrations
        .extract(&temp_dir)
        .map_err(MigrateResolveError::ExtractMigrations)?;

    let engine_state = EngineState::new(Some(datamodel.to_string()), None);

    let input = MarkMigrationAppliedInput {
        migration_name: migration.to_string(),
        migrations_directory_path: temp_dir_str.to_string(),
    };

    engine_state
        .with_connector_for_url(
            url.to_string(),
            Box::new(move |connector| Box::pin(commands::mark_migration_applied(input, connector))),
        )
        .await?;

    Ok(())
}
