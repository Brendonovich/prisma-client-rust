use datamodel::datamodel_connector::Diagnostics;
use user_facing_errors::UserFacingError;
use query_core::CoreError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NewClientError {
    #[error("Error configuring database connection: {0}")]
    Configuration(Diagnostics),

    #[error("Error loading database executor: {0}")]
    Executor(#[from] CoreError),

    #[error("Error getting database connection: {0}")]
    Connection(#[from] query_connector::error::ConnectorError),
}

impl From<Diagnostics> for NewClientError {
    fn from(diagnostics: Diagnostics) -> Self {
        NewClientError::Configuration(diagnostics)
    }
}

pub fn error_is_type<T: UserFacingError>(error: &user_facing_errors::Error) -> bool {
    error
        .as_known()
        .map(|e| e.error_code == <T as UserFacingError>::ERROR_CODE)
        .unwrap_or(false)
}
