use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};
use syn::Ident;

use crate::generator::dmmf::{Document, Method, Model, Type};

struct Outputs {
    pub fn_name: Ident,
    outputs: Vec<TokenStream>
}

impl Outputs {
    pub fn new(model: &Model) -> Self {
        Self {
            fn_name: Self::get_fn_name(&model.name),
            outputs: model
                .fields
                .iter()
                .filter(|f| f.kind.include_in_struct())
                .map(|field| {
                    let field_name_string = &field.name;
                    quote!(Output::new(#field_name_string))
                }).collect()
        }
    }
    
    pub fn quote(&self) -> TokenStream {
        let Self {
            fn_name,
            outputs
        } = self;
        
        quote! {
            fn #fn_name() -> Vec<Output> {
                vec![
                    #(#outputs),*
                ]
            }
        }
    }

    pub fn get_fn_name(model_name: &str) -> Ident {
        format_ident!("{}_outputs", model_name.to_case(Case::Snake))
    }
}

struct WhereParams {
    pub enum_name: Ident,
    variants: Vec<TokenStream>,
    match_arms: Vec<TokenStream>,
}

impl WhereParams {
    pub fn new(model: &Model) -> Self {
        let enum_name = Self::get_enum_name(&model.name);
        let mut params = Self {
            enum_name: enum_name.clone(),
            variants: vec![],
            match_arms: vec![],
        };
        
        for field in &model.fields {
            let field_type_string = field.field_type.string();
            let field_name_pascal = field.name.to_case(Case::Pascal);
            let field_name_string = &field.name;

            if field.kind.is_relation() {
                let actions = field.relation_methods();

                let field_type_where_param = format_ident!("{}WhereParam", field_type_string);

                for action in actions {
                    let action_name = action.name;
                    let variant_name = format_ident!("{}{}", field_name_pascal, &action_name);

                    params.add_variant(
                        quote!(#variant_name(Vec<#field_type_where_param>)), 
                        quote! {
                            Self::#variant_name(value) =>
                                Field {
                                    name: #field_name_string.into(),
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
                    
                    params.add_variant(
                    quote!(#variant_name(#field_type_value)),
                    quote! {
                        Self::#variant_name(value) =>
                            Field {
                                name: #field_name_string.into(),
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

        for op in Document::operators() {
            let name_ident = format_ident!("{}", op.name);
            let action = &op.action;

            params.add_variant(
                quote!(#name_ident(Vec<#enum_name>)),
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
        
        params
    }

    fn add_variant(&mut self, variant: TokenStream, match_arm: TokenStream) {
        self.variants.push(variant);
        self.match_arms.push(match_arm);
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            variants,
            match_arms,
            enum_name: name,
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

    pub fn get_enum_name(model_name: &str) -> Ident {
        format_ident!("{}WhereParam", model_name.to_case(Case::Pascal))
    }
}

struct WithParams {
    pub enum_name: Ident,
    pub struct_name: Ident,
    variants: Vec<TokenStream>,
    match_arms: Vec<TokenStream>,
}

impl WithParams {
    pub fn new(model: &Model, outputs: &Outputs) -> Self {
        let model_name_pascal_string = model.name.to_case(Case::Pascal);

        let mut params = Self {
            enum_name: format_ident!("{}WithParam", &model_name_pascal_string),
            struct_name: format_ident!("{}With", &model_name_pascal_string),
            variants: vec![],
            match_arms: vec![],
        };
            
        model.fields.iter()
            .filter(|f| f.kind.is_relation())
            .for_each(|field| {
                let field_name_string = &field.name;
                let relation_type_string = field.field_type.string();
                
                let field_name_pascal = format_ident!("{}", &field.name.to_case(Case::Pascal));
                let relation_outputs_fn = Outputs::get_fn_name(relation_type_string);
                let relation_where_param = WhereParams::get_enum_name(&relation_type_string);
                
                if field.is_list {
                    params.add_variant(
                        quote!(#field_name_pascal(Vec<#relation_where_param>)),
                        quote! {
                            Self::#field_name_pascal(where_params) => Output {
                                name: #field_name_string.into(),
                                outputs: #relation_outputs_fn(),
                                inputs: if where_params.len() > 0 {
                                    vec![Input {
                                        name: "where".into(),
                                        fields: where_params.into_iter().map(|f| f.field()).collect(),
                                        ..Default::default()
                                    }]
                                } else { vec![] },
                                ..Default::default()
                            }
                        }
                    )
                } else {
                    params.add_variant(
                        quote!(#field_name_pascal),
                        quote! {
                            Self::#field_name_pascal => Output {
                                name: #field_name_string.into(),
                                outputs: #relation_outputs_fn(),
                                ..Default::default()
                            }
                        }
                    )
                }
            });
        
        params
    }

    fn add_variant(&mut self, variant: TokenStream, match_arm: TokenStream) {
        self.variants.push(variant);
        self.match_arms.push(match_arm);
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            variants,
            match_arms,
            enum_name: name,
            struct_name,
        } = self;

        quote! {
            pub struct #struct_name {
                pub param: #name
            }
            
            pub enum #name {
                #(#variants),*
            }

            impl From<#name> for #struct_name {
                fn from(param: #name) -> Self {
                    Self {
                        param
                    }
                }
            }

            impl #name {
                pub fn output(self) -> Output {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        }
    }
    
    pub fn with_fn(&self) -> TokenStream {
        let struct_name = &self.struct_name;
        
        quote! {
            pub fn with(mut self, fetches: Vec<#struct_name>) -> Self {
                let outputs = fetches
                    .into_iter()
                    .map(|f| f.param.output())
                    .collect::<Vec<_>>();
                self.query.outputs.extend(outputs);
                self
            }
        }
    }
}

struct SetParams {
    pub enum_name: Ident,
    variants: Vec<TokenStream>,
    match_arms: Vec<TokenStream>,
}

impl SetParams {
    pub fn new(model: &Model) -> Self {
        let model_name_pascal = format_ident!("{}", model.name.to_case(Case::Pascal));
        
        let mut params = Self {
            enum_name: format_ident!("{}SetParam", model_name_pascal),
            variants: vec![],
            match_arms: vec![]
        };
        
        for field in &model.fields {
            let field_name_string = &field.name;
            let field_type_string = field.field_type.value();
            let field_name_pascal = format_ident!("{}", field.name.to_case(Case::Pascal));
            let field_type_value = format_ident!("{}", &field_type_string);
            let relation_where_param = WhereParams::get_enum_name(&field_type_string);
            
            let (variant, match_arm) = match (field.kind.include_in_struct(), field.is_list) {
                (true, _) => (
                    quote! { #field_name_pascal(#field_type_value) }, 
                    quote! {
                        Self::#field_name_pascal(value) => Field {
                            name: #field_name_string.into(),
                            value: Some(value.into()),
                            ..Default::default()
                        }
                    }),
                (_, true) => (
                    quote! { #field_name_pascal(Vec<#relation_where_param>) }, 
                    quote! {
                        Self::#field_name_pascal(where_params) => Field {
                            name: #field_name_string.into(),
                            fields: Some(vec![
                                Field {
                                    name: "connect".into(),
                                    fields: Some(builder::transform_equals(
                                        where_params.into_iter().map(|item| item.field()).collect()
                                    )),
                                    list: true,
                                    wrap_list: true,
                                    ..Default::default()
                                }
                            ]),
                            ..Default::default()
                        }
                    }),
                (_, false) => (
                    quote! { #field_name_pascal(#relation_where_param) }, 
                    quote! {
                        Self::#field_name_pascal(where_param) => Field {
                            name: #field_name_string.into(),
                            fields: Some(vec![
                                Field {
                                    name: "connect".into(),
                                    fields: Some(builder::transform_equals(vec![
                                        where_param.field()
                                    ])),
                                    ..Default::default()
                                }
                            ]),
                            ..Default::default()
                        }
                    })
            };
            
            params.add_variant(variant, match_arm);
        }
        
        params
    }
    
    fn add_variant(&mut self, variant: TokenStream, match_arm: TokenStream) {
        self.variants.push(variant);
        self.match_arms.push(match_arm);
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            variants,
            match_arms,
            enum_name,
        } = self;

        quote! {
            pub enum #enum_name {
                #(#variants),*
            }

            impl #enum_name {
                pub fn field(self) -> Field {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        }
    }
}

struct QueryStructs {
    pub name: Ident,
    methods: Vec<TokenStream>,
    field_structs: Vec<TokenStream>
}

impl QueryStructs {
    pub fn new(model: &Model, set_params: &SetParams, where_params: &WhereParams, with_params: &WithParams) -> Self {
        let model_name_pascal = format_ident!("{}", model.name.to_case(Case::Pascal));
        
        let model_set_param = &set_params.enum_name;
        let model_where_param = &where_params.enum_name;
        
        let field_methods = model
            .fields
            .iter()
            .map(|field| {
                let field_method_name = format_ident!("{}", field.name.to_case(Case::Snake));
                let field_struct_name = format_ident!(
                    "{}{}Field",
                    model.name.to_case(Case::Pascal),
                    field.name.to_case(Case::Pascal)
                );

                quote! {
                    pub fn #field_method_name() -> #field_struct_name {
                        #field_struct_name {}
                    }
                }
            })
            .collect::<Vec<_>>();
            
        let operator_methods = Document::operators()
            .iter()
            .map(|op| {
                let method_name = format_ident!("{}", op.name.to_case(Case::Snake));
                let variant_name = format_ident!("{}", op.name.to_case(Case::Pascal));

                quote! {
                    pub fn #method_name(params: Vec<#model_where_param>) -> #model_where_param {
                        #model_where_param::#variant_name(params)
                    }
                }
            })
            .collect::<Vec<_>>();
            
        let mut methods = vec![];
        methods.extend(field_methods);
        methods.extend(operator_methods);
        
        let field_structs = model
            .fields
            .iter()
            .map(|field| { 
                let field_name_pascal = format_ident!("{}", field.name.to_case(Case::Pascal));
                let field_struct_name =
                    format_ident!("{}{}Field", model_name_pascal, &field_name_pascal);
                let field_type = format_ident!("{}", field.field_type.value());
                let relation_where_param = WhereParams::get_enum_name(&field.field_type.value());

                let mut field_struct_fns = if field.kind.is_relation() {
                    let methods = field.relation_methods();

                    methods
                        .iter()
                        .map(|m| {
                            let variant_name = format_ident!(
                                "{}{}",
                                &field_name_pascal,
                                m.name.to_case(Case::Pascal)
                            );
                            let method_name_snake = format_ident!("{}", m.name.to_case(Case::Snake));

                            quote! {
                                pub fn #method_name_snake(&self, value: Vec<#relation_where_param>) -> #model_where_param {
                                    #model_where_param::#variant_name(value)
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                        
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

                    read_types
                        .methods
                        .iter()
                        .map(|m| {
                            let variant_name = format_ident!(
                                "{}{}",
                                &field_name_pascal,
                                m.name
                            );
                            let method_name = format_ident!("{}", &m.name.to_case(Case::Snake));

                            quote! {
                                pub fn #method_name(&self, value: #field_type) -> #model_where_param {
                                    #model_where_param::#variant_name(value)
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                };
                 
                let field_set_struct_name = format_ident!("{}Set{}", model_name_pascal, &field_name_pascal);

                let field_set_struct = if field.kind.is_relation() {
                    let with_struct = &with_params.struct_name;
                    let with_enum = &with_params.enum_name;
                        
                    if field.is_list {
                        field_struct_fns.push(quote! {
                            pub fn link<T: From<#field_set_struct_name>>(&self, value: Vec<#relation_where_param>) -> T {
                                #field_set_struct_name(value).into()
                            }
                            
                            pub fn fetch(&self, params: Vec<#relation_where_param>) -> #with_struct {
                                #with_enum::#field_name_pascal(params).into()
                            }
                        });
                        
                        Some(quote! {
                            pub struct #field_set_struct_name(Vec<#relation_where_param>);
                            
                            impl From<#field_set_struct_name> for #model_set_param {
                                fn from(value: #field_set_struct_name) -> Self {
                                    Self::#field_name_pascal(value.0.into_iter().map(|v| v.into()).collect())
                                }
                            }
                        })
                    } else {
                        field_struct_fns.push(quote! {
                            pub fn link<T: From<#field_set_struct_name>>(&self, value: #relation_where_param) -> T {
                                #field_set_struct_name(value).into()
                            }
                            
                            pub fn fetch(&self) -> #with_struct {
                                #with_enum::#field_name_pascal.into()
                            }
                        });
                        
                        Some(quote! {
                            pub struct #field_set_struct_name(#relation_where_param);
                            
                            impl From<#field_set_struct_name> for #model_set_param {
                                fn from(value: #field_set_struct_name) -> Self {
                                    Self::#field_name_pascal(value.0)
                                }
                            }
                        })
                    }
                } else { 
                    field_struct_fns.push(quote! {
                        pub fn set<T: From<#field_set_struct_name>>(&self, value: #field_type) -> T {
                            #field_set_struct_name(value).into()
                        }
                    });
                    
                    Some(quote! { 
                        pub struct #field_set_struct_name(#field_type);
                    
                        impl From<#field_set_struct_name> for #model_set_param {
                            fn from(value: #field_set_struct_name) -> Self {
                                Self::#field_name_pascal(value.0)
                            }
                        } 
                    })
                };

                quote! {
                    pub struct #field_struct_name {}
                    
                    #field_set_struct

                    impl #field_struct_name {
                        #(#field_struct_fns)*
                    }
                }
            })
            .collect();
            
        Self {
            name: format_ident!("{}", model.name.to_case(Case::Pascal)),
            methods,
            field_structs
        }
    }
    
    pub fn quote(&self) -> TokenStream {
        let Self {
            name,
            methods,
            field_structs
        } = self;
        
        quote! {
            pub struct #name {}
            
            impl #name {
                #(#methods)*
            }
            
            #(#field_structs)*
        }
    }
}

struct DataStruct {
    pub name: Ident,
    fields: Vec<TokenStream>,
    relation_accessors: Vec<TokenStream>
}

impl DataStruct {
    pub fn new(model: &Model) -> Self {
        let fields = model
            .fields
            .iter()
            .map(|field| {
                let field_name_string = &field.name;
                let field_name_snake = format_ident!("{}", field.name.to_case(Case::Snake));
                let field_type_string = field.field_type.value();

                if field.kind.is_relation() {           
                    let field_type = Self::get_struct_name(&field_type_string);
                    
                    match (field.is_list, field.is_required) {
                        (true, _) => quote! {
                           #[serde(rename = #field_name_string)]
                           #field_name_snake: Option<Vec<#field_type>>
                        },
                        (_, true) => quote! {
                            #[serde(rename = #field_name_string)]
                            #field_name_snake: Option<#field_type>
                        },
                        (_, false) => quote! {
                            #[serde(rename = #field_name_string)]
                            pub #field_name_snake: Option<#field_type>
                        }
                    }
                } else{
                    let field_type = format_ident!("{}", &field_type_string);

                    match (field.is_list, field.is_required) {
                        (true, _) => quote! {
                            #[serde(rename = #field_name_string)]
                            pub #field_name_snake: Vec<#field_type>
                        },
                        (_, true) => quote! {
                            #[serde(rename = #field_name_string)]
                            pub #field_name_snake: #field_type
                        },
                        (_, false) => quote! {
                            #[serde(rename = #field_name_string)]
                            pub #field_name_snake: Option<#field_type>
                        }
                    }
                }
            })
            .collect();
        
        let relation_accessors = model
            .fields
            .iter()
            .filter(|f| f.kind.is_relation())
            .map(|field| {
                let field_name_snake = format_ident!("{}", field.name.to_case(Case::Snake));
                let field_type = DataStruct::get_struct_name(&field.field_type.value());
                
                let return_type = match field.is_list {
                    true => quote!(Vec<#field_type>),
                    false => quote!(#field_type)
                };
                
                if field.is_required {
                    let err = format!(
                        "attempted to access {} but did not fetch it using the .with() syntax",
                        field_name_snake
                    );

                    quote! {
                        pub fn #field_name_snake(&self) -> Result<&#return_type, String> {
                            match &self.#field_name_snake {
                                Some(v) => Ok(v),
                                None => Err(#err.to_string()),
                            }
                        }
                    }
                } else {
                    // TODO: Figure out double option to allow for null check
                    // println!("attempted to access optional relation {} but did not fetch it using the .with() syntax", field_name_snake);
                    
                    quote! {
                        pub fn #field_name_snake(&self) -> Option<&#return_type> {
                            self.#field_name_snake.as_ref()
                        }
                    }
                }
            }).collect();
            
        Self {
            name: Self::get_struct_name(&model.name),
            fields,
            relation_accessors
        }
    }
    
    pub fn quote(&self) -> TokenStream {
        let Self {
            name,
            fields,
            relation_accessors
        } = self;
        
        quote! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
            pub struct #name {
                #(#fields),*
            }
            
            impl #name {
                #(#relation_accessors)*
            }
        }
    }

    pub fn get_struct_name(model_name: &str) -> Ident {
        format_ident!("{}Data", model_name.to_case(Case::Pascal))
    }
}

pub fn generate(model: &Model) -> TokenStream {
    let name_pascal_string = model.name.to_case(Case::Pascal);
    let actions_struct = format_ident!("{}Actions", &name_pascal_string);
    
    let data_struct = DataStruct::new(&model);
    let data_struct_name = &data_struct.name;
    
    let outputs = Outputs::new(&model);
    let outputs_fn_name = &outputs.fn_name;

    let where_params = WhereParams::new(&model);
    let with_params = WithParams::new(&model, &outputs);
    let set_params = SetParams::new(&model);
    let query_structs = QueryStructs::new(&model, &set_params, &where_params, &with_params);
    
    let where_param_enum = where_params.enum_name.clone();
    let set_param_enum = set_params.enum_name.clone();

    let create_one_required_args =
        model
            .fields
            .iter()
            .filter(|f| f.required_on_create())
            .map(|f| {
                let arg_name = format_ident!("{}", &f.name.to_case(Case::Snake));
                let arg_type =
                    format_ident!("{}Set{}", name_pascal_string, f.name.to_case(Case::Pascal));
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
                quote! {
                    input_fields.push(#set_param_enum::from(#arg_name).field());
                }
            });

    let model_create_one = format_ident!("{}CreateOne", &name_pascal_string);
    let model_find_first = format_ident!("{}FindFirst", &name_pascal_string);
    let model_find_unique = format_ident!("{}FindUnique", &name_pascal_string);
    let model_find_many = format_ident!("{}FindMany", &name_pascal_string);
    let model_delete = format_ident!("{}Delete", &name_pascal_string);
    
    let data_struct = data_struct.quote();
    let model_where_params = where_params.quote();
    let model_with_params = with_params.quote();
    let model_set_params = set_params.quote();
    let model_outputs_fn = outputs.quote();
    let model_query_structs = query_structs.quote();
    
    let query_with_fn = with_params.with_fn();

    quote! {
        #model_outputs_fn
        
        #data_struct
        
        #model_query_structs
        
        pub struct #actions_struct<'a> {
            client: &'a PrismaClient,
        }

        #model_where_params
        
        #model_with_params
        
        #model_set_params

        pub struct #model_find_many<'a> {
            query: Query<'a>
        }

        impl<'a> #model_find_many<'a> {
            pub async fn exec(self) -> Vec<#data_struct_name> {
                self.query.perform().await
            } 

            pub fn delete(self) -> #model_delete<'a> {
                #model_delete {
                    query: Query {
                        operation: "mutation".into(),
                        method: "deleteMany".into(),
                        model: #name_pascal_string.into(),
                        outputs: vec! [
                            Output::new("count"),
                        ],
                        ..self.query
                    }
                }
            }
            
            #query_with_fn
        }

