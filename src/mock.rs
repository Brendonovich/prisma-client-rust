use std::sync::Arc;

use query_core::{Operation, Selection};

use serde::Serialize;
use tokio::sync::Mutex;

use crate::BatchQuery;

#[derive(Default, Clone)]
pub struct MockStore {
    read: Arc<Mutex<Vec<(Selection, serde_value::Value)>>>,
    write: Arc<Mutex<Vec<(Selection, serde_value::Value)>>>,
}

impl MockStore {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn add_op(&self, op: Operation, expected: serde_value::Value) {
        let (sel, mutex) = match op {
            Operation::Read(sel) => (sel, &self.read),
            Operation::Write(sel) => (sel, &self.write),
        };

        mutex.lock().await.push((sel, expected))
    }

    pub async fn expect<Q: BatchQuery>(&self, query: Q, expected: Q::RawType)
    where
        <Q as BatchQuery>::RawType: Serialize,
    {
        self.add_op(query.graphql(), serde_value::to_value(expected).unwrap())
            .await;
    }

    pub async fn get_op(&self, op: &Operation) -> Option<serde_value::Value> {
        let (sel, mutex) = match op {
            Operation::Read(sel) => (sel, &self.read),
            Operation::Write(sel) => (sel, &self.write),
        };

        mutex
            .lock()
            .await
            .iter()
            .find(|(s, _)| s == sel)
            .map(|(_, e)| e.clone())
    }
}
