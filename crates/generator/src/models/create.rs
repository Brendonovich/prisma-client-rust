use std::{
    fmt::{self},
    str::FromStr,
};

use prisma_client_rust_sdk::prisma::{
    dmmf::TypeLocation,
    prisma_models::walkers::{FieldWalker, ModelWalker, RefinedFieldWalker},
};

use crate::prelude::*;

use super::ModelModulePart;

pub enum InputObjects {
    Create,
    UncheckedCreate,
    CreateWithout { model: String, field: String },
    UncheckedCreateWithout { model: String, field: String },
    CreateNestedOneWithout { model: String, field: String },
    UncheckedCreateNestedOneWithout { model: String, field: String },
    CreateNestedManyWithout { model: String, field: String },
    UncheckedCreateNestedManyWithout { model: String, field: String },
    CreateOrConnectWithout { model: String, field: String },
}

impl fmt::Display for InputObjects {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InputObjects::*;

        match self {
            Create => write!(f, "Create"),
            UncheckedCreate => write!(f, "UncheckedCreate"),
            CreateWithout { model, field } => write!(f, "{model}CreateWithout{field}Input"),
            UncheckedCreateWithout { model, field } => {
                write!(f, "{model}UncheckedCreateWithout{field}Input")
            }
            CreateNestedOneWithout { .. } => write!(f, "CreateNestedOneWithout"),
            UncheckedCreateNestedOneWithout { .. } => write!(f, "UncheckedCreateNestedOneWithout"),
            CreateNestedManyWithout { .. } => write!(f, "CreateNestedManyWithout"),
            UncheckedCreateNestedManyWithout { .. } => {
                write!(f, "UncheckedCreateNestedManyWithout")
            }
            CreateOrConnectWithout { model, field } => {
                write!(f, "{model}CreateOrConnectWithout{field}Input")
            }
        }
    }
}

impl FromStr for InputObjects {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::{branch::alt, bytes::complete::*, combinator::map, sequence::tuple, IResult};

        let parse = |typ: _| -> IResult<_, Self> {
            alt((
                map(
                    tuple((
                        take_until1("UncheckedCreateWithout"),
                        tag("UncheckedCreateWithout"),
                        take_until1("Input"),
                        tag("Input"),
                    )),
                    |(model, _, field, _): (&str, _, &str, _)| Self::UncheckedCreateWithout {
                        model: model.to_string(),
                        field: field.to_string(),
                    },
                ),
                map(
                    tuple((
                        take_until1("CreateWithout"),
                        tag("CreateWithout"),
                        take_until1("Input"),
                        tag("Input"),
                    )),
                    |(model, _, field, _): (&str, _, &str, _)| Self::CreateWithout {
                        model: model.to_string(),
                        field: field.to_string(),
                    },
                ),
                map(
                    tuple((
                        take_until1("UncheckedCreateNestedOneWithout"),
                        tag("UncheckedCreateNestedOneWithout"),
                        take_until1("Input"),
                        tag("Input"),
                    )),
                    |(model, _, field, _): (&str, _, &str, _)| {
                        Self::UncheckedCreateNestedOneWithout {
                            model: model.to_string(),
                            field: field.to_string(),
                        }
                    },
                ),
                map(
                    tuple((
                        take_until1("CreateNestedOneWithout"),
                        tag("CreateNestedOneWithout"),
                        take_until1("Input"),
                        tag("Input"),
                    )),
                    |(model, _, field, _): (&str, _, &str, _)| Self::CreateNestedOneWithout {
                        model: model.to_string(),
                        field: field.to_string(),
                    },
                ),
                map(
                    tuple((
                        take_until1("UncheckedCreateNestedManyWithout"),
                        tag("UncheckedCreateNestedManyWithout"),
                        take_until1("Input"),
                        tag("Input"),
                    )),
                    |(model, _, field, _): (&str, _, &str, _)| {
                        Self::UncheckedCreateNestedManyWithout {
                            model: model.to_string(),
                            field: field.to_string(),
                        }
                    },
                ),
                map(
                    tuple((
                        take_until1("CreateNestedManyWithout"),
                        tag("CreateNestedManyWithout"),
                        take_until1("Input"),
                        tag("Input"),
                    )),
                    |(model, _, field, _): (&str, _, &str, _)| Self::CreateNestedManyWithout {
                        model: model.to_string(),
                        field: field.to_string(),
                    },
                ),
                map(
                    tuple((
                        take_until1("CreateOrConnectWithout"),
                        tag("CreateOrConnectWithout"),
                        take_until1("Input"),
                        tag("Input"),
                    )),
                    |(model, _, field, _): (&str, _, &str, _)| Self::CreateOrConnectWithout {
                        model: model.to_string(),
                        field: field.to_string(),
                    },
                ),
            ))(typ)
        };

