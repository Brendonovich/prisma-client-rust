pub mod count;
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

pub use count::*;
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

use query_core::{Operation, QuerySchemaRef, Selection};
use serde::de::{DeserializeOwned, IntoDeserializer};
use thiserror::Error;
use user_facing_errors::query_engine::RecordRequiredButNotFound;

use crate::{error_is_type, prisma_value, Executor};

pub enum SerializedWhereValue {
    Object(Vec<(String, prisma_models::PrismaValue)>),
    List(Vec<prisma_models::PrismaValue>),
}

impl Into<prisma_models::PrismaValue> for SerializedWhereValue {
    fn into(self) -> prisma_models::PrismaValue {
        match self {
            Self::Object(v) => prisma_models::PrismaValue::Object(v),
            Self::List(v) => prisma_models::PrismaValue::List(v),
        }
    }
}

pub struct SerializedWhere {
    field: String,
    value: SerializedWhereValue,
}

impl SerializedWhere {
    pub fn new(field: &str, value: SerializedWhereValue) -> Self {
        Self {
            field: field.into(),
            value: value.into(),
        }
    }

    /// If the parameter is an 'equals' parameter, collapses the value provided directly
    /// into the where clause. This is necessary for unique queries that have no filters,
    /// only direct value comparisons.
    pub fn transform_equals(self) -> (String, prisma_models::PrismaValue) {
        let Self { field, value } = self;

        (
            field,
            match value {
                SerializedWhereValue::Object(mut params) => match params
                    .iter()
                    .position(|(key, _)| key == "equals")
                    .map(|i| params.swap_remove(i))
                {
                    Some((_, value)) => value,
                    None => prisma_models::PrismaValue::Object(params),
                },
                SerializedWhereValue::List(values) => prisma_models::PrismaValue::List(values),
            },
        )
    }
}

impl Into<(String, prisma_models::PrismaValue)> for SerializedWhere {
    fn into(self) -> (String, prisma_models::PrismaValue) {
        let SerializedWhere { field, value } = self;
        (field, value.into())
    }
}

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
        async fn inner<'a>(ctx: QueryContext<'a>, op: Operation) -> Result<serde_value::Value> {
            let response = ctx
                .executor
                .execute(None, op, ctx.schema, None)
                .await
                .map_err(|e| Error::Execute(e.into()))?;

            let data: prisma_value::Item = response.data.into();

            Ok(serde_value::to_value(data)?)
        }

        let value = inner(self, operation).await?;
        let ret = T::deserialize(value.into_deserializer())?;

        Ok(ret)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error executing query: {} - {}", .0.as_known().map(|k| k.error_code.to_string()).unwrap_or("Unknown".to_string()), .0.message())]
    Execute(user_facing_errors::Error),

    #[error("Error serializing query result: {0}")]
    Serialize(#[from] serde_value::SerializerError),

    #[error("Error deserializing query result into return type: {0}")]
    Deserialize(#[from] serde_value::DeserializerError),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn option_on_not_found<T>(res: Result<T>) -> Result<Option<T>> {
    match res {
        Err(Error::Execute(err)) if error_is_type::<RecordRequiredButNotFound>(&err) => Ok(None),
        res => res.map(Some),
    }
}
