use std::sync::Arc;

use query_core::{Operation, Selection};

use serde::Serialize;
use serde_value::Value;
use tokio::sync::Mutex;

use crate::Query;

#[derive(Default, Clone)]
pub struct MockStore {
    read: Arc<Mutex<Vec<(Selection, Value)>>>,
    write: Arc<Mutex<Vec<(Selection, Value)>>>,
}

impl MockStore {
    pub fn new() -> Self {
        Default::default()
    }

    // monomorphization optimisation moment
    async fn add_op(&self, op: Operation, expected: Value) {
        let (sel, mutex) = match op {
            Operation::Read(sel) => (sel, &self.read),
            Operation::Write(sel) => (sel, &self.write),
        };

        mutex.lock().await.push((sel, expected))
    }
    pub async fn expect<'a, Q: Query<'a>>(&self, query: Q, expected: Q::ReturnValue)
    where
        Q::ReturnValue: Serialize,
    {
        self.add_op(query.graphql().0, serde_value::to_value(expected).unwrap())
            .await;
    }

    pub(crate) async fn get_op(&self, op: &Operation) -> Option<Value> {
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
