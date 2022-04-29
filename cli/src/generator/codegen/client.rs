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

    quote! {
        #![allow(warnings, unused)]

        use prisma_client_rust::{
            chrono,
            datamodel::parse_configuration,
            operator::Operator,
            prisma_models::{InternalDataModelBuilder, PrismaValue},
            query::{QueryContext, Result as QueryResult},
            query_core::{
                executor, schema_builder, BuildMode, CoreError, InterpreterError, QueryExecutor,
                QueryGraphBuilderError, QuerySchema, QueryValue, Selection,
            },
            serde_json, transform_equals, Args, BatchResult, Direction, FindManyArgs,
            FindManySelectionArgs, SerializedWhere, SerializedWhereValue,
        };
        pub use prisma_client_rust::{query::Error as QueryError, NewClientError};
        use serde::{Deserialize, Serialize};
        use std::ops::Deref;
        use std::path::Path;
        use std::sync::Arc;

        static DATAMODEL_STR: &'static str = #datamodel;

        #[derive(Debug)]
        pub struct PrismaClient {
            executor: Box<dyn QueryExecutor + Send + Sync + 'static>,
            query_schema: Arc<QuerySchema>,
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
            // pub async fn _query_raw<T: serde::de::DeserializeOwned>(&self, query: &str) -> QueryResult<Vec<T>> {
            //     let query = Query {
            //         ctx: QueryContext::new(&self.executor, self.query_schema.clone()),
            //         operation: "mutation".into(),
            //         method: "queryRaw".into(),
            //         inputs: vec![
            //             Input {
            //                 name: "query".into(),
            //                 value: Some(query.into()),
            //                 ..Default::default()
            //             },
            //             Input {
            //                 name: "parameters".into(),
            //                 value: Some("[]".into()),
            //                 ..Default::default()
            //             }
            //         ],
            //         name: "".into(),
            //         model: "".into(),
            //         outputs: vec![]
            //     };

            //     query.perform().await
            // }

            // pub async fn _execute_raw(&self, query: &str) -> QueryResult<i64> {
            //     let query = Query {
            //         ctx: QueryContext::new(&self.executor, self.query_schema.clone()),
            //         operation: "mutation".into(),
            //         method: "executeRaw".into(),
            //         inputs: vec![
            //             Input {
            //                 name: "query".into(),
            //                 value: Some(query.into()),
            //                 ..Default::default()
            //             },
            //             Input {
            //                 // TODO: use correct value
            //                 name: "parameters".into(),
            //                 value: Some("[]".into()),
            //                 ..Default::default()
            //             },
            //         ],
            //         name: "".into(),
            //         model: "".into(),
            //         outputs: vec![]
            //     };

            //     query.perform().await.map(|result: i64| result)
            // }

            #(#model_actions)*
        }
    }
}
