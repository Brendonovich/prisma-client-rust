use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};

use crate::generator::dmmf::Model;

pub fn generate_models(models: &Vec<Model>) -> TokenStream {
    models.iter().map(|model| generate_model(model)).collect()
}

fn generate_model(model: &Model) -> TokenStream {
    let name_pascal = model.name.to_case(Case::Pascal);

    let model_ident = format_ident!("{}Model", name_pascal);

    let inner_fields = model
        .fields
        .iter()
        .filter(|f| !f.kind.is_relation())
        .map(|field| {
            let field_name = &field.name;
            let name = format_ident!("{}", field.name.to_case(Case::Snake));
            let field_type = format_ident!("{}", field.field_type.value());

            if field.is_list {
                quote! {
                    #[serde(rename = #field_name)]
                    pub #name: Vec<#field_type>
                }
            } else {
                if field.is_required {
                    quote! {
                        #[serde(rename = #field_name)]
                        pub #name: #field_type
                    }
                } else {
                    quote! {
                        #[serde(rename = #field_name)]
                        pub #name: Option<#field_type>
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let relations_fields = model
        .fields
        .iter()
        .filter(|f| f.kind.is_relation())
        .map(|field| {
            let field_name = &field.name;
            let name = format_ident!("{}", field.name.to_case(Case::Snake));
            let field_type = format_ident!("{}Model", field.field_type.value());

            if field.is_list {
                quote! {
                   #[serde(rename = #field_name)]
                   #name: Option<Vec<#field_type>>
                }
            } else {
                if field.is_required {
                    quote! {
                        #[serde(rename = #field_name)]
                        #name: Option<#field_type>
                    }
                } else {
                    quote! {
                        #[serde(rename = #field_name)]
                        pub #name: Option<#field_type>
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let model_fields = inner_fields
        .iter()
        .chain(relations_fields.iter())
        .collect::<Vec<_>>();

    let nullable_accessors = model
        .fields
        .iter()
        .filter(|f| f.kind.is_relation() && f.is_required)
        .map(|field| {
            let name = format_ident!("{}", field.name.to_case(Case::Snake));
            let field_type = format_ident!("{}Model", field.field_type.value());

            let return_type = if field.is_list {
                quote! {
                    Vec<#field_type>
                }
            } else {
                quote! {
                    #field_type
                }
            };

            let err = format!(
                "attempted to access {} but did not fetch it using the .with() syntax",
                name
            );

            quote! {
                pub fn #name(&self) -> Result<&#return_type, String> {
                    match &self.#name {
                        Some(v) => Ok(v),
                        None => Err(#err.to_string()),
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #[derive(serde::Deserialize, Debug)]
        pub struct #model_ident {
            #(#model_fields),*
        }

        impl #model_ident {
            #(#nullable_accessors)*
        }
    }
}
