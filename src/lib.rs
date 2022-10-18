pub mod actions;
mod errors;
#[cfg(feature = "migrations")]
pub mod migrations;
pub mod operator;
mod prisma_value;
pub mod queries;
mod raw;
pub mod serde;
mod traits;

use std::{collections::HashMap, sync::Arc};

pub use bigdecimal;
pub use chrono;
pub use datamodel;
pub use dmmf;
pub use prisma_models::{self, PrismaValue};
pub use query_core;
pub use query_core::Selection;
pub use schema;
use schema::QuerySchema;
pub use serde_json;
use thiserror::Error;
pub use user_facing_errors as prisma_errors;

pub use actions::*;
pub use actions::*;
pub use errors::*;
pub use operator::Operator;
pub use queries::*;
pub use raw::*;
pub use traits::*;

#[cfg(feature = "rspc")]
pub use rspc;

use ::serde::{
    de::{DeserializeOwned, IntoDeserializer},
    Deserialize, Serialize,
};

pub type Executor = Box<dyn query_core::QueryExecutor + Send + Sync + 'static>;

/// The data held by the generated PrismaClient
/// Do not use this in your own code!
pub struct PrismaClientInternals {
    pub executor: Executor,
    pub query_schema: Arc<QuerySchema>,
    pub url: String,
    pub action_notifier: ActionNotifier,
}

impl PrismaClientInternals {
    // reduce monomorphization a lil bit
    async fn execute_inner<'a>(&self, op: Operation) -> Result<serde_value::Value> {
        for callback in &self.action_notifier.operation_callbacks {
            (callback)(&op);
        }

        let response = self
            .executor
            .execute(None, op, self.query_schema.clone(), None)
            .await
            .map_err(|e| QueryError::Execute(e.into()))?;

        let data: prisma_value::Item = response.data.into();

        Ok(serde_value::to_value(data)?)
    }

    pub async fn execute<T: DeserializeOwned>(&self, operation: Operation) -> Result<T> {
        let value = self.execute_inner(operation).await?;
        // let value = dbg!(value);

        let ret = T::deserialize(value.into_deserializer())?;

        Ok(ret)
    }
}

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

#[derive(Error, Debug)]
pub struct RelationNotFetchedError {
    field: &'static str,
}

impl RelationNotFetchedError {
    pub fn new(field: &'static str) -> Self {
        RelationNotFetchedError { field }
    }
}

impl std::fmt::Display for RelationNotFetchedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Attempted to access field '{}' but did not fetch it using the .with() syntax",
            self.field
        )
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

/// Creates a PrismaValue::Object from a list of key-value pairs.
/// If a key has multiple values that are PrismaValue::Objects, they will be merged.
pub fn merged_object(elements: Vec<(String, PrismaValue)>) -> PrismaValue {
    let mut merged = HashMap::new();

    for el in elements {
        match (merged.get_mut(&el.0), el.1) {
            (Some(PrismaValue::Object(existing)), PrismaValue::Object(incoming)) => {
                existing.extend(incoming);
            }
            (None, v) => {
                merged.insert(el.0, v);
            }
            (Some(_), _) => {
                unreachable!("Cannot merge values if both are not objects")
            }
        }
    }

    PrismaValue::Object(merged.into_iter().collect())
}
