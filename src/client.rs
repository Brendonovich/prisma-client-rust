use std::sync::Arc;

use diagnostics::Diagnostics;
use query_core::{schema_builder, CoreError, Operation};
use schema::QuerySchema;
use serde::de::{DeserializeOwned, IntoDeserializer};
use thiserror::Error;

use crate::{
    prisma_value, ActionNotifier, MockStore, ModelAction, ModelActionType, ModelActions,
    ModelMutationCallbackData, QueryError, Result,
};

pub type Executor = Box<dyn query_core::QueryExecutor + Send + Sync + 'static>;

pub(crate) enum ExecutionEngine {
    Real {
        executor: Executor,
        query_schema: Arc<QuerySchema>,
        url: String,
    },
    Mock(MockStore),
}

impl ExecutionEngine {
    async fn execute(&self, op: Operation) -> Result<serde_value::Value> {
        match self {
            Self::Real {
                executor,
                query_schema,
                ..
            } => {
                let response = executor
                    .execute(None, op, query_schema.clone(), None)
                    .await
                    .map_err(|e| QueryError::Execute(e.into()))?;

                let data: prisma_value::Item = response.data.into();

                Ok(serde_value::to_value(data)?)
            }
            Self::Mock(store) => Ok(store.get_op(&op).await.expect("Mock data not found")),
        }
    }

    pub async fn execute_all(
        &self,
        ops: Vec<Operation>,
    ) -> Result<Vec<Result<serde_value::Value>>> {
        match self {
            Self::Real {
                executor,
                query_schema,
                ..
            } => {
                let response = executor
                    .execute_all(None, ops, None, query_schema.clone(), None)
                    .await
                    .map_err(|e| QueryError::Execute(e.into()))?;

                Ok(response
                    .into_iter()
                    .map(|result| {
                        let data: prisma_value::Item = result
                            .map_err(|e| QueryError::Execute(e.into()))?
                            .data
                            .into();

                        Ok(serde_value::to_value(data)?)
                    })
                    .collect())
            }
            Self::Mock(store) => {
                let mut ret = vec![];

                for op in ops {
                    ret.push(Ok(store.get_op(&op).await.expect("Mock data not found")))
                }

                Ok(ret)
            }
        }
    }
}

/// The data held by the generated PrismaClient
/// Do not use this in your own code!
pub struct PrismaClientInternals {
    pub(crate) engine: ExecutionEngine,
    pub action_notifier: crate::ActionNotifier,
}

impl PrismaClientInternals {
    pub async fn execute<T: DeserializeOwned>(&self, operation: Operation) -> Result<T> {
        // less monomorphization yay
        let value = self.engine.execute(operation).await?;

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

    pub async fn new(
        url: Option<String>,
        action_notifier: ActionNotifier,
        datamodel: &str,
    ) -> std::result::Result<Self, NewClientError> {
        let schema = Arc::new(psl::validate(datamodel.into()));
        let config = &schema.configuration;

        let source = config
            .datasources
            .first()
            .expect("Please supply a datasource in your schema.prisma file");

        let url = match url {
            Some(url) => url,
            None => {
                let url = if let Some(url) = source.load_shadow_database_url()? {
                    url
                } else {
                    source.load_url(|key| std::env::var(key).ok())?
                };
                match url.starts_with("file:") {
                    true => {
                        let path = url.split(":").nth(1).unwrap();
                        if std::path::Path::new("./prisma/schema.prisma").exists() {
                            format!("file:./prisma/{}", path)
                        } else {
                            url
                        }
                    }
                    _ => url,
                }
            }
        };

        let (db_name, executor) =
            query_core::executor::load(&source, config.preview_features(), &url).await?;

        let query_schema = Arc::new(schema_builder::build(
            prisma_models::convert(schema.clone(), db_name),
            true,
        ));

        executor.primary_connector().get_connection().await?;

        Ok(Self {
            engine: ExecutionEngine::Real {
                executor,
                query_schema,
                url,
            },
            action_notifier,
        })
    }

    pub async fn new_mock(action_notifier: ActionNotifier) -> (Self, MockStore) {
        let mock_store = MockStore::new();

        (
            Self {
                engine: ExecutionEngine::Mock(mock_store.clone()),
                action_notifier,
            },
            mock_store,
        )
    }

    pub fn url(&self) -> &str {
        match &self.engine {
            ExecutionEngine::Mock(_) => "mock",
            ExecutionEngine::Real { url, .. } => url,
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
