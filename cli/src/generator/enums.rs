use prisma_client_rust_sdk::{Case, Casing, GenerateArgs};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate(args: &GenerateArgs) -> TokenStream {
    let enums = args.dml.enums.iter().map(|e| {
        let name = format_ident!("{}", e.name.to_case(Case::Pascal));

        let variants = e
            .values
            .iter()
            .map(|v| {
                let name = &v.name;
                let variant_name = format_ident!("{}", v.name.to_case(Case::Pascal));

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
                let variant_name = format_ident!("{}", v.name.to_case(Case::Pascal));

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
        #(#enums)*
    }
}
