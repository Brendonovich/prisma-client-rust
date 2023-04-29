use prisma_client_rust_sdk::prisma::{
    prisma_models::walkers::{FieldWalker, ModelWalker, RefinedFieldWalker, RelationFieldWalker},
    psl::parser_database::ScalarFieldType,
};

use crate::generator::prelude::*;

pub struct RelationSetParamConfig {
    pub action: &'static str,
    pub typ: RelationSetParamType,
}

pub enum RelationSetParamType {
    /// Arguments are Vec of UniqueWhereParams
    Many,
    /// Arguments is a single WhereParam
    Single,
    /// No arguments, value is Boolean(true)
    True,
}

pub fn relation_field_set_params(field: RelationFieldWalker) -> Vec<RelationSetParamConfig> {
    let arity = field.ast_field().arity;

    match arity.is_list() {
        true => ["connect", "disconnect", "set"]
            .iter()
            .map(|action| RelationSetParamConfig {
                action,
                typ: RelationSetParamType::Many,
            })
            .collect(),
        false => {
            let mut params = vec![RelationSetParamConfig {
                action: "connect",
                typ: RelationSetParamType::Single,
            }];

            if arity.is_optional() {
                params.push(RelationSetParamConfig {
                    action: "disconnect",
                    typ: RelationSetParamType::True,
                });
            }

            params
        }
    }
}

struct SetParam {
    variant: TokenStream,
    into_pv_arm: TokenStream,
}

