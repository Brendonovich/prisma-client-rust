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

fn field_set_params(field: &dml::Field, args: &GenerateArgs) -> Vec<SetParam> {
    let field_name_pascal = pascal_ident(field.name());
    let field_name_str = field.name();
    
    let pcr = quote!(::prisma_client_rust);

    match &field {
        dml::Field::ScalarField(scalar_field) => {
            let field_type = field.type_tokens();

            let converter = field.type_prisma_value(&format_ident!("value"));
            let converter = field
                .arity()
                .is_optional()
                .then(|| quote!(value.map(|value| #converter).unwrap_or(#pcr::PrismaValue::Null)))
                .unwrap_or_else(|| converter);

            let set_variant_name = format_ident!("Set{}", &field_name_pascal);

            let set_variant = SetParam {
                variant: quote!(#set_variant_name(#field_type)),
                into_pv_arm: quote! {
                    SetParam::#set_variant_name(value) => (
                        #field_name_str.to_string(),
                        #converter
                    )
                },
            };

            let mut params = vec![set_variant];

            if let Some(write_type) = args.write_filter(&scalar_field) {
                for method in &write_type.methods {
                    let typ = method.type_tokens();
                    
                    let prisma_value_converter = method.base_type.to_prisma_value(&format_ident!("value"), method.is_list);

                    let variant_name = format_ident!("{}{}", pascal_ident(&method.name), field_name_pascal);
                    
                    let action = &method.action;
                    params.push(SetParam {
                        variant: quote!(#variant_name(#typ)),
                        into_pv_arm: quote! {
                            SetParam::#variant_name(value) => (
                                #field_name_str.to_string(),
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
            let relation_model_name_snake = snake_ident(&field.relation_info.to);
            let variant_name = format_ident!("{}{}", pascal_ident(action), &field_name_pascal);

            match param.typ {
                RelationSetParamType::Many => {
                    SetParam {
                        variant: quote!(#variant_name(Vec<super::#relation_model_name_snake::UniqueWhereParam>)),
                        into_pv_arm: quote! {
                            SetParam::#variant_name(where_params) => (
                                #field_name_str.to_string(),
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
                                #field_name_str.to_string(),
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
                                #field_name_str.to_string(),
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
        dml::Field::CompositeField(_) => panic!("Composite fields are not supported!"),
    }
}

pub fn enum_definition(model: &dml::Model, args: &GenerateArgs) -> TokenStream {
    let set_params = model
        .fields()
        .map(|f| field_set_params(f, args))
        .flatten()
        .collect::<Vec<_>>();

    let variants = set_params.iter().map(|p| &p.variant);
    let into_pv_arms = set_params.iter().map(|p| &p.into_pv_arm);

    let pcr = quote!(::prisma_client_rust);

    quote! {
        #[derive(Clone)]
        pub enum SetParam {
            #(#variants),*
        }

        impl Into<(String, #pcr::PrismaValue)> for SetParam {
            fn into(self) -> (String, #pcr::PrismaValue) {
                match self {
                    #(#into_pv_arms),*
                }
            }
        }
    }
}