        Ok(parse(s).unwrap().1)
    }
}

impl ToTokens for InputObjects {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::CreateWithout { model, field } => {
                let model = snake_ident(model);
                let field = snake_ident(field);

                tokens.extend(quote!(#model::#field::CreateWithout));
            }
            Self::UncheckedCreateWithout { model, field } => {
                let model = snake_ident(model);
                let field = snake_ident(field);

                tokens.extend(quote!(#model::#field::UncheckedCreateWithout));
            }
            Self::CreateNestedOneWithout { model, field } => {
                let model = snake_ident(model);
                let field = snake_ident(field);

                tokens.extend(quote!(#model::#field::CreateNestedOneWithout));
            }
            Self::UncheckedCreateNestedOneWithout { model, field } => {
                let model = snake_ident(model);
                let field = snake_ident(field);

                tokens.extend(quote!(#model::#field::UncheckedCreateNestedOneWithout));
            }
            Self::CreateNestedManyWithout { model, field } => {
                let model = snake_ident(model);
                let field = snake_ident(field);

                tokens.extend(quote!(#model::#field::CreateNestedManyWithout));
            }
            Self::UncheckedCreateNestedManyWithout { model, field } => {
                let model = snake_ident(model);
                let field = snake_ident(field);

                tokens.extend(quote!(#model::#field::UncheckedCreateNestedManyWithout));
            }
            Self::CreateOrConnectWithout { model, field } => {
                let model = snake_ident(model);
                let field = snake_ident(field);

                tokens.extend(quote!(#model::#field::CreateOrConnectWithout));
            }
            _ => {
                let ident = format_ident!("{}", self.to_string());
                tokens.extend(quote!(#ident));
            }
        }
    }
}

fn create_unchecked(model: ModelWalker, args: &GenerateArgs) -> Option<ModelModulePart> {
    let pcr = quote!(::prisma_client_rust);

    let model_name_snake = snake_ident(model.name());

    let mut module = ModelModulePart {
        data: quote!(),
        fields: Default::default(),
    };

    let contents = args.dmmf
        .schema
        .find_input_type(&format!("{}UncheckedCreateInput", model.name()))
        .map(|input_type| {
            let (all_fields, required_fields): (_, Vec<_>) =
                input_type.fields.iter().flat_map(|field| {
                    let field_name_str = &field.name;
                    let field_name_pascal = pascal_ident(&field.name);
                    let field_name_snake = snake_ident(&field.name);

                    let type_ref = &field.input_types[0];
                    let typ = match type_ref.location {
                    	TypeLocation::InputObjectTypes => {
                     		let obj = type_ref.typ.parse::<InputObjects>().unwrap();

                       		quote!(super::#obj)
                    	},
                    	_ => type_ref.to_tokens(&quote!(super::), &field.arity(), &args.schema.db)?
                    };

                    module.fields.entry(field.name.clone()).or_default().extend(
	                    match type_ref.location {
							TypeLocation::InputObjectTypes => {
								let obj = type_ref.typ.parse::<InputObjects>().unwrap();
								let string = obj.to_string();
								let ident = format_ident!("{}", &string);

								quote! {
							  // 		impl From<#ident> for super::CreateUncheckedParam {
									// 	fn from(input: #ident) -> Self {
									// 		Self::#field_name_pascal(input.into())
									// 	}
									// }
								}
							},
							_ => quote! {
		                    	impl From<Set> for super::CreateUncheckedParam {
								 	fn from(input: Set) -> Self {
									 	Self::#field_name_pascal(input.0)
								 	}
								}
		                    }
						}
                    );

                    let value_ident = format_ident!("param");
                    let pv = type_ref.to_prisma_value(&value_ident, &field.arity());

                    Some((
                        (
                            quote!(#field_name_pascal(#typ)),
                            quote! {
                            	Self::#field_name_pascal(#value_ident) => (
                                    #field_name_str,
                                    #pv
                                )
                            },
                        ),
                        field
                            .is_required
                            .then(|| (field_name_snake, typ))
                    ))
                }).unzip();

            let param = {
                let (variants, into_pv_arms): (Vec<_>, Vec<_>) = all_fields;
                quote! {
                    #[derive(Clone)]
                    pub enum CreateUncheckedParam {
                        #(#variants),*
                    }

                    impl Into<(String, #pcr::PrismaValue)> for CreateUncheckedParam {
                        fn into(self) -> (String, #pcr::PrismaValue) {
                            let (k, v) = match self {
                                #(#into_pv_arms),*
                            };

                            (k.to_string(), v)
                        }
                    }
                }
            };

            let create = {
                let (names, types): (Vec<_>, Vec<_>) =
                    required_fields.into_iter().flatten().unzip();

                quote! {
                    #[derive(Clone)]
                    pub struct CreateUnchecked {
                        #(pub #names: #types,)*
                        pub _params: Vec<CreateUncheckedParam>
                    }

                    impl CreateUnchecked {
                         pub fn to_query<'a>(self, client: &'a PrismaClient) -> CreateUncheckedQuery<'a> {
                            client.#model_name_snake()
                                .create_unchecked(
                                    #(self.#names,)*
                                    self._params
                                )
                        }

                        pub fn to_params(mut self) -> Vec<CreateUncheckedParam> {
                            self._params.extend([
                                #(#names::set(self.#names)),*
                            ]);

                            self._params
                        }
                    }

                    pub fn create_unchecked(#(#names: #types,)* _params: Vec<CreateUncheckedParam>)
                        -> CreateUnchecked {
                        CreateUnchecked {
                            #(#names,)*
                            _params
                        }
                    }
                }
            };

            quote! {
                #param

                #create
            }
        });

    contents.map(|contents| {
        module.data = contents;
        module
    })
}

fn create(model: ModelWalker, args: &GenerateArgs) -> Option<ModelModulePart> {
    let pcr = quote!(::prisma_client_rust);

    let model_name_snake = snake_ident(model.name());

    let mut module = ModelModulePart {
        data: quote!(),
        fields: Default::default(),
    };

    let contents = args
        .dmmf
        .schema
        .find_input_type(&format!("{}CreateInput", model.name()))
        .map(|input_type| {
            let (all_fields, required_fields): (_, Vec<_>) = model
                .fields()
                .filter_map(|field| input_type.fields.iter().find(|f| f.name == field.name()))
                .flat_map(|field| {
                    let field_name_str = &field.name;
                    let field_name_pascal = pascal_ident(&field.name);
                    let field_name_snake = snake_ident(&field.name);

                    let type_ref = &field.input_types[0];
                    let typ = match type_ref.location {
                        TypeLocation::InputObjectTypes => {
                            let obj = type_ref.typ.parse::<InputObjects>().unwrap();

                            quote!(super::#obj)
                        }
                        _ => {
                            type_ref.to_tokens(&quote!(super::), &field.arity(), &args.schema.db)?
                        }
                    };

                    module.fields.entry(field.name.clone()).or_default().extend(
                        match type_ref.location {
                            TypeLocation::InputObjectTypes => {
                                let obj = type_ref.typ.parse::<InputObjects>().unwrap();
                                let string = obj.to_string();
                                let ident = format_ident!("{}", &string);

                                quote! {
                                    // impl From<#ident> for super::CreateParam {
                                    //     fn from(input: #ident) -> Self {
                                    //         Self::#field_name_pascal(input.into())
                                    //     }
                                    // }
                                }
                            }
                            _ => quote! {
                                impl From<Set> for super::CreateParam {
                                     fn from(input: Set) -> Self {
                                         Self::#field_name_pascal(input.0)
                                     }
                                }
                            },
                        },
                    );

                    let value_ident = format_ident!("param");
                    let pv = type_ref.to_prisma_value(&value_ident, &field.arity());

                    Some((
                        (
                            quote!(#field_name_pascal(#typ)),
                            quote! {
                                Self::#field_name_pascal(#value_ident) => (
                                    #field_name_str,
                                    #pv
                                )
                            },
                        ),
                        field
                            .is_required
                            .then(|| ((field_name_snake, field_name_pascal), typ)),
                    ))
                })
                .unzip();

            let param = {
                let (variants, into_pv_arms): (Vec<_>, Vec<_>) = all_fields;

                quote! {
                    #[derive(Clone)]
                    pub enum CreateParam {
                        #(#variants),*
                    }

                    impl Into<(String, #pcr::PrismaValue)> for CreateParam {
                        fn into(self) -> (String, #pcr::PrismaValue) {
                            let (k, v) = match self {
                                #(#into_pv_arms),*
                            };

                            (k.to_string(), v)
                        }
                    }
                }
            };

            let create = {
                let ((names, push_wrappers), types): ((Vec<_>, Vec<_>), Vec<_>) =
                    required_fields.into_iter().flatten().unzip();

                quote! {
                    #[derive(Clone)]
                    pub struct Create {
                        #(pub #names: #types,)*
                        pub _params: Vec<CreateParam>
                    }

                    impl Create {
                        pub fn to_query<'a>(self, client: &'a PrismaClient) -> CreateQuery<'a> {
                            client.#model_name_snake()
                                .create(
                                    #(self.#names,)*
                                    self._params
                                )
                        }

                        pub fn to_params(mut self) -> Vec<CreateParam> {
                            self._params.extend([
                                #(CreateParam::#push_wrappers(self.#names)),*
                            ]);

                            self._params
                        }
                    }

                    pub fn create(#(#names: #types,)* _params: Vec<CreateParam>)
                        -> Create {
                        Create {
                            #(#names,)*
                            _params
                        }
                    }
                }
            };

            quote! {
                #param

                #create
            }
        });

    contents.map(|contents| {
        module.data = contents;
        module
    })
}

fn field_stuff(field: FieldWalker, args: &GenerateArgs) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);
    let field_name_pascal = pascal_ident(field.name());
    let model_name_snake = snake_ident(field.model().name());
    let schema = &args.dmmf.schema;

    let pv = quote!(#pcr::PrismaValue);

    match field.refine() {
        RefinedFieldWalker::Relation(relation_field) => {
            let relation_model = relation_field.related_model();
            let relation_model_name_snake = snake_ident(relation_model.name());
            let opposite_relation_field = relation_field.opposite_relation_field().unwrap();

            let functions = schema
                .find_input_type(&format!("{}CreateInput", field.model().name()))
                .and_then(|input_type| {
                    input_type
                        .fields
                        .iter()
                        .find(|f| f.name == field.name())
                })
                .and_then(|field| {
                    let type_ref = &field.input_types[0];

                    Some(match type_ref.location {
                        TypeLocation::InputObjectTypes => {
                            let obj = type_ref.typ.parse::<InputObjects>().unwrap();
                            let obj = &obj;

                            match obj {
	                            InputObjects::CreateNestedOneWithout { model, field } => {
	                            	let model_name_snake = snake_ident(&model);
									let field_name_snake = snake_ident(&field);

									let field_scope = quote!(#model_name_snake::#field_name_snake);
									let create_without = quote!(#field_scope::CreateWithout);
									let create_unchecked_without = quote!(#field_scope::CreateUncheckedWithout);
									let connect_or_create_without = quote!(#field_scope::CreateOrConnectWithout);

									let create_fn = args.dmmf.schema.find_input_type(&InputObjects::CreateWithout { model: model.clone(), field: field.clone() }.to_string()).map(|typ| {
										let (names_snake, types): (Vec<_>, Vec<_>) = typ
											.fields
											.iter()
											.filter(|f| f.is_required)
											.filter_map(|field| {
												let field_name_snake = snake_ident(&field.name);

												let type_ref = &field.input_types[0];
												let typ = match type_ref.location {
													TypeLocation::InputObjectTypes => {
														let obj = type_ref.typ.parse::<InputObjects>().unwrap();

														quote!(#obj)
													},
													_ => type_ref.to_tokens(&quote!(super::), &field.arity(), &args.schema.db)?
												};

												Some((field_name_snake, typ))
											})
											.unzip();

										quote! {
											pub fn create<T: From<#create_without>>(
												#(#names_snake: #types,)*
												mut _params: Vec<#model_name_snake::CreateParam>
											) -> T {
					                            #create_without {
					                            	_params,
					                            	#(#names_snake),*
					                            }.into()
					                        }

											impl From<#create_without> for super::CreateParam {
												fn from(value: #create_without) -> Self {
													Self::#field_name_pascal(value.into())
												}
											}
										}
									});

									let create_unchecked_fn = args.dmmf.schema.find_input_type(&InputObjects::UncheckedCreateWithout { model: model.clone(), field: field.clone() }.to_string()).map(|typ| {
										let (names_snake, types): (Vec<_>, Vec<_>) = typ
											.fields
											.iter()
											.filter(|f| f.is_required)
											.filter_map(|field| {
												let field_name_snake = snake_ident(&field.name);

												let type_ref = &field.input_types[0];
												let typ = match type_ref.location {
													TypeLocation::InputObjectTypes => {
														let obj = type_ref.typ.parse::<InputObjects>().unwrap();

														quote!(#obj)
													},
													_ => type_ref.to_tokens(&quote!(super::), &field.arity(), &args.schema.db)?
												};

												Some((field_name_snake, typ))
											})
											.unzip();

										quote! {
											pub fn create_unchecked<T: From<#create_unchecked_without>>(
												#(#names_snake: #types,)*
												mut _params: Vec<#model_name_snake::CreateUncheckedParam>
											) -> T {
					                            #create_unchecked_without {
					                            	_params,
					                            	#(#names_snake),*
					                            }.into()
					                        }

											impl From<#create_unchecked_without> for super::CreateParam {
												fn from(value: #create_unchecked_without) -> Self {
													Self::#field_name_pascal(value.into())
												}
											}
										}
									});

									let connect_or_create_fn = args.dmmf.schema.find_input_type(&InputObjects::CreateOrConnectWithout { model: model.clone(), field: field.clone() }.to_string()).map(|typ| {
										quote! {
											pub fn connect_or_create<T: From<#connect_or_create_without>>(
												r#where: #model_name_snake::UniqueWhereParam,
												create: #field_scope::Create
											) -> T {
					                            #connect_or_create_without {
					                            	r#where,
					                            	create
					                            }.into()
											}

											impl From<#connect_or_create_without> for super::CreateParam {
												fn from(value: #connect_or_create_without) -> Self {
													Self::#field_name_pascal(value.into())
												}
											}

											pub struct Connect(#model_name_snake::UniqueWhereParam);

											pub fn connect<T: From<Connect>>(
												r#where: #model_name_snake::UniqueWhereParam
											) -> T {
												Connect(r#where).into()
											}

											impl From<Connect> for super::CreateParam {
												fn from(value: Connect) -> Self {
													Self::#field_name_pascal(value.0.into())
												}
											}
										}
									});

	                                quote! {
										#create_fn

										#create_unchecked_fn

										#connect_or_create_fn
	                                }
	                            },
                                InputObjects::CreateNestedManyWithout { model, field } => {
                                	let model_name_snake = snake_ident(&model);
                                 	let field_name_snake = snake_ident(&field);

                                  	let field_scope = quote!(#model_name_snake::#field_name_snake);
                                   	let create = quote!(#field_scope::Create);
                                   	let create_multiple = quote!(#field_scope::CreateMultiple);
									let create_without = quote!(#field_scope::CreateWithout);
									let create_unchecked_without = quote!(#field_scope::CreateUncheckedWithout);
									let connect_or_create_without = quote!(#field_scope::CreateOrConnectWithout);

									let create_fn = args.dmmf.schema.find_input_type(&InputObjects::CreateWithout { model: model.clone(), field: field.clone() }.to_string()).map(|typ| {
										let (names_snake, types): (Vec<_>, Vec<_>) = typ
											.fields
											.iter()
											.filter(|f| f.is_required)
											.filter_map(|field| {
												let field_name_snake = snake_ident(&field.name);

												let type_ref = &field.input_types[0];
												let typ = match type_ref.location {
													TypeLocation::InputObjectTypes => {
														let obj = type_ref.typ.parse::<InputObjects>().unwrap();

														quote!(#obj)
													},
													_ => type_ref.to_tokens(&quote!(super::), &field.arity(), &args.schema.db)?
												};

												Some((field_name_snake, typ))
											})
											.unzip();

										quote! {
											pub fn create<T: From<#create_without>>(
												#(#names_snake: #types,)*
												mut _params: Vec<#model_name_snake::CreateParam>
											) -> T {
					                            #create_without {
					                            	_params,
					                            	#(#names_snake),*
					                            }.into()
					                        }
										}
									});

									let create_unchecked_fn = args.dmmf.schema.find_input_type(&InputObjects::UncheckedCreateWithout { model: model.clone(), field: field.clone() }.to_string()).map(|typ| {
										let (names_snake, types): (Vec<_>, Vec<_>) = typ
											.fields
											.iter()
											.filter(|f| f.is_required)
											.filter_map(|field| {
												let field_name_snake = snake_ident(&field.name);

												let type_ref = &field.input_types[0];
												let typ = match type_ref.location {
													TypeLocation::InputObjectTypes => {
														let obj = type_ref.typ.parse::<InputObjects>().unwrap();

														quote!(#obj)
													},
													_ => type_ref.to_tokens(&quote!(super::), &field.arity(), &args.schema.db)?
												};

												Some((field_name_snake, typ))
											})
											.unzip();

										quote! {
											pub fn create_unchecked<T: From<#create_unchecked_without>>(
												#(#names_snake: #types,)*
												mut _params: Vec<#model_name_snake::CreateUncheckedParam>
											) -> T {
					                            #create_unchecked_without {
					                            	_params,
					                            	#(#names_snake),*
					                            }.into()
					                        }
										}
									});

									let connect_or_create_fn = args.dmmf.schema.find_input_type(&InputObjects::CreateOrConnectWithout { model: model.clone(), field: field.clone() }.to_string()).map(|_| {
										quote! {
											pub fn connect_or_create<T: From<Vec<#connect_or_create_without>>>(
												values: impl IntoIterator<Item = (#model_name_snake::UniqueWhereParam, #field_scope::Create)>
											) -> T {
												values
													.into_iter()
													.map(|(r#where, create)| #connect_or_create_without { r#where, create })
													.collect::<Vec<_>>()
													.into()
											}

											impl From<Vec<#connect_or_create_without>> for super::CreateParam {
												fn from(value: Vec<#connect_or_create_without>) -> Self {
													Self::#field_name_pascal(value.into())
												}
											}

											struct Connect(pub Vec<#model_name_snake::UniqueWhereParam>);

											pub fn connect<T: From<Connect>>(
												r#where: Vec<#model_name_snake::UniqueWhereParam>
											) -> T {
												Connect(r#where).into()
											}

											impl From<Connect> for super::CreateParam {
												fn from(value: Connect) -> Self {
													Self::#field_name_pascal(value.0.into())
												}
											}
										}
									});

	                                quote! {
										pub fn create_multiple<T: From<#create_multiple>>(
											values: Vec<#create_without>
										) -> T {
					                        #create_multiple(#pcr::Either::Left(values)).into()
					                    }

					                    pub fn create_many_unchecked<T: From<#create_multiple>>(
											values: Vec<#create_unchecked_without>
										) -> T {
					                        #create_multiple(#pcr::Either::Right(values)).into()
					                    }

										impl From<#create_multiple> for super::CreateParam {
											fn from(value: #create_multiple) -> Self {
												Self::#field_name_pascal(value.into())
											}
										}

										#create_fn

										#create_unchecked_fn

										#connect_or_create_fn
	                                }
                                },
                                _ => return None,
                            }
                        },
                        _ => return None
                    })
                });

            let create_nested_one_without = schema
                .find_input_type(&format!(
                    "{}CreateNestedOneWithout{}Input",
                    field.model().name(),
                    capitalize(field.name())
                ))
                .map(|_| {
                    let enum_name = quote!(CreateNestedOneWithout);

                    quote! {
                    	#[derive(Clone)]
                        pub enum #enum_name {
                            Create(Create),
                            ConnectOrCreate(CreateOrConnectWithout),
                            Connect(#model_name_snake::UniqueWhereParam)
                        }

                        impl Into<#pv> for #enum_name {
                            fn into(self) -> #pv {
                                let (k, v) = match self {
                                    Self::Create(value) => ("create", value.into()),
                                    Self::ConnectOrCreate(value) => ("connectOrCreate", value.into()),
                                    Self::Connect(value) => ("connect", #pv::Object(vec![value.serialize().transform_equals()]))
                                };

                                #pv::Object(vec![(k.to_string(), v)])
                            }
                        }

                        impl From<Create> for #enum_name {
							fn from(value: Create) -> Self {
								Self::Create(value)
							}
						}

						impl From<CreateWithout> for #enum_name {
							fn from(value: CreateWithout) -> Self {
								Self::Create(value.into())
							}
						}

						impl From<CreateUncheckedWithout> for #enum_name {
							fn from(value: CreateUncheckedWithout) -> Self {
								Self::Create(value.into())
							}
						}

						impl From<CreateOrConnectWithout> for #enum_name {
							fn from(value: CreateOrConnectWithout) -> Self {
								Self::ConnectOrCreate(value)
							}
						}

						impl From<#model_name_snake::UniqueWhereParam> for #enum_name {
							fn from(value: #model_name_snake::UniqueWhereParam) -> Self {
								Self::Connect(value)
							}
						}
                    }
                });

            let create_nested_many_without = schema
                .find_input_type(&format!(
                    "{}CreateNestedManyWithout{}Input",
                    field.model().name(),
                    capitalize(field.name())
                ))
                .map(|_| {
                	let opposite_field_name_model = snake_ident(&opposite_relation_field.model().name());
                	let opposite_field_name_snake = snake_ident(&opposite_relation_field.name());

                    let enum_name = quote!(CreateNestedManyWithout);

                    let set_param_connect_variant = format_ident!("Connect{}", field_name_pascal);

                    let connect = quote!(#opposite_field_name_model::#opposite_field_name_snake::Connect);

                    quote! {
	                    #[derive(Clone)]
						pub struct CreateMultiple(pub #pcr::Either<
							Vec<CreateWithout>,
							Vec<CreateUncheckedWithout>
						>);

						impl Into<#pv> for CreateMultiple {
							fn into(self) -> #pv {
								self.0
									.map_left(|vec| #pv::List(vec.into_iter().map(Into::into).collect()))
									.map_right(|vec| #pv::List(vec.into_iter().map(Into::into).collect()))
									.either_into()
							}
						}

                        #[derive(Clone)]
                        pub enum #enum_name {
                            Create(CreateMultiple),
                            ConnectOrCreate(Vec<CreateOrConnectWithout>),
                            Connect(Vec<#model_name_snake::UniqueWhereParam>)
                        }

                        impl Into<#pv> for #enum_name {
                            fn into(self) -> #pv {
                                let (k, v) = match self {
                                    Self::Create(value) => ("create", value.into()),
                                    Self::ConnectOrCreate(value) => ("connectOrCreate", #pv::List(value.into_iter().map(Into::into).collect())),
                                    Self::Connect(value) => ("connect", #pv::List(value.into_iter().map(|value| {
	                                	#pv::Object(vec![value.serialize().transform_equals()])
	                                }).collect()))
                                };

                                #pv::Object(vec![(k.to_string(), v)])
                            }
                        }

                        impl From<CreateMultiple> for #enum_name {
 							fn from(value: CreateMultiple) -> Self {
        						Self::Create(value)
							}
                        }

                        impl From<Vec<CreateOrConnectWithout>> for #enum_name {
                        	fn from(value: Vec<CreateOrConnectWithout>) -> Self {
	      						Self::ConnectOrCreate(value)
							}
                        }

                        impl From<Vec<#model_name_snake::UniqueWhereParam>> for #enum_name {
							fn from(value: Vec<#model_name_snake::UniqueWhereParam>) -> Self {
								Self::Connect(value)
							}
                        }
                    }
                });

            let create_unchecked_nested_many_without = schema
                .find_input_type(&format!(
                    "{}UncheckedCreateNestedManyWithout{}Input",
                    field.model().name(),
                    capitalize(field.name())
                ))
                .map(|_| {
                	let enum_name = quote!(UncheckedCreateNestedManyWithout);

                    quote! {
                        #[derive(Clone)]
                        pub enum #enum_name {
                            Create(Create),
                            ConnectOrCreate(CreateOrConnectWithout),
                            Connect(Vec<#relation_model_name_snake::UniqueWhereParam>)
                        }

                        impl Into<#pv> for #enum_name {
	                        fn into(self) -> #pv {
	                            let (k, v) = match self {
	                                Self::Create(value) => ("create", value.into()),
	                                Self::ConnectOrCreate(value) => ("connectOrCreate", value.into()),
	                                Self::Connect(value) => ("connect", #pv::List(value.into_iter().map(|value| {
	                                	#pv::Object(vec![value.serialize().transform_equals()])
	                                }).collect()))
	                            };

	                            #pv::Object(vec![(k.to_string(), v)])
	                        }
                        }
                    }
                });

