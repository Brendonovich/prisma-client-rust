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
            query::QueryContext,
            query_core::{QueryExecutor, QuerySchema},
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
