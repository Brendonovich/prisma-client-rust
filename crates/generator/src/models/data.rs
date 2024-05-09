use prisma_client_rust_sdk::prisma::{
    prisma_models::{
        walkers::{ModelWalker, RefinedFieldWalker},
        FieldArity,
    },
    psl::parser_database::ScalarFieldType,
};

use crate::prelude::*;

use super::ModelModulePart;

pub fn model_data(model: ModelWalker) -> ModelModulePart {
    let fields = model
        .fields()
        .flat_map(|field| {
            let arity = field.ast_field().arity;

            let name = field.name().to_string();
            let (typ, recursive_safe_typ) = match field.refine() {
                RefinedFieldWalker::Relation(relation_field) => {
                    let relation_model_name_snake =
                        snake_ident(relation_field.related_model().name());

                    let base_data = quote!(#relation_model_name_snake::Data);

                    let typ = match arity {
                        FieldArity::List => (quote!(Vec<#base_data>), None),
                        FieldArity::Optional => (
                            quote!(Option<#base_data>),
                            Some(quote!(Option<Box<#base_data>>)),
                        ),
                        FieldArity::Required => (quote!(#base_data), Some(quote!(Box<#base_data>))),
                    };

                    typ
                }
                RefinedFieldWalker::Scalar(scalar_field) => {
                    match scalar_field.scalar_field_type() {
                        ScalarFieldType::CompositeType(id) => {
                            let comp_type = field.db.walk(id);

                            let comp_type = snake_ident(comp_type.name());

                            let base_data = quote!(#comp_type::Data);

                            let typ = match arity {
                                FieldArity::List => (quote!(Vec<#base_data>), None),
                                FieldArity::Optional => (
                                    quote!(Option<#base_data>),
                                    Some(quote!(Option<Box<#base_data>>)),
                                ),
                                FieldArity::Required => {
                                    (quote!(#base_data), Some(quote!(Box<#base_data>)))
                                }
                            };

                            typ
                        }
                        _ => (field.type_tokens(&quote!(super::super::))?, None),
                    }
                }
            };

            let recursive_safe_typ = recursive_safe_typ.unwrap_or_else(|| quote!(Type));

            Some((
                name,
                quote! {
                    pub type Type = #typ;
                    pub type RecursiveSafeType = #recursive_safe_typ;
                },
            ))
        })
        .collect();

    ModelModulePart {
        data: quote! {},
        fields,
    }
}

pub fn r#struct(model: ModelWalker) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let struct_fields = model
        .fields()
        .filter(|f| f.ast_field().field_type.as_unsupported().is_none())
        .map(|field| match field.refine() {
            RefinedFieldWalker::Relation(field) => {
                let field_name_str = field.name();
                let field_name_snake = snake_ident(field_name_str);

                let attrs = match field.ast_field().arity {
                    FieldArity::Optional => {
                        quote! {
                            #[serde(
                                rename = #field_name_str,
                                default,
                                skip_serializing_if = "Option::is_none",
                                with = "prisma_client_rust::serde::double_option"
                            )]
                        }
                    }
                    _ => quote! {
                        #[serde(rename = #field_name_str)]
                    },
                };

                let specta_attrs = cfg!(feature = "specta").then(|| quote!(#[specta(skip)]));

                quote! {
                    #attrs
                    #specta_attrs
                    pub #field_name_snake: Option<#field_name_snake::RecursiveSafeType>
                }
            }
            RefinedFieldWalker::Scalar(field) => {
                let field_name_str = field.name();
                let field_name_snake = snake_ident(field_name_str);

                quote! {
                    #[serde(rename = #field_name_str)]
                    pub #field_name_snake: #field_name_snake::Type
                }
            }
        });

    let relation_accessors = model.fields().filter_map(|field| match field.refine() {
        RefinedFieldWalker::Relation(relation_field) => {
            let field_name_snake = snake_ident(field.name());
            let relation_model_name_snake = snake_ident(&relation_field.related_model().name());

            let access_error =
                quote!(#pcr::RelationNotFetchedError::new(stringify!(#field_name_snake)));

            let (typ, map) = match field.ast_field().arity {
                FieldArity::List => (quote!(&#field_name_snake::Type), None),
                FieldArity::Required => (
                    quote!(&super::#relation_model_name_snake::Data),
                    Some(quote!(.map(|v| v.as_ref()))),
                ),
                FieldArity::Optional => (
                    quote!(Option<&super::#relation_model_name_snake::Data>),
                    Some(quote!(.map(|v| v.as_ref().map(|v| v.as_ref())))),
                ),
            };

            Some(quote! {
                pub fn #field_name_snake(&self) -> Result<#typ, #pcr::RelationNotFetchedError> {
                    self.#field_name_snake.as_ref().ok_or(#access_error) #map
                }
            })
        }
        _ => None,
    });

    let specta_derive = cfg!(feature = "specta").then(|| {
        let model_name_pascal_str = pascal_ident(model.name()).to_string();

        quote! {
            #[derive(::prisma_client_rust::specta::Type)]
            #[specta(rename = #model_name_pascal_str, crate = prisma_client_rust::specta)]
        }
    });

    quote! {
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
        #specta_derive
        pub struct Data {
            #(#struct_fields),*
        }

        impl Data {
            #(#relation_accessors)*
        }
    }
}
