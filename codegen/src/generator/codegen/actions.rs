use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};

use crate::generator::dmmf::{Document, Method, Model, Type};

pub fn generate_actions(models: &Vec<Model>) -> TokenStream {
    models
        .iter()
        .map(|model| generate_model_actions(model))
        .collect()
}

fn generate_model_actions(model: &Model) -> TokenStream {
    let model_name_pascal = model.name.to_case(Case::Pascal);
    let model_name_pascal_ident = format_ident!("{}", model_name_pascal);
    let model_pascal_ident = format_ident!("{}Model", model_name_pascal);
    let model_actions_pascal = format_ident!("{}Actions", model.name.to_case(Case::Pascal));
    let model_where_param = format_ident!("{}WhereParam", &model_name_pascal_ident);

    let mut match_arms = Vec::new();
    let mut variants = Vec::new();

    for field in &model.fields {
        let field_type_string = format_ident!("{}", field.field_type.string());
        let field_name_pascal = field.name.to_case(Case::Pascal);
        let field_name = &field.name;

        if field.kind.is_relation() {
            let actions = if field.is_list {
                vec!["Some", "Every"]
            } else {
                vec!["Is"]
            };

            let field_type_where_param = format_ident!("{}WhereParam", field_type_string);

            let type_actions = actions
                .iter()
                .map(|name| {
                    let action_pascal = name.to_case(Case::Pascal);
                    let action_snake = name.to_case(Case::Snake);
                    let variant_name = format_ident!("{}{}", field_name_pascal, &action_pascal);

                    match_arms.push(quote! {
                        Self::#variant_name(value) =>
                            Field {
                                name: #field_name.into(),
                                fields: Some(vec![
                                    Field {
                                        name: #action_snake.into(),
                                        fields: Some(vec![
                                            Field {
                                                name: "AND".into(),
                                                list: true,
                                                wrap_list: true,
                                                fields: Some(value.into_iter().map(|f| f.field()).collect()),
                                                ..Default::default()
                                            }
                                        ]),
                                        ..Default::default()
                                    }
                                ]),
                                ..Default::default()
                            }
                    });

                    let q = format_ident!("{}{}", field_name_pascal, name);
                    quote! {#q(Vec<#field_type_where_param>)}
                })
                .collect::<Vec<_>>();

            variants.push(quote! { #(#type_actions),* });
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
                let action_pascal = m.action.to_case(Case::Pascal);
                let action_snake = &m.action;
                let variant_name = format_ident!("{}{}", field_name_pascal, &action_pascal);

                match_arms.push(quote! {
                    Self::#variant_name(value) =>
                        Field {
                            name: #field_name.into(),
                            fields: Some(vec![
                                Field {
                                    name: #action_snake.into(),
                                    value: Some(serde_json::to_value(value).unwrap()),
                                    ..Default::default()
                                }
                            ]),
                            ..Default::default()
                        }
                });

                variants.push(quote! { #variant_name(#field_type_value)});
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
        let op_name = &op.name;
        let name_ident = format_ident!("{}", op.name);
        let action = &op.action;

        variants.push(quote! { #name_ident(Vec<#model_where_param>) });
        match_arms.push(quote! {
            Self::#name_ident(value) =>
                Field {
                    name: #op_name.into(),
                    fields: Some(vec![
                        Field {
                            name: #action.into(),
                            list: true,
                            wrap_list: true,
                            fields: Some(value.into_iter().map(|f| f.field()).collect()),
                            ..Default::default()
                        }
                    ]),
                    ..Default::default()
                }
        });
    }

    let model_find_many = format_ident!("{}FindMany", &model_name_pascal_ident);
    let model_find_first = format_ident!("{}FindFirst", &model_name_pascal_ident);
    let model_find_unique = format_ident!("{}FindUnique", &model_name_pascal_ident);

    quote! {
        pub struct #model_actions_pascal<'a> {
            client: &'a PrismaClient,
        }

        pub enum #model_where_param {
            #(#variants),*
        }

        impl #model_where_param {
            pub fn field(&self) -> Field {
                match self {
                    #(#match_arms),*
                }
            }
        }

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
        }

        impl<'a> #model_actions_pascal<'a> {
            pub fn find_many(&self, params: Vec<#model_where_param>) -> #model_find_many {
                let where_fields: Vec<Field> = params.iter().map(|param|
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
                    method: "findMany".into(),
                    model: #model_name_pascal.into(),
                    outputs: vec![
                        #(#outputs),*
                    ],
                    inputs
                };

                #model_find_many { query }
            }

            pub fn find_first(&self, params: Vec<#model_where_param>) -> #model_find_first {
                let where_fields: Vec<Field> = params.iter().map(|param|
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

            // TODO: Dedicated unique field
            pub fn find_unique(&self, param: #model_where_param) -> #model_find_unique {
                let mut field = param.field();

                if let Some(fields) = &field.fields {
                    if let Some(inner) = fields.iter().find(|f| f.name == "equals") {
                        field.value = inner.value.clone();
                        field.fields = None;
                    }
                }

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
                        fields: vec![field],
                        ..Default::default()
                    }]
                };

                #model_find_unique { query }
            }
        }
    }
}