            let create_without = schema
                .find_input_type(&format!(
                    "{}CreateWithout{}Input",
                    field.model().name(),
                    capitalize(field.name())
                ))
                .map(|input_type| {
                    let ((names_snake, names_pascal), types): ((Vec<_>, Vec<_>), Vec<_>) =
                        input_type
                            .fields
                            .iter()
                            .filter(|f| f.is_required)
                            .filter_map(|field| {
                                let field_name_pascal = pascal_ident(&field.name);
                                let field_name_snake = snake_ident(&field.name);

                                let type_ref = &field.input_types[0];
                                let typ = match type_ref.location {
                                    TypeLocation::InputObjectTypes => {
                                        let obj = type_ref.typ.parse::<InputObjects>().unwrap();

                                        quote!(#obj)
                                    }
                                    _ => type_ref.to_tokens(
                                        &quote!(super::),
                                        &field.arity(),
                                        &args.schema.db,
                                    )?,
                                };

                                Some(((field_name_snake, field_name_pascal), typ))
                            })
                            .unzip();

                    let param = quote!(#model_name_snake::CreateParam);

                    quote! {
                        #[derive(Clone)]
                        pub struct CreateWithout {
                            pub _params: Vec<#param>,
                            #(pub #names_snake: #types),*
                        }

                        impl Into<#pv> for CreateWithout {
                            fn into(mut self) -> #pv {
                                self._params.extend([
                                    #(#param::#names_pascal(self.#names_snake.into())),*
                                ]);

                                #pv::Object(self._params.into_iter().map(Into::into).collect())
                            }
                        }

                        #[derive(Clone)]
                        pub struct Create(pub #pcr::Either<CreateWithout, CreateUncheckedWithout>);

                        impl Into<#pv> for Create {
                            fn into(self) -> #pv {
                                self.0.either_into()
                            }
                        }

                        impl From<CreateWithout> for Create {
                            fn from(value: CreateWithout) -> Self {
                                Create(#pcr::Either::Left(value))
                            }
                        }

                        impl From<CreateUncheckedWithout> for Create {
                            fn from(value: CreateUncheckedWithout) -> Self {
                                Create(#pcr::Either::Right(value))
                            }
                        }
                    }
                });

            let create_unchecked_without = schema
                .find_input_type(&format!(
                    "{}UncheckedCreateWithout{}Input",
                    field.model().name(),
                    capitalize(field.name())
                ))
                .map(|input_type| {
                    let ((names_snake, names_pascal), types): ((Vec<_>, Vec<_>), Vec<_>) =
                        input_type
                            .fields
                            .iter()
                            .filter(|f| f.is_required)
                            .filter_map(|field| {
                                let field_name_pascal = pascal_ident(&field.name);
                                let field_name_snake = snake_ident(&field.name);

                                let type_ref = &field.input_types[0];
                                let typ = match type_ref.location {
                                    TypeLocation::InputObjectTypes => {
                                        let obj = type_ref.typ.parse::<InputObjects>().unwrap();

                                        quote!(#obj)
                                    }
                                    _ => type_ref.to_tokens(
                                        &quote!(super::),
                                        &field.arity(),
                                        &args.schema.db,
                                    )?,
                                };

                                Some(((field_name_snake, field_name_pascal), typ))
                            })
                            .unzip();

                    let param = quote!(#model_name_snake::CreateUncheckedParam);

                    quote! {
                        #[derive(Clone)]
                        pub struct CreateUncheckedWithout {
                            pub _params: Vec<#param>,
                            #(pub #names_snake: #types),*
                        }

                        impl Into<#pv> for CreateUncheckedWithout {
                            fn into(mut self) -> #pv {
                                self._params.extend([
                                    #(#param::#names_pascal(self.#names_snake)),*
                                ]);

                                #pv::Object(self._params.into_iter().map(Into::into).collect())
                            }
                        }
                    }
                });

            let create_or_connect_without = schema
                .find_input_type(&format!(
                    "{}CreateOrConnectWithout{}Input",
                    field.model().name(),
                    capitalize(field.name())
                ))
                .map(|_| {
	                let base = quote! {
						#[derive(Clone)]
                        pub struct CreateOrConnectWithout {
                            pub r#where: super::UniqueWhereParam,
                            pub create: Create
                        }

                        impl Into<#pv> for CreateOrConnectWithout {
                            fn into(self) -> #pv {
                                #pv::Object(vec![
                                    ("where".to_string(), #pv::Object(vec![self.r#where.serialize().transform_equals()])),
                                    ("create".to_string(), self.create.into())
                                ])
                            }
                        }
	                };

