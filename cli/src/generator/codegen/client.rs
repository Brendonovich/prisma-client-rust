use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};

use crate::generator::GeneratorArgs;

pub fn generate(args: &GeneratorArgs) -> TokenStream {
    let model_actions = args
        .dml
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

    quote! {
        use super::*;
        use prisma_client_rust::{
            queries::QueryContext,
            query_core::{QueryExecutor, QuerySchema},
            raw, QueryRaw, ExecuteRaw,
        };
        use serde::{Deserialize, Serialize};
        use std::fmt;
        use std::sync::Arc;

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

        impl PrismaClient {
            pub(super) fn _new_query_context(&self) -> QueryContext {
                QueryContext::new(&self.executor, self.query_schema.clone())
            }

            pub(super) fn _new(executor: Box<dyn QueryExecutor + Send + Sync + 'static>, query_schema: Arc<QuerySchema>) -> Self {
                Self {
                    executor,
                    query_schema,
                }
            }

            pub async fn _query_raw<T: serde::de::DeserializeOwned>(&self, query: raw::Raw) -> QueryResult<Vec<T>> {
                QueryRaw::new(
                   QueryContext::new(&self.executor, self.query_schema.clone()),
                   query,
                   DATABASE_STR
                ).exec().await
            }

            pub async fn _execute_raw(&self, query: raw::Raw) -> QueryResult<i64> {
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
