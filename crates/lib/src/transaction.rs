use std::{future::Future, marker::PhantomData};

use query_core::{protocol::EngineProtocol, TransactionOptions, TxId};

use crate::{ExecutionEngine, PrismaClient, PrismaClientInternals, QueryError};

pub struct TransactionBuilder<'a, TClient> {
    client: &'a TClient,
    internals: &'a PrismaClientInternals,
    timeout: u64,
    max_wait: u64,
    isolation_level: Option<String>,
}

impl<'a, TClient: PrismaClient> TransactionBuilder<'a, TClient> {
    pub fn _new(client: &'a TClient, internals: &'a PrismaClientInternals) -> Self {
        Self {
            client,
            internals,
            timeout: 5000,
            max_wait: 2000,
            isolation_level: None,
        }
    }

    pub fn with_timeout(self, timeout: u64) -> Self {
        Self { timeout, ..self }
    }

    pub fn with_max_wait(self, max_wait: u64) -> Self {
        Self { max_wait, ..self }
    }

    pub fn with_isolation_level(self, isolation_level: impl TransactionIsolationLevel) -> Self {
        Self {
            isolation_level: Some(isolation_level.to_string()),
            ..self
        }
    }

    pub async fn run<TErr, TRet, TFut, TFn>(self, tx: TFn) -> Result<TRet, TErr>
    where
        TFut: Future<Output = Result<TRet, TErr>>,
        TFn: FnOnce(TClient) -> TFut,
        TErr: From<crate::QueryError>,
    {
        match &self.internals.engine {
            ExecutionEngine::Real { connector, .. } => {
                let new_tx_id = connector
                    .executor
                    .start_tx(
                        connector.query_schema.clone(),
                        EngineProtocol::Graphql,
                        TransactionOptions::new(self.max_wait, self.timeout, self.isolation_level),
                    )
                    .await
                    .map_err(|e| QueryError::Execute(e.into()))?;

                match tx(self.client.with_tx_id(Some(new_tx_id.clone()))).await {
                    result @ Ok(_) => {
                        connector
                            .executor
                            .commit_tx(new_tx_id)
                            .await
                            .map_err(|e| QueryError::Execute(e.into()))?;

                        result
                    }
                    err @ Err(_) => {
                        connector.executor.rollback_tx(new_tx_id).await.ok();

                        err
                    }
                }
            }
            _ => tx(self.client.with_tx_id(None)).await,
        }
    }

    pub async fn begin(self) -> super::Result<(TransactionController<TClient>, TClient)> {
        Ok(match &self.internals.engine {
            ExecutionEngine::Real { connector, .. } => {
                let new_tx_id = connector
                    .executor
                    .start_tx(
                        connector.query_schema.clone(),
                        EngineProtocol::Graphql,
                        TransactionOptions::new(self.max_wait, self.timeout, self.isolation_level),
                    )
                    .await
                    .map_err(|e| QueryError::Execute(e.into()))?;

                (
                    TransactionController::new(new_tx_id.clone()),
                    self.client.with_tx_id(Some(new_tx_id)),
                )
            }
            _ => (
                TransactionController::new("".to_string().into()),
                self.client.with_tx_id(None),
            ),
        })
    }
}

pub struct TransactionController<TClient> {
    tx_id: TxId,
    _client: PhantomData<TClient>,
}

impl<TClient: PrismaClient> TransactionController<TClient> {
    fn new(tx_id: TxId) -> Self {
        Self {
            tx_id,
            _client: Default::default(),
        }
    }

    pub async fn commit(self, client: TClient) -> super::Result<()> {
        Ok(match &client.internals().engine {
            ExecutionEngine::Real { connector, .. } => connector
                .executor
                .commit_tx(self.tx_id)
                .await
                .map_err(|e| QueryError::Execute(e.into()))?,
            _ => {}
        })
    }

    pub async fn rollback(self, client: TClient) -> super::Result<()> {
        Ok(match &client.internals().engine {
            ExecutionEngine::Real { connector, .. } => {
                connector.executor.rollback_tx(self.tx_id).await.ok();
            }
            _ => {}
        })
    }
}

pub trait TransactionIsolationLevel: ToString {}
