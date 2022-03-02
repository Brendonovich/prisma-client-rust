use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};

use crate::generator::Root;

pub fn generate_client(root: &Root) -> TokenStream {
    let model_actions = root
        .dmmf
        .datamodel
        .models
        .iter()
        .map(|model| {
            let property_name = format_ident!("{}", model.name.to_case(Case::Snake));
            let property_type = format_ident!("{}Actions", model.name.to_case(Case::Pascal));

            quote! {
                pub fn #property_name(&self) -> #property_type {
                    #property_type {
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
        use prisma_client_rust::query_core::{schema_builder, BuildMode, QuerySchema, QueryExecutor};
        use prisma_client_rust::Executor;
        use prisma_client_rust::{datamodel, prisma_models, query_core};

        use std::sync::Arc;

        #[derive(serde::Deserialize)]
        pub struct DeleteResult {
            pub count: isize,
        }

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
                let (db_name, executor) = query_core::executor::load(&source, &[], &url)
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
