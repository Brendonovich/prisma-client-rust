use prisma_client_rust_sdk::prisma::{
    prisma_models::{
        walkers::{FieldWalker, ModelWalker, RefinedFieldWalker, ScalarFieldWalker},
        FieldArity,
    },
    psl::parser_database::ScalarFieldType,
};

use crate::prelude::*;

use super::ModelModulePart;

pub struct Operator {
    pub name: &'static str,
    pub action: &'static str,
    pub list: bool,
}

static OPERATORS: &'static [Operator] = &[
    Operator {
        name: "Not",
        action: "NOT",
        list: false,
    },
    Operator {
        name: "Or",
        action: "OR",
        list: true,
    },
    Operator {
        name: "And",
        action: "AND",
        list: false,
    },
];

#[derive(Debug)]
pub enum Variant {
    BaseVariant {
        definition: TokenStream,
        match_arm: TokenStream,
    },
    UniqueVariant {
        field_name: String,
        field_required_type: TokenStream,
        read_filter_name: String,
        optional: bool,
        value: TokenStream,
    },
    CompoundUniqueVariant {
        field_names_string: String,
        variant_data_types: Vec<TokenStream>,
        match_arm: TokenStream,
    },
}

impl Variant {
    pub fn unique(
        field: ScalarFieldWalker,
        read_filter: &Filter,
        module_path: &TokenStream,
    ) -> Self {
        Self::UniqueVariant {
            field_name: field.name().to_string(),
            field_required_type: field
                .scalar_field_type()
                .to_tokens(module_path, &FieldArity::Required, field.db)
                .unwrap(),
            read_filter_name: read_filter.name.to_string(),
            optional: field.ast_field().arity.is_optional(),
            value: {
                let value = field
                    .scalar_field_type()
                    .to_prisma_value(
                        &format_ident!("value"),
                        &match field.ast_field().arity {
                            FieldArity::Optional => FieldArity::Required,
                            a => a,
                        },
                    )
                    .unwrap();

                quote!(::prisma_client_rust::SerializedWhereValue::Value(#value))
            },
        }
    }
}

pub fn collate_entries(entries: Vec<Variant>) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let (variants, to_serialized_where): (Vec<_>, Vec<_>) = entries
        .iter()
        .filter_map(|e| match e {
            Variant::BaseVariant {
                definition,
                match_arm,
            } => Some((definition.clone(), Some(match_arm))),
            _ => None,
        })
        .unzip();

    let (optional_unique_impls, (unique_variants, unique_to_serialized_where)): (
        Vec<_>,
        (Vec<_>, Vec<_>),
    ) = entries
        .iter()
        .filter_map(|e| match e {
            Variant::UniqueVariant {
                field_name,
                field_required_type,
                read_filter_name,
                optional,
                value,
            } => {
                let field_pascal = pascal_ident(field_name);
                let field_snake = snake_ident(field_name);

                let variant_name = format_ident!("{}Equals", &field_pascal);
                let filter_enum = format_ident!("{}Filter", &read_filter_name);

                let optional_unique_impls = optional.then(|| {
                    quote! {
                        impl ::prisma_client_rust::FromOptionalUniqueArg<#field_snake::Equals> for WhereParam {
                            type Arg = Option<#field_required_type>;

                            fn from_arg(arg: Self::Arg) -> Self where Self: Sized {
                                Self::#field_pascal(super::_prisma::read_filters::#filter_enum::Equals(arg))
                            }
                        }

                        impl ::prisma_client_rust::FromOptionalUniqueArg<#field_snake::Equals> for UniqueWhereParam {
                            type Arg = #field_required_type;

                            fn from_arg(arg: Self::Arg) -> Self where Self: Sized {
                                Self::#variant_name(arg)
                            }
                        }
                    }
                });

                Some((
                    optional_unique_impls,
                    (
                        quote!(#variant_name(#field_required_type)),
                        quote!(UniqueWhereParam::#variant_name(value) => (#field_name, #value)),
                    ),
                ))
            }
            Variant::CompoundUniqueVariant {
                field_names_string,
                variant_data_types,
                match_arm,
            } => {
                let variant_name = format_ident!("{}Equals", field_names_string);

                Some((
                    None,
                    (
                        quote!(#variant_name(#(#variant_data_types),*)),
                        quote!(#match_arm),
                    ),
                ))
            }
            _ => None,
        })
        .unzip();

    quote! {
        #[derive(Debug, Clone)]
        pub enum WhereParam {
            #(#variants),*
        }

        impl #pcr::WhereInput for WhereParam {
            fn serialize(self) -> #pcr::SerializedWhereInput {
                let (name, value) = match self {
                    #(#to_serialized_where),*
                };

                #pcr::SerializedWhereInput::new(name.to_string(), value.into())
            }
        }

        #[derive(Debug, Clone)]
        pub enum UniqueWhereParam {
            #(#unique_variants),*
        }

        impl #pcr::WhereInput for UniqueWhereParam {
            fn serialize(self) -> #pcr::SerializedWhereInput {
                let (name, value) = match self {
                    #(#unique_to_serialized_where),*
                };

                #pcr::SerializedWhereInput::new(name.to_string(), value.into())
            }
        }

