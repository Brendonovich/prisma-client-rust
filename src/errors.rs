use datamodel::datamodel_connector::Diagnostics;
use query_core::CoreError;
use thiserror::Error;
use user_facing_errors::UserFacingError;

trait DiagnosticsToString {
    fn to_string(&self) -> String;
}

impl DiagnosticsToString for Diagnostics {
    fn to_string(&self) -> String {
        let strs: Vec<_> = self.errors().iter().map(|e| e.message()).collect();
        strs.join("\n")
    }
}

#[derive(Debug, Error)]
pub enum NewClientError {
    #[error("Error configuring database connection: {}", .0.to_string())]
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

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("Error executing query: {} - {}", .0.as_known().map(|k| k.error_code.to_string()).unwrap_or("Unknown".to_string()), .0.message())]
    Execute(user_facing_errors::Error),

    #[error("Error serializing query result: {0}")]
    Serialize(#[from] serde_value::SerializerError),

    #[error("Error deserializing query result into return type: {0}")]
    Deserialize(#[from] serde_value::DeserializerError),
}

impl QueryError {
    pub fn is_prisma_error<T: UserFacingError>(&self) -> bool {
        match self {
            Self::Execute(error) => error
                .as_known()
                .map(|e| e.error_code == <T as UserFacingError>::ERROR_CODE)
                .unwrap_or(false),
            _ => false,
        }
    }
}

pub type Result<T> = std::result::Result<T, QueryError>;

#[cfg(feature = "rspc")]
impl From<QueryError> for rspc::Error {
    fn from(err: QueryError) -> Self {
        rspc::Error::with_cause(
            rspc::ErrorCode::InternalServerError,
            "Internal server error occurred while completing database operation!".into(),
            err,
        )
    }
}
