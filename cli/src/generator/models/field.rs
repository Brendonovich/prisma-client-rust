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
    let field_type = root_field.type_tokens(module_path);

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
                            fn from(fetch: Fetch) -> Self {
                                WithParam::#field_name_pascal(fetch.0)
                            }
                        }

                        pub fn fetch(params: Vec<#relation_model_name_snake::WhereParam>) -> Fetch {
                            Fetch(#relation_model_name_snake::ManyArgs::new(params))
                        }

                        pub struct Connect(pub Vec<#relation_model_name_snake::UniqueWhereParam>);

                        impl From<Connect> for SetParam {
                            fn from(value: Connect) -> Self {
                                Self::#connect_variant(value.0)
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
                            fn from(fetch: Fetch) -> Self {
                                WithParam::#field_name_pascal(fetch.0)
                            }
                        }

                        pub fn fetch() -> Fetch {
                            Fetch(#relation_model_name_snake::UniqueArgs::new())
                        }

                        pub struct Connect(#relation_model_name_snake::UniqueWhereParam);

                        impl From<Connect> for SetParam {
                            fn from(value: Connect) -> Self {
                                Self::#connect_variant(value.0)
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
                let equals = match (model.field_is_primary(field_name), model.field_is_unique(field_name), field.arity.is_required()) {
                    (true, _, _) | (_, true, true) => quote! {
                        pub fn equals<T: From<UniqueWhereParam>>(value: #field_type) -> T {
                            UniqueWhereParam::#equals_variant(value).into()
                        }
                    },
                    (_, true, false) => quote! {
                        pub fn equals<A, T: #pcr::FromOptionalUniqueArg<Set, Arg = A>>(value: A) -> T {
                            T::from_arg(value)
                        }
                    },
                    (_, _, _) => quote! {
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

                    let typ = method.type_tokens(module_path);

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
                    fn from(value: Set) -> Self {
                        Self::#set_variant(value.0)
                    }
                }

                impl From<Set> for UncheckedSetParam {
                    fn from(value: Set) -> Self {
                        Self::#field_name_pascal(value.0)
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

            let set_fn = comp_type
                .fields
                .iter()
                .filter(|f| f.required_on_create())
                .map(|field| Some((field, field.type_tokens(module_path)?)))
                .collect::<Option<Vec<_>>>()
                .map(|v| {
                    let set_struct = quote!(#comp_type_snake::Set);

                    let (required_fields, required_field_types): (Vec<_>, Vec<_>) =
                        v.into_iter().unzip();

                    let required_field_names_snake = required_fields
                        .iter()
                        .map(|f| snake_ident(&f.name))
                        .collect::<Vec<_>>();

                    quote! {
                        pub struct Set(#set_struct);

                        pub fn set<T: From<Set>>(
                            #(#required_field_names_snake: #required_field_types,)*
                            _params: Vec<#comp_type_snake::SetParam>
                        ) -> T {
                            Set(#set_struct {
                                #(#required_field_names_snake,)*
                                _params
                            }).into()
                        }

                        impl From<Set> for SetParam {
                            fn from(Set(v): Set) -> Self {
                                Self::#set_variant(v)
                            }
                        }
                    }
                });

            let unset_fn = cf.arity.is_optional().then(|| {
                let unset_variant = format_ident!("Unset{field_name_pascal}");

                quote! {
                    pub fn unset() -> SetParam {
                        SetParam::#unset_variant
                    }
                }
            });

            quote! {
                #set_fn
                #unset_fn
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
