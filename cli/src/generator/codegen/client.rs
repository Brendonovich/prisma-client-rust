use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};

use crate::generator::Root;

pub fn generate(root: &Root) -> TokenStream {
    let model_actions = root
        .dmmf
        .datamodel
        .models
        .iter()
        .map(|model| {
            let model_name_snake = format_ident!("{}", model.name.to_case(Case::Snake));

            quote! {
                pub fn #model_name_snake(&self) -> #model_name_snake::Actions {
                    #model_name_snake::Actions {
                        client: &self,
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let datamodel = &root.datamodel;

    let database_string = &root.datasources[0].provider;

    quote! {
        #![allow(warnings, unused)]

        use prisma_client_rust::{
            bigdecimal::{self, FromPrimitive},
            chrono,
            datamodel::parse_configuration,
            operator::Operator,
            raw::Raw,
            prisma_models::{InternalDataModelBuilder, PrismaValue},
            queries::{QueryContext, QueryInfo, Result as QueryResult, SerializedWhere, SerializedWhereValue, transform_equals},
            query_core::{
                executor, schema_builder, BuildMode, CoreError, InterpreterError, QueryExecutor,
                QueryGraphBuilderError, QuerySchema, QueryValue, Selection,
            },
            serde::RelationResult,
            serde_json, UniqueArgs, ManyArgs, BatchResult, Direction, NewClientError,
            QueryRaw, ExecuteRaw
        };
        use serde::{Deserialize, Serialize};
        use std::fmt;
        use std::ops::Deref;
        use std::path::Path;
        use std::sync::Arc;

        static DATAMODEL_STR: &'static str = #datamodel;
        static DATABASE_STR: &'static str = #database_string;

        pub struct PrismaClient {
            executor: Box<dyn QueryExecutor + Send + Sync + 'static>,
            query_schema: Arc<QuerySchema>,
        }

        impl fmt::Debug for PrismaClient {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("PrismaClient")
                 .finish()
            }
        }

        pub async fn new_client() -> Result<PrismaClient, NewClientError> {
            let config = parse_configuration(DATAMODEL_STR)?.subject;
            let source = config
                .datasources
                .first()
                .expect("Pleasy supply a datasource in your schema.prisma file");
            let url = if let Some(url) = source.load_shadow_database_url()? {
                url
            } else {
                source.load_url(|key| std::env::var(key).ok())?
            };

            // sqlite fix
            let url = if url.starts_with("file:") {
                let path = url.split(":").nth(1).unwrap();

                if Path::new("./schema.prisma").exists() {
                    url
                } else if Path::new("./prisma/schema.prisma").exists() {
                    format!("file:./prisma/{}", path)
                } else {
                    url
                }
            } else {
                url
            };
            new_client_with_url(&url).await
        }

        // adapted from https://github.com/polytope-labs/prisma-client-rs/blob/0dec2a67081e78b42700f6a62f414236438f84be/codegen/src/prisma.rs.template#L182
        pub async fn new_client_with_url(url: &str) -> Result<PrismaClient, NewClientError> {
            let config = parse_configuration(DATAMODEL_STR)?.subject;
            let source = config
                .datasources
                .first()
                .expect("Pleasy supply a datasource in your schema.prisma file");
            let (db_name, executor) = executor::load(&source, &[], &url).await?;
            let internal_model = InternalDataModelBuilder::new(DATAMODEL_STR).build(db_name);
            let query_schema = Arc::new(schema_builder::build(
                internal_model,
                BuildMode::Modern,
                true,
                source.capabilities(),
                vec![],
                source.referential_integrity(),
            ));
            executor.primary_connector().get_connection().await?;
            Ok(PrismaClient {
                executor,
                query_schema,
            })
        }

        impl PrismaClient {
            pub async fn _query_raw<T: serde::de::DeserializeOwned>(&self, query: Raw) -> QueryResult<Vec<T>> {
                QueryRaw::new(
                   QueryContext::new(&self.executor, self.query_schema.clone()),
                   query,
                   DATABASE_STR
                ).exec().await
            }

            pub async fn _execute_raw(&self, query: Raw) -> QueryResult<i64> {
                ExecuteRaw::new(
                   QueryContext::new(&self.executor, self.query_schema.clone()),
                   query,
                   DATABASE_STR
                ).exec().await
            }

            #(#model_actions)*
        }
    }
}
