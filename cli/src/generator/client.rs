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
                pub fn #model_name_snake(&self) -> super::#model_name_snake::Actions {
                    super::#model_name_snake::Actions {
                        client: &self.0,
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let pcr = quote!(::prisma_client_rust);

    let migrate_fns = cfg!(feature = "migrations").then(|| {
        quote! {
            pub async fn _migrate_deploy(&self) -> Result<(), #pcr::migrations::MigrateDeployError> {
                let res = #pcr::migrations::migrate_deploy(super::DATAMODEL_STR, super::MIGRATIONS_DIR, &self.0.url()).await;

                // don't ask, just accept.
                // migration engine seems to want some time to process things
                #pcr::tokio::time::sleep(core::time::Duration::from_millis(1)).await;

                res
            }

            pub async fn _migrate_resolve(&self, migration: &str) -> Result<(), #pcr::migrations::MigrateResolveError> {
                #pcr::migrations::migrate_resolve(migration, super::DATAMODEL_STR, super::MIGRATIONS_DIR, &self.0.url(),).await
            }

            pub fn _db_push(&self) -> #pcr::migrations::DbPush {
                #pcr::migrations::db_push(super::DATAMODEL_STR, &self.0.url())
            }
        }
    });

    let callback_fn = cfg!(feature = "mutation-callbacks").then(|| {
        quote! {
            pub fn with_model_mutation_callback(mut self, callback: impl Fn(#pcr::ModelMutationCallbackData) + 'static + Send + Sync) -> Self {
                self.action_notifier.model_mutation_callbacks.push(Box::new(callback));
                self
            }
        }
    });

    let mock_ctor = cfg!(feature = "mocking").then(|| {
        quote! {
            pub fn _mock() -> (Self, #pcr::MockStore) {
                let (internals, store) = #pcr::PrismaClientInternals::new_mock(#pcr::ActionNotifier::new());

                (Self(internals), store)
            }
        }
    });

    quote! {
        pub struct PrismaClientBuilder {
            url: Option<String>,
            action_notifier: #pcr::ActionNotifier,
        }

        impl PrismaClientBuilder {
            fn new() -> Self {
                Self {
                    url: None,
                    action_notifier: #pcr::ActionNotifier::new()
                }
            }

            pub fn with_url(mut self, url: String) -> Self {
                self.url = Some(url);
                self
            }

            #callback_fn

            pub async fn build(self) -> Result<PrismaClient, #pcr::NewClientError> {
                let internals = #pcr::PrismaClientInternals::new(
                    self.url,
                    self.action_notifier,
                    super::DATAMODEL_STR
                ).await?;

                Ok(PrismaClient(internals))
            }
        }

        pub struct PrismaClient(#pcr::PrismaClientInternals);

        impl ::std::fmt::Debug for PrismaClient {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("PrismaClient")
                 .finish()
            }
        }

        impl PrismaClient {
            pub fn _builder() -> PrismaClientBuilder {
                PrismaClientBuilder::new()
            }

            #mock_ctor

            pub fn _query_raw<T: #pcr::Data>(&self, query: #pcr::Raw) -> #pcr::QueryRaw<T> {
                #pcr::QueryRaw::new(
                    &self.0,
                    query,
                    super::DATABASE_STR,
                )
            }

            pub fn _execute_raw(&self, query: #pcr::Raw) -> #pcr::ExecuteRaw {
                #pcr::ExecuteRaw::new(
                    &self.0,
                    query,
                    super::DATABASE_STR,
                )
            }

            pub async fn _batch<'a, T: #pcr::BatchContainer<'a, Marker>, Marker>(&'a self, queries: T) -> #pcr::Result<<T as #pcr::BatchContainer<'a, Marker>>::ReturnType> {
                #pcr::batch(queries, &self.0).await
            }

            #migrate_fns

            #(#model_actions)*
        }
    }
}
