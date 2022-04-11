pub mod operator;
pub mod query;
pub mod serde;
pub mod traits;

pub use chrono;
pub use datamodel;
pub use prisma_models;
pub use query_core;
pub use request_handlers;
pub use serde_json;
use datamodel::datamodel_connector::Diagnostics;
use thiserror::Error;
use query_core::CoreError;
use ::serde::{Serialize, Deserialize};

pub type Executor = Box<dyn query_core::QueryExecutor + Send + Sync + 'static>;

#[derive(Deserialize)]
pub struct DeleteResult {
    pub count: isize,
}

#[derive(Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
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
