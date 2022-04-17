use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};
use syn::Ident;

use crate::generator::{
    ast::{dmmf::Document, Model, Field},
    Root,
};

struct Outputs {
    outputs: Vec<TokenStream>,
}

impl Outputs {
    pub fn new(model: &Model) -> Self {
        Self {
            outputs: model
                .fields
                .iter()
                .filter(|f| f.kind.include_in_struct())
                .map(|field| {
                    let field_name_string = &field.name;
                    quote!(Output::new(#field_name_string))
                })
                .collect(),
        }
    }

    pub fn quote(&self) -> TokenStream {
        let Self { outputs } = self;

        quote! {
            pub fn _outputs() -> Vec<Output> {
                vec![
                    #(#outputs),*
                ]
            }
        }
    }
}

struct WithParams {
    pub with_fn: TokenStream,
    variants: Vec<TokenStream>,
    match_arms: Vec<TokenStream>,
}

impl WithParams {
    pub fn new() -> Self {

        Self {
            with_fn: quote! {
                pub fn with(mut self, param: WithParam) -> Self {
                    self.with_params.push(param);
                    self
                }
            },
            variants: vec![],
            match_arms: vec![],
        }
    }

    fn add_variant(&mut self, variant: TokenStream, match_arm: TokenStream) {
        self.variants.push(variant);
        self.match_arms.push(match_arm);
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            variants,
            match_arms,
            ..
        } = self;

        quote! {
            pub enum WithParam {
                #(#variants),*
            }

            impl WithParam {
                pub fn to_output(self) -> Output {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        }
    }
}

struct SetParams {
    variants: Vec<TokenStream>,
    match_arms: Vec<TokenStream>,
}

impl SetParams {
    pub fn new() -> Self {
        Self {
            variants: vec![],
            match_arms: vec![],
        }
    }

    fn add_variant(&mut self, variant: TokenStream, match_arm: TokenStream) {
        self.variants.push(variant);
        self.match_arms.push(match_arm);
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            variants,
            match_arms,
            ..
        } = self;

        quote! {
            pub enum SetParam {
                #(#variants),*
            }

            impl SetParam {
                pub fn to_field(self) -> Field {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        }
    } 

    pub fn field_link_variant(field_name: &str) -> Ident {
        format_ident!("Link{}", field_name.to_case(Case::Pascal))
    }

    pub fn field_unlink_variant(field_name: &str) -> Ident {
        format_ident!("Unlink{}", field_name.to_case(Case::Pascal))
    }

    pub fn field_set_variant(field_name: &str) -> Ident {
        format_ident!("Set{}", field_name.to_case(Case::Pascal))
    }
}

struct OrderByParams {
    pub order_by_fn: TokenStream,
    variants: Vec<TokenStream>,
    match_arms: Vec<TokenStream>,
}

impl OrderByParams {
    pub fn new() -> Self {
        Self {
            order_by_fn: quote! {
                pub fn order_by(mut self, param: OrderByParam) -> Self {
                    self.order_by_params.push(param);
                    self
                }
            },
            variants: vec![],
            match_arms: vec![],
        }
    }

    fn add_variant(&mut self, variant: TokenStream, match_arm: TokenStream) {
        self.variants.push(variant);
        self.match_arms.push(match_arm);
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            variants,
            match_arms,
            ..
        } = self;

        quote! {
            pub enum OrderByParam {
                #(#variants),*
            }

            impl OrderByParam {
                pub fn to_field(self) -> Field {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        }
    }
}

struct PaginationParams {
    pub pagination_fns: TokenStream,
    cursor_variants: Vec<TokenStream>,
    cursor_match_arms: Vec<TokenStream>,
}

impl PaginationParams {
    pub fn new() -> Self {
        let pagination_fns = quote! {
            pub fn skip(mut self, skip: usize) -> Self {
                self.query.inputs.push(Input {
                    name: "skip".into(),
                    value: Some(serde_json::to_value(skip).unwrap()),
                    ..Default::default()
                });
                self
            }

            pub fn take(mut self, take: usize) -> Self {
                self.query.inputs.push(Input {
                    name: "take".into(),
                    value: Some(serde_json::to_value(take).unwrap()),
                    ..Default::default()
                });
                self
            }

            pub fn cursor(mut self, cursor: Cursor) -> Self {
                self.query.inputs.push(Input {
                    name: "cursor".into(),
                    fields: vec![cursor.to_field()],
                    ..Default::default()
                });
                self
            }
        };

        Self {
            pagination_fns,
            cursor_variants: vec![],
            cursor_match_arms: vec![],
        }
    }

    pub fn add_variant(&mut self, variant: TokenStream, match_arm: TokenStream) {
        self.cursor_variants.push(variant);
        self.cursor_match_arms.push(match_arm);
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            cursor_variants,
            cursor_match_arms,
            ..
        } = self;

        quote! {
            pub enum Cursor {
                #(#cursor_variants),*
            }

            impl Cursor {
                fn to_field(self) -> Field {
                    match self {
                        #(#cursor_match_arms),*
                    }
                }
            }
        }
    }
}

struct FieldQueryModule {
    name: Ident,
    methods: Vec<TokenStream>,
    structs: Vec<TokenStream>
}

impl FieldQueryModule {
    pub fn new(field: &Field) -> Self {
        Self {
            name: format_ident!("{}", field.name.to_case(Case::Snake)),
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
                use super::{WhereParam, UniqueWhereParam, OrderByParam, Cursor, WithParam, SetParam};
                
                #(#methods)*
                
                #(#structs)*
            }
        }
    }
}

struct ModelQueryModules {
    field_modules: Vec<FieldQueryModule>,
    compound_field_accessors: Vec<TokenStream>
}

impl ModelQueryModules {
    pub fn new() -> Self {
        Self {
            field_modules: vec![],
            compound_field_accessors: vec![]
        }
    }
    
    pub fn add_field_module(&mut self, field_module: FieldQueryModule) {
        self.field_modules.push(field_module);
    }
    
    pub fn add_compound_field(&mut self, compound_field_accessor: TokenStream) {
        self.compound_field_accessors.push(compound_field_accessor);
    }
    
    pub fn quote(&self) -> TokenStream {
        let Self {
            
            field_modules,
            compound_field_accessors
        } = self;
        
        let field_modules = field_modules.iter().map(|field| field.quote()).collect::<Vec<_>>();
        
        quote! {
            #(#field_modules)*
            
            #(#compound_field_accessors)*
        }
    }
}

struct WhereParams {
    pub variants: Vec<TokenStream>,
    pub to_field: Vec<TokenStream>,
    pub unique_variants: Vec<TokenStream>,
    pub from_unique_match_arms: Vec<TokenStream>,
    pub from_optional_uniques: Vec<TokenStream>
}

impl WhereParams {
    pub fn new() -> Self {
        Self {
            variants: vec![],
            to_field: vec![],
            unique_variants: vec![],
            from_unique_match_arms: vec![],
            from_optional_uniques: vec![]
        }
    }

    pub fn add_variant(&mut self, variant: TokenStream, match_arm: TokenStream) {
        self.variants.push(variant);
        self.to_field.push(match_arm);
    }

    pub fn add_unique_variant(
        &mut self,
        variant: TokenStream,
        match_arm: TokenStream,
        from_unique_match_arm: TokenStream,
        unique_variant: TokenStream
    ) {
        self.add_variant(variant, match_arm);
        self.unique_variants.push(unique_variant);
        self.from_unique_match_arms.push(from_unique_match_arm);
    }
    
    pub fn add_optional_unique_variant(
        &mut self,
        variant: TokenStream,
        match_arm: TokenStream,
        from_unique_match_arm: TokenStream,
        unique_variant: TokenStream,
        arg_type: &TokenStream,
        variant_name: &syn::Ident,
        struct_name: TokenStream
    ) {
        self.add_unique_variant(variant, match_arm, from_unique_match_arm, unique_variant);
        
        self.from_optional_uniques.push(quote!{
            impl prisma_client_rust::traits::FromOptionalUniqueArg<#struct_name> for WhereParam {
                type Arg = Option<#arg_type>;
                
                fn from_arg(arg: Self::Arg) -> Self where Self: Sized {
                    Self::#variant_name(arg)
                }
            }
            
            impl prisma_client_rust::traits::FromOptionalUniqueArg<#struct_name> for UniqueWhereParam {
                type Arg = #arg_type;
                
                fn from_arg(arg: Self::Arg) -> Self where Self: Sized {
                    Self::#variant_name(arg)
                }
            }
        });
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            variants,
            to_field,
            unique_variants,
            from_unique_match_arms,
            from_optional_uniques
        } = self;

        quote! {
            pub enum WhereParam {
                #(#variants),*
            }

            impl WhereParam {
                pub fn to_field(self) -> Field {
                    match self {
                        #(#to_field),*
                    }
                }
            }

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

            impl From<Operator<Self>> for WhereParam {
                fn from(op: Operator<Self>) -> Self {
                    match op {
                        Operator::Not(value) => Self::Not(value),
                        Operator::And(value) => Self::And(value),
                        Operator::Or(value) => Self::Or(value),
                    }
                }
            }
        }
    }
}

