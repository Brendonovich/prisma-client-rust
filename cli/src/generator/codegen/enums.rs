use convert_case::{Case, Casing};
use quote::{__private::TokenStream, quote, format_ident};

use crate::generator::dmmf::Enum;

pub fn generate(e: &Enum) -> TokenStream {
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

    quote! {
        #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
        enum #name {
            #(#variants),*
        }
    }
}
