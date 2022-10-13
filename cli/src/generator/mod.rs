mod client;
mod enums;
mod header;
mod internal_enums;
mod models;
mod prelude;

use prelude::*;
use serde::Deserialize;

fn default_module_path() -> String {
    "prisma".to_string()
}

#[derive(Deserialize)]
pub struct PrismaClientRustGenerator {
    #[serde(default = "default_module_path")]
    module_path: String,
}

impl PrismaGenerator for PrismaClientRustGenerator {
    const NAME: &'static str = "Prisma Client Rust";
    const DEFAULT_OUTPUT: &'static str = "./prisma.rs";

    fn generate(self, args: GenerateArgs) -> String {
        let mut header = header::generate(&args);

        header.extend(models::generate(
            &args,
            self.module_path.parse().expect("Invalid module path"),
        ));

        let internal_enums = internal_enums::generate(&args);
        let client = client::generate(&args);

        let use_query_mode = match &args.connector {
            #[cfg(feature = "postgresql")]
            c if c.is_provider(datamodel::builtin_connectors::POSTGRES.name()) => true,
            #[cfg(feature = "mongodb")]
            c if c.is_provider(datamodel::builtin_connectors::MONGODB.name()) => true,
            _ => false,
        }
        .then(|| {
            quote!(
                pub use _prisma::QueryMode;
            )
        });

        let read_filters = args.read_filters.iter().map(|filter| {
            let name = format_ident!("{}Filter", &filter.name);

            let method_tokens = filter.methods.iter().map(|method| {
                let typ = method.typ.to_tokens();

                let variant_name = format_ident!("{}", method.name.to_case(Case::Pascal));
                let method_action_string = &method.action;

                let value_as_prisma_value = method
                    .typ
                    .to_prisma_value(&format_ident!("value"), method.is_list);
                let typ = method.is_list.then(|| quote!(Vec<#typ>)).unwrap_or(typ);

                (
                    quote!(#variant_name(#typ)),
                    quote! {
                        Self::#variant_name(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![
                            (#method_action_string.to_string(), #value_as_prisma_value)
                        ])
                    },
                )
            });

            let method_variants = method_tokens.clone().map(|(v, _)| v);
            let method_matches = method_tokens.clone().map(|(_, m)| m);

            quote! {
                #[derive(Clone)]
                pub enum #name {
                    #(#method_variants),*
                }
 
                impl Into<::prisma_client_rust::SerializedWhereValue> for #name {
                    fn into(self) -> ::prisma_client_rust::SerializedWhereValue {
                        match self {
                            #(#method_matches),*
                        }
                    }
                }
            }
        });

        header.extend(quote! {
            pub mod _prisma {
                #client
                #internal_enums

                pub mod read_filters {
                    #(#read_filters)*
                }
            }

            pub use _prisma::PrismaClient;
            #use_query_mode
        });

        header.extend(enums::generate(&args));

        header.to_string()
    }
}
