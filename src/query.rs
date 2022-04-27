use query_core::{Operation, QuerySchemaRef};
use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;

use crate::Executor;

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
            let data = ctx.executor.execute(None, op, ctx.schema, None).await?;

            let ret = serde_json::to_value(data.data)?;

            Ok(ret)
        }
        
        dbg!(&operation);

        let value = inner(self, operation).await?;
        let ret = serde_json::from_value(value)?;

        Ok(ret)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error executing query: {0}")]
    Execute(#[from] query_core::CoreError),

    #[error("Error parsing query result: {0}")]
    Parse(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