fn field_set_params(
    field: FieldWalker,
    args: &GenerateArgs,
    module_path: &TokenStream,
) -> Option<Vec<SetParam>> {
    let field_name_pascal = pascal_ident(field.name());
    let field_name_snake = snake_ident(field.name());

    let pcr = quote!(::prisma_client_rust);

    let arity = field.ast_field().arity;

    Some(match field.refine() {
        RefinedFieldWalker::Scalar(scalar_field) => {
            match scalar_field.scalar_field_type() {
                ScalarFieldType::CompositeType(id) => {
                    let comp_type = field.db.walk(id);

                    let field_type_snake = snake_ident(comp_type.name());

                    let set_variant = {
                        let variant_name = format_ident!("Set{field_name_pascal}");

                        let contents = arity.wrap_type(&quote!(super::#field_type_snake::Create));
                        let value_ident = format_ident!("value");
                        let value = arity.wrap_pv(&value_ident, quote! {
                            #pcr::PrismaValue::Object(value
                                .to_params()
                                .into_iter()
                                .map(Into::into)
                                .collect()
                            )
                        });

                        SetParam {
                            variant: quote!(#variant_name(#contents)),
                            into_pv_arm: quote! {
                                SetParam::#variant_name(#value_ident) =>
                                    (#field_name_snake::NAME.to_string(), #value)
                            },
                        }
                    };

                    let unset_variant = arity.is_optional().then(|| {
                        let variant_name = format_ident!("Unset{}", &field_name_pascal);

                        SetParam {
                            variant: quote!(#variant_name),
                            into_pv_arm: quote! {
                                SetParam::#variant_name => (
                                    #field_name_snake::NAME.to_string(),
                                    #pcr::PrismaValue::Object(vec![(
                                        "unset".to_string(),
                                        #pcr::PrismaValue::Boolean(true)
                                    )])
                                )
                            },
                        }
                    });

                    let update_variant = (!arity.is_list()).then(|| {
                        let variant_name = format_ident!("Update{field_name_pascal}");

                        SetParam {
                            variant: quote!(#variant_name(Vec<super::#field_type_snake::SetParam>)),
                            into_pv_arm: quote! {
                                SetParam::#variant_name(value) =>
                                    (#field_name_snake::NAME.to_string(),
                                        #pcr::PrismaValue::Object(vec![(
                                            "update".to_string(),
                                                #pcr::PrismaValue::Object(value
                                                .into_iter()
                                                .map(Into::into)
                                                .collect()
                                            )
                                        )])
                                    )
                            },
                        }
                    });

                    let upsert_variant = arity.is_optional().then(|| {
                        let variant_name = format_ident!("Upsert{field_name_pascal}");

                        SetParam {
                            variant: quote!(#variant_name(
                                super::#field_type_snake::Create,
                                Vec<super::#field_type_snake::SetParam>
                            )),
                            into_pv_arm: quote! {
                                SetParam::#variant_name(create, update) =>
                                    (#field_name_snake::NAME.to_string(),
                                        #pcr::PrismaValue::Object(vec![(
                                            "upsert".to_string(),
                                            #pcr::PrismaValue::Object(vec![
                                                (
                                                    "set".to_string(),
                                                    #pcr::PrismaValue::Object(
                                                        create
                                                            .to_params()
                                                            .into_iter()
                                                            .map(Into::into)
                                                            .collect()
                                                    )
                                                ),
                                                (
                                                    "update".to_string(),
                                                    #pcr::PrismaValue::Object(
                                                        update
                                                            .into_iter()
                                                            .map(Into::into)
                                                            .collect()
                                                    )
                                                )
                                            ])
                                        )])
                                    )
                            },
                        }
                    });

                    let push_variant = arity.is_list().then(|| {
                        let variant_name = format_ident!("Push{field_name_pascal}");

                        SetParam {
                            variant: quote!(#variant_name(Vec<super::#field_type_snake::Create>)),
                            into_pv_arm: quote! {
                                SetParam::#variant_name(creates) => (
                                    #field_name_snake::NAME.to_string(),
                                    #pcr::PrismaValue::Object(vec![(
                                        "push".to_string(),
                                        #pcr::PrismaValue::List(
                                            creates
                                                .into_iter()
                                                .map(|create| #pcr::PrismaValue::Object(
                                                    create
                                                        .to_params()
                                                        .into_iter()
                                                        .map(Into::into)
                                                        .collect()
                                                ))
                                                .collect()
                                        )
                                    )])
                                )
                            }
                        }
                    });

                    let update_many_variant = arity.is_list().then(|| {
                        let variant_name = format_ident!("UpdateMany{field_name_pascal}");

                        SetParam {
                            variant: quote!(#variant_name(
                                Vec<super::#field_type_snake::WhereParam>,
                                Vec<super::#field_type_snake::SetParam>
                            )),
                            into_pv_arm: quote! {
                                SetParam::#variant_name(_where, updates) => (
                                    #field_name_snake::NAME.to_string(),
                                    #pcr::PrismaValue::Object(vec![(
                                        "updateMany".to_string(),
                                        #pcr::PrismaValue::Object(vec![
                                            (
                                                "where".to_string(),
                                                #pcr::PrismaValue::Object(
                                                    _where
                                                        .into_iter()
                                                        .map(#pcr::WhereInput::serialize)
                                                        .map(#pcr::SerializedWhereInput::transform_equals)
                                                        .collect()
                                                )
                                            ),
                                            (
                                                "data".to_string(),
                                                #pcr::PrismaValue::Object(
                                                    updates
                                                        .into_iter()
                                                        .map(Into::into)
                                                        .collect()
                                                )
                                            )
                                        ])
                                    )])
                                )
                            }
                        }
                    });

                    let delete_many_variant = arity.is_list().then(|| {
                        let variant_name = format_ident!("DeleteMany{field_name_pascal}");

                        SetParam {
                            variant: quote!(#variant_name(
                                Vec<super::#field_type_snake::WhereParam>
                            )),
                            into_pv_arm: quote! {
                                SetParam::#variant_name(_where) => (
                                    #field_name_snake::NAME.to_string(),
                                    #pcr::PrismaValue::Object(vec![(
                                        "deleteMany".to_string(),
                                        #pcr::PrismaValue::Object(vec![
                                            (
                                                "where".to_string(),
                                                #pcr::PrismaValue::Object(
                                                    _where
                                                        .into_iter()
                                                        .map(#pcr::WhereInput::serialize)
                                                        .map(#pcr::SerializedWhereInput::transform_equals)
                                                        .collect()
                                                )
                                            ),
                                        ])
                                    )])
                                )
                            }
                        }
                    });

                    let params = [
                        Some(set_variant),
                        unset_variant,
                        update_variant,
                        upsert_variant,
                        push_variant,
                        update_many_variant,
                        delete_many_variant
                    ];

                    params.into_iter().flatten().collect()
                },
                _ => {
                    let field_type = field.type_tokens(&quote!(super))?;

                    let converter = field.type_prisma_value(&format_ident!("value"))?;

                    let set_variant_name = format_ident!("Set{}", &field_name_pascal);

                    let set_variant = SetParam {
                        variant: quote!(#set_variant_name(#field_type)),
                        into_pv_arm: quote! {
                            SetParam::#set_variant_name(value) => (
                                #field_name_snake::NAME.to_string(),
                                #converter
                            )
                        },
                    };

                    let mut params = vec![set_variant];

                    if let Some(write_type) = args.write_filter(scalar_field) {
                        for method in &write_type.methods {
                            let typ = method.type_tokens(module_path, &field.db);

                            let prisma_value_converter = method.base_type.to_prisma_value(&format_ident!("value"), &method.arity()).unwrap();

                            let variant_name = format_ident!("{}{}", pascal_ident(&method.name), field_name_pascal);

                            let action = &method.action;
                            params.push(SetParam {
                                variant: quote!(#variant_name(#typ)),
                                into_pv_arm: quote! {
                                    SetParam::#variant_name(value) => (
                                        #field_name_snake::NAME.to_string(),
                                        #pcr::PrismaValue::Object(
                                            vec![(
                                                #action.to_string(),
                                                #prisma_value_converter
                                            )]
                                        )
                                    )
                                }
                            });
                        }
                    }

                    params
                }
            }
        }
        RefinedFieldWalker::Relation(relation_field) => relation_field_set_params(relation_field).iter().map(|param| {
            let action = param.action;
            let relation_model_name_snake = snake_ident(relation_field.related_model().name());
            let variant_name = format_ident!("{}{}", pascal_ident(action), &field_name_pascal);

            match param.typ {
                RelationSetParamType::Many => {
                    SetParam {
                        variant: quote!(#variant_name(Vec<super::#relation_model_name_snake::UniqueWhereParam>)),
                        into_pv_arm: quote! {
                            SetParam::#variant_name(where_params) => (
                                #field_name_snake::NAME.to_string(),
                                #pcr::PrismaValue::Object(
                                    vec![(
                                        #action.to_string(),
                                        #pcr::PrismaValue::List(
                                            where_params
                                                .into_iter()
                                                .map(Into::<super::#relation_model_name_snake::WhereParam>::into)
                                                .map(#pcr::WhereInput::serialize)
                                                .map(#pcr::SerializedWhereInput::transform_equals)
                                                .map(|v| #pcr::PrismaValue::Object(vec![v]))
                                                .collect()
                                        )
                                    )]
                                )
                            )
                        }
                    }
                }
                RelationSetParamType::Single => {
                    SetParam {
                        variant: quote!(#variant_name(super::#relation_model_name_snake::UniqueWhereParam)),
                        into_pv_arm: quote! {
                            SetParam::#variant_name(where_param) => (
                                #field_name_snake::NAME.to_string(),
                                #pcr::PrismaValue::Object(
                                    vec![(
                                        #action.to_string(),
                                        #pcr::PrismaValue::Object(
                                            [where_param]
                                                .into_iter()
                                                .map(Into::<super::#relation_model_name_snake::WhereParam>::into)
                                                .map(#pcr::WhereInput::serialize)
                                                .map(#pcr::SerializedWhereInput::transform_equals)
                                                .collect()
                                        )
                                    )]
                                )
                            )
                        }
                    }
                }
                RelationSetParamType::True => {
                    SetParam {
                        variant: quote!(#variant_name),
                        into_pv_arm: quote! {
                            SetParam::#variant_name => (
                                #field_name_snake::NAME.to_string(),
                                #pcr::PrismaValue::Object(
                                    vec![(
                                        #action.to_string(),
                                        #pcr::PrismaValue::Boolean(true)
                                    )]
                                )
                            )
                        }
                    }
                }
            }
        }).collect(),
    })
}