        #(#optional_unique_impls)*

        impl From<#pcr::Operator<Self>> for WhereParam {
            fn from(op: #pcr::Operator<Self>) -> Self {
                match op {
                    #pcr::Operator::Not(value) => Self::Not(value),
                    #pcr::Operator::And(value) => Self::And(value),
                    #pcr::Operator::Or(value) => Self::Or(value),
                }
            }
        }
    }
}

pub fn model_data(
    model: ModelWalker,
    args: &GenerateArgs,
    module_path: &TokenStream,
) -> ModelModulePart {
    let pcr = quote!(::prisma_client_rust);

    let mut entries = vec![];

    entries.extend(OPERATORS.iter().map(|op| {
        let variant_name = pascal_ident(&op.name);
        let op_action = &op.action;

        let value = match op.list {
            true => quote! {
                #pcr::SerializedWhereValue::List(
                    value
                        .into_iter()
                        .map(#pcr::WhereInput::serialize)
                        .map(|p| #pcr::PrismaValue::Object(vec![p.into()]))
                        .collect()
                )
            },
            false => quote! {
                #pcr::SerializedWhereValue::Object(
                    ::prisma_client_rust::merge_fields(
                        value
                            .into_iter()
                            .map(#pcr::WhereInput::serialize)
                            .map(Into::into)
                            .collect()
                    )
                )
            },
        };

        Variant::BaseVariant {
            definition: quote!(#variant_name(Vec<WhereParam>)),
            match_arm: quote! {
                Self::#variant_name(value) => (
                    #op_action,
                    #value,
                )
            },
        }
    }));

    let compound_field_accessors = unique_field_combos(model).iter().flat_map(|fields| {
        if fields.len() == 1 {
            let field = fields[0];

            let read_filter = args.read_filter(
                field
            ).unwrap();

            entries.push(Variant::unique(field, read_filter, module_path));

            None
        } else {
            let variant_name_string = fields.iter().map(|f| pascal_ident(f.name()).to_string()).collect::<String>();
            let variant_name = format_ident!("{}Equals", &variant_name_string);

            let variant_data_names = fields.iter().map(|f| snake_ident(f.name())).collect::<Vec<_>>();

            let ((field_defs, field_types), (prisma_values, field_names_snake)):
                ((Vec<_>, Vec<_>), (Vec<_>, Vec<_>)) = fields.into_iter().map(|field| {
                let field_type = match field.ast_field().arity {
                    FieldArity::List | FieldArity::Required => field.type_tokens(module_path),
                    FieldArity::Optional => field.scalar_field_type().to_tokens(module_path, &FieldArity::Required, field.db)
                }.unwrap();

                let field_name_snake = snake_ident(field.name());

                (
                    (quote!(#field_name_snake: Impl Into<#field_type>), field_type),
                    (field.scalar_field_type().to_prisma_value(&field_name_snake, &FieldArity::Required), field_name_snake)
                )
            }).unzip();

            let field_names_joined = fields.iter().map(|f| f.name()).collect::<Vec<_>>().join("_");

            entries.extend([
                Variant::CompoundUniqueVariant {
                    field_names_string: variant_name_string.clone(),
                    variant_data_types: field_types,
                    match_arm: quote! {
                    	Self::#variant_name(#(#field_names_snake),*) => (
                    		#field_names_joined,
                     		#pcr::SerializedWhereValue::Object(vec![#((#variant_data_names::NAME.to_string(), #prisma_values)),*])
                     	)
                    },
                }
            ]);

            let accessor_name = snake_ident(&variant_name_string);

            Some(quote! {
                pub fn #accessor_name<T: From<UniqueWhereParam>>(#(#field_defs),*) -> T {
                    UniqueWhereParam::#variant_name(#(#field_names_snake.into()),*).into()
                }
            })
        }
    }).collect::<TokenStream>();

    let (field_stuff, field_where_param_entries): (_, Vec<_>) = model
        .fields()
        .filter(|f| f.ast_field().field_type.as_unsupported().is_none())
        .map(|f| field_module(f, args, module_path))
        .unzip();

    entries.extend(field_where_param_entries.into_iter().flatten());

    let collated_entries = collate_entries(entries);

    ModelModulePart {
        data: quote! {
            #compound_field_accessors
            #collated_entries
        },
        fields: field_stuff,
    }
}

pub fn unique_field_combos(model: ModelWalker) -> Vec<Vec<ScalarFieldWalker>> {
    let mut combos = model
        .indexes()
        .filter(|f| f.is_unique())
        .map(|unique| {
            unique
                .fields()
                .filter_map(|field| {
                    model
                        .scalar_fields()
                        .find(|mf| mf.field_id() == field.field_id())
                })
                .collect()
        })
        .collect::<Vec<_>>();

    if let Some(primary_key) = model.primary_key() {
        let primary_key_is_also_unique = model.indexes().any(|i| {
            primary_key.contains_exactly_fields(
                i.fields()
                    .map(|f| f.as_scalar_field())
                    .flatten()
                    .collect::<Vec<_>>()
                    .into_iter(),
            )
        });

        if !primary_key_is_also_unique {
            combos.push(
                primary_key
                    .fields()
                    .filter_map(|field| {
                        model
                            .scalar_fields()
                            .find(|mf| mf.field_id() == field.field_id())
                    })
                    .collect(),
            );
        }
    }

    combos
}

pub fn field_module(
    field: FieldWalker,
    args: &GenerateArgs,
    module_path: &TokenStream,
) -> ((String, TokenStream), Vec<Variant>) {
    let pcr = quote!(::prisma_client_rust);
    let mut where_param_entries = vec![];

    let field_name = field.name();
    let field_name_pascal = pascal_ident(field_name);
    let field_name_snake = snake_ident(field_name);
    let field_type = field.type_tokens(&quote!());

    let is_null_variant = format_ident!("{field_name_pascal}IsNull");
    let equals_variant = format_ident!("{field_name_pascal}Equals");

    let arity = field.ast_field().arity;

    let field_module_contents = match field.refine() {
        RefinedFieldWalker::Relation(relation_field) => {
            let relation_model_name_snake = snake_ident(relation_field.related_model().name());

            if let FieldArity::Optional = arity {
                where_param_entries.push(Variant::BaseVariant {
                    definition: quote!(#is_null_variant),
                    match_arm: quote! {
                        Self::#is_null_variant => (
                            #field_name_snake::NAME,
                            #pcr::SerializedWhereValue::Value(#pcr::PrismaValue::Null)
                        )
                    },
                });
            };

            let relation_methods = field.relation_methods().iter().map(|method| {
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
                #relation_methods
            }
        }
        RefinedFieldWalker::Scalar(scalar_field) => match scalar_field.scalar_field_type() {
            ScalarFieldType::CompositeType(cf_id) => {
                let comp_type = field.db.walk(cf_id);

                let comp_type_snake = snake_ident(comp_type.name());

                // Filters

                let optional_filters = arity
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

                let many_filters: Vec<_> = arity
                    .is_list()
                    .then(|| {
                        let equals_filter = {
                            let where_param_variant = format_ident!("{field_name_pascal}Equals");
                            let content_type =
                                quote!(Vec<#module_path #comp_type_snake::WhereParam>);

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
                            let content_type =
                                quote!(Vec<#module_path #comp_type_snake::WhereParam>);

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
                    .unwrap_or_else(|| {
                        ["equals", "is", "isNot"]
                            .iter()
                            .map(|method| {
                                let method_snake = snake_ident(method);
                                let method_pascal = pascal_ident(method);

                                let where_param_variant =
                                    format_ident!("{field_name_pascal}{method_pascal}");
                                let content_type =
                                    quote!(Vec<#module_path #comp_type_snake::WhereParam>);

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
                    });

                quote! {
                    #(#optional_filters)*
                    #(#many_filters)*
                }
            }
            _ => {
                let read_fns = args.read_filter(scalar_field).map(|read_filter| {
					let filter_enum = format_ident!("{}Filter", &read_filter.name);

					let model = field.model();

					// Add equals query functions. Unique/Where enum variants are added in unique/primary key sections earlier on.
					let equals = match (
						scalar_field.is_single_pk(),
						model.indexes().any(|idx| {
							let mut fields = idx.fields();
							idx.is_unique() && fields.len() == 1 && fields.next().map(|f| f.field_id()) == Some(scalar_field.field_id())
						}),
						arity.is_required()
					) {
						(true, _, _) | (_, true, true) => quote! {
							pub fn equals<T: From<Equals>>(value: #field_type) -> T {
								Equals(value).into()
							}

							impl From<Equals> for UniqueWhereParam {
								fn from(Equals(v): Equals) -> Self {
									UniqueWhereParam::#equals_variant(v)
								}
							}
						},
						(_, true, false) => quote! {
							pub fn equals<T: #pcr::FromOptionalUniqueArg<Equals>>(value: T::Arg) -> T {
								T::from_arg(value)
							}
						},
						(_, _, _) => quote! {
							pub fn equals<T: From<Equals>>(value: #field_type) -> T {
								Equals(value).into()
							}
						}
					};

					where_param_entries.push(Variant::BaseVariant {
						definition: quote!(#field_name_pascal(super::_prisma::read_filters::#filter_enum)),
						match_arm: quote! {
							Self::#field_name_pascal(value) => (
								#field_name_snake::NAME,
								value.into()
							)
						},
					});

					let read_methods = read_filter.fields.iter().filter_map(|field| {
						let name = match field.name.as_str() {
							"equals" => return None,
							"in" => "inVec",
							"notIn" => "notInVec",
							n => n
						};

						let method_name_snake = snake_ident(name);
						let method_name_pascal = pascal_ident(name);

						let typ = field.type_tokens(&quote!());

						Some(quote!(fn #method_name_snake(_: #typ) -> #method_name_pascal;))
					});

					quote! {
						pub struct Equals(pub #field_type);

						#equals

						impl From<Equals> for WhereParam {
							fn from(Equals(v): Equals) -> Self {
								WhereParam::#field_name_pascal(_prisma::read_filters::#filter_enum::Equals(v))
							}
						}

						#pcr::scalar_where_param_fns!(
							_prisma::read_filters::#filter_enum,
							#field_name_pascal,
							{ #(#read_methods)* }
						);
					}
				});

                quote! {
                    #read_fns
                }
            }
        },
    };

    (
        (field_name.to_string(), field_module_contents),
        where_param_entries,
    )
}
