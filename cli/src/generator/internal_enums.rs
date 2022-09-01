use prisma_client_rust_sdk::{Case, Casing, GenerateArgs};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate(args: &GenerateArgs) -> TokenStream {
    let internal_enums = args
        .schema
        .enum_types
        .get("prisma")
        .unwrap()
        .iter()
        .map(|e| {
            let name = format_ident!("{}", e.name.to_case(Case::Pascal));

            let variants = e
                .values
                .iter()
                .map(|v| {
                    let variant_name = format_ident!("{}", v.to_case(Case::Pascal));

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
                    let variant_name = format_ident!("{}", name.to_case(Case::Pascal));

                    quote!(Self::#variant_name => #name.to_string())
                })
                .collect::<Vec<_>>();

            quote! {
                #[derive(Debug, Clone, Copy, ::serde::Serialize, ::serde::Deserialize)]
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
            }
        });

    quote! {
       #(#internal_enums)*
    }
}
