use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};
use syn::Ident;

use crate::generator::dmmf::{Document, Method, Model, Type};

pub fn generate_actions(models: &Vec<Model>) -> TokenStream {
    models
        .iter()
        .map(|model| generate_model_actions(model))
        .collect()
}

struct ModelWhereParam {
    pub name: Ident,
    variants: Vec<TokenStream>,
    match_arms: Vec<TokenStream>,
}

impl ModelWhereParam {
    pub fn new(model_name: &str) -> Self {
        Self {
            name: format_ident!("{}WhereParam", model_name.to_case(Case::Pascal)),
            variants: vec![],
            match_arms: vec![],
        }
    }

    pub fn add_variant(&mut self, variant: TokenStream, match_arm: TokenStream) {
        self.variants.push(variant);
        self.match_arms.push(match_arm);
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            variants,
            match_arms,
            name,
        } = self;

        quote! {
            pub enum #name {
                #(#variants),*
            }

            impl #name {
                pub fn field(self) -> Field {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        }
    }
}

fn generate_model_actions(model: &Model) -> TokenStream {
    let model_name = &model.name;
    let model_name_pascal = model_name.to_case(Case::Pascal);
    let model_name_pascal_ident = format_ident!("{}", model_name_pascal);
    let model_pascal_ident = format_ident!("{}Model", model_name_pascal);
    let model_actions_pascal = format_ident!("{}Actions", model_name.to_case(Case::Pascal));
    let model_set_param = format_ident!("{}SetParam", &model_name_pascal_ident);

    let mut model_where_params = ModelWhereParam::new(&model.name);
    let model_where_param_ident = model_where_params.name.clone();

    for field in &model.fields {
        let field_type_string = format_ident!("{}", field.field_type.string());
        let field_name_pascal = field.name.to_case(Case::Pascal);
        let field_name = &field.name;

        if field.kind.is_relation() {
            let actions = field.relation_methods();

            let field_type_where_param = format_ident!("{}WhereParam", field_type_string);

            for action in actions {
                let action_name = action.name;
                let action = action.action;
                let variant_name = format_ident!("{}{}", field_name_pascal, &action_name);
                let q = format_ident!("{}{}", field_name_pascal, action_name);

                model_where_params.add_variant(quote! {#q(Vec<#field_type_where_param>)}, 
                quote! {
                        Self::#variant_name(value) =>
                            Field {
                                name: #field_name.into(),
                                fields: Some(vec![
                                    Field {
                                        name: "AND".into(),
                                        fields: Some(value.into_iter().map(|f| f.field()).collect()),
                                        ..Default::default()
                                    }
                                ]),
                                ..Default::default()
                            }
                    }
                );
            }
            
            // Link
            
            let variant_name = format_ident!("{}Link", field_name_pascal);
            let list_data = field.is_list.then(|| quote! {
                list: true,
                wrap_list: true,
            });
            
            model_where_params.add_variant(quote! { #variant_name(Box<#field_type_where_param>) }, 
            quote! {
                Self::#variant_name(value) =>
                    Field {
                        name: #field_name.into(),
                        fields: Some(vec![
                            Field {
                                name: "connect".into(),
                                fields: Some(vec![
                                    Field {
                                        name: "AND".into(),
                                        fields: Some(builder::transform_equals(vec![value.field()])),
                                        #list_data
                                        ..Default::default()
                                    }
                                ]),
                                ..Default::default()
                            }
                        ]),
                        ..Default::default()
                    }
            })
        } else {
            let read_types = match Document::read_types()
                .into_iter()
                .find(|t| t.name == field.field_type.string())
            {
                Some(mut t) => Type {
                    methods: {
                        t.methods.append(&mut vec![Method {
                            name: "Equals".into(),
                            action: "equals".into(),
                        }]);
                        t.methods
                    },
                    ..t
                },
                None => panic!("{:?}", field.field_type.string()),
            };

            let field_type_value = format_ident!("{}", field.field_type.value());

            for m in read_types.methods {
                let variant_name = format_ident!("{}{}", field_name_pascal, &m.name);
                let method_action = m.action;
                
                model_where_params.add_variant(
                quote! { #variant_name(#field_type_value)},
                quote! {
                    Self::#variant_name(value) =>
                        Field {
                            name: #field_name.into(),
                            fields: Some(vec![
                                Field {
                                    name: #method_action.into(), 
                                    value: Some(value.into()),
                                    ..Default::default()
                                }
                            ]),
                            ..Default::default()
                        }
                    }
                );
            }
        }
    }

    let outputs = model
        .fields
        .iter()
        .filter(|f| f.kind.include_in_struct())
        .map(|field| {
            let field_name = &field.name;
            quote! {
                Output::new(#field_name)
            }
        })
        .collect::<Vec<_>>();

    for op in Document::operators() {
        let name_ident = format_ident!("{}", op.name);
        let action = &op.action;

        model_where_params.add_variant(
            quote! { #name_ident(Vec<#model_where_param_ident>) },
            quote! {
                Self::#name_ident(value) =>
                    Field {
                        name: #action.into(),
                        list: true,
                        wrap_list: true,
                        fields: Some(value.into_iter().map(|f| f.field()).collect()),
                        ..Default::default()
                    }
            }
        );
    }

    let create_one_required_args =
        model
            .fields
            .iter()
            .filter(|f| f.required_on_create())
            .map(|f| {
                let arg_name = format_ident!("{}", &f.name.to_case(Case::Snake));
                let arg_type =
                    format_ident!("{}Set{}", model_name_pascal, f.name.to_case(Case::Pascal));
                quote! {
                    #arg_name: #arg_type,
                }
            });

    let create_one_required_arg_pushes =
        model
            .fields
            .iter()
            .filter(|f| f.required_on_create())
            .map(|f| {
                let arg_name = format_ident!("{}", &f.name.to_case(Case::Snake));
                let arg_type =
                    format_ident!("{}Set{}", model_name_pascal, f.name.to_case(Case::Pascal));
                quote! {
                    input_fields.push(#model_set_param::from(#arg_name).field());
                }
            });

    let model_create_one = format_ident!("{}CreateOne", &model_name_pascal_ident);
    let model_find_first = format_ident!("{}FindFirst", &model_name_pascal_ident);
    let model_find_unique = format_ident!("{}FindUnique", &model_name_pascal_ident);
    let model_find_many = format_ident!("{}FindMany", &model_name_pascal_ident);
    let model_delete = format_ident!("{}Delete", &model_name_pascal_ident);
    let model_where_params = model_where_params.quote();

    quote! {
        pub struct #model_actions_pascal<'a> {
            client: &'a PrismaClient,
        }

        #model_where_params

        pub struct #model_find_many<'a> {
            query: Query<'a>
        }

        impl<'a> #model_find_many<'a> {
            pub async fn exec(self) -> Vec<#model_pascal_ident> {
                let request = engine::GQLRequest {
                    query: self.query.build(),
                    variables: std::collections::HashMap::new(),
                };

                self.query.perform(request).await
            }
            

            pub fn delete(self) -> #model_delete<'a> {
                #model_delete {
                    query: Query {
                        operation: "mutation".into(),
                        method: "deleteMany".into(),
                        model: #model_name.into(),
                        outputs: vec! [
                            Output::new("count"),
                        ],
                        ..self.query
                    }
                }
            }
        }

        pub struct #model_find_first<'a> {
            query: Query<'a>
        }

        impl<'a> #model_find_first<'a> {
            pub async fn exec(self) -> #model_pascal_ident {
                let request = engine::GQLRequest {
                    query: self.query.build(),
                    variables: std::collections::HashMap::new(),
                };

                self.query.perform(request).await
            }
        }

        pub struct #model_find_unique<'a> {
            query: Query<'a>
        }

        impl<'a> #model_find_unique<'a> {
            pub async fn exec(self) -> #model_pascal_ident {
                let request = engine::GQLRequest {
                    query: self.query.build(),
                    variables: std::collections::HashMap::new(),
                };

                self.query.perform(request).await
            }

            pub fn delete(self) -> #model_delete<'a> {
                #model_delete {
                    query: Query {
                        operation: "mutation".into(),
                        method: "deleteOne".into(),
                        model: #model_name.into(),
                        ..self.query
                    }
                }
            }
        }

        pub struct #model_create_one<'a> {
            query: Query<'a>
        }

        impl<'a> #model_create_one<'a> {
            pub async fn exec(self) -> #model_pascal_ident {
                let request = engine::GQLRequest {
                    query: self.query.build(),
                    variables: std::collections::HashMap::new(),
                };

                self.query.perform(request).await
            }
        }

        pub struct #model_delete<'a> {
            query: Query<'a>
        }

        impl<'a> #model_delete<'a> {
            pub async fn exec(self) -> isize {
                let request = engine::GQLRequest {
                    query: self.query.build(),
                    variables: std::collections::HashMap::new(),
                };

                let result: DeleteResult = self.query.perform(request).await;
                
                result.count
            }
        }

        impl<'a> #model_actions_pascal<'a> {
            // TODO: Dedicated unique field
            pub fn find_unique(&self, param: #model_where_param_ident) -> #model_find_unique {
                let fields = builder::transform_equals(vec![param.field()]);

                let query = Query {
                    engine: self.client.engine.as_ref(),
                    name: String::new(),
                    operation: "query".into(),
                    method: "findUnique".into(),
                    model: #model_name_pascal.into(),
                    outputs: vec![
                        #(#outputs),*
                    ],
                    inputs: vec![Input {
                        name: "where".into(),
                        fields,
                        ..Default::default()
                    }]
                };

                #model_find_unique { query }
            }

            pub fn find_first(&self, params: Vec<#model_where_param_ident>) -> #model_find_first {
                let where_fields: Vec<Field> = params.into_iter().map(|param|
                    param.field()
                ).collect();

                let inputs = if where_fields.len() > 0 {
                    vec![Input {
                        name: "where".into(),
                        fields: vec![Field {
                            name: "AND".into(),
                            list: true,
                            wrap_list: true,
                            fields: Some(where_fields),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }]
                } else {
                    Vec::new()
                };

                let query = Query {
                    engine: self.client.engine.as_ref(),
                    name: String::new(),
                    operation: "query".into(),
                    method: "findFirst".into(),
                    model: #model_name_pascal.into(),
                    outputs: vec![
                        #(#outputs),*
                    ],
                    inputs
                };

                #model_find_first { query }
            }

            pub fn find_many(&self, params: Vec<#model_where_param_ident>) -> #model_find_many {
                let where_fields: Vec<Field> = params.into_iter().map(|param|
                    param.field()
                ).collect();

                let inputs = if where_fields.len() > 0 {
                    vec![Input {
                        name: "where".into(),
                        fields: where_fields,
                        ..Default::default()
                    }]
                } else {
                    Vec::new()
                };

                let query = Query {
                    engine: self.client.engine.as_ref(),
                    name: String::new(),
                    operation: "query".into(),
                    method: "findMany".into(),
                    model: #model_name_pascal.into(),
                    outputs: vec![
                        #(#outputs),*
                    ],
                    inputs
                };

                #model_find_many { query }
            }

            pub fn create_one(&self, #(#create_one_required_args)* params: Vec<#model_set_param>) -> #model_create_one {
                let mut input_fields = params.into_iter().map(|p| p.field()).collect::<Vec<_>>();
                
                #(#create_one_required_arg_pushes)*
                
                let query = Query {
                    engine: self.client.engine.as_ref(),
                    name: String::new(),
                    operation: "mutation".into(),
                    method: "createOne".into(),
                    model: #model_name_pascal.into(),
                    outputs: vec![
                        #(#outputs),*
                    ],
                    inputs: vec![Input {
                        name: "data".into(),
                        fields: input_fields,
                        ..Default::default()
                    }]
                };

                #model_create_one { query }
            }
        }


    }
}
