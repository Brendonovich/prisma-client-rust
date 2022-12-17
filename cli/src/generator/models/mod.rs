mod select;
mod set_params;
mod with_params;
mod data;
mod order_by;
mod pagination;
mod actions;
mod create;
mod include;

use datamodel::dml::FieldArity;
use prisma_client_rust_sdk::prelude::*;

use std::ops::Deref;

pub struct Operator {
    pub name: &'static str,
    pub action: &'static str,
    pub list: bool,
}

static OPERATORS: &'static [Operator] = &[
    Operator {
        name: "Not",
        action: "NOT",
        list: false
    },
    Operator {
        name: "Or",
        action: "OR",
        list: true
    },
    Operator {
        name: "And",
        action: "AND",
        list: false
    },
];

fn compound_field_accessor(accessor_name_str: &str, variant_data_args: &Vec<TokenStream>, variant_data_destructured: &Vec<Ident>) -> TokenStream {
    let accessor_name = format_ident!("{}", accessor_name_str);
    let variant_name = format_ident!("{}Equals", accessor_name_str.to_case(Case::Pascal));
    
    quote! {
        pub fn #accessor_name<T: From<UniqueWhereParam>>(#(#variant_data_args),*) -> T {
            UniqueWhereParam::#variant_name(#(#variant_data_destructured),*).into()
        }
    }
}

struct WhereParams {
    pub variants: Vec<TokenStream>,
    pub to_serialized_where: Vec<TokenStream>,
    pub unique_variants: Vec<TokenStream>,
    pub from_unique_match_arms: Vec<TokenStream>,
    pub from_optional_uniques: Vec<TokenStream>,
}

impl WhereParams {
    pub fn new() -> Self {
        Self {
            variants: vec![],
            to_serialized_where: vec![],
            unique_variants: vec![],
            from_unique_match_arms: vec![],
            from_optional_uniques: vec![],
        }
    }

    pub fn add_variant(&mut self, variant: TokenStream, match_arm: TokenStream) {
        self.variants.push(variant);
        self.to_serialized_where.push(match_arm);
    }

