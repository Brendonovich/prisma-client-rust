use std::collections::BTreeMap;

use prisma_client_rust_sdk::prisma::{
    prisma_models::{
        walkers::{FieldWalker, ModelWalker, RefinedFieldWalker, RelationFieldWalker},
        FieldArity,
    },
    psl::parser_database::ScalarFieldType,
};

use crate::{prelude::*, write_params};

use super::ModelModulePart;

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

fn relation_field_set_params(field: RelationFieldWalker) -> Vec<RelationSetParamConfig> {
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

fn field_set_params(
    field: FieldWalker,
    args: &GenerateArgs,
) -> Option<(Vec<TokenStream>, Vec<TokenStream>, (String, TokenStream))> {
    let field_name_pascal = pascal_ident(field.name());
    let field_name_snake = snake_ident(field.name());
    let field_type = field.type_tokens(&quote!());

    let pcr = quote!(::prisma_client_rust);

    let arity = field.ast_field().arity;

    let mut variants = vec![];
    let mut functions = vec![];

    let field_module_contents = match field.refine() {
        RefinedFieldWalker::Scalar(scalar_field) => match scalar_field.scalar_field_type() {
            ScalarFieldType::CompositeType(id) => {
                let comp_type = field.db.walk(id);
                let comp_type_snake = snake_ident(comp_type.name());

                let field_type_snake = snake_ident(comp_type.name());

                let set_variant = comp_type
                    .fields()
                    .filter(|f| f.required_on_create())
                    .map(|field| {
                        field.type_tokens(&quote!())?;
                        Some(field)
                    })
                    .collect::<Option<Vec<_>>>()
                    .map(|_| {
                        let variant_name = format_ident!("Set{field_name_pascal}");

                        let value_ident = format_ident!("value");
                        let value = arity.wrap_pv(
                            &value_ident,
                            quote! {
                                #pcr::PrismaValue::Object(value
                                    .to_params()
                                    .into_iter()
                                    .map(Into::into)
                                    .collect()
                                )
                            },
                        );

                        let base_type = quote!(#comp_type_snake::Create);

                        let model_type = arity.wrap_type(&quote!(super::#base_type));
                        let field_type = arity.wrap_type(&base_type);

                        (
                            (
                                quote!(#variant_name(#model_type)),
                                quote! {
                                    Self::#variant_name(#value_ident) =>
                                        (#field_name_snake::NAME, #value)
                                },
                            ),
                            quote! {
                                pub struct Set(#field_type);

                                pub fn set<T: From<Set>>(create: Impl Into<#field_type>) -> T {
                                    Set(create.into()).into()
                                }

                                impl From<Set> for SetParam {
                                    fn from(Set(create): Set) -> Self {
                                         SetParam::#variant_name(create)
                                    }
                                }
                            },
                        )
                    });

                let unset_variant = arity.is_optional().then(|| {
                    let variant_name = format_ident!("Unset{field_name_pascal}");

                    (
                        (
                            quote!(#variant_name),
                            quote! {
                                Self::#variant_name => (
                                    #field_name_snake::NAME,
                                    #pcr::PrismaValue::Object(vec![(
                                        "unset".to_string(),
                                        #pcr::PrismaValue::Boolean(true)
                                    )])
                                )
                            },
                        ),
                        quote! {
                            pub fn unset() -> SetParam {
                                SetParam::#variant_name
                            }
                        },
                    )
                });

                let update_variant = (!arity.is_list()).then(|| {
                    let variant_name = format_ident!("Update{field_name_pascal}");

                    (
                        (
                            quote!(#variant_name(Vec<super::#field_type_snake::SetParam>)),
                            quote! {
                                Self::#variant_name(value) =>
                                    (#field_name_snake::NAME,
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
                        ),
                        quote! {
                            pub fn update(params: Vec<#comp_type_snake::SetParam>) -> SetParam {
                                SetParam::#variant_name(params)
                            }
                        },
                    )
                });

                let upsert_variant = arity.is_optional().then(|| {
                    let variant_name = format_ident!("Upsert{field_name_pascal}");

                    (
                        (
                            quote!(#variant_name(
                                super::#field_type_snake::Create,
                                Vec<super::#field_type_snake::SetParam>
                            )),
                            quote! {
                                Self::#variant_name(create, update) =>
                                    (#field_name_snake::NAME,
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
                        ),
                        quote! {
                            pub fn upsert(
                                create: #comp_type_snake::Create,
                                update: Vec<#comp_type_snake::SetParam>
                            ) -> SetParam {
                                SetParam::#variant_name(create, update)
                            }
                        },
                    )
                });

                let push_variant = arity.is_list().then(|| {
                    let variant_name = format_ident!("Push{field_name_pascal}");

                    (
                        (
                            quote!(#variant_name(Vec<super::#field_type_snake::Create>)),
                            quote! {
                                Self::#variant_name(creates) => (
                                    #field_name_snake::NAME,
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
                            },
                        ),
                        quote! {
                            pub fn push(creates: Vec<#comp_type_snake::Create>) -> SetParam {
                                SetParam::#variant_name(creates)
                            }
                        },
                    )
                });

                let update_many_variant = arity.is_list().then(|| {
                    let variant_name = format_ident!("UpdateMany{field_name_pascal}");

                    (
                        (
	                        quote!(#variant_name(
	                            Vec<super::#field_type_snake::WhereParam>,
	                            Vec<super::#field_type_snake::SetParam>
	                        )),
                            quote! {
                            	Self::#variant_name(_where, updates) => (
                                    #field_name_snake::NAME,
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
						),
						quote! {
							pub fn update_many(
								_where: Vec<#comp_type_snake::WhereParam>,
								update: Vec<#comp_type_snake::SetParam>
							) -> SetParam {
								SetParam::#variant_name(_where, update)
							}
						}
                    )
                });

                let delete_many_variant = arity.is_list().then(|| {
                    let variant_name = format_ident!("DeleteMany{field_name_pascal}");

                    (
                        (
                            quote!(#variant_name(
                                Vec<super::#field_type_snake::WhereParam>
                            )),
                            quote! {
                                Self::#variant_name(_where) => (
                                    #field_name_snake::NAME,
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
                        ),
                        quote! {
                            pub fn delete_many(
                                _where: Vec<#comp_type_snake::WhereParam>,
                            ) -> SetParam {
                                SetParam::#variant_name(_where)
                            }
                        }
                    )
                });

                let params = [
                    set_variant,
                    unset_variant,
                    update_variant,
                    upsert_variant,
                    push_variant,
                    update_many_variant,
                    delete_many_variant,
                ];

                let ((v, f), field_fns): ((Vec<_>, Vec<_>), TokenStream) =
                    params.into_iter().flatten().unzip();

                variants.extend(v);
                functions.extend(f);

                (field.name().to_string(), field_fns)
            }
            _ => {
                if let Some(write_param) = args.write_param(scalar_field) {
                    let param_enum = write_params::enum_name(write_param);

                    let param_enum_path = quote!(_prisma::write_params::#param_enum);

                    let other_fns = write_param
	                    .fields
	                    .iter()
	                    .flat_map(|field| {
		                    if field.name == "set" { return None }

		                    let method_name_snake = snake_ident(&field.name);
		                    let method_name_pascal = pascal_ident(&field.name);

		                    let typ = field.type_tokens(&quote!());

		                    Some(quote! {
			                    pub fn #method_name_snake<T: From<UpdateOperation>>(value: #typ) -> T {
				                    UpdateOperation(#param_enum_path::#method_name_pascal(value)).into()
			                    }
		                    })
	                    })
	                    .collect::<TokenStream>();

                    variants.push(
                        quote!(#field_name_pascal(super::_prisma::write_params::#param_enum)),
                    );
                    functions.push(quote! {
                        Self::#field_name_pascal(value) => (
                            #field_name_snake::NAME,
                            value.into()
                        )
                    });

                    (
                        field.name().to_string(),
                        quote! {
                            pub struct Set(pub #field_type);

                            impl From<Set> for SetParam {
                                fn from(Set(v): Set) -> Self {
                                    Self::#field_name_pascal(#param_enum_path::Set(v))
                                }
                            }

                            pub fn set<T: From<Set>>(value: #field_type) -> T {
                                Set(value).into()
                            }

                            pub struct UpdateOperation(pub #param_enum_path);

                            impl From<UpdateOperation> for SetParam {
                                fn from(UpdateOperation(v): UpdateOperation) -> Self {
                                    Self::#field_name_pascal(v)
                                }
                            }

                            #other_fns
                        },
                    )
                } else {
                    return None;
                }
            }
        },
        RefinedFieldWalker::Relation(relation_field) => {
            let (v, f): (Vec<_>, Vec<_>) = relation_field_set_params(relation_field).iter().map(|param| {
                let action = param.action;
                let relation_model_name_snake = snake_ident(relation_field.related_model().name());
                let variant_name = format_ident!("{}{}", pascal_ident(action), &field_name_pascal);

                match param.typ {
                    RelationSetParamType::Many => {
                        (
                        	quote!(#variant_name(Vec<super::#relation_model_name_snake::UniqueWhereParam>)),
                            quote! {
                                Self::#variant_name(where_params) => (
                                    #field_name_snake::NAME,
                                    #pcr::PrismaValue::Object(
                                        vec![(
                                            #action.to_string(),
                                            #pcr::PrismaValue::List(
                                                where_params
                                                    .into_iter()
                                                    .map(#pcr::WhereInput::serialize)
                                                    .map(#pcr::SerializedWhereInput::transform_equals)
                                                    .map(|v| #pcr::PrismaValue::Object(vec![v]))
                                                    .collect()
                                            )
                                        )]
                                    )
                                )
                            }
                        )
                    }
                    RelationSetParamType::Single => {
                        (quote!(#variant_name(super::#relation_model_name_snake::UniqueWhereParam)),
                            quote! {
                                Self::#variant_name(where_param) => (
                                    #field_name_snake::NAME,
                                    #pcr::PrismaValue::Object(
                                        vec![(
                                            #action.to_string(),
                                            #pcr::PrismaValue::Object(
                                                [where_param]
                                                    .into_iter()
                                                    .map(#pcr::WhereInput::serialize)
                                                    .map(#pcr::SerializedWhereInput::transform_equals)
                                                    .collect()
                                            )
                                        )]
                                    )
                                )
                            }
                        )
                    }
                    RelationSetParamType::True => {
                        (quote!(#variant_name),
                            quote! {
                                Self::#variant_name => (
                                    #field_name_snake::NAME,
                                    #pcr::PrismaValue::Object(
                                        vec![(
                                            #action.to_string(),
                                            #pcr::PrismaValue::Boolean(true)
                                        )]
                                    )
                                )
                            }
                        )
                    }
                }
            }).unzip();

            let relation_model_name_snake = snake_ident(relation_field.related_model().name());

            let connect_variant = format_ident!("Connect{field_name_pascal}");
            let disconnect_variant = format_ident!("Disconnect{field_name_pascal}");
            let set_variant = format_ident!("Set{field_name_pascal}");
            let is_null_variant = format_ident!("{field_name_pascal}IsNull");

            let base = match arity {
                FieldArity::List => {
                    quote! {
                        pub struct Connect(pub Vec<#relation_model_name_snake::UniqueWhereParam>);

                        impl From<Connect> for SetParam {
                            fn from(Connect(v): Connect) -> Self {
                                Self::#connect_variant(v)
                            }
                        }

                        pub fn connect<T: From<Connect>>(params: Vec<#relation_model_name_snake::UniqueWhereParam>) -> T {
                            Connect(params).into()
                        }

                        pub fn disconnect(params: Vec<#relation_model_name_snake::UniqueWhereParam>) -> SetParam {
                            SetParam::#disconnect_variant(params)
                        }

                        pub fn set(params: Vec<#relation_model_name_snake::UniqueWhereParam>) -> SetParam {
                            SetParam::#set_variant(params)
                        }
                    }
                }
                _ => {
                    let optional_fns = arity.is_optional().then(|| {
                        quote! {
                            pub fn disconnect() -> SetParam {
                                SetParam::#disconnect_variant
                            }

                            pub fn is_null() -> WhereParam {
                                WhereParam::#is_null_variant
                            }
                        }
                    });

                    quote! {
                        pub struct Connect(#relation_model_name_snake::UniqueWhereParam);

                        impl From<Connect> for SetParam {
                            fn from(Connect(v): Connect) -> Self {
                                Self::#connect_variant(v)
                            }
                        }

                        pub fn connect<T: From<Connect>>(value: #relation_model_name_snake::UniqueWhereParam) -> T {
                            Connect(value).into()
                        }

                        #optional_fns
                    }
                }
            };

            variants.extend(v);
            functions.extend(f);

            (field.name().to_string(), base)
        }
    };

    Some((variants, functions, field_module_contents))
}

pub fn model_data(model: ModelWalker, args: &GenerateArgs) -> ModelModulePart {
    let (variants, into_pv_arms, field_stuff) =
        model.fields().flat_map(|f| field_set_params(f, args)).fold(
            (vec![], vec![], BTreeMap::new()),
            |(mut a, mut b, mut c), (d, e, f)| {
                a.extend(d);
                b.extend(e);
                c.insert(f.0, f.1);

                (a, b, c)
            },
        );

    let pcr = quote!(::prisma_client_rust);

    let (unchecked_enum, unchecked_fields) = {
        let ((variants, into_pv_arms), field_stuff): ((Vec<_>, Vec<_>), Vec<_>) = model
            .scalar_fields()
            .flat_map(|field| {
                let field_name_str = field.name();
                let field_name_pascal = pascal_ident(field_name_str);
                let field_name_snake = snake_ident(field_name_str);

                Some(match field.scalar_field_type() {
                    ScalarFieldType::CompositeType(id) => {
                        let comp_type = model.db.walk(id);

                        let comp_type_snake = snake_ident(comp_type.name());

                        let base_type = quote!(#comp_type_snake::Create);

                        let arity = &field.ast_field().arity;

                        let model_type = arity
	                        .wrap_type(&quote!(super::#base_type));

                        let value_ident = format_ident!("value");
                        let value = arity.wrap_pv(
                            &value_ident,
                            quote! {
                                #pcr::PrismaValue::Object(value
                                    .to_params()
                                    .into_iter()
                                    .map(Into::into)
                                    .collect()
                                )
                            },
                        );

                        ((
                        	quote!(#field_name_pascal(#model_type)),
                            quote! {
	                            Self::#field_name_pascal(#value_ident) =>
	                                (#field_name_snake::NAME, #value)
                            },
                        ), (
                        	field.name().to_string(),
                         	quote! {
		                        impl From<Set> for UncheckedSetParam {
		                            fn from(Set(v): Set) -> Self {
		                                Self::#field_name_pascal(v)
		                            }
		                        }
                         	}
                        ))
                    }
                    ScalarFieldType::Unsupported(_) => return None,
                    _ => args.write_param(field).map(|write_param| {
                        let param_enum = write_params::enum_name(write_param);
                        let param_enum_path = quote!(_prisma::write_params::#param_enum);

                        (
                            (
	                            quote!(#field_name_pascal(super::_prisma::write_params::#param_enum)),
	                            quote! {
	                                Self::#field_name_pascal(value) => (
	                                    #field_name_str,
	                                    value.into()
	                                )
	                            }),
                            (
                                field.name().to_string(),
                                quote! {
                                    impl From<Set> for UncheckedSetParam {
                                        fn from(Set(v): Set) -> Self {
                                            Self::#field_name_pascal(#param_enum_path::Set(v))
                                        }
                                    }

                                    impl From<UpdateOperation> for UncheckedSetParam {
                                        fn from(UpdateOperation(v): UpdateOperation) -> Self {
                                            Self::#field_name_pascal(v)
                                        }
                                    }
                                },
                            ),
                        )
                    })?,
                })
            })
            .unzip();

        (
            quote! {
                #[derive(Debug, Clone)]
                pub enum UncheckedSetParam {
                      #(#variants),*
                }

                impl Into<(String, #pcr::PrismaValue)> for UncheckedSetParam {
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
    };

    ModelModulePart {
        data: quote! {
            #[derive(Debug, Clone)]
            pub enum SetParam {
                #(#variants),*
            }

            impl Into<(String, #pcr::PrismaValue)> for SetParam {
                fn into(self) -> (String, #pcr::PrismaValue) {
                    let (k, v) = match self {
                        #(#into_pv_arms),*
                    };

                    (k.to_string(), v)
                }
            }

            #unchecked_enum
        },
        fields: unchecked_fields
            .into_iter()
            .fold(field_stuff, |mut acc, (k, v)| {
                let entry = acc.entry(k).or_insert_with(|| quote!());
                entry.extend(v);
                acc
            }),
    }
}
