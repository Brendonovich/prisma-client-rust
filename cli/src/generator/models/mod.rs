mod select;
mod outputs;
mod set_params;
mod with_params;
mod data;
mod order_by;
mod pagination;
mod actions;
mod create;
mod include;

use crate::generator::prelude::*;
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

struct FieldQueryModule {
    name: Ident,
    methods: Vec<TokenStream>,
    structs: Vec<TokenStream>,
}

impl FieldQueryModule {
    pub fn new(field: &dml::Field) -> Self {
        Self {
            name: format_ident!("{}", field.name().to_case(Case::Snake)),
            methods: vec![],
            structs: vec![],
        }
    }

    pub fn add_method(&mut self, method: TokenStream) {
        self.methods.push(method);
    }

    pub fn add_struct(&mut self, struct_: TokenStream) {
        self.structs.push(struct_);
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            name,
            methods,
            structs,
        } = self;

        quote! {
            pub mod #name {
                use super::super::*;
                use super::{WhereParam, UniqueWhereParam, OrderByParam, WithParam, SetParam};
                use super::_prisma::*;

                #(#methods)*

                #(#structs)*
            }
        }
    }
}

struct ModelQueryModules {
    field_modules: Vec<FieldQueryModule>,
    compound_field_accessors: Vec<TokenStream>,
}

impl ModelQueryModules {
    pub fn new() -> Self {
        Self {
            field_modules: vec![],
            compound_field_accessors: vec![],
        }
    }

    pub fn add_field_module(&mut self, field_module: FieldQueryModule) {
        self.field_modules.push(field_module);
    }

