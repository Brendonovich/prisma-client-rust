use prisma_client_rust_sdk::prisma::{
    dmmf::TypeLocation,
    prisma_models::{walkers::CompositeTypeWalker, FieldArity},
    psl::parser_database::ScalarFieldType,
};

use crate::prelude::*;

pub fn enum_definition(comp_type: CompositeTypeWalker, args: &GenerateArgs) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let (variants, into_pv_arms): (Vec<_>, Vec<_>) = comp_type
        .fields()
        .flat_map(|field| {
            let field_name_snake = snake_ident(field.name());
            let field_name_pascal = pascal_ident(field.name());

            if field.ast_field().arity.is_list() {
                return None;
            }

            Some(match field.r#type() {
                ScalarFieldType::CompositeType(id) => {
                    let comp_type = field.db.walk(id);

                    let composite_type_snake = snake_ident(comp_type.name());

                    (
                        quote!(#field_name_pascal(Vec<super::#composite_type_snake::OrderByParam>)),
                        quote! {
                            Self::#field_name_pascal(params) => (
                                #field_name_snake::NAME,
                                #pcr::PrismaValue::Object(
                                    params
                                         .into_iter()
                                         .map(Into::into)
                                         .collect()
                                )
                            )
                        },
                    )
                }
                _ => (
                    quote!(#field_name_pascal(SortOrder)),
                    quote! {
                        Self::#field_name_pascal(direction) => (
                            #field_name_snake::NAME,
                            #pcr::PrismaValue::String(direction.to_string())
                        )
                    },
                ),
            })
        })
        .unzip();

    let _ = args
        .dmmf
        .schema
        .find_input_type(&format!("{}OrderByInput", comp_type.name()))
        .map(|input_type| {
            let ((variants, into_pv_arms), field_stuff): ((Vec<_>, Vec<_>), Vec<_>) = input_type
                .fields
                .iter()
                .flat_map(|field| {
                    let field_name_str = &field.name;
                    let field_name_pascal = pascal_ident(&field.name);

                    let typ_ref = &field.input_types[0];
                    let typ = typ_ref.to_tokens(
                        &quote!(super::),
                        &FieldArity::Required,
                        &args.schema.db,
                    )?;

                    let pv = match &typ_ref.location {
                        TypeLocation::EnumTypes | TypeLocation::Scalar => quote!(param.into()),
                        TypeLocation::InputObjectTypes => quote! {
                            #pcr::PrismaValue::Object(
                                param.into_iter().map(Into::into).collect()
                            )
                        },
                        _ => return None,
                    };

                    Some((
                        (
                            quote!(#field_name_pascal(#typ)),
                            quote! {
                            Self::#field_name_pascal(param) => (
                            #field_name_str,
                            #pv
                            )
                            },
                        ),
                        (
                            field_name_str,
                            (
                                typ_ref.to_tokens(
                                    &quote!(),
                                    &FieldArity::Required,
                                    &args.schema.db,
                                )?,
                                quote! {
                                impl From<Order> for super::OrderByWithRelationParam {
                                fn from(Order(v): Order) -> Self {
                                Self::#field_name_pascal(v)
                                }
                                }
                                },
                            ),
                        ),
                    ))
                })
                .unzip();

            (
                quote! {
                    #[derive(Debug, Clone)]
                    pub enum OrderByParam {
                        #(#variants),*
                    }

                    impl Into<(String, #pcr::PrismaValue)> for OrderByParam {
                        fn into(self) -> (String, #pcr::PrismaValue) {
                            let (k, v) = match self {
                                #(#into_pv_arms),*
                            };

                            (k.to_string(), v)
                        }
                    }
                },
                field_stuff,
            )
        })
        .unwrap_or_default();

    quote! {
        #[derive(Debug, Clone)]
        pub enum OrderByParam {
            #(#variants),*
        }

        impl Into<(String, #pcr::PrismaValue)> for OrderByParam {
            fn into(self) -> (String, #pcr::PrismaValue) {
                let (k, v) = match self {
                    #(#into_pv_arms),*
                };

                (k.to_string(), v)
            }
        }
    }
}
