mod errors;
pub mod operator;
mod prisma_value;
pub mod queries;
pub mod raw;
pub mod serde;
pub mod traits;

pub use bigdecimal;
pub use chrono;
pub use datamodel;
pub use prisma_models::{self, PrismaValue};
pub use query_core;
pub use serde_json;
pub use user_facing_errors as prisma_errors;

pub use errors::*;
pub use queries::*;

use ::serde::{Deserialize, Serialize};
use query_core::Selection;

pub type Executor = Box<dyn query_core::QueryExecutor + Send + Sync + 'static>;

/// The return type of `findMany` queries.
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

/// Direction that a query's results should be ordered by.
///
/// Only needs to be used in the `order` function of fields.
#[derive(Serialize, Deserialize, Clone)]
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
