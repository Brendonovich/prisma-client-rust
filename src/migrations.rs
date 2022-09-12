pub use enumflags2::BitFlags;
pub use include_dir;
use include_dir::Dir;
pub use migration_core::migration_connector::ConnectorParams;
use migration_core::{
    commands::apply_migrations,
    json_rpc::types::{ApplyMigrationsInput, ApplyMigrationsOutput},
    migration_connector::{ConnectorError, MigrationConnector},
};
pub use mongodb_migration_connector::MongoDbMigrationConnector;
pub use quaint::connector::ConnectionInfo;
pub use sql_migration_connector::SqlMigrationConnector;
use tempdir::TempDir;
use thiserror::Error;
use tokio::fs::remove_dir_all;
use tracing::debug;

#[derive(Error, Debug)]
pub enum MigrateError {
    #[error("The temporary file path for the database migrations is invalid.")]
    InvalidDirectory,
    #[error("An error occurred creating the temporary directory for the migrations: {0}")]
    CreateDir(std::io::Error),
    #[error("An error occurred extracting the migrations to the temporary directory: {0}")]
    ExtractMigrations(std::io::Error),
    #[error("An error occurred creating the database connection for migrations: {0}")]
    Quaint(#[from] quaint::error::Error),
    #[error("An error occurred running the migrations: {0}")]
    Connector(#[from] ConnectorError),
    #[error("An error occurred removing the temporary directory for the migrations: {0}")]
    RemoveDir(std::io::Error),
}

pub async fn extract_and_run_migrations(
    migrations_dir: &Dir<'_>,
    connector: &mut dyn MigrationConnector,
) -> Result<(), MigrateError> {
    let temp_dir = TempDir::new("prisma-client-rust-migrations")
        .map_err(MigrateError::CreateDir)?
        .into_path();

    let temp_dir_str = match temp_dir.to_str() {
        Some(path) => path,
        None => {
            remove_dir_all(temp_dir)
                .await
                .map_err(MigrateError::RemoveDir)?;
            return Err(MigrateError::InvalidDirectory)?;
        }
    };

    migrations_dir
        .extract(&temp_dir)
        .map_err(MigrateError::ExtractMigrations)?;

    let output = run_migrations(temp_dir_str.to_string(), connector).await?;

    remove_dir_all(temp_dir)
        .await
        .map_err(MigrateError::RemoveDir)?;

    for migration in output.applied_migration_names {
        debug!("Applied migration '{}'", migration);
    }

    Ok(())
}

pub async fn run_migrations(
    migrations_directory_path: String,
    connector: &mut dyn MigrationConnector,
) -> Result<ApplyMigrationsOutput, MigrateError> {
    let x = apply_migrations(
        ApplyMigrationsInput {
            migrations_directory_path,
        },
        connector,
    )
    .await?;

    Ok(x)
}
