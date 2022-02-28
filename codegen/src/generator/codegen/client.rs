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

    let engine_module_declarations = if let Some(engine_modules) = &root.engine_modules {
        engine_modules
            .iter()
            .map(|module| {
                let module_name = format_ident!("{}", module.to_case(Case::Snake));
                quote! {
                    mod #module_name;
                }
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    let engine_module_inits = if let Some(engine_modules) = &root.engine_modules {
        engine_modules
            .iter()
            .map(|module| {
                let module_name = format_ident!("{}", module.to_case(Case::Snake));
                quote! {
                    #module_name::init();
                }
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    quote! {
        #(#engine_module_declarations)*

        use prisma_client_rust::builder::{Query, Output, Input, Field, self};
        use prisma_client_rust::engine::{Engine, QueryEngine, self};

        #[derive(serde::Deserialize)]
        pub struct DeleteResult {
            count: usize,
        }

        pub struct PrismaClient {
            pub engine: Box<dyn Engine>,
        }

        impl PrismaClient {
            pub fn new() -> Self {
                #(#engine_module_inits)*

                Self {
                    engine: Box::new(QueryEngine::new(#datamodel.to_string(), true)),
                }
            }

            #(#model_actions)*
        }
    }
}