    pub fn add_unique_variant(&mut self, field: &dml::Field, args: &GenerateArgs) {
        if matches!(field.arity(), dml::FieldArity::Optional) {
            panic!("add_unique_variant cannot add optional fields. Perhaps you meant add_optional_unique_variant?");
        }
        
        let field_type = field.type_tokens(quote!());

        let field_pascal = format_ident!("{}", field.name().to_case(Case::Pascal));

        let variant_name = format_ident!("{}Equals", &field_pascal);
        self.unique_variants
            .push(quote!(#variant_name(#field_type)));

        let read_filter = args.read_filter(&field.as_scalar_field().unwrap()).unwrap();
        let filter_enum = format_ident!("{}Filter", &read_filter.name);

        self.from_unique_match_arms.push(quote! {
            UniqueWhereParam::#variant_name(value) => Self::#field_pascal(_prisma::read_filters::#filter_enum::Equals(value))
        });
    }

    pub fn add_optional_unique_variant(
        &mut self,
        field: &dml::Field, args: &GenerateArgs
    ) {
        if !matches!(field.arity(), dml::FieldArity::Optional) {
            panic!("add_optional_unique_variant only adds optional fields. Perhaps you meant add_unique_variant?");
        }
        
        let field_base_type = field.field_type().to_tokens(quote!(), &FieldArity::Required);

        let field_pascal = format_ident!("{}", field.name().to_case(Case::Pascal));
        let field_snake = format_ident!("{}", field.name().to_case(Case::Snake));

        let variant_name = format_ident!("{}Equals", &field_pascal);
        
        self.unique_variants
            .push(quote!(#variant_name(#field_base_type)));

        let read_filter = args.read_filter(&field.as_scalar_field().unwrap()).unwrap();
        let filter_enum = format_ident!("{}Filter", &read_filter.name);

        self.from_unique_match_arms.push(quote! {
            UniqueWhereParam::#variant_name(value) => Self::#field_pascal(_prisma::read_filters::#filter_enum::Equals(Some(value)))
        });

        self.from_optional_uniques.push(quote!{
            impl ::prisma_client_rust::FromOptionalUniqueArg<#field_snake::Set> for WhereParam {
                type Arg = Option<#field_base_type>;
                
                fn from_arg(arg: Self::Arg) -> Self where Self: Sized {
                    Self::#field_pascal(_prisma::read_filters::#filter_enum::Equals(arg))
                }
            }
            
            impl ::prisma_client_rust::FromOptionalUniqueArg<#field_snake::Set> for UniqueWhereParam {
                type Arg = #field_base_type;
                
                fn from_arg(arg: Self::Arg) -> Self where Self: Sized {
                    Self::#variant_name(arg)
                }
            }
        });
    }

    pub fn add_compound_unique_variant(
        &mut self,
        field_names_str: &str,
        variant_data_destructured: &Vec<Ident>,
        variant_data_types: &Vec<TokenStream>,
    ) {
        let variant_name = format_ident!("{}Equals", field_names_str);

        self.unique_variants
            .push(quote!(#variant_name(#(#variant_data_types),*)));
        self.from_unique_match_arms.push(quote! {
            UniqueWhereParam::#variant_name(#(#variant_data_destructured),*) => Self::#variant_name(#(#variant_data_destructured),*)
        });
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            variants,
            to_serialized_where,
            unique_variants,
            from_unique_match_arms,
            from_optional_uniques,
        } = self;

        let pcr = quote!(::prisma_client_rust);

        quote! {
            #[derive(Clone)]
            pub enum WhereParam {
                #(#variants),*
            }

            impl #pcr::WhereInput for WhereParam {
                fn serialize(self) -> #pcr::SerializedWhereInput {
                    let (name, value) = match self {
                        #(#to_serialized_where),*
                    };

                    #pcr::SerializedWhereInput::new(name, value.into())
                }
            }

            #[derive(Clone)]
            pub enum UniqueWhereParam {
                #(#unique_variants),*
            }

            impl From<UniqueWhereParam> for WhereParam {
                fn from(value: UniqueWhereParam) -> Self {
                    match value {
                        #(#from_unique_match_arms),*
                    }
                }
            }

            #(#from_optional_uniques)*

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
}

pub struct RequiredField<'a> {
    pub push_wrapper: TokenStream,
    pub typ: TokenStream,
    pub field: &'a dml::Field,
}

impl Deref for RequiredField<'_> {
    type Target = dml::Field;
    fn deref(&self) -> &Self::Target {
        self.field
    }
}

pub fn required_fields(model: &dml::Model) -> Vec<RequiredField> {
    model
        .fields()
        .filter(|field| match field {
            dml::Field::ScalarField(scalar_field) => {
                !model.scalar_field_has_relation(scalar_field) && field.required_on_create()
            }
            dml::Field::RelationField(_) => field.required_on_create(),
            _ => unreachable!(),
        })
        .map(|field| {
            let field_name_snake = snake_ident(&field.name());

            let typ = match field {
                dml::Field::ScalarField(_) => field.type_tokens(quote!()),
                dml::Field::RelationField(relation_field) => {
                    let relation_model_name_snake = snake_ident(&relation_field.relation_info.to);

                    quote!(super::#relation_model_name_snake::UniqueWhereParam)
                }
                _ => unreachable!(),
            };

            let push_wrapper = match field {
                dml::Field::ScalarField(_) => quote!(set),
                dml::Field::RelationField(_) => quote!(connect),
                _ => unreachable!(),
            };

            RequiredField {
                field,
                push_wrapper: quote!(#field_name_snake::#push_wrapper),
                typ,
            }
        })
        .collect()
}

pub fn unique_field_combos(model: &dml::Model) -> Vec<Vec<&dml::Field>> {
    let mut combos = model.indices.iter()
        .filter(|i| matches!(i.tpe, dml::IndexType::Unique))
        .map(|unique| {
            unique.fields.iter().filter_map(|field| {
                model.fields.iter().find(|mf| mf.name() == &field.path[0].0)
            }).collect()
        }).collect::<Vec<_>>();

    if let Some(primary_key) = &model.primary_key {
        // if primary key is marked as unique, skip primary key handling
        let primary_key_also_unique = primary_key.fields.len() == 1 && !model.field_is_unique(&primary_key.fields[0].name);

        // TODO: understand why i wrote this
        let primary_key_idk = !model.indices
            .iter()
            .filter(|i| i.tpe == dml::IndexType::Unique)
            .any(|i|
                i.fields
                    .iter()
                    .map(|f| f.path[0].0.as_str())
                    .collect::<Vec<_>>()
                    == 
                primary_key.fields
                    .iter()
                    .map(|f| f.name.as_str())
                    .collect::<Vec<_>>()
                );

        if primary_key_also_unique || primary_key_idk {
            combos.push(primary_key.fields.iter()
                .filter_map(|field| {
                    model.fields.iter().find(|mf| mf.name() == field.name.as_str())
                }).collect()
            );
        }
    }

    combos
}

pub fn generate(args: &GenerateArgs, module_path: TokenStream) -> Vec<TokenStream> {
    let pcr = quote!(::prisma_client_rust);

    args.dml.models.iter().map(|model| {
        let mut model_where_params = WhereParams::new();

        let model_name_snake = format_ident!("{}", model.name.to_case(Case::Snake));

        for op in OPERATORS {
            let variant_name = format_ident!("{}", op.name.to_case(Case::Pascal));
            let op_action = &op.action;
            
            let value = match op.list {
                true => quote! {
                    #pcr::SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(#pcr::WhereInput::serialize)
                            .map(Into::into)
                            .map(|v| vec![v])
                            .map(#pcr::PrismaValue::Object)
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

            model_where_params.add_variant(
                quote!(#variant_name(Vec<WhereParam>)),
                quote! {
                    Self::#variant_name(value) => (
                        #op_action,
                        #value,
                    )
                },
            );
        }

        let compound_field_accessors = unique_field_combos(&model).iter().flat_map(|fields| {
            if fields.len() == 1 {
                let field = fields[0];
                
                match field.arity()  {
                    dml::FieldArity::Optional => model_where_params.add_optional_unique_variant(field, args),
                    _ => model_where_params.add_unique_variant(field, args),
                }

                None
            } else {
                let variant_name_string = fields.iter().map(|f| f.name().to_case(Case::Pascal)).collect::<String>();
                let variant_name = format_ident!("{}Equals", &variant_name_string);
                
                let variant_data_names = fields.iter().map(|f| f.name()).collect::<Vec<_>>();
            
                let ((field_defs, field_types), (prisma_values, field_names_snake)): 
                    ((Vec<_>, Vec<_>), (Vec<_>, Vec<_>)) = fields.into_iter().map(|field| {
                    let field_type = match field.arity() {
                        FieldArity::List | FieldArity::Required => field.type_tokens(quote!()),
                        FieldArity::Optional => field.field_type().to_tokens(quote!(), &FieldArity::Required)
                    };
                    
                    let field_name_snake = format_ident!("{}", field.name().to_case(Case::Snake));
                    
                    (
                        (quote!(#field_name_snake: #field_type), field_type),
                        (field.field_type().to_prisma_value(&field_name_snake, &FieldArity::Required), field_name_snake)
                    )
                }).unzip();

                let field_names_joined = fields.iter().map(|f| f.name()).collect::<Vec<_>>().join("_");

                model_where_params.add_variant(
                    quote!(#variant_name(#(#field_types),*)),
                    quote! {
                        Self::#variant_name(#(#field_names_snake),*) => (
                            #field_names_joined,
                            #pcr::SerializedWhereValue::Object(vec![#((#variant_data_names.to_string(), #prisma_values)),*])
                        )
                    },
                );
                
                model_where_params.add_compound_unique_variant(&variant_name_string, &field_names_snake, &field_types);

                Some(compound_field_accessor(
                    &variant_name_string.to_case(Case::Snake),
                    &field_defs,
                    &field_names_snake
                ))
            }
        }).collect::<TokenStream>();

        let field_modules = model
            .fields
            .iter()
            .map(|root_field| {
                let field_name = root_field.name();
                let field_name_pascal = format_ident!("{}", field_name.to_case(Case::Pascal));
                let field_name_snake = format_ident!("{}", field_name.to_case(Case::Snake));
                let field_type = root_field.type_tokens(quote!());

                let connect_variant = format_ident!("Connect{}", field_name_pascal);
                let disconnect_variant = format_ident!("Disconnect{}", field_name_pascal);
                let set_variant = format_ident!("Set{}", field_name_pascal);
                let is_null_variant = format_ident!("{}IsNull", field_name_pascal);
                let equals_variant = format_ident!("{}Equals", &field_name_pascal);

                let field_module_contents = match root_field {
                    dml::Field::RelationField(field) => {
                        let relation_model_name_snake = snake_ident(&field.relation_info.to);

                        let with_fn = with_params::builder_fn(&field);
                        
                        let base = if field.arity.is_list() {
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

                                pub fn connect<T: From<Connect>>(params: Vec<#relation_model_name_snake::UniqueWhereParam>) -> T {
                                    Connect(params).into()
                                }

                                pub fn disconnect(params: Vec<#relation_model_name_snake::UniqueWhereParam>) -> SetParam {
                                    SetParam::#disconnect_variant(params)
                                }

                                pub fn set(params: Vec<#relation_model_name_snake::UniqueWhereParam>) -> SetParam {
                                    SetParam::#set_variant(params)
                                }

                                pub struct Connect(pub Vec<#relation_model_name_snake::UniqueWhereParam>);

                                impl From<Connect> for SetParam {
                                    fn from(value: Connect) -> Self {
                                        Self::#connect_variant(value.0)
                                    }
                                }
                            }
                        } else {
                            let optional_fns = field.arity.is_optional().then(|| {
                                model_where_params.add_variant(
                                    quote!(#is_null_variant),
                                    quote! {
                                        Self::#is_null_variant => (
                                            #field_name,
                                            #pcr::SerializedWhereValue::Value(#pcr::PrismaValue::Null)
                                        )
                                    },
                                );

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
                        };

                        let relation_methods = root_field.relation_methods().iter().map(|method| {
                            let method_action_string = method.to_case(Case::Camel);
                            let variant_name = format_ident!("{}{}", &field_name_pascal, method.to_case(Case::Pascal));
                            let method_name_snake = format_ident!("{}", method.to_case(Case::Snake));
                            
                            model_where_params.add_variant(
                                quote!(#variant_name(Vec<super::#relation_model_name_snake::WhereParam>)),
                                quote! {
                                    Self::#variant_name(where_params) => (
                                        #field_name,
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
                            );
                            
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
                    },
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

                            model_where_params.add_variant(
                                quote!(#field_name_pascal(_prisma::read_filters::#filter_enum)),
                                quote! {
                                    Self::#field_name_pascal(value) => (
                                        #field_name,
                                        value.into()
                                    )
                                },
                            );

                            let read_methods = read_filter.methods.iter().filter_map(|method| {
                                if method.name == "Equals" { return None }

                                let method_name_snake = format_ident!("{}", method.name.to_case(Case::Snake));
                                let method_name_pascal =
                                    format_ident!("{}", method.name.to_case(Case::Pascal));
                                
                                let typ = method.type_tokens(quote!());

                                Some(quote! {
                                    pub fn #method_name_snake(value: #typ) -> WhereParam {
                                        WhereParam::#field_name_pascal(_prisma::read_filters::#filter_enum::#method_name_pascal(value))
                                    }
                                })
                            });

                            quote! {
                                #equals

                                #(#read_methods)*
                            }
                        });

                        let write_fns = args.write_filter(&field).map(|write_type| {
                            write_type
                                .methods
                                .iter()
                                .map(|method|{
                                    let method_name_snake = format_ident!("{}", method.name.to_case(Case::Snake));

                                    let typ = method.type_tokens(quote!());

                                    let variant_name = format_ident!("{}{}", method.name.to_case(Case::Pascal), field_name_pascal);

                                    quote! {
                                        pub fn #method_name_snake(value: #typ) -> SetParam {
                                            SetParam::#variant_name(value)
                                        }
                                    }
                                })
                                .collect::<TokenStream>()
                        });

                        quote! {
                            pub fn set<T: From<Set>>(value: #field_type) -> T {
                                Set(value).into()
                            }

                            pub struct Set(pub #field_type);
                            impl From<Set> for SetParam {
                                fn from(value: Set) -> Self {
                                    Self::#set_variant(value.0)
                                }
                            }

                            pub fn order(direction: #pcr::Direction) -> OrderByParam {
                                OrderByParam::#field_name_pascal(direction)
                            }

                            #read_fns

                            #write_fns
                        }
                    },
                    _ => unreachable!("Cannot codegen for composite field")
                };
                
                let selection_enums = [
                    include::field_module_enum(&root_field, &pcr),
                    select::field_module_enum(&root_field, &pcr)
                ];
                
                quote! {
                    pub mod #field_name_snake {
                        use super::super::*;
                        use super::{WhereParam, UniqueWhereParam, OrderByParam, WithParam, SetParam};
                        use super::_prisma::*;

                        #field_module_contents

                        #(#selection_enums)*
                    }
                }
            })
            .collect::<TokenStream>();
        
        let data_struct = data::struct_definition(&model);
        let with_params_enum = with_params::enum_definition(&model);
        let set_params_enum = set_params::enum_definition(&model, args);
        let order_by_params_enum = order_by::enum_definition(&model);
        let create_fn = create::model_fns(&model);
        let where_params = model_where_params.quote();
        let select_macro = select::generate_macro(model, &module_path);
        let select_params_enum = select::model_module_enum(&model, &pcr);
        let include_macro = include::generate_macro(model, &module_path);
        let include_params_enum = include::model_module_enum(&model, &pcr);
        let actions_struct = actions::struct_definition(&model, args);

        quote! {
            pub mod #model_name_snake {
                use super::*;
                use super::_prisma::*;
                
                #field_modules

                #compound_field_accessors

                #create_fn
                
                #select_macro
                #select_params_enum

                #include_macro
                #include_params_enum

                #data_struct

                #with_params_enum

                #set_params_enum

                #order_by_params_enum

                #where_params

                // 'static since the actions struct is only used for types

                pub type UniqueArgs = ::prisma_client_rust::UniqueArgs<Actions<'static>>;
                pub type ManyArgs = ::prisma_client_rust::ManyArgs<Actions<'static>>;
                
                pub type Count<'a> = ::prisma_client_rust::Count<'a, Actions<'static>>;
                pub type Create<'a> = ::prisma_client_rust::Create<'a, Actions<'static>>;
                pub type CreateMany<'a> = ::prisma_client_rust::CreateMany<'a, Actions<'static>>;
                pub type FindUnique<'a> = ::prisma_client_rust::FindUnique<'a, Actions<'static>>;
                pub type FindMany<'a> = ::prisma_client_rust::FindMany<'a, Actions<'static>>;
                pub type FindFirst<'a> = ::prisma_client_rust::FindFirst<'a, Actions<'static>>;
                pub type Update<'a> = ::prisma_client_rust::Update<'a, Actions<'static>>;
                pub type UpdateMany<'a> = ::prisma_client_rust::UpdateMany<'a, Actions<'static>>;
                pub type Upsert<'a> = ::prisma_client_rust::Upsert<'a, Actions<'static>>;
                pub type Delete<'a> = ::prisma_client_rust::Delete<'a, Actions<'static>>;
                pub type DeleteMany<'a> = ::prisma_client_rust::DeleteMany<'a, Actions<'static>>;
              
                #actions_struct
            }
        }
    }).collect()
}
