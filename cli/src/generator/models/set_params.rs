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

pub fn relation_field_set_params(field: &dml::RelationField) -> Vec<RelationSetParamConfig> {
    match field.is_list() {
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

            if field.arity.is_optional() {
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
    field: &dml::Field,
    args: &GenerateArgs,
    module_path: &TokenStream,
) -> Option<Vec<SetParam>> {
    let field_name_pascal = pascal_ident(field.name());
    let field_name_snake = snake_ident(field.name());

    let pcr = quote!(::prisma_client_rust);

    Some(match &field {
        dml::Field::ScalarField(scalar_field) => {
            let field_type = field.type_tokens(module_path)?;

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

            if let Some(write_type) = args.write_filter(&scalar_field) {
                for method in &write_type.methods {
                    let typ = method.type_tokens(module_path);

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
        dml::Field::RelationField(field) => relation_field_set_params(field).iter().map(|param| {
            let action = param.action;
            let relation_model_name_snake = snake_ident(&field.relation_info.referenced_model);
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
        dml::Field::CompositeField(cf) => {
        	let field_type_snake = snake_ident(&cf.composite_type);

	        let set_variant = {
		        let variant_name = format_ident!("Set{}", &field_name_pascal);

				let contents = cf.arity.wrap_type(&quote!(super::#field_type_snake::Create));
				let value_ident = format_ident!("value");
				let value = cf.arity.wrap_pv(&value_ident, quote! {
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

			let unset_variant = cf.arity.is_optional().then(|| {
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

			let params = [
				Some(set_variant),
				unset_variant
			];

			params.into_iter().flatten().collect()
        },
    })
}

pub fn enum_definition(
    model: &dml::Model,
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

    let unchecked_enum = {
        let (variants, into_pv_arms): (Vec<_>, Vec<_>) = model
            .scalar_fields()
            .flat_map(|field| {
                let field_name_pascal = pascal_ident(&field.name);

                let set_variant = format_ident!("Set{}", field_name_pascal);

                let field_type = field.field_type.to_tokens(module_path, &field.arity)?;

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
