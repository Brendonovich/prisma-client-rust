use proc_macro2::TokenStream;
use quote::quote;

use crate::generator::GenerateArgs;

pub fn generate(args: &GenerateArgs) -> TokenStream {
    let datamodel = &args.datamodel_str;
    let database_string = &args.datasources[0].provider;

    let pcr = quote!(::prisma_client_rust);

    quote! {
        #![allow(warnings, unused)]

        static DATAMODEL_STR: &'static str = #datamodel;
        static DATABASE_STR: &'static str = #database_string;

        pub async fn new_client() -> Result<PrismaClient, #pcr::NewClientError> {
            let config = #pcr::datamodel::parse_configuration(DATAMODEL_STR)?.subject;
            let source = config
                .datasources
                .first()
                .expect("Please supply a datasource in your schema.prisma file");
            let url = if let Some(url) = source.load_shadow_database_url()? {
                url
            } else {
                source.load_url(|key| std::env::var(key).ok())?
            };

            // sqlite fix
            let url = if url.starts_with("file:") {
                let path = url.split(":").nth(1).unwrap();

                if std::path::Path::new("./schema.prisma").exists() {
                    url
                } else if std::path::Path::new("./prisma/schema.prisma").exists() {
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
        pub async fn new_client_with_url(url: &str) -> Result<PrismaClient, #pcr::NewClientError> {
            let config = #pcr::datamodel::parse_configuration(DATAMODEL_STR)?.subject;
            let source = config
                .datasources
                .first()
                .expect("Please supply a datasource in your schema.prisma file");
            let (db_name, executor) = #pcr::query_core::executor::load(&source, &[], &url).await?;
            let internal_model = #pcr::prisma_models::InternalDataModelBuilder::new(DATAMODEL_STR).build(db_name);
            let query_schema = std::sync::Arc::new(prisma_client_rust::query_core::schema_builder::build(
                internal_model,
                true,
                source.capabilities(),
                vec![],
                source.referential_integrity(),
            ));
            executor.primary_connector().get_connection().await?;
            Ok(PrismaClient::_new(
                executor,
                query_schema,
            ))
        }
    }
}
