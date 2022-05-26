pub mod operator;
pub mod queries;
pub mod raw;
pub mod serde;
pub mod traits;

pub use bigdecimal;
pub use chrono;
pub use datamodel;
use prisma_errors::UserFacingError;
pub use prisma_models::{self, PrismaValue};
pub use queries::*;
pub use query_core;
pub use serde_json;
pub use user_facing_errors as prisma_errors;

use ::serde::{Deserialize, Serialize};
use datamodel::datamodel_connector::Diagnostics;
use query_core::{CoreError, Selection};
use thiserror::Error;

pub type Executor = Box<dyn query_core::QueryExecutor + Send + Sync + 'static>;

#[derive(Deserialize)]
pub struct BatchResult {
    pub count: i64,
}

impl BatchResult {
    pub fn selection() -> Selection {
        let selection = Selection::builder("count");
        selection.build()
    }
}

#[derive(Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Asc => "asc".to_string(),
            Direction::Desc => "desc".to_string(),
        }
    }
}

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

#[macro_export]
macro_rules! not {
    ($($x:expr),+ $(,)?) => {
        $crate::operator::not(vec![$($x),+])
    };
}

#[macro_export]
macro_rules! and {
    ($($x:expr),+ $(,)?) => {
        $crate::operator::and(vec![$($x),+])
    };
}

#[macro_export]
macro_rules! or {
    ($($x:expr),+ $(,)?) => {
        $crate::operator::or(vec![$($x),+])
    };
}
