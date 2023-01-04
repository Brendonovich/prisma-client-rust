use std::future::Future;

use crate::{ExecutionEngine, PrismaClient, PrismaClientInternals, QueryError};

pub struct TransactionBuilder<'a, TClient> {
    client: &'a TClient,
    internals: &'a PrismaClientInternals,
    timeout: u64,
    max_wait: u64,
    isolation_level: Option<String>,
}

impl<'a, TClient> TransactionBuilder<'a, TClient>
where
    TClient: PrismaClient,
{
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
                        self.max_wait,
                        self.timeout,
                        self.isolation_level,
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
}

pub trait TransactionIsolationLevel: ToString {}
