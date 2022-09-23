pub use include_dir;
pub use migration_core::CoreError;
use migration_core::{
    commands,
    json_rpc::types::{ApplyMigrationsInput, SchemaPushInput, MarkMigrationAppliedInput},
    state::EngineState,
    GenericApi,
};
use thiserror::Error;
use tokio::fs::remove_dir_all;

fn format_error_array(arr: &[String]) -> String {
    arr.join("\n")
}

#[derive(Error, Debug)]
pub enum DbPushError {
    #[error("Failed to reset database: ${0}")]
    ResetFailed(migration_core::CoreError),
    #[error("Some changes could not be executed:\n {}", format_error_array(&.0))]
    UnexecutableChanges(Vec<String>),
    #[error("Data loss may occur:\n {}", format_error_array(&.0))]
    PossibleDataLoss(Vec<String>),
    #[error("An error occured pushing schema to the database: ${0}")]
    Other(#[from] migration_core::CoreError),
}

pub async fn db_push(datamodel: &str, url: &str, force_reset: bool) -> Result<u32, DbPushError> {
    let engine_state = EngineState::new(Some(datamodel.to_string()), None);

    if force_reset {
        engine_state
            .reset()
            .await
            .map_err(DbPushError::ResetFailed)?;
    }

    let input = SchemaPushInput {
        force: force_reset,
        schema: datamodel.to_string(),
    };

    let output = engine_state
        .with_connector_for_url(
            url.to_string(),
            Box::new(|connector| Box::pin(commands::schema_push(input, connector))),
        )
        .await?;

    if output.unexecutable.len() > 0 {
        return Err(DbPushError::UnexecutableChanges(output.unexecutable));
    }

    if output.warnings.len() > 0 {
        return Err(DbPushError::PossibleDataLoss(output.warnings));
    }

    Ok(output.executed_steps)
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
    Connector(#[from] migration_core::CoreError),
    #[error("An error occurred removing the temporary directory for the migrations: {0}")]
    RemoveDir(std::io::Error),
}

pub async fn migrate_deploy(
    datamodel: &str,
    migrations: &include_dir::Dir<'_>,
    url: &str,
) -> Result<(), MigrateDeployError> {
    let temp_dir = tempdir::TempDir::new("prisma-client-rust-migrations")
        .map_err(MigrateDeployError::CreateDir)?
        .into_path();

    let temp_dir_str = match temp_dir.to_str() {
        Some(p) => p.to_string(),
        None => {
            remove_dir_all(&temp_dir)
                .await
                .map_err(MigrateDeployError::RemoveDir)?;

            return Err(MigrateDeployError::InvalidDirectory);
        }
    };

    migrations
        .extract(&temp_dir)
        .map_err(MigrateDeployError::ExtractMigrations)?;

    let engine_state = EngineState::new(Some(datamodel.to_string()), None);

    let input = ApplyMigrationsInput {
        migrations_directory_path: temp_dir_str.to_string(),
    };

    let output = engine_state
        .with_connector_for_url(
            url.to_string(),
            Box::new(|connector| Box::pin(commands::apply_migrations(input, connector))),
        )
        .await;

    remove_dir_all(&temp_dir)
        .await
        .map_err(MigrateDeployError::RemoveDir)?;

    for migration in output?.applied_migration_names {
        tracing::debug!("Applied migration '{}'", migration);
    }

    Ok(())
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
    Connector(#[from] migration_core::CoreError),
    #[error("An error occurred removing the temporary directory for the migrations: {0}")]
    RemoveDir(std::io::Error),
}

pub async fn migrate_resolve(
    migration: &str,
    datamodel: &str,
    migrations: &include_dir::Dir<'_>,
    url: &str,
) -> Result<(), MigrateResolveError> {
    let temp_dir = tempdir::TempDir::new("prisma-client-rust-migrations")
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
