use std::collections::BTreeMap;

use prisma_client_rust_sdk::prisma::{
    dmmf::TypeLocation,
    prisma_models::{
        walkers::{FieldWalker, ModelWalker, RefinedFieldWalker, ScalarFieldWalker},
        FieldArity,
    },
    psl::parser_database::ScalarFieldType,
};

use crate::generator::prelude::*;

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
    },
    CompoundUniqueVariant {
        field_names_string: String,
        variant_data_destructured: Vec<Ident>,
        variant_data_types: Vec<TokenStream>,
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
        }
    }
}

pub fn collate_entries(entries: Vec<Variant>, model: ModelWalker) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let where_ident = where_input_ident(model);
    let where_unique_ident = where_unique_input_ident(model);

    let optional_unique_impls: Vec<_> = entries.iter().filter_map(|e| match e {
        Variant::UniqueVariant {
            field_name,
            field_required_type,
            read_filter_name,
            optional,
        } => {
            let field_pascal = pascal_ident(field_name);
            let field_snake = snake_ident(field_name);

            let filter_enum = format_ident!("{}Filter", &read_filter_name);

            let optional_unique_impls = optional.then(|| {
                quote!{
                    impl ::prisma_client_rust::FromOptionalUniqueArg<#field_snake::Set> for #where_ident {
                        type Arg = Option<#field_required_type>;

                        fn from_arg(arg: Self::Arg) -> Self where Self: Sized {
                            Self::#field_pascal(super::_prisma::#filter_enum::Equals(arg))
                        }
                    }

                    impl ::prisma_client_rust::FromOptionalUniqueArg<#field_snake::Set> for #where_unique_ident {
                        type Arg = #field_required_type;

                        fn from_arg(arg: Self::Arg) -> Self where Self: Sized {
                            Self::#field_pascal(arg)
                        }
                    }
                }
            });

            optional_unique_impls
        }
        _ => None,
    }).collect();

    quote! {
        pub type WhereInput = #where_ident;
        pub type WhereUniqueInput = #where_unique_ident;
        // #[derive(Clone)]
        // pub enum WhereParam {
        //     #(#variants),*
        // }

        // impl #pcr::WhereInput for WhereParam {
        //     fn serialize(self) -> #pcr::SerializedWhereInput {
        //         let (name, value) = match self {
        //             #(#to_serialized_where),*
        //         };

        //         #pcr::SerializedWhereInput::new(name.to_string(), value.into())
        //     }
        // }

        // #[derive(Clone)]
        // pub enum UniqueWhereParam {
        //     #(#unique_variants),*
        // }

        // impl From<UniqueWhereParam> for WhereParam {
        //     fn from(value: UniqueWhereParam) -> Self {
        //         match value {
        //             #(#unique_to_where_arms),*
        //         }
        //     }
        // }

        #(#optional_unique_impls)*

        impl From<#pcr::Operator<Self>> for WhereInput {
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

    let where_unique = where_unique_input_ident(model);

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

    unique_field_combos(model).iter().for_each(|fields| {
        if fields.len() == 1 {
            let field = fields[0];

            let read_filter = args.read_filter(
                field
            ).unwrap();

            entries.push(Variant::unique(field, read_filter, module_path));
        } else {
            let variant_name_string = fields.iter().map(|f| pascal_ident(f.name()).to_string()).collect::<String>();
            let variant_name = format_ident!("{}Equals", &variant_name_string);

            let variant_data_names = fields.iter().map(|f| snake_ident(f.name())).collect::<Vec<_>>();

            let ((_, field_types), (prisma_values, field_names_snake)):
                ((Vec<_>, Vec<_>), (Vec<_>, Vec<_>)) = fields.into_iter().map(|field| {
                let field_type = match field.ast_field().arity {
                    FieldArity::List | FieldArity::Required => field.type_tokens(module_path),
                    FieldArity::Optional => field.scalar_field_type().to_tokens(module_path, &FieldArity::Required, field.db)
                }.unwrap();

                let field_name_snake = snake_ident(field.name());

                (
                    (quote!(#field_name_snake: #field_type), field_type),
                    (field.scalar_field_type().to_prisma_value(&field_name_snake, &FieldArity::Required), field_name_snake)
                )
            }).unzip();

            let field_names_joined = fields.iter().map(|f| f.name()).collect::<Vec<_>>().join("_");

            entries.extend([
                Variant::BaseVariant {
                    definition: quote!(#variant_name(#(#field_types),*)),
                    match_arm: quote! {
                        Self::#variant_name(#(#field_names_snake),*) => (
                            #field_names_joined,
                            #pcr::SerializedWhereValue::Object(vec![#((#variant_data_names::NAME.to_string(), #prisma_values)),*])
                        )
                    },
                },
                Variant::CompoundUniqueVariant {
                    field_names_string: variant_name_string.clone(),
                    variant_data_destructured: field_names_snake.clone(),
                    variant_data_types: field_types
                }
            ]);
        }
    });

    let compound_field_accessors = args
        .dmmf
        .schema
        .find_input_type(&where_unique_input_ident(model).to_string())
        .map(|t| {
            t.fields
                .iter()
                .map(|f| (&f.name, f))
                // duplicate fields aren't uncommon
                .collect::<BTreeMap<_, _>>()
                .into_values()
                .filter_map(|f| {
                    let field_name_snake = snake_ident(&f.name);
                    let field_name_pascal = pascal_ident(&f.name);

                    let input_type = &f.input_types[0];

                    if !matches!(input_type.location, TypeLocation::InputObjectTypes) {
                        return None;
                    }

                    let input_type = args.dmmf.schema.find_input_type(&input_type.typ).unwrap();

                    let type_name = format_ident!("{}", &input_type.name);

                    let (field_names, field_types): (Vec<_>, Vec<_>) = input_type
                        .fields
                        .iter()
                        .map(|f| (snake_ident(&f.name), f.type_tokens(&quote!(super::), input_type, args)))
                        .unzip();

                    Some(quote! {
                        pub fn #field_name_snake<T: From<#type_name>>(#(#field_names: #field_types),*) -> T {
                            #type_name {
                                #(#field_names),*
                            }.into()
                        }

                        impl From<#type_name> for #where_unique {
                            fn from(v: #type_name) -> Self {
                                Self::#field_name_pascal(v)
                            }
                        }
                    })
                })
                .collect::<TokenStream>()
        });

    let (field_stuff, field_where_param_entries): (_, Vec<_>) = model
        .fields()
        .filter(|f| f.ast_field().field_type.as_unsupported().is_none())
        .map(|f| field_module(f, args, module_path))
        .unzip();

    entries.extend(field_where_param_entries.into_iter().flatten());

    let collated_entries = collate_entries(entries, model);

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

    let arity = field.ast_field().arity;

    let field_module_contents = match field.refine() {
        RefinedFieldWalker::Relation(_) => {
            // let relation_model_name_snake = snake_ident(relation_field.related_model().name());

            // if let FieldArity::Optional = arity {
            //     where_param_entries.push(Variant::BaseVariant {
            //         definition: quote!(#is_null_variant),
            //         match_arm: quote! {
            //             Self::#is_null_variant => (
            //                 #field_name_snake::NAME,
            //                 #pcr::SerializedWhereValue::Value(#pcr::PrismaValue::Null)
            //             )
            //         },
            //     });
            // };

            // let relation_methods = field.relation_methods().iter().map(|method| {
            // 	let method_action_string = method.to_case(Case::Camel, false);
            // 	let variant_name = format_ident!("{}{}", &field_name_pascal, pascal_ident(method));
            // 	let method_name_snake = snake_ident(method);

            // 	where_param_entries.push(Variant::BaseVariant {
            // 		definition: quote!(#variant_name(Vec<super::#relation_model_name_snake::WhereInput>)),
            // 		match_arm: quote! {
            // 			Self::#variant_name(where_params) => (
            // 				#field_name_snake::NAME,
            // 				#pcr::SerializedWhereValue::Object(vec![(
            // 					#method_action_string.to_string(),
            // 					#pcr::PrismaValue::Object(
            // 						where_params
            // 							.into_iter()
            // 							.map(#pcr::WhereInput::serialize)
            // 							.map(#pcr::SerializedWhereInput::transform_equals)
            // 							.collect()
            // 					),
            // 				)])
            // 			)
            // 		},
            // 	});

            // 	quote! {
            // 		pub fn #method_name_snake(value: Vec<#relation_model_name_snake::WhereInput>) -> WhereInput {
            // 			WhereInput::#field_name_pascal(value)
            // 		}
            // 	}
            // }).collect::<TokenStream>();

            quote! {
                // #relation_methods
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
                                pub fn is_set() -> WhereInput {
                                    WhereInput::#where_param_variant
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
                                quote!(Vec<#module_path::#comp_type_snake::WhereInput>);

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
                                pub fn equals(params: Vec<#content_type>) -> WhereInput {
                                    WhereInput::#where_param_variant(params)
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
                                pub fn is_empty() -> WhereInput {
                                    WhereInput::#where_param_variant
                                }
                            }
                        };

                        let general_filters = ["every", "some", "none"].iter().map(|method| {
                            let method_snake = snake_ident(method);
                            let method_pascal = pascal_ident(method);

                            let where_param_variant =
                                format_ident!("{field_name_pascal}{method_pascal}");
                            let content_type =
                                quote!(Vec<#module_path::#comp_type_snake::WhereInput>);

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
                                pub fn #method_snake(params: #content_type) -> WhereInput {
                                    WhereInput::#where_param_variant(params)
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
                                    quote!(Vec<#module_path::#comp_type_snake::WhereInput>);

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
                                    pub fn #method_snake(params: #content_type) -> WhereInput {
                                        WhereInput::#where_param_variant(params)
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
					let filter_enum = format_ident!("{}Filter", match read_filter.name.as_str() {
                        "Boolean" => "Bool",
                        n => n
                    });

					let model = field.model();

                    let where_ident = where_input_ident(model);
                    let where_unique_ident = where_unique_input_ident(model);

					// Add equals query functions. Unique/Where enum variants are added in unique/primary key sections earlier on.
					let equals = match (
						scalar_field.is_single_pk(),
						model.indexes().any(|idx| {
							let mut fields = idx.fields();
							idx.is_unique() && fields.len() == 1 && fields.next().map(|f| f.field_id()) == Some(scalar_field.field_id())
						}),
						arity.is_required()
					) {
						(true, _, _) | (_, true, true) => {
                            quote! {
                                struct Equals(#field_type);

                                pub fn equals<T: From<Equals>>(value: #field_type) -> T {
                                    Equals(value).into()
                                }

                                impl From<Equals> for #where_ident {
                                    fn from(Equals(value): Equals) -> Self {
                                        Self::#field_name_pascal(#filter_enum::Equals(value))
                                    }
                                }

                                impl From<Equals> for #where_unique_ident {
                                    fn from(Equals(value): Equals) -> Self {
                                        Self::#field_name_pascal(value)
                                    }
                                }
                            }
						},
						(_, true, false) => quote! {
							pub fn equals<A, T: #pcr::FromOptionalUniqueArg<Set, Arg = A>>(value: A) -> T {
								T::from_arg(value)
							}
						},
						(_, _, _) => quote! {
							pub fn equals<T: From<WhereInput>>(value: #field_type) -> T {
								WhereInput(#filter_enum::Equals(value)).into()
							}
						}
					};

					where_param_entries.push(Variant::BaseVariant {
						definition: quote!(#field_name_pascal(super::_prisma::#filter_enum)),
						match_arm: quote! {
							Self::#field_name_pascal(value) => (
								#field_name_snake::NAME,
								value.into()
							)
						},
					});

					quote! {
						#equals
					}
				});

                quote! {
                    #read_fns
                }
            }
        },
    };

    let input_type = args
        .dmmf
        .schema
        .find_input_type(&format!("{}WhereInput", capitalize(field.model().name())))
        .unwrap();

    let field_ref = input_type
        .fields
        .iter()
        .find(|f| f.name.as_str() == field.name())
        .unwrap();

    let new_stuff = {
        let field_type_ref = field_ref.primary_input_type();
        let field_type_name_pascal = pascal_ident(&field_type_ref.typ);
        let field_type = args
            .dmmf
            .schema
            .find_input_type(&field_type_ref.typ)
            .unwrap();

        let fns = field_type
            .fields
            .iter()
            .filter(|f| f.name != "equals")
            .map(|field| {
                let method_name_snake = snake_ident(match field.name.as_str() {
                    "in" => "inVec",
                    "notIn" => "notInVec",
                    n => n,
                });
                let method_name_pascal = pascal_ident(&field.name);

                match field.arity() {
                    FieldArity::Optional => {
                        let extra_data = field.extra_data(field_type, args);

                        let typ = extra_data.meta_wrapper.wrap_type(field.raw_type_tokens(&quote!(), args));

                        let method_null_snake = format_ident!("{}_null", method_name_snake);

                        quote! {
                            pub fn #method_null_snake<T: From<WhereInput>>() -> T {
                                WhereInput(#field_type_name_pascal::#method_name_pascal(None)).into()
                            }

                            pub fn #method_name_snake<T: From<WhereInput>>(value: #typ) -> T {
                                WhereInput(#field_type_name_pascal::#method_name_pascal(Some(value))).into()
                            }
                        }
                    }
                    _ => {
                        let typ = field.type_tokens(&quote!(), &field_type, args);

                        quote! {
                            pub fn #method_name_snake<T: From<WhereInput>>(value: #typ) -> T {
                                WhereInput(#field_type_name_pascal::#method_name_pascal(value)).into()
                            }
                        }
                    }
                }
            });

        let value_ident = format_ident!("value");
        let value = match field_ref.arity() {
            FieldArity::Optional if !field_type_ref.typ.ends_with("NullableFilter") => {
                quote!(Some(#value_ident))
            }
            _ => quote!(#value_ident),
        };

        let nested_scalar_from_impl = args
            .dmmf
            .schema
            .find_input_type(&format!("Nested{}", field_type_ref.typ))
            .map(|typ| {
                let name = format_ident!("{}", &typ.name);

                quote! {
                    impl From<WhereInput> for #name {
                        fn from(WhereInput(v): WhereInput) -> Self {
                            v.into()
                        }
                    }
                }
            });

        quote! {
            struct WhereInput(#field_type_name_pascal);

            impl From<WhereInput> for super::WhereInput {
                fn from(WhereInput(#value_ident): WhereInput) -> Self {
                    Self::#field_name_pascal(#value)
                }
            }

            #nested_scalar_from_impl

            #(#fns)*
        }
    };

    (
        (
            field_name.to_string(),
            quote! {
                #field_module_contents

                #new_stuff
            },
        ),
        where_param_entries,
    )
}

pub fn where_input_ident(model: ModelWalker) -> Ident {
    format_ident!("{}WhereInput", capitalize(model.name()))
}

pub fn where_unique_input_ident(model: ModelWalker) -> Ident {
    format_ident!("{}WhereUniqueInput", capitalize(model.name()))
}
