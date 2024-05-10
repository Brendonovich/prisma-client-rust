use convert_case::Casing;
use prisma_client_rust_generator_shared::{Arity, RelationArity};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;

use super::Input;

pub fn definitions(input: &Input) -> TokenStream {
    let Input {
        dollar,
        model_path,
        schema_struct,
        selectable_fields,
        macro_rules,
        ..
    } = input;

    let mut attrs = quote! {
        #[allow(warnings)]
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
    };

    if cfg!(feature = "specta") {
        attrs.extend(quote! {
            #[derive(::prisma_client_rust::specta::Type)]
            #[specta(crate = prisma_client_rust::specta)]
        });

        attrs.extend(match &macro_rules.name {
            None => quote!(#[specta(inline)]),
            Some(name) => {
                let name_pascal = name.to_string().to_case(convert_case::Case::Pascal);

                quote! {
                    #[specta(rename = #name_pascal)]
                }
            }
        });
    }

    let (fields, field_modules): (Vec<_>, Vec<_>) = schema_struct
        .fields
        .iter()
        .filter_map(|field| {
            let Field {
                attrs, ty, ident, ..
            } = &field;

            let field_in_selectables = selectable_fields
                .iter()
                .find(|item| Some(&item.name) == ident.as_ref());
            let field_in_selection = macro_rules
                .selection
                .iter()
                .find(|item| Some(&item.name) == ident.as_ref());

            if field_in_selectables.is_some() && field_in_selection.is_none() {
                return None;
            }

            let (field_type, field_module) = field_in_selectables
                .zip(field_in_selection.and_then(|f| f.sub_selection.as_ref()))
                .and_then(|(field_in_selectables, (variant, sub_selection))| {
                    let Arity::Relation(relation_model_path, arity) = &field_in_selectables.arity
                    else {
                        return None;
                    };

                    let value = quote! {
                        pub mod #ident {
                            #dollar::#relation_model_path::#variant! {
                                definitions @ #sub_selection
                            }
                        }
                    };

                    let base = quote!(#ident::Data);

                    let typ = match arity {
                        RelationArity::One => base,
                        RelationArity::Many => quote!(Vec<#base>),
                        RelationArity::Optional => quote!(Option<#base>),
                    };

                    Some((typ, Some(value)))
                })
                .unwrap_or_else(|| (quote!(#dollar::#model_path::#ty), None));

            let field = quote! {
                #(#attrs)*
                pub #ident: #field_type
            };

            Some((field, field_module))
        })
        .unzip();

    quote! {
        #attrs
        pub struct Data {
            #(#fields),*
        }

        #(#field_modules)*
    }
}
