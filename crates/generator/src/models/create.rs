use prisma_client_rust_sdk::prisma::{
    dmmf::TypeLocation,
    prisma_models::walkers::{FieldWalker, ModelWalker, RefinedFieldWalker},
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
    let field_name_pascal = pascal_ident(field.name());
    let model_name_snake = snake_ident(field.model().name());
    let model_name_pascal = pascal_ident(field.model().name());
    let schema = &args.dmmf.schema;

    match field.refine() {
        RefinedFieldWalker::Relation(relation_field) => {
            let relation_model = relation_field.related_model();
            let relation_model_name_snake = snake_ident(relation_model.name());

            let create_type = quote!(#pcr::Either<CreateWithout, CreateUncheckedWithout>);

            let connect = {
                quote! {
                    pub struct Connect(#model_name_snake::UniqueWhereParam);

                    pub fn connect<T: From<Connect>>(r#where: #model_name_snake::UniqueWhereParam) -> T {
                        Connect(r#where).into()
                    }
                }
            };

            let create_or_connect_without = schema
                .find_input_type(&format!(
                    "{}CreateOrConnectWithout{}Input",
                    field.model().name(),
                    capitalize(field.name())
                ))
                .map(|_| {
                    quote! {
                        pub struct CreateOrConnectWithout {
                            r#where: #relation_model_name_snake::UniqueWhereParam,
                            create: #pcr::Either<CreateWithout, CreateUncheckedWithout>
                        }

                        pub fn connect_or_create<T: From<CreateOrConnectWithout>>(
                            r#where: #model_name_snake::UniqueWhereParam,
                            create: #create_type
                        ) -> T {
                            CreateOrConnectWithout {
                                r#where,
                                create: create.into()
                            }.into()
                        }
                    }
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
                        pub enum #enum_name {
                            Create(#create_type),
                            CreateOrConnect(CreateOrConnectWithout),
                            Connect(#model_name_snake::UniqueWhereParam)
                        }

                        impl Into<(String, #pcr::PrismaValue)> for #enum_name {
                            fn into(self) -> (String, #pcr::PrismaValue) {
                                let (k, v) = match self {
                                    Self::Create(value) => ("create", value.into()),
                                    Self::CreateOrConnect(value) => ("connectOrCreate", value.into()),
                                    Self::Connect(value) => ("connect", value.into())
                                };

                                (k.to_string(), v)
                            }
                        }

                        impl Into<#enum_name> for CreateWithout {
                            fn into(self) -> CreateNestedOneWithout {
                                CreateNestedOneWithout::Create(#pcr::Either::Left(self))
                            }
                        }

                        impl Into<#enum_name> for CreateUncheckedWithout {
                            fn into(self) -> CreateNestedOneWithout {
                                CreateNestedOneWithout::Create(#pcr::Either::Right(self))
                            }
                        }

                        pub struct Connect(#relation_model_name_snake::UniqueWhereParam);

                        pub fn connect<T: From<Connect>>(r#where: #relation_model_name_snake::UniqueWhereParam) -> T {
                            Connect(r#where).into()
                        }

                        impl From<Connect> for super::CreateParam {
                            fn from(Connect(value): Connect) -> Self {
                                Self::#field_name_pascal(
                                    #relation_model_name_snake::#enum_name::Connect(value)
                                )
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
                    let enum_name = quote!(CreateNestedManyWithout);

                    quote! {
                        #[derive(Clone)]
                        pub enum #enum_name {
                            Create(#pcr::Either<Vec<CreateWithout>, Vec<CreateUncheckedWithout>>),
                            CreateOrConnect(Vec<CreateOrConnectWithout>),
                            Connect(Vec<#model_name_snake::UniqueWhereParam>)
                        }

                        impl Into<(String, #pcr::PrismaValue)> for #enum_name {
                            fn into(self) -> (String, #pcr::PrismaValue) {
                                let (k, v) = match self {
                                    Self::Create(value) => ("create", value.into()),
                                    Self::CreateOrConnect(value) => ("connectOrCreate", value.into()),
                                    Self::Connect(value) => ("connect", value.into())
                                };

                                (k.to_string(), v)
                            }
                        }

                        pub struct Connect(Vec<#relation_model_name_snake::UniqueWhereParam>);

                        pub fn connect<T: From<Connect>>(r#where: Vec<#relation_model_name_snake::UniqueWhereParam>) -> T {
                            Connect(r#where).into()
                        }

                        impl From<Connect> for super::CreateParam {
                            fn from(Connect(value): Connect) -> Self {
                                Self::#field_name_pascal(
                                    #relation_model_name_snake::#enum_name::Connect(value)
                                )
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
                    quote! {
                        #[derive(Clone)]
                        pub enum CreateUncheckedNestedManyWithout {
                            Create(#pcr::Either<Vec<CreateWithout>, Vec<CreateUncheckedWithout>>),
                            CreateOrConnect(Vec<CreateOrConnectWithout>),
                            Connect(Vec<#model_name_snake::UniqueWhereParam>)
                        }

                        impl Into<(String, #pcr::PrismaValue)> for CreateUncheckedNestedManyWithout {
                            fn into(self) -> (String, #pcr::PrismaValue) {
                                let (k, v) = match self {
                                    Self::Create(value) => ("create", value.into()),
                                    Self::CreateOrConnect(value) => ("connectOrCreate", value.into()),
                                    Self::Connect(value) => ("connect", value.into())
                                };

                                (k.to_string(), v)
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
                .map(|_| {
                    quote! {
                        pub struct CreateWithout {
                            pub _params: Vec<#model_name_snake::SetParam>
                        }

                        impl Into<#pcr::PrismaValue> for CreateWithout {
                            fn into(self) -> #pcr::PrismaValue {
                                #pcr::PrismaValue::Object(vec![])
                            }
                        }

                        pub fn create<T: From<CreateWithout>>(_params: Vec<#model_name_snake::SetParam>) -> T {
                            CreateWithout {
                                _params
                            }.into()
                        }
                    }
                });

            let create_unchecked_without = schema
                .find_input_type(&format!(
                    "{}UncheckedCreateWithout{}Input",
                    field.model().name(),
                    capitalize(field.name())
                ))
                .map(|_| {
                    quote! {
                        pub struct CreateUncheckedWithout {
                            pub _params: Vec<#model_name_snake::SetParam>
                        }

                        impl Into<#pcr::PrismaValue> for CreateUncheckedWithout {
                            fn into(self) -> #pcr::PrismaValue {
                                #pcr::PrismaValue::Object(vec![])
                            }
                        }

                        pub fn create_unchecked<T: From<CreateUncheckedWithout>>
                            (_params: Vec<#model_name_snake::SetParam>) -> T {
                            CreateUncheckedWithout {
                                _params
                            }.into()
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
                    quote! {
                        #[derive(Clone)]
                        pub enum CreateUncheckedNestedOneWithout {
                            Create(#pcr::Either<CreateWithout, CreateUncheckedWithout>),
                            CreateOrConnect(CreateOrConnectWithout),
                            Connect(#model_name_snake::UniqueWhereParam)
                        }

                        impl Into<(String, #pcr::PrismaValue)> for CreateUncheckedNestedOneWithout {
                            fn into(self) -> (String, #pcr::PrismaValue) {
                                let (k, v) = match self {
                                    Self::Create(value) => ("create", value.into()),
                                    Self::CreateOrConnect(value) => ("connectOrCreate", value.into()),
                                    Self::Connect(value) => ("connect", value.into())
                                };

                                (k.to_string(), v)
                            }
                        }
                    }
                });

            quote! {
                #create_or_connect_without

                #create_without

                #create_unchecked_without

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

    ModelModulePart {
        data: quote! {
            #create_unchecked

            #create
        },
        fields,
    }
}
