use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};

use crate::generator::Root;

pub fn generate(root: &Root) -> TokenStream {
    let enums = root.dmmf.datamodel.enums.iter().map(|e| {
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
    });

    let internal_enums = root.dmmf.schema.enum_types.prisma.iter().map(|e| {
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

        quote! {
            #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
            enum #name {
                #(#variants),*
            }
        }
    });

    quote! {
        #(#enums)*
        #(#internal_enums)*
    }
}
