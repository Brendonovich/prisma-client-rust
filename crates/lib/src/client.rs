use crate::ActionNotifier;
use psl::Diagnostics;
use query_core::{
    protocol::EngineProtocol,
    schema::{self, QuerySchema},
    BatchDocumentTransaction, CoreError, Operation, TxId,
};

use std::sync::Arc;
use thiserror::Error;

use crate::{prisma_value, QueryError, Result};

pub type Executor = Box<dyn query_core::QueryExecutor + Send + Sync + 'static>;

pub trait PrismaClient {
    fn internals(&self) -> &PrismaClientInternals;
    fn internals_mut(&mut self) -> &mut PrismaClientInternals;
    fn with_tx_id(&self, tx_id: Option<TxId>) -> Self;
}

pub struct ExecutorConnector {
    pub executor: Executor,
    pub query_schema: Arc<QuerySchema>,
    pub url: String,
}

#[derive(Clone)]
pub(crate) enum ExecutionEngine {
    Real {
        connector: Arc<ExecutorConnector>,
        tx_id: Option<TxId>,
    },
    #[cfg(feature = "mocking")]
    Mock(crate::MockStore),
}

impl ExecutionEngine {
    async fn execute(&self, op: Operation) -> Result<serde_value::Value> {
        match self {
            Self::Real { connector, tx_id } => {
                let response = connector
                    .executor
                    .execute(
                        tx_id.clone(),
                        op,
                        connector.query_schema.clone(),
                        None,
                        EngineProtocol::Json,
                    )
                    .await
                    .map_err(|e| QueryError::Execute(e.into()))?;

                let data: prisma_value::Item = response.data.into();

                let data = serde_value::to_value(data)
                    .map_err(|e| e.to_string())
                    .map_err(QueryError::Deserialize)?;

                Ok(data)
            }
            #[cfg(feature = "mocking")]
            Self::Mock(store) => Ok(store.get_op(&op).await.expect("Mock data not found")),
        }
    }

    pub async fn execute_all(
        &self,
        ops: Vec<Operation>,
    ) -> Result<Vec<Result<serde_value::Value>>> {
        match self {
            Self::Real { connector, .. } => {
                let response = connector
                    .executor
                    .execute_all(
                        None,
                        ops,
                        Some(BatchDocumentTransaction::new(None)),
                        connector.query_schema.clone(),
                        None,
                        EngineProtocol::Json,
                    )
                    .await
                    .map_err(|e| QueryError::Execute(e.into()))?;

                Ok(response
                    .into_iter()
                    .map(|result| {
                        let data: prisma_value::Item = result
                            .map_err(|e| QueryError::Execute(e.into()))?
                            .data
                            .into();

                        Ok(serde_value::to_value(data)
                            .map_err(|e| e.to_string())
                            .map_err(QueryError::Deserialize)?)
                    })
                    .collect())
            }
            #[cfg(feature = "mocking")]
            Self::Mock(store) => {
                let mut ret = vec![];

                for op in ops {
                    ret.push(Ok(store.get_op(&op).await.expect("Mock data not found")))
                }

                Ok(ret)
            }
        }
    }

    fn with_tx_id(&self, tx_id: Option<TxId>) -> Self {
        match self {
            Self::Real { connector, .. } => Self::Real {
                connector: connector.clone(),
                tx_id,
            },
            #[cfg(feature = "mocking")]
            _ => self.clone(),
        }
    }
}

/// The data held by the generated PrismaClient
/// Do not use this in your own code!
#[derive(Clone)]
pub struct PrismaClientInternals {
    pub(crate) engine: ExecutionEngine,
    pub action_notifier: Arc<crate::ActionNotifier>,
}

impl PrismaClientInternals {
    pub(crate) async fn execute(&self, operation: Operation) -> Result<serde_value::Value> {
        self.engine.execute(operation).await
    }

    // pub fn notify_model_mutation<'a, Action>(&self)
    // where
    //     Action: ModelQuery<'a>,
    // {
    //     match Action::TYPE {
    //         ModelOperation::Write(action) => {
    //             for callback in &self.action_notifier.model_mutation_callbacks {
    //                 (callback)(ModelMutationCallbackData {
    //                     model: Action::Types::MODEL,
    //                     action,
    //                 })
    //             }
    //         }
    //         ModelOperation::Read(_) => {
    //             println!("notify_model_mutation only acceps mutations, not queries!")
    //         }
    //     }
    // }

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
                let url = match source.load_url(|key| dotenvy::var(key).ok()) {
                    Ok(url) => Some(url),
                    Err(_) => source.load_shadow_database_url()?,
                }
                .expect("Datasource could not be fetched, please check your schema.prisma file or your environment variables");

                match url.starts_with("file:") {
                    true => {
                        let path = url.split(':').nth(1).unwrap();
                        if std::path::Path::new("./prisma/schema.prisma").exists() {
                            format!("file:./prisma/{path}")
                        } else {
                            url
                        }
                    }
                    _ => url,
                }
            }
        };

        let executor =
            request_handlers::load_executor(source, config.preview_features(), &url).await?;

        executor.primary_connector().get_connection().await?;

        Ok(Self {
            engine: ExecutionEngine::Real {
                connector: Arc::new(ExecutorConnector {
                    executor,
                    query_schema: Arc::new(schema::build(schema.clone(), true)),
                    url,
                }),
                tx_id: None,
            },
            action_notifier: Arc::new(action_notifier),
        })
    }

    #[cfg(feature = "mocking")]
    pub fn new_mock(action_notifier: ActionNotifier) -> (Self, crate::MockStore) {
        let mock_store = crate::MockStore::new();

        (
            Self {
                engine: ExecutionEngine::Mock(mock_store.clone()),
                action_notifier: Arc::new(action_notifier),
            },
            mock_store,
        )
    }

    pub fn url(&self) -> &str {
        match &self.engine {
            #[cfg(feature = "mocking")]
            ExecutionEngine::Mock(_) => "mock",
            ExecutionEngine::Real { connector, .. } => &connector.url,
        }
    }

    pub fn with_tx_id(&self, tx_id: Option<TxId>) -> Self {
        Self {
            engine: self.engine.with_tx_id(tx_id),
            action_notifier: self.action_notifier.clone(),
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
    Connection(#[from] query_core::ConnectorError),
}

impl From<Diagnostics> for NewClientError {
    fn from(diagnostics: Diagnostics) -> Self {
        NewClientError::Configuration(diagnostics)
    }
}
