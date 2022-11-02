use std::sync::Arc;

use datamodel::datamodel_connector::Diagnostics;
use query_core::{CoreError, Operation, TxId};
use schema::QuerySchema;
use serde::de::{DeserializeOwned, IntoDeserializer};
use thiserror::Error;

use crate::{
    prisma_value, ModelAction, ModelActionType, ModelActions, ModelMutationCallbackData,
    QueryError, Result,
};

pub type Executor = Arc<dyn query_core::QueryExecutor + Send + Sync + 'static>;

pub trait PrismaClient {
    fn internals(&self) -> &PrismaClientInternals;
    fn internals_mut(&mut self) -> &mut PrismaClientInternals;
}

/// The data held by the generated PrismaClient
/// Do not use this in your own code!
#[derive(Clone)]
pub struct PrismaClientInternals {
    pub executor: Executor,
    pub query_schema: Arc<QuerySchema>,
    pub url: String,
    pub action_notifier: Arc<crate::ActionNotifier>,
    pub tx_id: Option<TxId>,
}

impl PrismaClientInternals {
    // reduce monomorphization a lil bit
    async fn execute_inner<'a>(&self, op: Operation) -> Result<serde_value::Value> {
        let response = self
            .executor
            .execute(self.tx_id.clone(), op, self.query_schema.clone(), None)
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

    pub fn notify_model_mutation<Action>(&self)
    where
        Action: ModelAction,
    {
        match Action::TYPE {
            ModelActionType::Mutation(action) => {
                for callback in &self.action_notifier.model_mutation_callbacks {
                    (callback)(ModelMutationCallbackData {
                        model: Action::Actions::MODEL,
                        action,
                    })
                }
            }
            ModelActionType::Query(_) => {
                println!("notify_model_mutation only acceps mutations, not queries!")
            }
        }
    }
}

trait DiagnosticsToString {
    fn to_string(&self) -> String;
}

impl DiagnosticsToString for Diagnostics {
    fn to_string(&self) -> String {
        let strs: Vec<_> = self.errors().iter().map(|e| e.message()).collect();
        strs.join("\n")
    }
}

#[derive(Debug, Error)]
pub enum NewClientError {
    #[error("Error configuring database connection: {}", .0.to_string())]
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