struct DataStruct {
    fields: Vec<TokenStream>,
    relation_accessors: Vec<TokenStream>,
}

impl DataStruct {
    pub fn new() -> Self {
        Self {
            fields: vec![],
            relation_accessors: vec![],
        }
    }

    pub fn add_field(&mut self, field: TokenStream) {
        self.fields.push(field);
    }

    pub fn add_relation(&mut self, field: TokenStream, accessor: TokenStream) {
        self.fields.push(field);
        self.relation_accessors.push(accessor);
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            fields,
            relation_accessors,
        } = self;

        quote! {
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct Data {
                #(#fields),*
            }

            impl Data {
                #(#relation_accessors)*
            }
        }
    }
}

struct Actions {
    pub required_args: Vec<TokenStream>,
    pub required_arg_pushes: Vec<TokenStream>,
}

impl Actions {
    pub fn new() -> Self {
        Self {
            required_args: vec![],
            required_arg_pushes: vec![],
        }
    }

    pub fn push_required_arg(&mut self, arg: TokenStream, arg_push: TokenStream) {
        self.required_args.push(arg);
        self.required_arg_pushes.push(arg_push);
    }
}

pub fn generate(root: &Root) -> Vec<TokenStream> {
    root.ast.as_ref().unwrap().models.iter().map(|model| {
        let model_outputs = Outputs::new(&model);
        let mut model_data_struct = DataStruct::new();
        let mut model_order_by_params = OrderByParams::new();
        let mut model_pagination_params = PaginationParams::new();
        let mut model_with_params = WithParams::new();
        let mut model_query_module = ModelQueryModules::new();
        let mut model_set_params = SetParams::new();
        let mut model_where_params = WhereParams::new();
        let mut model_actions = Actions::new();

        let model_name_pascal_string = model.name.to_case(Case::Pascal);
        let model_name_snake = format_ident!("{}", model.name.to_case(Case::Snake));
 
        for op in Document::operators() {
            let variant_name = format_ident!("{}", op.name.to_case(Case::Pascal));
            let op_action = &op.action;

            model_where_params.add_variant(
                quote!(#variant_name(Vec<WhereParam>)),
                quote! {
                    Self::#variant_name(value) => Field {
                        name: #op_action.into(),
                        list: true,
                        wrap_list: true,
                        fields: Some(value.into_iter().map(|f| f.to_field()).collect()),
                        ..Default::default()
                    }
                },
            );
        }

        for unique in &model.indexes {
            let variant_name_string = unique.fields.iter().map(|f| f.to_case(Case::Pascal)).collect::<String>();
            let variant_name = format_ident!("{}Equals", &variant_name_string);
            let accessor_name = format_ident!("{}", &variant_name_string.to_case(Case::Snake));
            
            let mut variant_data_as_types = vec![];
            let mut variant_data_as_args = vec![];
            let mut variant_data_as_destructured = vec![];
            
            for field in &unique.fields {
                let model_field = model.fields.iter().find(|mf| &mf.name == field).unwrap();
                let field_type = model_field.field_type.tokens();
                
                let field_name_snake = format_ident!("{}", field.to_case(Case::Snake));
                
                let field_type = match (model_field.is_list, model_field.is_required) {
                    (true, _) => quote!(Vec<#field_type>),
                    (_, true) => quote!(#field_type),
                    (_, false) => quote!(Option<#field_type>),
                };
                
                variant_data_as_args.push(quote!(#field_name_snake: #field_type));
                variant_data_as_types.push(field_type);
                variant_data_as_destructured.push(quote!(#field_name_snake));
            }

            let field_name_string = unique.fields.join("_");

            let variant_data_where_params = unique.fields.iter().map(|f| {
                let field_name = format_ident!("{}", f.to_case(Case::Snake));
                let equals_variant = format_ident!("{}Equals", f.to_case(Case::Pascal));

                quote!(super::WhereParam::#equals_variant(#field_name))
            }).collect::<Vec<_>>();

            model_query_module.add_compound_field(
                quote! {
                    pub fn #accessor_name<T: From<UniqueWhereParam>>(#(#variant_data_as_args),*) -> T {
                        UniqueWhereParam::#variant_name(#(#variant_data_as_destructured),*).into()
                    }
                }
            );

            model_where_params.add_unique_variant(
                quote!(#variant_name(#(#variant_data_as_types),*)),
                quote! {
                    Self::#variant_name(#(#variant_data_as_destructured),*) => {
                        Field {
                            name: #field_name_string.into(),
                            fields: Some(transform_equals(vec![
                                    #(#variant_data_where_params),*
                                ]
                                .into_iter()
                                .map(|f| f.to_field())
                                .collect()
                            )),
                            ..Default::default()
                        }
                    }
                },
                quote! {
                    UniqueWhereParam::#variant_name(#(#variant_data_as_destructured),*) => Self::#variant_name(#(#variant_data_as_destructured),*)
                },
                quote!(#variant_name(#(#variant_data_as_types),*)),
            );
        }

        for field in &model.fields {
            let mut field_query_module =
                FieldQueryModule::new(&field);

            let field_string = &field.name;
            let field_snake = format_ident!("{}", field.name.to_case(Case::Snake));
            let field_pascal = format_ident!("{}", field.name.to_case(Case::Pascal));
            let field_type_tokens_string = field.field_type.value();
            let field_type = field.field_type.tokens();

            if field.kind.is_relation() {
                let link_variant = SetParams::field_link_variant(&field.name);
                let unlink_variant = SetParams::field_unlink_variant(&field.name);

                let relation_type_snake = format_ident!("{}", field_type_tokens_string.to_case(Case::Snake));  

                let relation_data_access_error = format!(
                    "Attempted to access {} but did not fetch it using the .with() syntax",
                    field_string.to_case(Case::Snake)
                );

                // Relation methods eg. Every, Some, Is
                for method in field.relation_methods() {
                    let method_action_string = &method.action;
                    let variant_name =
                        format_ident!("{}{}", &field_pascal, method.name.to_case(Case::Pascal));
                    let method_name_snake = format_ident!("{}", method.name.to_case(Case::Snake));

                    model_where_params.add_variant(
                        quote!(#variant_name(Vec<super::#relation_type_snake::WhereParam>)),
                        quote! {
                            Self::#variant_name(value) => Field {
                                name: #field_string.into(),
                                fields: Some(vec![Field {
                                    name: #method_action_string.into(),
                                    fields: Some(value.into_iter().map(|f| f.to_field()).collect()),
                                    ..Default::default()
                                }]),
                                ..Default::default()
                            }
                        },
                    );

                    field_query_module.add_method(quote! {
                        pub fn #method_name_snake(value: Vec<#relation_type_snake::WhereParam>) -> WhereParam {
                            WhereParam::#variant_name(value)
                        }
                    });
                }

                // Relation actions eg. Fetch, Link, Unlink
                if field.is_list {
                    field_query_module.add_method(quote! {
                        pub fn fetch(params: Vec<#relation_type_snake::WhereParam>) -> WithParam {
                            WithParam::#field_pascal(params)
                        }

                        pub fn link<T: From<Link>>(params: Vec<#relation_type_snake::UniqueWhereParam>) -> T {
                            Link(params).into()
                        }

                        pub fn unlink(params: Vec<#relation_type_snake::UniqueWhereParam>) -> SetParam {
                            SetParam::#unlink_variant(params)
                        }
                    });

                    field_query_module.add_struct(quote! {
                        pub struct Link(Vec<#relation_type_snake::UniqueWhereParam>);

                        impl From<Link> for SetParam {
                            fn from(value: Link) -> Self {
                                Self::#link_variant(value.0)
                            }
                        }
                    });

                    model_set_params.add_variant(
                        quote!(#link_variant(Vec<super::#relation_type_snake::UniqueWhereParam>)),
                        quote! {
                            Self::#link_variant(where_params) => Field {
                                name: #field_string.into(),
                                fields: Some(vec![
                                    Field {
                                        name: "connect".into(),
                                        fields: Some(transform_equals(
                                            where_params
                                                .into_iter()
                                                .map(|param| Into::<super::#relation_type_snake::WhereParam>::into(param)
                                                    .to_field())
                                                .collect()
                                        )),
                                        list: true,
                                        wrap_list: true,
                                        ..Default::default()
                                    }
                                ]),
                                ..Default::default()
                            }
                        }
                    );

                    model_set_params.add_variant(
                        quote!(#unlink_variant(Vec<super::#relation_type_snake::UniqueWhereParam>)),
                        quote! {
                            Self::#unlink_variant(where_params) => Field {
                                name: #field_string.into(),
                                fields: Some(vec![
                                    Field {
                                        name: "disconnect".into(),
                                        list: true,
                                        wrap_list: true,
                                        fields: Some(transform_equals(
                                            where_params
                                                .into_iter()
                                                .map(|param| Into::<super::#relation_type_snake::WhereParam>::into(param)
                                                    .to_field())
                                                .collect()
                                        )),
                                        ..Default::default()
                                    }
                                ]),
                                ..Default::default()
                            }
                        },
                    );

                    model_with_params.add_variant(
                        quote!(#field_pascal(Vec<super::#relation_type_snake::WhereParam>)),
                        quote! {
                            Self::#field_pascal(where_params) => Output {
                                name: #field_string.into(),
                                outputs: super::#relation_type_snake::_outputs(),
                                inputs: if where_params.len() > 0 {
                                    vec![Input {
                                        name: "where".into(),
                                        fields: where_params
                                            .into_iter()
                                            .map(|param| Into::<super::#relation_type_snake::WhereParam>::into(param)
                                                .to_field())
                                            .collect(),
                                        ..Default::default()
                                    }]
                                } else { vec![] },
                                ..Default::default()
                            }
                        },
                    );

                    model_data_struct.add_relation(
                        quote! {
                           #[serde(rename = #field_string)]
                           #field_snake: Option<Vec<super::#relation_type_snake::Data>>
                        },
                        quote! {
                            pub fn #field_snake(&self) -> Result<&Vec<super::#relation_type_snake::Data>, String> {
                                match self.#field_snake.as_ref() {
                                    Some(v) => Ok(v),
                                    None => Err(#relation_data_access_error.to_string()),
                                }
                            }
                        },
                    );
                } else {
                    field_query_module.add_method(quote! {
                        pub fn fetch() -> WithParam {
                            WithParam::#field_pascal
                        }

                        pub fn link<T: From<Link>>(value: #relation_type_snake::UniqueWhereParam) -> T {
                            Link(value).into()
                        }
                    });

                    field_query_module.add_struct(quote! {
                        pub struct Link(#relation_type_snake::UniqueWhereParam);

                        impl From<Link> for SetParam {
                            fn from(value: Link) -> Self {
                                Self::#link_variant(value.0)
                            }
                        }
                    });

                    model_set_params.add_variant(
                        quote!(#link_variant(super::#relation_type_snake::UniqueWhereParam)),
                        quote! {
                            Self::#link_variant(where_param) => Field {
                                name: #field_string.into(),
                                fields: Some(vec![
                                    Field {
                                        name: "connect".into(),
                                        fields: Some(transform_equals(vec![
                                            Into::<super::#relation_type_snake::WhereParam>::into(where_param).to_field()
                                        ])),
                                        ..Default::default()
                                    }
                                ]),
                                ..Default::default()
                            }
                        }
                    );

                    if !field.is_required {
                        field_query_module.add_method(quote! {
                            pub fn unlink() -> SetParam {
                                SetParam::#unlink_variant
                            }
                        });

                        model_set_params.add_variant(
                            quote!(#unlink_variant),
                            quote! {
                                Self::#unlink_variant => Field {
                                    name: #field_string.into(),
                                    fields: Some(vec![Field {
                                        name: "disconnect".into(),
                                        value: Some(true.into()),
                                        ..Default::default()
                                    }]),
                                    ..Default::default()
                                }
                            },
                        );
                    }

                    model_with_params.add_variant(
                        quote!(#field_pascal),
                        quote! {
                            Self::#field_pascal => Output {
                                name: #field_string.into(),
                                outputs: super::#relation_type_snake::_outputs(),
                                ..Default::default()
                            }
                        },
                    );
                    
                    let accessor_type = if field.is_required {
                        quote!(& super::#relation_type_snake::Data)
                    } else {
                        quote!(Option<& super::#relation_type_snake::Data>)
                    };
                    
                    let struct_field_type = if field.is_required {
                        quote!(Box<super::#relation_type_snake::Data>)
                    } else {
                        quote!(Option<Box<super::#relation_type_snake::Data>>)
                    };
                    
                    let serde_attr = if field.is_required {
                        quote!(#[serde(rename = #field_string)])
                    } else {
                        quote! {
                           #[serde(
                                rename = #field_string, 
                                default, 
                                skip_serializing_if = "Option::is_none", 
                                with = "prisma_client_rust::serde::double_option"
                            )]
                        }
                    };
                    
                    if field.is_required {
                        model_data_struct.add_relation(
                            quote! {
                                #serde_attr
                                #field_snake: Option<#struct_field_type>
                            },
                            quote! {
                                pub fn #field_snake(&self) -> Result<#accessor_type, String> {
                                    match self.#field_snake.as_ref() {
                                        Some(v) => Ok(v),
                                        None => Err(#relation_data_access_error.to_string()),
                                    }
                                }
                            } 
                        );
                    } else {
                        model_data_struct.add_relation(
                            quote! {
                                #[serde(rename = #field_string)]
                                #field_snake: Option<#struct_field_type>
                            },
                            quote! {
                                pub fn #field_snake(&self) -> Result<#accessor_type, String> {
                                    match self.#field_snake.as_ref() {
                                        Some(v) => Ok(v.as_ref().map(|v| v.as_ref())),
                                        None => Err(#relation_data_access_error.to_string()),
                                    }
                                }
                            },
                        );
                    }
                };

                if field.required_on_create() {
                    model_actions.push_required_arg(
                        quote!(#field_snake: #field_snake::Link,),
                        quote!(input_fields.push(SetParam::from(#field_snake).to_field());),
                   );
                }
            }
            // Scalar actions
            else {
                let field_set_variant = SetParams::field_set_variant(&field.name);

                if !field.prisma {
                    let (field_set_variant_type, field_content) = if field.is_list {
                        (
                            quote!(Vec<#field_type>),
                            quote!(fields: Some(value.iter().map(|f| f.to_field()).collect())),
                        )
                    } else {
                        let typ = if field.is_required {
                            quote!(#field_type)
                        } else {
                            quote!(Option<#field_type>)
                        };
                        
                        (
                            typ,
                            quote!(value: Some(serde_json::to_value(value).unwrap())),
                        )
                    };

                    field_query_module.add_method(quote! {
                        pub fn set<T: From<Set>>(value: #field_set_variant_type) -> T {
                            Set(value).into()
                        }
                    });

                    field_query_module.add_struct(quote! {
                        pub struct Set(#field_set_variant_type);
                        impl From<Set> for SetParam {
                            fn from(value: Set) -> Self {
                                Self::#field_set_variant(value.0)
                            }
                        }
                    });

                    model_set_params.add_variant(
                        quote!(#field_set_variant(#field_set_variant_type)),
                        quote! {
                            Self::#field_set_variant(value) => Field {
                                name: #field_string.into(),
                                value: Some(serde_json::to_value(value).unwrap()),
                                ..Default::default()
                            }
                        },
                    );

                    let equals_variant_name = format_ident!("{}Equals", &field_pascal);
                    let equals_variant = quote!(#equals_variant_name(#field_set_variant_type));

                    let match_arm = quote! {
                        Self::#equals_variant_name(value) => Field {
                            name: #field_string.into(),
                            fields: Some(vec![Field {
                                name: "equals".into(),
                                #field_content,
                                ..Default::default()
                            }]),
                            ..Default::default()
                        }
                    };

                    match (field.is_id, field.is_unique, field.is_required)  {
                        (true, _, _) | (_, true, true) => {
                            model_where_params.add_unique_variant(
                                equals_variant.clone(),
                                match_arm,
                                quote! {
                                    UniqueWhereParam::#equals_variant_name(value) => Self::#equals_variant_name(value)
                                },
                                equals_variant
                            );
                            field_query_module.add_method(quote! {
                                pub fn equals<T: From<UniqueWhereParam>>(value: #field_set_variant_type) -> T {
                                    UniqueWhereParam::#equals_variant_name(value).into()
                                }
                            });
                        }
                        (_, true, false) => {
                            model_where_params.add_optional_unique_variant(
                                equals_variant,
                                match_arm,
                                quote! {
                                    UniqueWhereParam::#equals_variant_name(value) => Self::#equals_variant_name(Some(value))
                                },
                                quote!(#equals_variant_name(#field_type)),
                                &field_type,
                                &equals_variant_name,
                                quote!(#field_snake::Set)
                            );
                            
                            field_query_module.add_method(quote! {
                                pub fn equals<A, T: prisma_client_rust::traits::FromOptionalUniqueArg<Set, Arg = A>>(value: A) -> T {
                                    T::from_arg(value)
                                }
                            });
                        },
                        (_, _, _) => {
                            model_where_params.add_variant(equals_variant, match_arm);
                            field_query_module.add_method(quote! {
                                pub fn equals(value: #field_set_variant_type) -> WhereParam {
                                    WhereParam::#equals_variant_name(value).into()
                                }
                            });
                        }
                    };

                    // Pagination
                    field_query_module.add_method(quote! {
                        pub fn order(direction: Direction) -> OrderByParam {
                            OrderByParam::#field_pascal(direction)
                        }

                        pub fn cursor(cursor: #field_type) -> Cursor {
                            Cursor::#field_pascal(cursor)
                        }
                    });

                    model_data_struct.add_field(match (field.is_list, field.is_required) {
                        (true, _) => quote! {
                            #[serde(rename = #field_string)]
                            pub #field_snake: Vec<#field_type>
                        },
                        (_, true) => quote! {
                            #[serde(rename = #field_string)]
                            pub #field_snake: #field_type
                        },
                        (_, false) => quote! {
                            #[serde(rename = #field_string)]
                            pub #field_snake: Option<#field_type>
                        },
                    });
                }
                
                let write_type = root
                    .ast
                    .as_ref()
                    .unwrap()
                    .write_filter(field.field_type.string(), field.is_list);

                if let Some(write_type) = write_type {
                    for method in &write_type.methods {
                        let typ = match method.typ.string() {
                            "" => field.field_type.tokens(),
                            _ => method.typ.tokens(),
                        };

                        let method_name_snake = format_ident!("{}", method.name.to_case(Case::Snake));

                        let typ = if method.is_list {
                            quote!(Vec<#typ>)
                        } else { typ };

                        let variant_name = format_ident!("{}{}", method.name.to_case(Case::Pascal), field_pascal);

                        field_query_module.add_method(quote! {
                            pub fn #method_name_snake(value: #typ) -> SetParam {
                                SetParam::#variant_name(value)
                            }
                        });
                        
                        let method_action = &method.action;
                        model_set_params.add_variant(
                            quote!(#variant_name(#typ)),
                            quote! {
                                Self::#variant_name(value) => Field {
                                    name: #field_string.into(),
                                    fields: Some(vec![Field{
                                        name: #method_action.into(),
                                        value: Some(serde_json::to_value(value).unwrap()),
                                        ..Default::default()
                                    }]),
                                    ..Default::default()
                                }
                            }
                        );
                    }
                }

                model_order_by_params.add_variant(
                    quote!(#field_pascal(Direction)),
                    quote! {
                        Self::#field_pascal(direction) => Field {
                            name: #field_string.into(),
                            value: Some(serde_json::to_value(direction).unwrap()),
                            ..Default::default()
                        }
                    },
                );

                model_pagination_params.add_variant(
                    quote!(#field_pascal(#field_type)),
                    quote! {
                        Self::#field_pascal(value) => Field {
                            name: #field_string.into(),
                            value: Some(serde_json::to_value(value).unwrap()),
                            ..Default::default()
                        }
                    },
                );

                if field.required_on_create() {
                    model_actions.push_required_arg(
                        quote!(#field_snake: #field_snake::Set,),
                        quote!(input_fields.push(SetParam::from(#field_snake).to_field());),
                    );
                }
            }

            if let Some(read_type) = root
                .ast
                .as_ref()
                .unwrap()
                .read_filter(field.field_type.string(), field.is_list)
            {
                for method in &read_type.methods {
                    let typ = match method.typ.string() {
                        "" => field.field_type.tokens(),
                        _ => method.typ.tokens(),
                    };

                    let method_name = format_ident!("{}", method.name.to_case(Case::Snake));
                    let variant_name =
                        format_ident!("{}{}", &field_pascal, method.name.to_case(Case::Pascal));
                    let method_action_string = &method.action;

                    let field_name = if field.prisma {
                        format!("_{}", &field.name)
                    } else {
                        field.name.to_string()
                    };

                    let (typ, field_contents) = if method.is_list {
                        (
                            quote!(Vec<#typ>),
                            quote! {
                                list: true,
                                fields: Some(value.iter().map(|v| Field {
                                    value: Some(serde_json::to_value(v).unwrap()),
                                    ..Default::default()
                                }).collect()),
                            },
                        )
                    } else {
                        (
                            typ,
                            quote! {
                                value: Some(serde_json::to_value(value).unwrap()),
                            },
                        )
                    };

                    model_where_params.add_variant(
                        quote!(#variant_name(#typ)),
                        quote! {
                            Self::#variant_name(value) => Field {
                                name: #field_name.into(),
                                fields: Some(vec![Field {
                                    name: #method_action_string.into(),
                                    #field_contents
                                    ..Default::default()
                                }]),
                                ..Default::default()
                            }
                        },
                    );

                    field_query_module.add_method(quote! {
                        pub fn #method_name(value: #typ) -> WhereParam {
                            WhereParam::#variant_name(value)
                        }
                    });
                }
            }

            model_query_module.add_field_module(field_query_module);
        }

        let Actions {
            required_args,
            required_arg_pushes,
            ..
        } = &model_actions;
        let WithParams { with_fn, .. } = &model_with_params;
        let OrderByParams { order_by_fn, .. } = &model_order_by_params;
        let PaginationParams { pagination_fns, .. } = &model_pagination_params;

        let data_struct = model_data_struct.quote();
        let with_params = model_with_params.quote();
        let set_params = model_set_params.quote();
        let order_by_params = model_order_by_params.quote();
        let pagination_params = model_pagination_params.quote();
        let outputs_fn = model_outputs.quote();
        let query_modules = model_query_module.quote();
        let where_params = model_where_params.quote();

        quote! {
            pub mod #model_name_snake {
                use super::*;
                
                #query_modules
                
                #outputs_fn

                #data_struct

                #with_params

                #set_params

                #order_by_params

                #pagination_params

                #where_params

                pub struct FindMany<'a> {
                    query: Query<'a>,
                    order_by_params: Vec<OrderByParam>,
                    with_params: Vec<WithParam>
                }

                impl<'a> FindMany<'a> {
                    pub async fn exec(self) -> QueryResult<Vec<Data>> {
                        let Self {
                            mut query,
                            order_by_params,
                            with_params
                        } = self;

                        if order_by_params.len() > 0 {
                            query.inputs.push(Input {
                                name: "orderBy".into(),
                                fields: order_by_params
                                    .into_iter()
                                    .map(|f| f.to_field())
                                    .collect(),
                                ..Default::default()
                            });
                        }

                        query.outputs.extend(with_params
                            .into_iter()
                            .map(|f| f.to_output())
                            .collect::<Vec<_>>());

                        query.perform().await
                    }

                    pub fn delete(self) -> DeleteMany<'a> {
                        DeleteMany {
                            query: Query {
                                operation: "mutation".into(),
                                method: "deleteMany".into(),
                                model: #model_name_pascal_string.into(),
                                outputs: vec! [
                                    Output::new("count"),
                                ],
                                ..self.query
                            }
                        }
                    }

                    pub fn update(mut self, params: Vec<SetParam>) -> UpdateMany<'a> {
                        self.query.inputs.push(Input {
                            name: "data".into(),
                            fields: params
                                .into_iter()
                                .map(|param| {
                                    let mut field = param.to_field();

                                    if let Some(value) = field.value {
                                        field.fields = Some(vec![Field {
                                            name: "set".into(),
                                            value: Some(value),
                                            ..Default::default()
                                        }]);
                                        field.value = None;
                                    }

                                    field
                                })
                                .collect(),
                            ..Default::default()
                        });

                        UpdateMany {
                            query: Query {
                                operation: "mutation".into(),
                                method: "updateMany".into(),
                                outputs: vec! [
                                    Output::new("count"),
                                ],
                                ..self.query
                            }
                        }
                    }

                    #order_by_fn

                    #with_fn

                    #pagination_fns
                }

                pub struct FindFirst<'a> {
                    query: Query<'a>,
                    order_by_params: Vec<OrderByParam>,
                    with_params: Vec<WithParam>
                }

                impl<'a> FindFirst<'a> {
                    pub async fn exec(self) -> QueryResult<Option<Data>> {
                        let Self {
                            mut query,
                            order_by_params,
                            with_params
                        } = self;

                        if order_by_params.len() > 0 {
                            query.inputs.push(Input {
                                name: "orderBy".into(),
                                fields: order_by_params
                                    .into_iter()
                                    .map(|f| f.to_field())
                                    .collect(),
                                ..Default::default()
                            });
                        }

                        query.outputs.extend(with_params
                            .into_iter()
                            .map(|f| f.to_output())
                            .collect::<Vec<_>>());

                        query.perform().await
                    }

                    #with_fn

                    #order_by_fn

                    #pagination_fns
                }

                pub struct FindUnique<'a> {
                    query: Query<'a>,
                    with_params: Vec<WithParam>
                }

                impl<'a> FindUnique<'a> {
                    pub async fn exec(self) -> QueryResult<Option<Data>> {
                        let Self {
                            mut query,
                            with_params
                        } = self;

                        query.outputs.extend(with_params
                            .into_iter()
                            .map(|f| f.to_output())
                            .collect::<Vec<_>>());

                        query.perform().await
                    }

                    pub fn delete(self) -> Delete<'a> {
                        Delete {
                            query: Query {
                                operation: "mutation".into(),
                                method: "deleteOne".into(),
                                model: #model_name_pascal_string.into(),
                                ..self.query
                            },
                            with_params: vec![]
                        }
                    }

                    pub fn update(mut self, params: Vec<SetParam>) -> UpdateUnique<'a> {
                        self.query.inputs.push(Input {
                            name: "data".into(),
                            fields: params
                                .into_iter()
                                .map(|param| {
                                    let mut field = param.to_field();

                                    if let Some(value) = field.value {
                                        field.fields = Some(vec![Field {
                                            name: "set".into(),
                                            value: Some(value),
                                            ..Default::default()
                                        }]);
                                        field.value = None;
                                    }

                                    field
                                })
                                .collect(),
                            ..Default::default()
                        });

                        UpdateUnique {
                            query: Query {
                                operation: "mutation".into(),
                                method: "updateOne".into(),
                                ..self.query
                            },
                            with_params: vec![]
                        }
                    }

                    #with_fn
                }

                pub struct Create<'a> {
                    query: Query<'a>,
                    with_params: Vec<WithParam>
                }

                impl<'a> Create<'a> {
                    pub async fn exec(self) -> QueryResult<Data> {
                        let Self {
                            mut query,
                            with_params
                        } = self;

                        query.outputs.extend(with_params
                            .into_iter()
                            .map(|f| f.to_output())
                            .collect::<Vec<_>>());

                        query.perform().await
                    }

                    #with_fn
                }

                pub struct UpdateUnique<'a> {
                    query: Query<'a>,
                    with_params: Vec<WithParam>
                }

                impl<'a> UpdateUnique<'a> {
                    pub async fn exec(self) -> QueryResult<Option<Data>> {
                        let Self {
                            mut query,
                            with_params,
                        } = self;
                        
                        query.outputs.extend(
                            with_params
                                .into_iter()
                                .map(|f| f.to_output())
                                .collect::<Vec<_>>(),
                        );
                        
                        match query.perform().await {
                            Err(QueryError::Execute(CoreError::InterpreterError(InterpreterError::InterpretationError(
                                msg,
                                Some(interpreter_error),
                            )))) => match *interpreter_error {
                                InterpreterError::QueryGraphBuilderError(
                                    QueryGraphBuilderError::RecordNotFound(_),
                                ) => Ok(None),
                                res => Err(QueryError::Execute(CoreError::InterpreterError(InterpreterError::InterpretationError(
                                    msg,
                                    Some(Box::new(res)),
                                )))),
                            },
                            res => res,
                        }
                    }

                    #with_fn
                }

                pub struct UpdateMany<'a> {
                    query: Query<'a>
                }

                impl<'a> UpdateMany<'a> {
                    pub async fn exec(self) -> QueryResult<usize> {
                        self.query.perform().await.map(|res: CountResult| res.count)
                    }
                }

                pub struct Upsert<'a> {
                    query: Query<'a>,
                }

                impl<'a> Upsert<'a> {
                    pub async fn exec(self) -> QueryResult<Data> {
                        self.query.perform().await
                    }

                    pub fn create(
                        mut self,
                        #(#required_args)*
                        params: Vec<SetParam>
                    ) -> Self {
                        let mut input_fields = params.into_iter().map(|p| p.to_field()).collect::<Vec<_>>();

                        #(#required_arg_pushes)*

                        self.query.inputs.push(Input {
                            name: "create".into(),
                            fields: input_fields,
                            ..Default::default()
                        });

                        self
                    }

                    pub fn update(mut self, params: Vec<SetParam>) -> Self {
                        self.query.inputs.push(Input {
                            name: "update".into(),
                            fields: params
                                .into_iter()
                                .map(|param| {
                                    let mut field = param.to_field();
                                    if let Some(value) = field.value {
                                        field.fields = Some(vec![Field {
                                            name: "set".into(),
                                            value: Some(value),
                                            ..Default::default()
                                        }]);
                                        field.value = None;
                                    }
                                    field
                                })
                                .collect(),
                            ..Default::default()
                        });
                        self
                    }
                }

                pub struct Delete<'a> {
                    query: Query<'a>,
                    with_params: Vec<WithParam>
                }

                impl<'a> Delete<'a> {
                    pub async fn exec(self) -> QueryResult<Option<Data>> {
                        let Self {
                            mut query,
                            with_params
                        } = self;

                        query.outputs.extend(with_params
                            .into_iter()
                            .map(|f| f.to_output())
                            .collect::<Vec<_>>());

                        match query.perform().await {
                            Err(QueryError::Execute(CoreError::InterpreterError(InterpreterError::InterpretationError(
                                msg,
                                Some(interpreter_error),
                            )))) => match *interpreter_error {
                                InterpreterError::QueryGraphBuilderError(
                                    QueryGraphBuilderError::RecordNotFound(_),
                                ) => Ok(None),
                                res => Err(QueryError::Execute(CoreError::InterpreterError(InterpreterError::InterpretationError(
                                    msg,
                                    Some(Box::new(res)),
                                )))),
                            },
                            res => res,
                        }
                    }
                    
                    #with_fn
                }
                
                pub struct DeleteMany<'a> {
                    query: Query<'a>
                }

                impl<'a> DeleteMany<'a> {
                    pub async fn exec(self) -> QueryResult<usize> {
                        self.query.perform().await.map(|res: CountResult| res.count)
                    }
                }

                pub struct Actions<'a> {
                    pub client: &'a PrismaClient,
                }

                impl<'a> Actions<'a> {
                    pub fn create(&self, #(#required_args)* params: Vec<SetParam>) -> Create {
                        let mut input_fields = params.into_iter().map(|p| p.to_field()).collect::<Vec<_>>();

                        #(#required_arg_pushes)*

                        let query = Query {
                            ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                            name: String::new(),
                            operation: "mutation".into(),
                            method: "createOne".into(),
                            model: #model_name_pascal_string.into(),
                            outputs: _outputs(),
                            inputs: vec![Input {
                                name: "data".into(),
                                fields: input_fields,
                                ..Default::default()
                            }]
                        };

                        Create {
                            query,
                            with_params: vec![]
                        }
                    }

                    pub fn find_unique(&self, param: UniqueWhereParam) -> FindUnique {
                        let param: WhereParam = param.into();
                        let fields = transform_equals(vec![param.to_field()]);

                        let query = Query {
                            ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                            name: String::new(),
                            operation: "query".into(),
                            method: "findUnique".into(),
                            model: #model_name_pascal_string.into(),
                            outputs: _outputs(),
                            inputs: vec![Input {
                                name: "where".into(),
                                fields,
                                ..Default::default()
                            }]
                        };

                        FindUnique {
                            query,
                            with_params: vec![]
                        }
                    }

                    pub fn find_first(&self, params: Vec<WhereParam>) -> FindFirst {
                        let where_fields: Vec<Field> = params.into_iter().map(|param|
                            param.to_field()
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
                            model: #model_name_pascal_string.into(),
                            outputs: _outputs(),
                            inputs
                        };

                        FindFirst {
                            query,
                            order_by_params: vec![],
                            with_params: vec![]
                        }
                    }

                    pub fn find_many(&self, params: Vec<WhereParam>) -> FindMany {
                        let where_fields: Vec<Field> = params.into_iter().map(|param|
                            param.to_field()
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
                            model: #model_name_pascal_string.into(),
                            outputs: _outputs(),
                            inputs
                        };

                        FindMany {
                            query,
                            order_by_params: vec![],
                            with_params: vec![]
                        }
                    }

                    pub fn upsert(&self, param: UniqueWhereParam) -> Upsert {
                        let param: WhereParam = param.into();
                        let fields = transform_equals(vec![param.to_field()]);

                        let query = Query {
                            ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                            name: String::new(),
                            operation: "mutation".into(),
                            method: "upsertOne".into(),
                            model: #model_name_pascal_string.into(),
                            outputs: _outputs(),
                            inputs: vec![Input {
                                name: "where".into(),
                                fields,
                                ..Default::default()
                            }]
                        };

                        Upsert { query }
                    }
                }
            }
        }
    }).collect()
}