    pub fn add_compound_field(&mut self, accessor_name_str: &str, variant_data_args: &Vec<TokenStream>, variant_data_destructured: &Vec<Ident>) {
        let accessor_name = format_ident!("{}", accessor_name_str);
        let variant_name = format_ident!("{}Equals", accessor_name_str.to_case(Case::Pascal));
        
        self.compound_field_accessors.push(quote! {
            pub fn #accessor_name<T: From<UniqueWhereParam>>(#(#variant_data_args),*) -> T {
                UniqueWhereParam::#variant_name(#(#variant_data_destructured),*).into()
            }
        });
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            field_modules,
            compound_field_accessors,
        } = self;

        let field_modules = field_modules
            .iter()
            .map(|field| field.quote())
            .collect::<Vec<_>>();

        quote! {
            #(#field_modules)*

            #(#compound_field_accessors)*
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

    pub fn add_unique_variant(&mut self, field: &dml::Field) {
        if matches!(field.arity(), dml::FieldArity::Optional) {
            panic!("add_unique_variant cannot add optional fields. Perhaps you meant add_optional_unique_variant?");
        }
        
        let field_type = field.type_tokens();

        let field_pascal = format_ident!("{}", field.name().to_case(Case::Pascal));

        let variant_name = format_ident!("{}Equals", &field_pascal);
        self.unique_variants
            .push(quote!(#variant_name(#field_type)));

        self.from_unique_match_arms.push(quote! {
            UniqueWhereParam::#variant_name(value) => Self::#variant_name(value)
        });
    }

    pub fn add_optional_unique_variant(
        &mut self,
        field: &dml::Field
    ) {
        if !matches!(field.arity(), dml::FieldArity::Optional) {
            panic!("add_optional_unique_variant only adds optional fields. Perhaps you meant add_unique_variant?");
        }
        
        let field_base_type = field.field_type().to_tokens();

        let field_pascal = format_ident!("{}", field.name().to_case(Case::Pascal));
        let field_snake = format_ident!("{}", field.name().to_case(Case::Snake));

        let variant_name = format_ident!("{}Equals", &field_pascal);
        
        self.unique_variants
            .push(quote!(#variant_name(#field_base_type)));

        self.from_unique_match_arms.push(quote! {
            UniqueWhereParam::#variant_name(value) => Self::#variant_name(Some(value))
        });

        self.from_optional_uniques.push(quote!{
            impl ::prisma_client_rust::FromOptionalUniqueArg<#field_snake::Set> for WhereParam {
                type Arg = Option<#field_base_type>;
                
                fn from_arg(arg: Self::Arg) -> Self where Self: Sized {
                    Self::#variant_name(arg)
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
                dml::Field::ScalarField(_) => field.type_tokens(),
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

pub fn generate(args: &GenerateArgs, module_path: TokenStream) -> Vec<TokenStream> {
    let pcr = quote!(::prisma_client_rust);

    args.dml.models.iter().map(|model| {
        let mut model_query_modules = ModelQueryModules::new();
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
                        value
                            .into_iter()
                            .map(#pcr::WhereInput::serialize)
                            .map(Into::into)
                            .collect()
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

        let mut add_unique_variant = |fields: Vec<&dml::Field>| {
            if fields.len() == 1 {
                let field = fields[0];
                
                match field.arity()  {
                    dml::FieldArity::Optional => model_where_params.add_optional_unique_variant(field),
                    _ => model_where_params.add_unique_variant(field),
                }
            } else {
                let variant_name_string = fields.iter().map(|f| f.name().to_case(Case::Pascal)).collect::<String>();
                let variant_name = format_ident!("{}Equals", &variant_name_string);
                
                let mut variant_data_as_types = vec![];
                let mut variant_data_as_args = vec![];
                let mut variant_data_as_destructured = vec![];
                let mut variant_data_as_prisma_values = vec![];
                
                let variant_data_names = fields.iter().map(|f| f.name()).collect::<Vec<_>>();
            
                for field in &fields {
                    let field_base_type = field.field_type().to_tokens();
                    
                    let field_name_snake = format_ident!("{}", field.name().to_case(Case::Snake));
                    
                    let field_type = match field.arity().is_list() {
                        true => quote!(Vec<#field_base_type>),
                        false => quote!(#field_base_type),
                    };
                    
                    variant_data_as_args.push(quote!(#field_name_snake: #field_type));
                    variant_data_as_types.push(field_type);
                    variant_data_as_prisma_values.push(field.type_prisma_value(&field_name_snake));
                    variant_data_as_destructured.push(field_name_snake);
                }

                let field_name_string = fields.iter().map(|f| f.name()).collect::<Vec<_>>().join("_");

                model_query_modules.add_compound_field(&variant_name_string.to_case(Case::Snake), &variant_data_as_args, &variant_data_as_destructured);

                model_where_params.add_variant(
                    quote!(#variant_name(#(#variant_data_as_types),*)),
                    quote! {
                        Self::#variant_name(#(#variant_data_as_destructured),*) => (
                            #field_name_string,
                            #pcr::SerializedWhereValue::Object(vec![#((#variant_data_names.to_string(), #variant_data_as_prisma_values)),*])
                        )
                    },
                );
                
                model_where_params.add_compound_unique_variant(&variant_name_string, &variant_data_as_destructured, &variant_data_as_types);
            }
        };
        
        for unique in &model.indices {
            if unique.tpe != dml::IndexType::Unique { continue }
            
            add_unique_variant(unique.fields.iter().map(|field| model.fields.iter().find(|mf| mf.name() == &field.path[0].0).unwrap()).collect::<Vec<_>>());
        }
        
        if let Some(primary_key) = &model.primary_key {
            // if primary key is marked as unique, skip primary key handling
            if (primary_key.fields.len() == 1 && !model.field_is_unique(&primary_key.fields[0].name.as_str())) || (!model.indices
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
                    )) {
                add_unique_variant(primary_key.fields.iter().map(|field| model.fields.iter().find(|mf| mf.name() == field.name.as_str()).unwrap()).collect::<Vec<_>>());
            }
        }

        for root_field in &model.fields {
            let mut field_query_module =
                FieldQueryModule::new(&root_field);

            let field_string = root_field.name();
            let field_name_pascal = format_ident!("{}", field_string.to_case(Case::Pascal));
            let field_type = root_field.type_tokens();
            
            let set_variant = format_ident!("Set{}", field_name_pascal);

            match root_field {
                dml::Field::RelationField(field) => {
                    let connect_variant = format_ident!("Connect{}", field_name_pascal);
                    let disconnect_variant = format_ident!("Disconnect{}", field_name_pascal);
                    
                    let relation_model_name_snake = snake_ident(&field.relation_info.to);
                    
                    for method in root_field.relation_methods() {
                        let method_action_string = method.to_case(Case::Camel);
                        let variant_name = format_ident!("{}{}", &field_name_pascal, method.to_case(Case::Pascal));
                        let method_name_snake = format_ident!("{}", method.to_case(Case::Snake));
                        
                        model_where_params.add_variant(
                            quote!(#variant_name(Vec<super::#relation_model_name_snake::WhereParam>)),
                            quote! {
                                Self::#variant_name(where_params) => (
                                    #field_string,
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
                        
                        field_query_module.add_method(quote! {
                            pub fn #method_name_snake(value: Vec<#relation_model_name_snake::WhereParam>) -> WhereParam {
                                WhereParam::#variant_name(value)
                            }
                        });
                    }
                    
                    let with_fn = with_params::builder_fn(&field);
                    
                    if field.arity.is_list() {
                        let order_by_fn = order_by::fetch_builder_fn(&relation_model_name_snake);
                        let pagination_fns = pagination::fetch_builder_fns(&relation_model_name_snake);

                        field_query_module.add_method(quote! {
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
                        });
                    } else {
                        field_query_module.add_method(quote! {
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

                            pub fn connect<T: From<Connect>>(value: #relation_model_name_snake::UniqueWhereParam) -> T {
                                Connect(value).into()
                            }
                        });

                        field_query_module.add_struct(quote! {
                            pub struct Connect(#relation_model_name_snake::UniqueWhereParam);

                            impl From<Connect> for SetParam {
                                fn from(value: Connect) -> Self {
                                    Self::#connect_variant(value.0)
                                }
                            }
                        });

                        // Only allow disconnect if field is not required
                        if field.arity.is_optional() {
                            field_query_module.add_method(quote! {
                                pub fn disconnect() -> SetParam {
                                    SetParam::#disconnect_variant
                                }
                            });
                        }
                    }
                },
                dml::Field::ScalarField(field) => {
                    let field_set_variant = format_ident!("Set{}", field_name_pascal);
                    field_query_module.add_method(quote! {
                        pub fn set<T: From<Set>>(value: #field_type) -> T {
                            Set(value).into()
                        }
                    });

                    field_query_module.add_struct(quote! {
                        pub struct Set(pub #field_type);
                        impl From<Set> for SetParam {
                            fn from(value: Set) -> Self {
                                Self::#field_set_variant(value.0)
                            }
                        }
                    });

                    let equals_variant_name = format_ident!("{}Equals", &field_name_pascal);
                    let equals_variant = quote!(#equals_variant_name(#field_type));
                    let type_as_prisma_value= root_field.type_prisma_value(&format_ident!("value"));
                    
                    let type_as_prisma_value = if !root_field.arity().is_optional() {
                        type_as_prisma_value
                    } else {
                        quote!(value.map(|value| #type_as_prisma_value).unwrap_or(#pcr::PrismaValue::Null))
                    };
                    
                    model_where_params.add_variant(
                        equals_variant.clone(), 
                        quote! {
                            Self::#equals_variant_name(value) => (
                                #field_string,
                                #pcr::SerializedWhereValue::Object(vec![("equals".to_string(), #type_as_prisma_value)])
                            )
                        }
                    );
                    
                    // Add equals query functions. Unique/Where enum variants are added in unique/primary key sections earlier on.
                    field_query_module.add_method(
                        match (model.field_is_primary(field_string), model.field_is_unique(field_string), field.arity.is_required()) {
                            (true, _, _) | (_, true, true) => quote! {
                                pub fn equals<T: From<UniqueWhereParam>>(value: #field_type) -> T {
                                    UniqueWhereParam::#equals_variant_name(value).into()
                                }
                            },
                            (_, true, false) => quote! {
                                pub fn equals<A, T: #pcr::FromOptionalUniqueArg<Set, Arg = A>>(value: A) -> T {
                                    T::from_arg(value)
                                }
                            },
                            (_, _, _) => quote! {
                                pub fn equals(value: #field_type) -> WhereParam {
                                    WhereParam::#equals_variant_name(value).into()
                                }
                            }
                        }
                    );

                    // Pagination
                    field_query_module.add_method(quote! {
                        pub fn order(direction: #pcr::Direction) -> OrderByParam {
                            OrderByParam::#field_name_pascal(direction)
                        }
                    });

                    if let Some(read_type) = args.read_filter(&field) {
                        let filter_enum = format_ident!("{}Filter", &read_type.name);

                        model_where_params.add_variant(
                            quote!(#field_name_pascal(_prisma::read_filters::#filter_enum)),
                            quote! {
                                Self::#field_name_pascal(value) => (
                                    #field_string,
                                    value.into()
                                )
                            },
                        );

                        for method in &read_type.methods {
                            let method_name_snake = format_ident!("{}", method.name.to_case(Case::Snake));
                            let method_name_pascal =
                                format_ident!("{}", method.name.to_case(Case::Pascal));
                            
                            let typ = method.type_tokens();

                            field_query_module.add_method(quote! {
                                pub fn #method_name_snake(value: #typ) -> WhereParam {
                                    WhereParam::#field_name_pascal(_prisma::read_filters::#filter_enum::#method_name_pascal(value))
                                }
                            });
                        }
                    }

                    if let Some(write_type) = args.write_filter(&field) {
                        for method in &write_type.methods {
                            let method_name_snake = format_ident!("{}", method.name.to_case(Case::Snake));

                            let typ = method.type_tokens();

                            let variant_name = format_ident!("{}{}", method.name.to_case(Case::Pascal), field_name_pascal);

                            field_query_module.add_method(quote! {
                                pub fn #method_name_snake(value: #typ) -> SetParam {
                                    SetParam::#variant_name(value)
                                }
                            });
                        }
                    }
                },
                _ => unreachable!("Cannot codegen for composite field")
            };
            
            field_query_module.add_struct(include::field_module_enum(&root_field, &pcr));
            field_query_module.add_struct(select::field_module_enum(&root_field, &pcr));
            
            model_query_modules.add_field_module(field_query_module);
        }
        
        let data_struct = data::struct_definition(&model);
        let with_params_enum = with_params::enum_definition(&model);
        let set_params_enum = set_params::enum_definition(&model, args);
        let order_by_params_enum = order_by::enum_definition(&model);
        let outputs_fn = outputs::model_fn(&model);
        let create_fn = create::model_fns(&model);
        let query_modules = model_query_modules.quote();
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
                
                #query_modules
                
                #outputs_fn

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
