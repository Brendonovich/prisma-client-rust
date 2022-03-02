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
            let model_actions_struct_name = format_ident!("{}Actions", model.name.to_case(Case::Pascal));

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

    quote! {
        use prisma_client_rust::builder::{self, Field, Input, Output, Query, QueryContext};
        use prisma_client_rust::datamodel::parse_configuration;
        use prisma_client_rust::prisma_models::InternalDataModelBuilder;
        use prisma_client_rust::query_core::{schema_builder, executor, BuildMode, QuerySchema, QueryExecutor};
        use prisma_client_rust::DeleteResult;

        use std::sync::Arc;
        
        pub struct PrismaClient {
            executor: Box<dyn QueryExecutor + Send + Sync + 'static>,
            query_schema: Arc<QuerySchema>,
        }
        
        impl PrismaClient {
            // adapted from https://github.com/polytope-labs/prisma-client-rs/blob/0dec2a67081e78b42700f6a62f414236438f84be/codegen/src/prisma.rs.template#L182
            pub async fn new() -> Self {
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
                let (db_name, executor) = executor::load(&source, &[], &url)
                    .await
                    .unwrap();
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
                Self {
                    executor,
                    query_schema,
                }
            }

            #(#model_actions)*
        }
    }
}
