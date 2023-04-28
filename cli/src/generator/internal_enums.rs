use prisma_client_rust_sdk::prelude::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(args: &GenerateArgs) -> TokenStream {
    let internal_enums = args
        .dmmf
        .schema
        .enum_types
        .get("prisma")
        .unwrap()
        .iter()
        .map(|e| {
            let name = pascal_ident(&e.name);

            let variants = e
                .values
                .iter()
                .map(|v| {
                    let variant_name = pascal_ident(v);

                    quote! {
                        #[serde(rename=#v)]
                        #variant_name
                    }
                })
                .collect::<Vec<_>>();

            let match_arms = e
                .values
                .iter()
                .map(|name| {
                    let variant_name = pascal_ident(name);

                    quote!(Self::#variant_name => #name.to_string())
                })
                .collect::<Vec<_>>();

            let isolation_level_impl = (&e.name == "TransactionIsolationLevel").then(|| quote! {
                impl ::prisma_client_rust::TransactionIsolationLevel for TransactionIsolationLevel {}
            });

            quote! {
                #[derive(Debug, Clone, Copy, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                pub enum #name {
                    #(#variants),*
                }

                impl ToString for #name {
                    fn to_string(&self) -> String {
                        match self {
                            #(#match_arms),*
                        }
                    }
                }

                #isolation_level_impl
            }
        });

    quote! {
        #(#internal_enums)*

    }
}
