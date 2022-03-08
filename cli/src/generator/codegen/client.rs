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
            let model_actions_struct_name =
                format_ident!("{}Actions", model.name.to_case(Case::Pascal));

            quote! {
                pub fn #model_name_snake(&self) -> #model_actions_struct_name {
                    #model_actions_struct_name {
                        client: &self,
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let datamodel = &root.datamodel;

    let schema_path_root = format!("./{}", &root.schema_path);
    let schema_path_prisma_folder = format!("./prisma/{}", &root.schema_path);

    quote! {
        use prisma_client_rust::query::*;
        use prisma_client_rust::datamodel::parse_configuration;
        use prisma_client_rust::prisma_models::InternalDataModelBuilder;
        use prisma_client_rust::query_core::{schema_builder, executor, BuildMode, QuerySchema, QueryExecutor, CoreError};
        use prisma_client_rust::{serde_json, chrono, operator::Operator, DeleteResult};

        use serde::{Serialize, Deserialize};

        use std::path::Path;
        use std::sync::Arc;

        pub struct PrismaClient {
            executor: Box<dyn QueryExecutor + Send + Sync + 'static>,
            query_schema: Arc<QuerySchema>,
        }

        pub async fn new_client() -> PrismaClient {
            let datamodel_str = #datamodel;
            let config = parse_configuration(datamodel_str).unwrap().subject;
            let source = config
                .datasources
                .first()
                .expect("Pleasy supply a datasource in your schema.prisma file");
            let url = if let Some(url) = source.load_shadow_database_url().unwrap() {
                url
            } else {
                source.load_url(|key| std::env::var(key).ok()).unwrap()
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
        pub async fn new_client_with_url(url: &str) -> PrismaClient {
            let datamodel_str = #datamodel;
            let config = parse_configuration(datamodel_str).unwrap().subject;
            let source = config
                .datasources
                .first()
                .expect("Pleasy supply a datasource in your schema.prisma file");

            let (db_name, executor) = executor::load(&source, &[], &url).await.unwrap();
            let internal_model = InternalDataModelBuilder::new(&datamodel_str).build(db_name);
            let query_schema = Arc::new(schema_builder::build(
                internal_model,
                BuildMode::Modern,
                true,
                source.capabilities(),
                vec![],
                source.referential_integrity(),
            ));
            executor.primary_connector().get_connection().await.unwrap();
            PrismaClient {
                executor,
                query_schema,
            }
        }

        impl PrismaClient {
            pub async fn _query_raw<T: serde::de::DeserializeOwned>(&self, query: &str) -> Result<Vec<T>, CoreError> {
                let query = Query {
                    ctx: QueryContext::new(&self.executor, self.query_schema.clone()),
                    operation: "mutation".into(),
                    method: "queryRaw".into(),
                    inputs: vec![
                        Input {
                            name: "query".into(),
                            value: Some(query.into()),
                            ..Default::default()
                        },
                        Input {
                            name: "parameters".into(),
                            value: Some("[]".into()),
                            ..Default::default()
                        }
                    ],
                    name: "".into(),
                    model: "".into(),
                    outputs: vec![]
                };

                query.perform().await
            }

            pub async fn _execute_raw(&self, query: &str) -> Result<i64, CoreError> {
                let query = Query {
                    ctx: QueryContext::new(&self.executor, self.query_schema.clone()),
                    operation: "mutation".into(),
                    method: "executeRaw".into(),
                    inputs: vec![
                        Input {
                            name: "query".into(),
                            value: Some(query.into()),
                            ..Default::default()
                        },
                        Input {
                            name: "parameters".into(),
                            value: Some("[]".into()),
                            ..Default::default()
                        },
                    ],
                    name: "".into(),
                    model: "".into(),
                    outputs: vec![]
                };

                query.perform().await.map(|result: i64| result)
            }

            pub async fn _execute_gql<T: DeserializeOwned>(&self, gql: &str) -> Result<T, CoreError> {
                let document = parse_query(&query_string).unwrap();
                let operation = GraphQLProtocolAdapter::convert(document, None).unwrap();

                self
                    .executor
                    .execute(None, operation, self.ctx.schema, None)
                    .await
                    .map(|response| {
                        serde_json::from_value(serde_json::to_value(response.data).unwrap()).unwrap()
                    })
            }
            
            #(#model_actions)*
        }
    }
}