pub fn enum_definition(
    model: ModelWalker,
    args: &GenerateArgs,
    module_path: &TokenStream,
) -> TokenStream {
    let (variants, into_pv_arms): (Vec<_>, Vec<_>) = model
        .fields()
        .flat_map(|f| field_set_params(f, args, module_path))
        .flatten()
        .map(|p| (p.variant, p.into_pv_arm))
        .unzip();

    let pcr = quote!(::prisma_client_rust);

    let unchecked_enum =
        {
            let (variants, into_pv_arms): (Vec<_>, Vec<_>) = model
            .scalar_fields()
            .flat_map(|field| {
                let field_name_pascal = pascal_ident(field.name());

                let set_variant = format_ident!("Set{}", field_name_pascal);

                let field_type = match field.scalar_field_type() {
                    ScalarFieldType::CompositeType(id) => {
                        let comp_type = model.db.walk(id);

                        let comp_type_snake = snake_ident(comp_type.name());

                        field.ast_field().arity.wrap_type(&quote!(super::#comp_type_snake::Create))
                    }
                    t => t.to_tokens(&quote!(super), &field.ast_field().arity, &field.db)?,
                };

                Some((
                    quote!(#field_name_pascal(#field_type)),
                    quote! {
                        UncheckedSetParam::#field_name_pascal(value) => Self::#set_variant(value)
                    },
                ))
            })
            .unzip();

            quote! {
                #[derive(Clone)]
                pub enum UncheckedSetParam {
                      #(#variants),*
                }

                impl From<UncheckedSetParam> for SetParam {
                    fn from(param: UncheckedSetParam) -> Self {
                        match param {
                             #(#into_pv_arms),*
                         }
                    }
                }
            }
        };

    quote! {
        #[derive(Clone)]
        pub enum SetParam {
            #(#variants),*
        }

        impl From<SetParam> for (String, #pcr::PrismaValue) {
            fn from(param: SetParam) -> Self {
                match param {
                    #(#into_pv_arms),*
                }
            }
        }

        #unchecked_enum
    }
}
