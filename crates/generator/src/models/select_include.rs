use prisma_client_rust_generator_shared::select_include::{SelectableFields, Variant};
use prisma_client_rust_sdk::prisma::prisma_models::{
    walkers::{FieldWalker, ModelWalker, RefinedFieldWalker},
    FieldArity,
};
use syn::{parse_quote, ItemStruct};

use crate::prelude::*;

fn model_macro<'a>(
    model: ModelWalker<'a>,
    module_path: &TokenStream,
    variant: Variant,
    // Fields that can be picked from
    selection_fields: impl Iterator<Item = FieldWalker<'a>>,
) -> TokenStream {
    let model_name_snake = snake_ident(model.name());
    let model_name_snake_raw = snake_ident_raw(model.name());
    let macro_name = format_ident!("_{variant}_{model_name_snake_raw}");

    let factory_name = format_ident!("{variant}_factory");

    let data_struct: ItemStruct = {
        let fields = model.fields().map(|field| {
            let field_name_str = field.name();
            let field_name_snake = snake_ident(field_name_str);

            quote! {
                #[serde(rename = #field_name_str)]
                #field_name_snake: #field_name_snake::Type
            }
        });

        parse_quote! {
            struct Data {
                #(#fields),*
            }
        }
    };

    let selectable_fields = SelectableFields::new(selection_fields, module_path);

    quote! {
        ::prisma_client_rust::macros::#factory_name!(
            #macro_name,
            #variant,
            #module_path #model_name_snake,
            #data_struct,
            #selectable_fields
        );
    }
}

fn field_module_enum(field: FieldWalker, variant: Variant) -> Option<TokenStream> {
    let pcr = quote!(::prisma_client_rust);

    let field_name_pascal = pascal_ident(field.name());
    let field_name_str = field.name();

    let variant_pascal = pascal_ident(&variant.to_string());
    let variant_param = variant.param();

    Some(match field.refine() {
        RefinedFieldWalker::Relation(relation_field) => {
            let relation_model_name_snake = snake_ident(relation_field.related_model().name());

            let initial_nested_selections = match variant {
                Variant::Include => {
                    quote!(<#relation_model_name_snake::Types as #pcr::ModelTypes>::scalar_selections())
                }
                Variant::Select => quote!(vec![]),
            };

            match field.ast_field().arity {
                FieldArity::List => quote! {
                    pub enum #variant_pascal {
                        Select(#relation_model_name_snake::ManyArgs, Vec<#relation_model_name_snake::SelectParam>),
                        Include(#relation_model_name_snake::ManyArgs, Vec<#relation_model_name_snake::IncludeParam>),
                        Fetch(#relation_model_name_snake::ManyArgs)
                    }

                    impl Into<super::#variant_param> for #variant_pascal {
                        fn into(self) -> super::#variant_param {
                            super::#variant_param::#field_name_pascal(self)
                        }
                    }

                    impl #variant_pascal {
                        pub fn select(args: #relation_model_name_snake::ManyArgs, nested_selections: Vec<#relation_model_name_snake::SelectParam>) -> Self {
                            Self::Select(args, nested_selections)
                        }

                        pub fn include(args: #relation_model_name_snake::ManyArgs, nested_selections: Vec<#relation_model_name_snake::IncludeParam>) -> Self {
                            Self::Include(args, nested_selections)
                        }
                    }

                    impl Into<#pcr::Selection> for #variant_pascal {
                        fn into(self) -> #pcr::Selection {
                             let (args, selections) = match self {
                                 Self::Select(args, selections) => (
                                     args.to_graphql().0,
                                     selections.into_iter().map(Into::into).collect()
                                 ),
                                 Self::Include(args, selections) => (
                                     args.to_graphql().0,
                                     {
                                         let mut nested_selections = #initial_nested_selections;
                                         nested_selections.extend(selections.into_iter().map(Into::into));
                                         nested_selections
                                     }
                                 ),
                                 Self::Fetch(args) => (
                                     args.to_graphql().0,
                                     <#relation_model_name_snake::Types as #pcr::ModelTypes>::scalar_selections()
                                 )
                             };

                             #pcr::Selection::new(NAME, None, args, selections)
                        }
                    }
                },
                _ => quote! {
                    pub enum #variant_pascal {
                        Select(Vec<#relation_model_name_snake::SelectParam>),
                        Include(Vec<#relation_model_name_snake::IncludeParam>),
                        Fetch
                    }

                    impl Into<super::#variant_param> for #variant_pascal {
                        fn into(self) -> super::#variant_param {
                            super::#variant_param::#field_name_pascal(self)
                        }
                    }

                    impl #variant_pascal {
                        pub fn select(nested_selections: Vec<#relation_model_name_snake::SelectParam>) -> Self {
                            Self::Select(nested_selections)
                        }

                        pub fn include(nested_selections: Vec<#relation_model_name_snake::IncludeParam>) -> Self {
                            Self::Include(nested_selections)
                        }
                    }

                    impl Into<#pcr::Selection> for #variant_pascal {
                        fn into(self) -> #pcr::Selection {
                            let selections = match self {
                                Self::Select(selections) => {
                                    selections.into_iter().map(Into::into).collect()
                                },
                                Self::Include(selections) => {
                                    let mut nested_selections = #initial_nested_selections;
                                    nested_selections.extend(selections.into_iter().map(Into::into));
                                    nested_selections
                                },
                                Self::Fetch => {
                                    <#relation_model_name_snake::Types as #pcr::ModelTypes>::scalar_selections()
                                }
                            };

                            #pcr::Selection::new(#field_name_str, None, [], selections)
                        }
                    }
                },
            }
        }
        RefinedFieldWalker::Scalar(_) => quote! {
            pub struct #variant_pascal;

            impl Into<super::#variant_param> for #variant_pascal {
                fn into(self) -> super::#variant_param {
                    super::#variant_param::#field_name_pascal(self)
                }
            }

            impl Into<#pcr::Selection> for #variant_pascal {
                fn into(self) -> #pcr::Selection {
                    #pcr::sel(NAME)
                }
            }
        },
    })
}

