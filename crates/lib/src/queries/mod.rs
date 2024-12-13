mod batch;
mod count;
mod create;
mod create_many;
mod create_unchecked;
mod delete;
mod delete_many;
mod error;
mod execute_raw;
mod find_first;
mod find_many;
mod find_unique;
mod include;
mod mongo_raw;
mod query;
mod query_raw;
mod select;
mod update;
mod update_many;
mod update_unchecked;
mod upsert;

pub use batch::*;
pub use count::*;
pub use create::*;
pub use create_many::*;
pub use create_unchecked::*;
pub use delete::*;
pub use delete_many::*;
pub use error::*;
pub use execute_raw::*;
pub use find_first::*;
pub use find_many::*;
pub use find_unique::*;
pub use include::*;
pub use mongo_raw::*;
pub use query::*;
pub use query_raw::*;
pub use select::*;
pub use update::*;
pub use update_many::*;
pub use update_unchecked::*;
pub use upsert::*;

use futures::FutureExt;
pub use query_core::{schema::QuerySchemaRef, Operation, Selection};
use serde::de::IntoDeserializer;
use serde::Deserialize;
use std::future::Future;

use crate::{ExecutionEngine, PrismaValue};

pub enum SerializedWhereValue {
    Object(Vec<(String, PrismaValue)>),
    List(Vec<PrismaValue>),
    Value(PrismaValue),
}

impl Into<PrismaValue> for SerializedWhereValue {
    fn into(self) -> PrismaValue {
        match self {
            Self::Object(v) => PrismaValue::Object(v),
            Self::List(v) => PrismaValue::List(v),
            Self::Value(v) => v,
        }
    }
}

pub struct SerializedWhereInput {
    field: String,
    value: SerializedWhereValue,
}

impl SerializedWhereInput {
    pub fn new(field: String, value: SerializedWhereValue) -> Self {
        Self {
            field,
            value: value.into(),
        }
    }

    /// If the parameter is an 'equals' parameter, collapses the value provided directly
    /// into the where clause. This is necessary for unique queries that have no filters,
    /// only direct value comparisons.
    pub fn transform_equals(self) -> (String, PrismaValue) {
        let Self { field, value } = self;

        (
            field.to_string(),
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
                SerializedWhereValue::Value(v) => v,
            },
        )
    }
}

impl Into<(String, PrismaValue)> for SerializedWhereInput {
    fn into(self) -> (String, PrismaValue) {
        let SerializedWhereInput { field, value } = self;
        (field, value.into())
    }
}

pub fn exec<'a, Q: Query<'a> + 'a>(
    query: Q,
) -> impl Future<Output = Result<<Q as QueryConvert>::ReturnValue>> + 'a {
    let (op, client) = query.graphql();

    client.execute(op).map(|value| {
        let value = value?;

        Ok(match client.engine {
            ExecutionEngine::Real { .. } => Q::RawType::deserialize(value.into_deserializer())
                .map_err(|e| e.to_string())
                .map_err(QueryError::Deserialize)
                .and_then(Q::convert)?,
            #[cfg(feature = "mocking")]
            ExecutionEngine::Mock(_) => Q::ReturnValue::deserialize(value.into_deserializer())
                .map_err(|e| e.to_string())
                .map_err(QueryError::Deserialize)?,
        })
    })
}
