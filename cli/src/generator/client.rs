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

                // don't ask, just accept.
                // migration engine seems to want some time to process things
                #pcr::tokio::time::sleep(core::time::Duration::from_millis(1)).await;

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
        pub struct PrismaClient {
            executor: #pcr::Executor,
            query_schema: ::std::sync::Arc<#pcr::schema::QuerySchema>,
            url: String,
        }

        impl ::std::fmt::Debug for PrismaClient {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("PrismaClient")
                 .finish()
            }
        }

        impl PrismaClient {
            pub(super) fn _new_query_context(&self) -> #pcr::queries::QueryContext {
                #pcr::queries::QueryContext::new(&self.executor, &self.query_schema)
            }

            pub(super) fn _new(executor: #pcr::Executor, query_schema: std::sync::Arc<#pcr::schema::QuerySchema>, url: String) -> Self {
                Self {
                    executor,
                    query_schema,
                    url,
                }
            }

            pub fn _query_raw<T: serde::de::DeserializeOwned>(&self, query: #pcr::raw::Raw) -> #pcr::QueryRaw<T> {
                #pcr::QueryRaw::new(
                   #pcr::queries::QueryContext::new(
                        &self.executor,
                        &self.query_schema
                    ),
                    query,
                    super::DATABASE_STR
                )
            }

            pub fn _execute_raw(&self, query: #pcr::raw::Raw) -> #pcr::ExecuteRaw {
                #pcr::ExecuteRaw::new(
                   #pcr::queries::QueryContext::new(
                        &self.executor,
                        &self.query_schema
                    ),
                    query,
                    super::DATABASE_STR
                )
            }

            pub async fn _batch<T: #pcr::BatchContainer<Marker>, Marker>(&self, queries: T) -> #pcr::queries::Result<T::ReturnType> {
                #pcr::batch(queries, &self.executor, &self.query_schema).await
            }

            #migrate_fns

            #(#model_actions)*
        }
    }
}
