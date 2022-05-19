use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};

use crate::generator::GeneratorArgs;

pub fn generate(args: &GeneratorArgs) -> TokenStream {
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
            #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
                #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
        #(#internal_enums)*
    }
}
