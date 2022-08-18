use datamodel::datamodel_connector::Diagnostics;
use query_core::CoreError;
use thiserror::Error;

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
