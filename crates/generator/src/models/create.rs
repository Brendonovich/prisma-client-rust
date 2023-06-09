use prisma_client_rust_sdk::prisma::{
    dmmf::TypeLocation,
    prisma_models::walkers::{FieldWalker, ModelWalker},
};

use crate::prelude::*;

use super::ModelModulePart;

fn create_unchecked(model: ModelWalker, args: &GenerateArgs) -> Option<TokenStream> {
    let pcr = quote!(::prisma_client_rust);

    let model_name_snake = snake_ident(model.name());

    args.dmmf
        .schema
        .find_input_type(&format!("{}UncheckedCreateInput", model.name()))
        .map(|input_type| {
            let (all_fields, required_fields): (_, Vec<_>) =
                input_type.fields.iter().flat_map(|field| {
                    let field_name_str = &field.name;
                    let field_name_pascal = pascal_ident(&field.name);
                    let field_name_snake = snake_ident(&field.name);

                    let type_ref = &field.input_types[0];
                    let typ =
                        type_ref.to_tokens(&quote!(super::), &field.arity(), &args.schema.db)?;

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
        })
}

fn create(model: ModelWalker, args: &GenerateArgs) -> Option<TokenStream> {
    let pcr = quote!(::prisma_client_rust);

    let model_name_snake = snake_ident(model.name());

    args.dmmf
        .schema
        .find_input_type(&format!("{}CreateInput", model.name()))
        .map(|input_type| {
            let (all_fields, required_fields): (_, Vec<_>) = input_type
                .fields
                .iter()
                .flat_map(|field| {
                    let field_name_str = &field.name;
                    let field_name_pascal = pascal_ident(&field.name);
                    let field_name_snake = snake_ident(&field.name);

                    let type_ref = &field.input_types[0];
                    let typ =
                        type_ref.to_tokens(&quote!(super::), &field.arity(), &args.schema.db)?;

                    let value_ident = format_ident!("param");
                    let pv = type_ref.to_prisma_value(&value_ident, &field.arity());

                    let push_wrapper = match type_ref.location {
                        TypeLocation::Scalar | TypeLocation::EnumTypes => quote!(set),
                        TypeLocation::InputObjectTypes => quote!(connect),
                        _ => todo!(),
                    };

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
                            .then(|| ((field_name_snake, push_wrapper), typ)),
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
                        #(#names: #types,)*
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
                                #(#names::#push_wrappers(self.#names)),*
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

                    impl #pcr::CreateModelTypes for Types {
                        type Create = CreateParam;
                        type CreateUnchecked = CreateUncheckedParam;
                    }
                }
            };

            quote! {
                #param

                #create
            }
        })
}

fn field_stuff(field: FieldWalker, args: &GenerateArgs) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);
    let model_name_snake = snake_ident(field.model().name());

    let create_nested_one_without = args
        .dmmf
        .schema
        .find_input_type(&format!(
            "{}CreateNestedOneWithout{}Input",
            field.model().name(),
            capitalize(field.name())
        ))
        .map(|_| {
            quote! {
                pub enum CreateOption {
                     Create(Create),
                     CreateUnchecked(CreateUnchecked)
                }

                pub enum CreateNestedOneWithout {
                    Create(CreateOption),
                    ConnectOrCreate {
                        r#where: #model_name_snake::UniqueWhereParam,
                        create: CreateOption
                    },
                    Connect(#model_name_snake::UniqueWhereParam)
                }

                impl Into<(String, #pcr::PrismaValue)> for CreateNestedOneWithout {
                    fn into(self) -> (String, #pcr::PrismaValue) {
                        let (k, v) = match self {
                            CreateNestedOneWithout::Create(value) =>
                                ("create", value.into()),
                            CreateNestedOneWithout::ConnectOrCreate { r#where, create } =>
                                ("connectOrCreate", #pcr::PrismaValue::Object(vec![
                                    ("where".to_string(), r#where.into()),
                                    ("create".to_string(), create.into().into()),
                                ])),
                            CreateNestedOneWithout::Connect(connect) =>
                                ("connect", value.into())
                        };

                        (k.to_string(), v)
                    }
                }

                pub fn connect_or_create<T: From<CreateNestedOneWithout>>(
                    r#where: #model_name_snake::UniqueWhereParam,
                    create: CreateOption
                ) -> T {
                    CreateNestedOneWithout::ConnectOrCreate {
                        r#where,
                        create: create.into()
                    }.into()
                }

                pub fn connect<T: From<CreateNestedOneWithout>>(r#where: #model_name_snake::UniqueWhereParam) -> T {
					CreateNestedOneWithout::Connect(r#where).into()
				}
            }
        });

    let create_without = args
        .dmmf
        .schema
        .find_input_type(&format!(
            "{}CreateWithout{}Input",
            field.model().name(),
            capitalize(field.name())
        ))
        .map(|_| {
            quote! {
                pub struct Create {
                    pub _params: Vec<#model_name_snake::SetParam>
                }

                impl Into<#pcr::PrismaValue> for Create {
                    fn into(self) -> #pcr::PrismaValue {
                        #pcr::PrismaValue::Object(vec![])
                    }
                }

                impl Into<CreateNestedOneWithout> for Create {
                    fn into(self) -> CreateNestedOneWithout {
                        CreateNestedOneWithout::Create(self.into())
                    }
                }

                impl Into<CreateOption> for Create {
                    fn into(self) -> CreateNestedOneWithout {
                        CreateOption::Create(self)
                    }
                }

                pub fn create<T: From<Create>>(_params: Vec<#model_name_snake::SetParam>) -> T {
                    Create {
                        _params
                    }.into()
                }
            }
        });

    let create_unchecked_without = args
        .dmmf
        .schema
        .find_input_type(&format!(
            "{}UncheckedCreateWithout{}Input",
            field.model().name(),
            capitalize(field.name())
        ))
        .map(|_| {
            quote! {
                pub struct CreateUnchecked {
                    pub _params: Vec<#model_name_snake::SetParam>
                }

                impl Into<#pcr::PrismaValue> for CreateUnchecked {
                    fn into(self) -> #pcr::PrismaValue {
                        #pcr::PrismaValue::Object(vec![])
                    }
                }

                impl Into<CreateNestedOneWithout> for CreateUnchecked {
                    fn into(self) -> CreateNestedOneWithout {
                        CreateNestedOneWithout::Create(self.into())
                    }
                }

                impl Into<CreateOption> for CreateUnchecked {
                    fn into(self) -> CreateNestedOneWithout {
                        CreateOption::CreateUnchecked(self)
                    }
                }

                pub fn create_unchecked<T: From<CreateUnchecked>>(_params: Vec<#model_name_snake::SetParam>) -> T {
                    CreateUnchecked {
                    	_params
                    }.into()
                }
            }
        });

    quote! {
        #create_nested_one_without

        #create_without

        #create_unchecked_without
    }
}

pub fn types(model: ModelWalker, args: &GenerateArgs) -> ModelModulePart {
    let create_unchecked = create_unchecked(model, args);
    let create = create(model, args);

    let fields = model
        .fields()
        .map(|field| (field.name().to_string(), field_stuff(field, args)))
        .collect();

    ModelModulePart {
        data: quote! {
            #create_unchecked

            #create
        },
        fields,
    }
}
