use std::future::Future;

use crate::{PrismaClient, PrismaClientInternals, QueryError};

pub struct TransactionBuilder<'a, TClient> {
    client: TClient,
    internals: &'a PrismaClientInternals,
    timeout: u64,
    max_wait: u64,
    isolation_level: Option<String>,
}

impl<'a, TClient> TransactionBuilder<'a, TClient>
where
    TClient: PrismaClient,
{
    pub fn _new(client: TClient, internals: &'a PrismaClientInternals) -> Self {
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

    pub async fn run<TErr, TRet, TFut, TFn>(mut self, tx: TFn) -> Result<TRet, TErr>
    where
        TFut: Future<Output = Result<TRet, TErr>>,
        TFn: Fn(TClient) -> TFut,
        TErr: From<crate::QueryError>,
    {
        let tx_id = self
            .internals
            .executor
            .start_tx(
                self.internals.query_schema.clone(),
                self.max_wait,
                self.timeout,
                self.isolation_level,
            )
            .await
            .map_err(|e| QueryError::Execute(e.into()))?;

        self.client.internals_mut().tx_id = Some(tx_id.clone());

        match tx(self.client).await {
            result @ Ok(_) => {
                self.internals
                    .executor
                    .commit_tx(tx_id)
                    .await
                    .map_err(|e| QueryError::Execute(e.into()))?;

                result
            }
            err @ Err(_) => {
                self.internals.executor.rollback_tx(tx_id).await.ok();

                err
            }
        }
    }
}

pub trait TransactionIsolationLevel: ToString {}
