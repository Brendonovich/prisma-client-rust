pub mod query;

pub use query_core;
pub use datamodel;
pub use prisma_models;
pub use request_handlers;
pub use chrono;
pub use serde_json;

pub type Executor = Box<dyn query_core::QueryExecutor + Send + Sync + 'static>;

#[derive(serde::Deserialize)]
pub struct DeleteResult {
    pub count: isize,
}