use serde::Serialize;
use thiserror::Error;
use user_facing_errors::UserFacingError;

#[derive(Debug, Error, Serialize)]
pub enum QueryError {
    #[error("Error executing query: {} - {}", .0.as_known().map(|k| k.error_code.to_string()).unwrap_or("Unknown".to_string()), .0.message())]
    Execute(user_facing_errors::Error),

    #[error("Error serializing query result: {0}")]
    Serialize(String),

    #[error("Error deserializing query result into return type: {0}")]
    Deserialize(String),
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