        pub struct #model_find_first<'a> {
            query: Query<'a>
        }

        impl<'a> #model_find_first<'a> {
            pub async fn exec(self) -> #data_struct_name {
                self.query.perform().await
            }
            
            #query_with_fn
        }

        pub struct #model_find_unique<'a> {
            query: Query<'a>
        }

        impl<'a> #model_find_unique<'a> {
            pub async fn exec(self) -> #data_struct_name {
                self.query.perform().await
            }

            pub fn delete(self) -> #model_delete<'a> {
                #model_delete {
                    query: Query {
                        operation: "mutation".into(),
                        method: "deleteOne".into(),
                        model: #name_pascal_string.into(),
                        ..self.query
                    }
                }
            }
            
            #query_with_fn
        }

        pub struct #model_create_one<'a> {
            query: Query<'a>
        }

        impl<'a> #model_create_one<'a> {
            pub async fn exec(self) -> #data_struct_name {
                self.query.perform().await
            }
        }

        pub struct #model_delete<'a> {
            query: Query<'a>
        }

        impl<'a> #model_delete<'a> {
            pub async fn exec(self) -> isize {
                let result: DeleteResult = self.query.perform().await;
                
                result.count
            }
        }

        impl<'a> #actions_struct<'a> {
            // TODO: Dedicated unique field
            pub fn find_unique(&self, param: #where_param_enum) -> #model_find_unique {
                let fields = builder::transform_equals(vec![param.field()]);

                let query = Query {
                    ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                    name: String::new(),
                    operation: "query".into(),
                    method: "findUnique".into(),
                    model: #name_pascal_string.into(),
                    outputs: #outputs_fn_name(),
                    inputs: vec![Input {
                        name: "where".into(),
                        fields,
                        ..Default::default()
                    }]
                };

                #model_find_unique { query }
            }

            pub fn find_first(&self, params: Vec<#where_param_enum>) -> #model_find_first {
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
                    ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                    name: String::new(),
                    operation: "query".into(),
                    method: "findFirst".into(),
                    model: #name_pascal_string.into(),
                    outputs: #outputs_fn_name(),
                    inputs
                };

                #model_find_first { query }
            }

            pub fn find_many(&self, params: Vec<#where_param_enum>) -> #model_find_many {
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
                    ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                    name: String::new(),
                    operation: "query".into(),
                    method: "findMany".into(),
                    model: #name_pascal_string.into(),
                    outputs: #outputs_fn_name(),
                    inputs
                };

                #model_find_many { query }
            }

            pub fn create_one(&self, #(#create_one_required_args)* params: Vec<#set_param_enum>) -> #model_create_one {
                let mut input_fields = params.into_iter().map(|p| p.field()).collect::<Vec<_>>();
                
                #(#create_one_required_arg_pushes)*
                
                let query = Query {
                    ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                    name: String::new(),
                    operation: "mutation".into(),
                    method: "createOne".into(),
                    model: #name_pascal_string.into(),
                    outputs: #outputs_fn_name(),
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
