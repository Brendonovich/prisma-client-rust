use crate::generator::prelude::*;

use super::{include, order_by, pagination, select, where_params::Variant, with_params};

pub fn module(
    root_field: &dml::Field,
    model: &dml::Model,
    args: &GenerateArgs,
    module_path: &TokenStream,
) -> (TokenStream, Vec<Variant>) {
    let pcr = quote!(::prisma_client_rust);
    let mut where_param_entries = vec![];

    let field_name = root_field.name();
    let field_name_pascal = pascal_ident(field_name);
    let field_name_snake = snake_ident(field_name);
    let field_type = root_field.type_tokens(&quote!(self));

    let connect_variant = format_ident!("Connect{field_name_pascal}");
    let disconnect_variant = format_ident!("Disconnect{field_name_pascal}");
    let set_variant = format_ident!("Set{field_name_pascal}");
    let is_null_variant = format_ident!("{field_name_pascal}IsNull");
    let equals_variant = format_ident!("{field_name_pascal}Equals");

    let field_module_contents = match root_field {
        dml::Field::RelationField(field) => {
            let relation_model_name_snake = snake_ident(&field.relation_info.referenced_model);

            let with_fn = with_params::builder_fn(&field);

            let base = match field.arity {
                dml::FieldArity::List => {
                    let order_by_fn = order_by::fetch_builder_fn(&relation_model_name_snake);
                    let pagination_fns = pagination::fetch_builder_fns(&relation_model_name_snake);

                    quote! {
                        pub struct Fetch(pub #relation_model_name_snake::ManyArgs);

                        impl Fetch {
                            #with_fn

                            #order_by_fn

                            #pagination_fns
                        }

                        impl From<Fetch> for WithParam {
                            fn from(Fetch(v): Fetch) -> Self {
                                WithParam::#field_name_pascal(v)
                            }
                        }

                        pub fn fetch(params: Vec<#relation_model_name_snake::WhereParam>) -> Fetch {
                            Fetch(#relation_model_name_snake::ManyArgs::new(params))
                        }

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
                    let optional_fns = field.arity.is_optional().then(|| {
                        where_param_entries.push(Variant::BaseVariant {
                            definition: quote!(#is_null_variant),
                            match_arm: quote! {
                                Self::#is_null_variant => (
                                    #field_name_snake::NAME,
                                    #pcr::SerializedWhereValue::Value(#pcr::PrismaValue::Null)
                                )
                            },
                        });

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
                        pub struct Fetch(pub #relation_model_name_snake::UniqueArgs);

                        impl Fetch {
                            #with_fn
                        }

                        impl From<Fetch> for WithParam {
                            fn from(Fetch(v): Fetch) -> Self {
                                WithParam::#field_name_pascal(v)
                            }
                        }

                        pub fn fetch() -> Fetch {
                            Fetch(#relation_model_name_snake::UniqueArgs::new())
                        }

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

            let relation_methods = root_field.relation_methods().iter().map(|method| {
                let method_action_string = method.to_case(Case::Camel, false);
                let variant_name = format_ident!("{}{}", &field_name_pascal, pascal_ident(method));
                let method_name_snake = snake_ident(method);

                where_param_entries.push(Variant::BaseVariant {
                    definition: quote!(#variant_name(Vec<super::#relation_model_name_snake::WhereParam>)),
                    match_arm: quote! {
                        Self::#variant_name(where_params) => (
                            #field_name_snake::NAME,
                            #pcr::SerializedWhereValue::Object(vec![(
                                #method_action_string.to_string(),
                                #pcr::PrismaValue::Object(
                                    where_params
                                        .into_iter()
                                        .map(#pcr::WhereInput::serialize)
                                        .map(#pcr::SerializedWhereInput::transform_equals)
                                        .collect()
                                ),
                            )])
                        )
                    },
                });

                quote! {
                    pub fn #method_name_snake(value: Vec<#relation_model_name_snake::WhereParam>) -> WhereParam {
                        WhereParam::#variant_name(value)
                    }
                }
            }).collect::<TokenStream>();

            quote! {
                #base

                #relation_methods
            }
        }
        dml::Field::ScalarField(field) => {
            let read_fns = args.read_filter(&field).map(|read_filter| {
                let filter_enum = format_ident!("{}Filter", &read_filter.name);

                // Add equals query functions. Unique/Where enum variants are added in unique/primary key sections earlier on.
                let equals = match (model.field_is_primary(field_name), model.field_is_unique(field_name), field.arity.is_optional()) {
                    (true, _, _) | (_, true, false) => quote! {
                        pub fn equals<T: From<UniqueWhereParam>>(value: #field_type) -> T {
                            UniqueWhereParam::#equals_variant(value).into()
                        }
                    },
                    (_, true, true) => quote! {
                        pub fn equals<A, T: #pcr::FromOptionalUniqueArg<Set, Arg = A>>(value: A) -> T {
                            T::from_arg(value)
                        }
                    },
                    (false, false, _) => quote! {
                        pub fn equals(value: #field_type) -> WhereParam {
                            WhereParam::#field_name_pascal(_prisma::read_filters::#filter_enum::Equals(value))
                        }
                    }
                };

                where_param_entries.push(Variant::BaseVariant {
                    definition: quote!(#field_name_pascal(_prisma::read_filters::#filter_enum)),
                    match_arm: quote! {
                        Self::#field_name_pascal(value) => (
                            #field_name_snake::NAME,
                            value.into()
                        )
                    },
                });

                let read_methods = read_filter.methods.iter().filter_map(|method| {
                    if method.name == "Equals" { return None }

                    let method_name_snake = snake_ident(&method.name);
                    let method_name_pascal = pascal_ident(&method.name);

                    let typ = method.type_tokens(&quote!(super::super));

                    Some(quote!(fn #method_name_snake(_: #typ) -> #method_name_pascal;))
                });

                quote! {
                    #equals

                    #pcr::scalar_where_param_fns!(
                        _prisma::read_filters::#filter_enum,
                        #field_name_pascal,
                        { #(#read_methods)* }
                    );
                }
            });

            let write_fns = args.write_filter(&field).map(|write_type| {
                write_type
                    .methods
                    .iter()
                    .map(|method| {
                        let method_name_snake = snake_ident(&method.name);

                        let typ = method.type_tokens(module_path);

                        let variant_name =
                            format_ident!("{}{}", pascal_ident(&method.name), field_name_pascal);

                        quote! {
                            pub fn #method_name_snake(value: #typ) -> SetParam {
                                SetParam::#variant_name(value)
                            }
                        }
                    })
                    .collect::<TokenStream>()
            });

            quote! {
                pub struct Set(pub #field_type);

                impl From<Set> for SetParam {
                    fn from(Set(v): Set) -> Self {
                        Self::#set_variant(v)
                    }
                }

                impl From<Set> for UncheckedSetParam {
                    fn from(Set(v): Set) -> Self {
                        Self::#field_name_pascal(v)
                    }
                }

                pub fn set<T: From<Set>>(value: #field_type) -> T {
                    Set(value).into()
                }

                pub fn order(direction: #pcr::Direction) -> OrderByParam {
                    OrderByParam::#field_name_pascal(direction)
                }

                #read_fns

                #write_fns
            }
        }
        dml::Field::CompositeField(cf) => {
            let comp_type_snake = snake_ident(&cf.composite_type);

            let comp_type = args
                .dml
                .composite_types
                .iter()
                .find(|ty| ty.name == cf.composite_type)
                .unwrap();

            // Filters

            let single_filters = (!cf.arity.is_list())
                .then(|| {
                    ["equals", "is", "isNot"]
                        .iter()
                        .map(|method| {
                            let method_snake = snake_ident(method);
                            let method_pascal = pascal_ident(method);

                            let where_param_variant =
                                format_ident!("{field_name_pascal}{method_pascal}");
                            let content_type =
                                quote!(Vec<#module_path::#comp_type_snake::WhereParam>);

                            where_param_entries.push(Variant::BaseVariant {
                                definition: quote!(#where_param_variant(#content_type)),
                                match_arm: quote! {
                                    Self::#where_param_variant(where_params) => (
                                        #field_name_snake::NAME,
                                        #pcr::SerializedWhereValue::Object(vec![(
                                            #method.to_string(),
                                            #pcr::PrismaValue::Object(
                                                where_params
                                                    .into_iter()
                                                    .map(#pcr::WhereInput::serialize)
                                                    .map(#pcr::SerializedWhereInput::transform_equals)
                                                    .collect()
                                            )
                                        )])
                                     )
                                },
                            });

                            quote! {
                                pub fn #method_snake(params: #content_type) -> WhereParam {
                                    WhereParam::#where_param_variant(params)
                                }
                            }
                        })
                        .collect()
                })
                .unwrap_or(vec![]);

            let optional_filters = cf
                .arity
                .is_optional()
                .then(|| {
                    let is_set_filter = {
                        let where_param_variant = format_ident!("{field_name_pascal}IsSet");

                        where_param_entries.push(Variant::BaseVariant {
                            definition: quote!(#where_param_variant),
                            match_arm: quote! {
                                Self::#where_param_variant => (
                                    #field_name_snake::NAME,
                                    #pcr::SerializedWhereValue::Value(
                                        #pcr::PrismaValue::Boolean(true)
                                    )
                                 )
                            },
                        });

                        quote! {
                            pub fn is_set() -> WhereParam {
                                WhereParam::#where_param_variant
                            }
                        }
                    };

                    vec![is_set_filter]
                })
                .unwrap_or(vec![]);

            let many_filters = cf
                .arity
                .is_list()
                .then(|| {
                    let equals_filter = {
                        let where_param_variant = format_ident!("{field_name_pascal}Equals");
                        let content_type = quote!(Vec<#module_path::#comp_type_snake::WhereParam>);

                        where_param_entries.push(Variant::BaseVariant {
                            definition: quote!(#where_param_variant(Vec<#content_type>)),
                            match_arm: quote! {
                                Self::#where_param_variant(where_params) => (
                                    #field_name_snake::NAME,
                                    #pcr::SerializedWhereValue::Object(vec![(
                                        "equals".to_string(),
                                        #pcr::PrismaValue::List(
                                        	where_params
                                         		.into_iter()
                                           		.map(|params|
	                                         		#pcr::PrismaValue::Object(
			                                            params
			                                                .into_iter()
			                                                .map(#pcr::WhereInput::serialize)
			                                                .map(#pcr::SerializedWhereInput::transform_equals)
			                                                .collect()
	                                           		)
	                                         	)
												.collect()
                                        )
                                    )])
                                )
                            },
                        });

                        quote! {
                            pub fn equals(params: Vec<#content_type>) -> WhereParam {
                                WhereParam::#where_param_variant(params)
                            }
                        }
                    };

                    let is_empty_filter = {
                        let where_param_variant = format_ident!("{field_name_pascal}IsEmpty");

                        where_param_entries.push(Variant::BaseVariant {
                            definition: quote!(#where_param_variant),
                            match_arm: quote! {
                                Self::#where_param_variant => (
                                    #field_name_snake::NAME,
                                    #pcr::SerializedWhereValue::Object(vec![(
                                        "isEmpty".to_string(),
                                        #pcr::PrismaValue::Boolean(true)
                                    )])
                                )
                            },
                        });

                        quote! {
                            pub fn is_empty() -> WhereParam {
                                WhereParam::#where_param_variant
                            }
                        }
                    };

                    let general_filters = ["every", "some", "none"].iter().map(|method| {
                        let method_snake = snake_ident(method);
                        let method_pascal = pascal_ident(method);

                        let where_param_variant =
                            format_ident!("{field_name_pascal}{method_pascal}");
                        let content_type = quote!(Vec<#module_path::#comp_type_snake::WhereParam>);

                        where_param_entries.push(Variant::BaseVariant {
                            definition: quote!(#where_param_variant(#content_type)),
                            match_arm: quote! {
                            Self::#where_param_variant(where_params) => (
                                #field_name_snake::NAME,
                                #pcr::SerializedWhereValue::Object(vec![(
                                    #method.to_string(),
                                    #pcr::PrismaValue::Object(
                                        where_params
                                            .into_iter()
                                            .map(#pcr::WhereInput::serialize)
                                            .map(#pcr::SerializedWhereInput::transform_equals)
                                            .collect()
                                    )
                                )])
                                )
                            },
                        });

                        quote! {
                            pub fn #method_snake(params: #content_type) -> WhereParam {
                                WhereParam::#where_param_variant(params)
                            }
                        }
                    });

                    general_filters
                        .chain([equals_filter, is_empty_filter])
                        .collect()
                })
                .unwrap_or(vec![]);

            // Writes

            let set_fn = comp_type
                .fields
                .iter()
                .filter(|f| f.required_on_create())
                .map(|field| {
                    field.type_tokens(module_path)?;
                    Some(field)
                })
                .collect::<Option<Vec<_>>>()
                .map(|_| {
                    let create_struct = cf.arity.wrap_type(&quote!(#comp_type_snake::Create));

                    quote! {
                        pub struct Set(#create_struct);

                        pub fn set<T: From<Set>>(create: #create_struct) -> T {
                            Set(create).into()
                        }

                        impl From<Set> for SetParam {
                            fn from(Set(create): Set) -> Self {
                                 SetParam::#set_variant(create)
                            }
                        }

                        impl From<Set> for UncheckedSetParam {
                            fn from(Set(create): Set) -> Self {
                                 UncheckedSetParam::#field_name_pascal(create)
                            }
                        }
                    }
                });

            let unset_fn = cf.arity.is_optional().then(|| {
                let set_param_variant = format_ident!("Unset{field_name_pascal}");

                quote! {
                    pub fn unset() -> SetParam {
                        SetParam::#set_param_variant
                    }
                }
            });
            let update_fn = (!cf.arity.is_list()).then(|| {
                let set_param_variant = format_ident!("Update{field_name_pascal}");

                quote! {
                    pub fn update(params: Vec<#comp_type_snake::SetParam>) -> SetParam {
                        SetParam::#set_param_variant(params)
                    }
                }
            });
            let upsert_fn = cf.arity.is_optional().then(|| {
                let set_param_variant = format_ident!("Upsert{field_name_pascal}");

                quote! {
                    pub fn upsert(
                        create: #comp_type_snake::Create,
                        update: Vec<#comp_type_snake::SetParam>
                    ) -> SetParam {
                        SetParam::#set_param_variant(create, update)
                    }
                }
            });
            let push_fn = cf.arity.is_list().then(|| {
                let set_param_variant = format_ident!("Push{field_name_pascal}");

                quote! {
                    pub fn push(creates: Vec<#comp_type_snake::Create>) -> SetParam {
                        SetParam::#set_param_variant(creates)
                    }
                }
            });
            let update_many_fn = cf.arity.is_list().then(|| {
                let set_param_variant = format_ident!("UpdateMany{field_name_pascal}");

                quote! {
                    pub fn update_many(
                        _where: Vec<#comp_type_snake::WhereParam>,
                        update: Vec<#comp_type_snake::SetParam>
                    ) -> SetParam {
                        SetParam::#set_param_variant(_where, update)
                    }
                }
            });
            let delete_many_fn = cf.arity.is_list().then(|| {
                let set_param_variant = format_ident!("DeleteMany{field_name_pascal}");

                quote! {
                    pub fn delete_many(
                        _where: Vec<#comp_type_snake::WhereParam>,
                    ) -> SetParam {
                        SetParam::#set_param_variant(_where)
                    }
                }
            });

            let order_by_fn = (!cf.arity.is_list()).then(|| {
                quote! {
                    pub fn order(params: Vec<#comp_type_snake::OrderByParam>) -> OrderByParam {
                          OrderByParam::#field_name_pascal(params)
                      }
                }
            });

            quote! {
                #(#single_filters)*
                #(#optional_filters)*
                #(#many_filters)*

                #set_fn
                #unset_fn
                #update_fn
                #upsert_fn
                #push_fn
                #update_many_fn
                #delete_many_fn

                #order_by_fn
            }
        }
    };

    let include_enum = include::field_module_enum(&root_field, &pcr);
    let select_enum = select::field_module_enum(&root_field, &pcr);

    (
        quote! {
            pub mod #field_name_snake {
                use super::super::*;
                use super::{WhereParam, UniqueWhereParam, OrderByParam, WithParam, SetParam, UncheckedSetParam};
                use super::_prisma::*;

                pub const NAME: &str = #field_name;

                #field_module_contents

                #include_enum
                #select_enum
            }
        },
        where_param_entries,
    )
}
