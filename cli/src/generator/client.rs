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

    quote! {
        pub struct PrismaClient {
            executor: Box<dyn #pcr::query_core::QueryExecutor + Send + Sync + 'static>,
            query_schema: ::std::sync::Arc<#pcr::schema::QuerySchema>,
        }

        impl ::std::fmt::Debug for PrismaClient {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("PrismaClient")
                 .finish()
            }
        }

        impl PrismaClient {
            pub(super) fn _new_query_context(&self) -> #pcr::queries::QueryContext {
                #pcr::queries::QueryContext::new(&self.executor, self.query_schema.clone())
            }

            pub(super) fn _new(executor: Box<dyn #pcr::query_core::QueryExecutor + Send + Sync + 'static>, query_schema: std::sync::Arc<#pcr::schema::QuerySchema>) -> Self {
                Self {
                    executor,
                    query_schema,
                }
            }

            pub fn _query_raw<T: serde::de::DeserializeOwned>(&self, query: #pcr::raw::Raw) -> #pcr::QueryRaw<T> {
                #pcr::QueryRaw::new(
                   #pcr::queries::QueryContext::new(
                        &self.executor,
                        self.query_schema.clone()
                    ),
                    query,
                    super::DATABASE_STR
                )
            }

            pub fn _execute_raw(&self, query: #pcr::raw::Raw) -> #pcr::ExecuteRaw {
                #pcr::ExecuteRaw::new(
                   #pcr::queries::QueryContext::new(
                        &self.executor,
                        self.query_schema.clone()
                    ),
                    query,
                    super::DATABASE_STR
                )
            }

            pub async fn _batch<T: #pcr::BatchContainer<Marker>, Marker>(&self, queries: T) -> #pcr::queries::Result<T::ReturnType> {
                #pcr::batch(queries, &self.executor, &self.query_schema).await
            }

            #(#model_actions)*
        }
    }
}
