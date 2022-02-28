mod lifecycle;
mod port;
mod protocol;
use async_trait::async_trait;
pub use protocol::*;
use std::{fmt::Debug, process::Child};

#[derive(Debug)]
pub enum QueryEngineState {
    NotRunning,
    Running { url: String, child: Child },
}

#[derive(Debug)]
pub struct QueryEngine {
    http: reqwest::Client,
    schema: String,
    has_binary_targets: bool,
    state: QueryEngineState,
}

#[async_trait]
pub trait Engine: Debug {
    async fn connect(&mut self);
    fn disconnect(&mut self);
    async fn perform(&self, request: GQLRequest) -> GQLResponse;
    fn batch(&mut self);
    fn name(&self) -> String;
}
