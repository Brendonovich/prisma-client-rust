pub mod create;
pub mod delete;
pub mod delete_many;
pub mod execute_raw;
pub mod find_first;
pub mod find_many;
pub mod find_unique;
pub mod query_raw;
pub mod update;
pub mod update_many;
pub mod upsert;

pub use create::*;
pub use delete::*;
pub use delete_many::*;
pub use execute_raw::*;
pub use find_first::*;
pub use find_many::*;
pub use find_unique::*;
pub use query_raw::*;
pub use update::*;
pub use update_many::*;
pub use upsert::*;

use prisma_models::PrismaValue;
use query_core::{Operation, QuerySchemaRef, Selection};
use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;
use user_facing_errors::{query_engine::RecordRequiredButNotFound, UserFacingError};

use crate::Executor;

pub enum SerializedWhereValue {
    Object(Vec<(String, PrismaValue)>),
    List(Vec<PrismaValue>),
}

pub type SerializedWhere = (String, SerializedWhereValue);
pub struct QueryInfo {
    model: &'static str,
    scalar_selections: Vec<Selection>,
}

impl QueryInfo {
    pub fn new(model: &'static str, scalar_selections: Vec<Selection>) -> Self {
        Self {
            model,
            scalar_selections,
        }
    }
}

pub struct QueryContext<'a> {
    executor: &'a Executor,
    schema: QuerySchemaRef,
}

impl<'a> QueryContext<'a> {
    pub fn new(executor: &'a Executor, schema: QuerySchemaRef) -> Self {
        Self { executor, schema }
    }

    pub async fn execute<T: DeserializeOwned>(self, operation: Operation) -> Result<T> {
        async fn inner<'a>(ctx: QueryContext<'a>, op: Operation) -> Result<Value> {
            let data = ctx
                .executor
                .execute(None, op, ctx.schema, None)
                .await
                .map_err(Into::<user_facing_errors::Error>::into)
                .map_err(Error::Execute)?;

            let ret = serde_json::to_value(data.data)?;

            Ok(ret)
        }

        let value = inner(self, operation).await?;
        let ret = serde_json::from_value(value)?;

        Ok(ret)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error executing query: {} - {}", .0.as_known().map(|k| k.error_code.to_string()).unwrap_or("Unknown".to_string()), .0.message())]
    Execute(user_facing_errors::Error),

    #[error("Error parsing query result: {0}")]
    Parse(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn transform_equals<T: Into<SerializedWhere>>(
    params: impl Iterator<Item = T>,
) -> Vec<(String, PrismaValue)> {
    params
        .map(Into::<SerializedWhere>::into)
        .map(|(field, value)| {
            (
                field,
                match value {
                    SerializedWhereValue::Object(mut params) => match params
                        .iter()
                        .position(|(key, _)| key == "equals")
                        .map(|i| params.swap_remove(i))
                    {
                        Some((_, value)) => value,
                        None => PrismaValue::Object(params),
                    },
                    SerializedWhereValue::List(values) => PrismaValue::List(values),
                },
            )
        })
        .collect()
}

pub fn option_on_not_found<T>(res: Result<T>) -> Result<Option<T>> {
    match res {
        Err(Error::Execute(err)) if error_is_type::<RecordRequiredButNotFound>(err) => Ok(None),
        res => res.map(Some),
    }
}
