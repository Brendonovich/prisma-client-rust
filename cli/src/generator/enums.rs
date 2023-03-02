use prisma_client_rust_sdk::{GenerateArgs, prelude::pascal_ident};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(args: &GenerateArgs) -> TokenStream {
    let enums = args.dml.enums.iter().map(|e| {
        let name = pascal_ident(&e.name);

        let variants = e
            .values
            .iter()
            .map(|v| {
                let name = &v.name;
                let variant_name = pascal_ident(&v.name);

                quote! {
                    #[serde(rename=#name)]
                    #variant_name
                }
            })
            .collect::<Vec<_>>();

        let match_arms = e
            .values
            .iter()
            .map(|v| {
                let name = &v.name;
                let variant_name = pascal_ident(&v.name);

                quote!(Self::#variant_name => #name.to_string())
            })
            .collect::<Vec<_>>();

        let specta_derive = cfg!(feature = "rspc").then(|| {
            let model_name_pascal_str = name.to_string();
    
            quote! {
                #[derive(::prisma_client_rust::rspc::Type)]
                #[specta(rename = #model_name_pascal_str, crate = "prisma_client_rust::rspc::internal::specta")]
            }
        });

        quote! {
            #specta_derive
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
        }
    });

    quote! {
        #(#enums)*
    }
}