fn model_module_enum(model: ModelWalker, variant: Variant) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let variant_pascal = pascal_ident(&variant.to_string());

    let variants = model
        .fields()
        .filter(|f| !f.ast_field().field_type.as_unsupported().is_some())
        .map(|field| {
            let field_name_snake = snake_ident(field.name());
            let field_name_pascal = pascal_ident(field.name());

            quote!(#field_name_pascal(#field_name_snake::#variant_pascal))
        });

    let field_names_pascal = model
        .fields()
        .filter(|f| !f.ast_field().field_type.as_unsupported().is_some())
        .map(|field| pascal_ident(field.name()));

    let variant_param = variant.param();

    quote! {
        pub enum #variant_param {
            #(#variants),*
        }

        impl Into<#pcr::Selection> for #variant_param {
            fn into(self) -> #pcr::Selection {
                match self {
                    #(Self::#field_names_pascal(data) => data.into()),*
                }
            }
        }
    }
}

pub mod include {
    use prisma_client_rust_sdk::prisma::prisma_models::walkers::{ModelWalker, RefinedFieldWalker};

    use crate::models::ModelModulePart;

    use super::*;

    pub fn model_data(model: ModelWalker, module_path: &TokenStream) -> ModelModulePart {
        let r#macro = super::model_macro(
            model,
            module_path,
            Variant::Include,
            model
                .fields()
                .filter(|f| matches!(f.refine(), RefinedFieldWalker::Relation(_))),
        );

        let r#enum = super::model_module_enum(model, Variant::Include);

        ModelModulePart {
            data: quote! {
                #r#macro
                 #r#enum
            },
            fields: model
                .fields()
                .filter(|f| f.ast_field().field_type.as_unsupported().is_none())
                .flat_map(|field| {
                    super::field_module_enum(field, Variant::Include)
                        .map(|e| (field.name().to_string(), e))
                })
                .collect(),
        }
    }
}

pub mod select {
    use prisma_client_rust_sdk::prisma::prisma_models::walkers::ModelWalker;

    use crate::models::ModelModulePart;

    use super::*;

    pub fn model_data(model: ModelWalker, module_path: &TokenStream) -> ModelModulePart {
        let r#macro = super::model_macro(
            model,
            module_path,
            Variant::Select,
            model
                .fields()
                .filter(|f| f.ast_field().field_type.as_unsupported().is_none()),
        );

        let r#enum = super::model_module_enum(model, Variant::Select);

        ModelModulePart {
            data: quote! {
                #r#macro
                #r#enum
            },
            fields: model
                .fields()
                .filter(|f| f.ast_field().field_type.as_unsupported().is_none())
                .flat_map(|field| {
                    super::field_module_enum(field, Variant::Select)
                        .map(|e| (field.name().to_string(), e))
                })
                .collect(),
        }
    }
}
