use prisma_client_rust_sdk::{Case, Casing, GenerateArgs};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate(args: &GenerateArgs) -> TokenStream {
    let model_actions = args
        .dml
        .models
        .iter()
        .map(|model| {
            let model_name_snake = format_ident!("{}", model.name.to_case(Case::Snake));

            quote! {
                pub fn #model_name_snake(&self) -> super:: #model_name_snake::Actions {
                    super::#model_name_snake::Actions {
                        client: &self,
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let pcr = quote!(::prisma_client_rust);

    let migrate_fns = cfg!(feature = "migrations").then(|| {
        quote! {
            pub async fn _migrate_deploy(&self) -> Result<(), #pcr::migrations::MigrateDeployError> {
                let res = #pcr::migrations::migrate_deploy(super::DATAMODEL_STR, super::MIGRATIONS_DIR, &self.url).await;

                // don't ask, just accept
                tokio::time::sleep(core::time::Duration::from_millis(1)).await;

                res
            }

            pub async fn _migrate_resolve(&self, migration: &str) -> Result<(), #pcr::migrations::MigrateResolveError> {
                #pcr::migrations::migrate_resolve(migration, super::DATAMODEL_STR, super::MIGRATIONS_DIR, &self.url,).await
            }

            pub fn _db_push(&self) -> #pcr::migrations::DbPush {
                #pcr::migrations::db_push(super::DATAMODEL_STR, &self.url)
            }
        }
    });

    quote! {
        pub struct PrismaClientBuilder {
            url: Option<String>,
            operation_callbacks: Vec<#pcr::OperationCallback>
        }

        impl PrismaClientBuilder {
            pub fn new() -> Self {
                Self {
                    url: None,
                    operation_callbacks: vec![]
                }
            }

            pub fn with_url(mut self, url: String) -> Self {
                self.url = Some(url);
                self
            }

            pub fn with_operation_callback(mut self, callback: impl Fn(&#pcr::Operation) + 'static) -> Self {
                self.operation_callbacks.push(Box::new(callback));
                self
            }

            pub async fn build(self) -> Result<PrismaClient, #pcr::NewClientError> {
                let config = #pcr::datamodel::parse_configuration(super::DATAMODEL_STR)?.subject;
                let source = config
                    .datasources
                    .first()
                    .expect("Please supply a datasource in your schema.prisma file");

                let url = match self.url {
                    Some(url) => url,
                    None => {
                        if let Some(url) = source.load_shadow_database_url()? {
                            url
                        } else {
                            source.load_url(|key| std::env::var(key).ok())?
                        }
                    }
                };

                let url = match url.starts_with("file:") {
                    true => {
                        let path = url.split(":").nth(1).unwrap();

                        if std::path::Path::new("./prisma/schema.prisma").exists() {
                            format!("file:./prisma/{}", path)
                        } else { url }
                    },
                    _ => url,
                };

                let (db_name, executor) = #pcr::query_core::executor::load(&source, &[], &url).await?;

                let internal_model = #pcr::prisma_models::InternalDataModelBuilder::new(super::DATAMODEL_STR).build(db_name);

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
                    url,
                    self.operation_callbacks
                ))
            }
        }

        pub struct PrismaClient {
            executor: #pcr::Executor,
            query_schema: ::std::sync::Arc<#pcr::schema::QuerySchema>,
            url: String,
            operation_callbacks: Vec<#pcr::OperationCallback>
        }

        impl ::std::fmt::Debug for PrismaClient {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("PrismaClient")
                 .finish()
            }
        }

        impl PrismaClient {
            pub(super) fn _new_query_context(&self) -> #pcr::queries::QueryContext {
                #pcr::queries::QueryContext::new(&self.executor, &self.query_schema, &self.operation_callbacks)
            }

            pub(super) fn _new(executor: #pcr::Executor, query_schema: std::sync::Arc<#pcr::schema::QuerySchema>, url: String, operation_callbacks: Vec<#pcr::OperationCallback>) -> Self {
                Self {
                    executor,
                    query_schema,
                    url,
                    operation_callbacks
                }
            }

            pub fn _builder() -> PrismaClientBuilder {
                PrismaClientBuilder::new()
            }

            pub fn _query_raw<T: serde::de::DeserializeOwned>(&self, query: #pcr::raw::Raw) -> #pcr::QueryRaw<T> {
                #pcr::QueryRaw::new(
                    self._new_query_context(),
                    query,
                    super::DATABASE_STR,
                )
            }

            pub fn _execute_raw(&self, query: #pcr::raw::Raw) -> #pcr::ExecuteRaw {
                #pcr::ExecuteRaw::new(
                    self._new_query_context(),
                    query,
                    super::DATABASE_STR,
                )
            }

            pub async fn _batch<T: #pcr::BatchContainer<Marker>, Marker>(&self, queries: T) -> #pcr::queries::Result<T::ReturnType> {
                #pcr::batch(queries, self._new_query_context()).await
            }

            #migrate_fns

            #(#model_actions)*
        }
    }
}
