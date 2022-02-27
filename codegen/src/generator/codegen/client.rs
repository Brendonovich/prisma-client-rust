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
        use prisma_client_rust::builder::{Query, Output, Input, Field, self};
        use prisma_client_rust::engine::{Engine, QueryEngine, self};

        pub struct PrismaClient {
            pub engine: Box<dyn Engine>,
        }

        impl PrismaClient {
            pub fn new() -> Self {
                Self {
                    engine: Box::new(QueryEngine::new(#datamodel.to_string(), true)),
                }
            }

            #(#model_actions)*
        }
    }
}
