use std::sync::Arc;

use diagnostics::Diagnostics;
use query_core::{CoreError, Operation};
use schema::QuerySchema;
use serde::de::{DeserializeOwned, IntoDeserializer};
use thiserror::Error;

use crate::{
    prisma_value, ActionNotifier, ModelAction, ModelActionType, ModelActions,
    ModelMutationCallbackData, QueryError, Result,
};

pub type Executor = Box<dyn query_core::QueryExecutor + Send + Sync + 'static>;

/// The data held by the generated PrismaClient
/// Do not use this in your own code!
pub struct PrismaClientInternals {
    pub executor: Executor,
    pub query_schema: Arc<QuerySchema>,
    pub url: String,
    pub action_notifier: crate::ActionNotifier,
}

impl PrismaClientInternals {
    // reduce monomorphization a lil bit
    async fn execute_inner<'a>(&self, op: Operation) -> Result<serde_value::Value> {
        let response = self
            .executor
            .execute(None, op, self.query_schema.clone(), None)
            .await
            .map_err(|e| QueryError::Execute(e.into()))?;

        let data: prisma_value::Item = response.data.into();

        Ok(serde_value::to_value(data)?)
    }

    pub async fn execute<T: DeserializeOwned>(&self, operation: Operation) -> Result<T> {
        let value = self.execute_inner(operation).await?;
        // let value = dbg!(value);

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
        let config = datamodel::parse_configuration(datamodel)?.subject;
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
        let (db_name, executor) = query_core::executor::load(
            &source,
            &config.preview_features().iter().collect::<Vec<_>>(),
            &url,
        )
        .await?;
        let internal_model = prisma_models::InternalDataModelBuilder::new(datamodel).build(db_name);
        let query_schema = Arc::new(query_core::schema_builder::build(
            internal_model,
            true,
            source.capabilities(),
            vec![],
            source.referential_integrity(),
        ));
        executor.primary_connector().get_connection().await?;

        Ok(Self {
            executor,
            query_schema,
            url,
            action_notifier,
        })
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