					if relation_field.ast_field().arity.is_list() {
						quote! {
							#base
						}
					} else {
						quote! {
							#base
						}
					}
                });

            let create_unchecked_nested_one_without = schema
	            .find_input_type(&format!(
	                "{}UncheckedCreateNestedOneWithout{}Input",
					field.model().name(),
                    capitalize(field.name())
	            ))
                .map(|_| {
                	let enum_name = quote!(UncheckedCreateNestedOneWithout);

                    quote! {
                        #[derive(Clone)]
                        pub enum #enum_name {
                            Create(Create),
                            ConnectOrCreate(CreateOrConnectWithout),
                            Connect(#model_name_snake::UniqueWhereParam)
                        }

                        impl Into<#pv> for #enum_name {
                            fn into(self) -> #pv {
                                let (k, v) = match self {
                                    Self::Create(value) => ("create", value.into()),
                                    Self::ConnectOrCreate(value) => ("connectOrCreate", value.into()),
                                    Self::Connect(value) => ("connect", #pv::Object(vec![value.serialize().transform_equals()]))
                                };

                                #pv::Object(vec![(k.to_string(), v)])
                            }
                        }
                    }
                });

            quote! {
                #functions

                #create_without

                #create_unchecked_without

                #create_or_connect_without

                #create_nested_one_without

                #create_unchecked_nested_one_without

                #create_nested_many_without

                #create_unchecked_nested_many_without
            }
        }
        _ => quote!(),
    }
}

pub fn types(model: ModelWalker, args: &GenerateArgs) -> ModelModulePart {
    let create_unchecked = create_unchecked(model, args);
    let create = create(model, args);

    let fields = model
        .fields()
        .map(|field| (field.name().to_string(), field_stuff(field, args)))
        .collect();

    let mut module = ModelModulePart {
        data: quote!(),
        fields,
    };

    if let Some((create, create_unchecked)) = create.zip(create_unchecked) {
        module = module.merge(create);
        module = module.merge(create_unchecked);
        module.data.extend(quote! {
            impl ::prisma_client_rust::CreateModelTypes for Types {
                type Create = CreateParam;
                type CreateUnchecked = CreateUncheckedParam;
            }
        });
    }

    module
}
