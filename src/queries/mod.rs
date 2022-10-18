pub mod batch;
pub mod count;
pub mod create;
pub mod create_many;
pub mod delete;
pub mod delete_many;
pub mod execute_raw;
pub mod find_first;
pub mod find_many;
pub mod find_unique;
pub mod include;
pub mod query_raw;
pub mod select;
pub mod update;
pub mod update_many;
pub mod upsert;

pub use batch::*;
pub use count::*;
pub use create::*;
pub use create_many::*;
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

pub use query_core::{schema::QuerySchemaRef, Operation, Selection};
use serde::de::{DeserializeOwned, IntoDeserializer};
use thiserror::Error;
use user_facing_errors::UserFacingError;

use crate::{prisma_value, Executor};

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

pub type OperationCallback = Box<dyn Fn(&Operation)>;

#[derive()]
pub struct QueryContext<'a> {
    pub executor: &'a Executor,
    pub schema: &'a QuerySchemaRef,
    pub operation_callbacks: &'a [OperationCallback],
}

impl<'a> QueryContext<'a> {
    pub fn new(
        executor: &'a Executor,
        schema: &'a QuerySchemaRef,
        operation_callbacks: &'a [OperationCallback],
    ) -> Self {
        Self {
            executor,
            schema,
            operation_callbacks,
        }
    }

    pub async fn execute<T: DeserializeOwned>(self, operation: Operation) -> Result<T> {
        // reduce monomorphization a lil bit
        async fn inner<'a>(ctx: QueryContext<'a>, op: Operation) -> Result<serde_value::Value> {
            for callback in ctx.operation_callbacks {
                (callback)(&op);
            }

            let response = ctx
                .executor
                .execute(None, op, ctx.schema.clone(), None)
                .await
                .map_err(|e| QueryError::Execute(e.into()))?;

            let data: prisma_value::Item = response.data.into();

            Ok(serde_value::to_value(data)?)
        }

        let value = inner(self, operation).await?;
        // let value = dbg!(value);

        let ret = T::deserialize(value.into_deserializer())?;

        Ok(ret)
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
