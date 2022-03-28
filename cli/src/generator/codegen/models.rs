use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};
use syn::Ident;

use crate::generator::{
    ast::{dmmf::Document, Model},
    Root,
};

struct Outputs {
    pub fn_name: Ident,
    outputs: Vec<TokenStream>,
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
                })
                .collect(),
        }
    }

    pub fn quote(&self) -> TokenStream {
        let Self { fn_name, outputs } = self;

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

struct WithParams {
    pub enum_name: Ident,
    pub with_fn: TokenStream,
    variants: Vec<TokenStream>,
    match_arms: Vec<TokenStream>,
}

impl WithParams {
    pub fn new(model: &Model) -> Self {
        let model_name_pascal_string = model.name.to_case(Case::Pascal);
        let enum_name = format_ident!("{}WithParam", &model_name_pascal_string);

        Self {
            with_fn: quote! {
                pub fn with(mut self, param: #enum_name) -> Self {
                    self.with_params.push(param);
                    self
                }
            },
            enum_name,
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
            enum_name: name,
            ..
        } = self;

        quote! {
            pub enum #name {
                #(#variants),*
            }

            impl #name {
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
    pub enum_name: Ident,
    model_name: Ident,
    variants: Vec<TokenStream>,
    match_arms: Vec<TokenStream>,
}

impl SetParams {
    pub fn new(model_name: &str) -> Self {
        Self {
            enum_name: format_ident!("{}SetParam", model_name.to_case(Case::Pascal)),
            model_name: format_ident!("{}", model_name.to_case(Case::Pascal)),
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
            enum_name,
            ..
        } = self;

        quote! {
            pub enum #enum_name {
                #(#variants),*
            }

            impl #enum_name {
                pub fn to_field(self) -> Field {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        }
    }

    pub fn field_link_struct(&self, field_name: &str) -> Ident {
        format_ident!(
            "{}Link{}",
            &self.model_name,
            field_name.to_case(Case::Pascal)
        )
    }

    pub fn field_link_variant(field_name: &str) -> Ident {
        format_ident!("Link{}", field_name.to_case(Case::Pascal))
    }

    pub fn field_unlink_variant(field_name: &str) -> Ident {
        format_ident!("UnLink{}", field_name.to_case(Case::Pascal))
    }

    pub fn field_set_struct(&self, field_name: &str) -> Ident {
        format_ident!(
            "{}Set{}",
            &self.model_name,
            field_name.to_case(Case::Pascal)
        )
    }

    pub fn field_set_variant(field_name: &str) -> Ident {
        format_ident!("Set{}", field_name.to_case(Case::Pascal))
    }
}

struct OrderByParams {
    pub enum_name: Ident,
    pub order_by_fn: TokenStream,
    variants: Vec<TokenStream>,
    match_arms: Vec<TokenStream>,
}

impl OrderByParams {
    pub fn new(model: &Model) -> Self {
        let enum_name = format_ident!("{}OrderByParam", model.name.to_case(Case::Pascal));

        Self {
            enum_name: enum_name.clone(),
            order_by_fn: quote! {
                pub fn order_by(mut self, param: #enum_name) -> Self {
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
            enum_name,
            ..
        } = self;

        quote! {
            pub enum #enum_name {
                #(#variants),*
            }

            impl #enum_name {
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
    pub cursor_enum_name: Ident,
    pub pagination_fns: TokenStream,
    cursor_variants: Vec<TokenStream>,
    cursor_match_arms: Vec<TokenStream>,
}

impl PaginationParams {
    pub fn new(model: &Model) -> Self {
        let model_name_pascal = format_ident!("{}", model.name.to_case(Case::Pascal));
        let cursor_enum_name = format_ident!("{}Cursor", model_name_pascal);

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

            pub fn cursor(mut self, cursor: #cursor_enum_name) -> Self {
                self.query.inputs.push(Input {
                    name: "cursor".into(),
                    fields: vec![cursor.to_field()],
                    ..Default::default()
                });
                self
            }
        };

        Self {
            cursor_enum_name,
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
            cursor_enum_name,
            cursor_variants,
            cursor_match_arms,
            ..
        } = self;

        quote! {
            pub enum #cursor_enum_name {
                #(#cursor_variants),*
            }

            impl #cursor_enum_name {
                fn to_field(self) -> Field {
                    match self {
                        #(#cursor_match_arms),*
                    }
                }
            }
        }
    }
}

struct QueryStructs {
    pub name: Ident,
    accessors: Vec<TokenStream>,
    field_structs: Vec<TokenStream>,
}

impl QueryStructs {
    pub fn new(
        model: &Model,
    ) -> Self {
        Self {
            name: format_ident!("{}", model.name.to_case(Case::Pascal)),
            accessors: vec![],
            field_structs: vec![],
        }
    }
    
    pub fn add_field(&mut self, accessor: TokenStream, field_struct: FieldQueryStructs) {
        self.accessors.push(accessor);
        self.field_structs.push(field_struct.quote());
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            name,
            accessors: methods,
            field_structs,
        } = self;

        quote! {
            pub struct #name;

            impl #name {
                #(#methods)*
            }

            #(#field_structs)*
        }
    }
}

struct FieldQueryStructs {
    pub struct_name: Ident,
    queries: Vec<TokenStream>,
    query_structs: Vec<TokenStream>
}

impl FieldQueryStructs {
    pub fn new(model: &str, field: &str) -> Self {
        Self {
            struct_name: format_ident!(
                "{}{}Field",
                model.to_case(Case::Pascal),
                field.to_case(Case::Pascal)
            ),
            queries: vec![],
            query_structs: vec![],
        }
    }

    pub fn push_query(&mut self, query: TokenStream) {
        self.queries.push(query);
    }
    
    pub fn push_query_struct(&mut self, query_struct: TokenStream) {
        self.query_structs.push(query_struct);
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            struct_name,
            queries,
            query_structs,
        } = self;

        quote! {
            pub struct #struct_name {}

            impl #struct_name {
                #(#queries)*
            }
            
            #(#query_structs)*
        }
    }
}

struct WhereParams {
    pub enum_name: Ident,
    pub unique_enum_name: Ident,
    pub variants: Vec<TokenStream>,
    pub to_field: Vec<TokenStream>,
    pub unique_variants: Vec<TokenStream>,
    pub from_unique_match_arms: Vec<TokenStream>,
}

impl WhereParams {
    pub fn new(name: &str) -> Self {
        Self {
            enum_name: Self::get_enum_ident(name),
            unique_enum_name: Self::get_unique_enum_ident(name),
            variants: vec![],
            to_field: vec![],
            unique_variants: vec![],
            from_unique_match_arms: vec![],
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
    ) {
        self.add_variant(variant.clone(), match_arm);
        self.unique_variants.push(variant);
        self.from_unique_match_arms.push(from_unique_match_arm);
    }

    pub fn get_enum_ident(name: &str) -> Ident {
        format_ident!("{}WhereParam", name.to_case(Case::Pascal))
    }

    pub fn get_unique_enum_ident(name: &str) -> Ident {
        format_ident!("{}WhereUniqueParam", name.to_case(Case::Pascal))
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            enum_name,
            unique_enum_name,
            variants,
            to_field,
            unique_variants,
            from_unique_match_arms,
        } = self;

        quote! {
            pub enum #enum_name {
                #(#variants),*
            }

            impl #enum_name {
                pub fn to_field(&self) -> Field {
                    match &self {
                        #(#to_field),*
                    }
                }
            }

            pub enum #unique_enum_name {
                #(#unique_variants),*
            }

            impl From<#unique_enum_name> for #enum_name {
                fn from(value: #unique_enum_name) -> Self {
                    match value {
                        #(#from_unique_match_arms),*
                    }
                }
            }

            impl From<Operator<Self>> for #enum_name {
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
    pub name: Ident,
    fields: Vec<TokenStream>,
    relation_accessors: Vec<TokenStream>,
}

impl DataStruct {
    pub fn new(model: &Model) -> Self {
        Self {
            name: Self::get_struct_name(&model.name),
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
            name,
            fields,
            relation_accessors,
        } = self;

        quote! {
            #[derive(Debug, Clone, Serialize, Deserialize)]
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

struct Actions {
    pub struct_name: Ident,
    pub required_args: Vec<TokenStream>,
    pub required_arg_pushes: Vec<TokenStream>,
}

impl Actions {
    pub fn new(model_name: &str) -> Self {
        Self {
            struct_name: Self::get_struct_name(model_name),
            required_args: vec![],
            required_arg_pushes: vec![],
        }
    }

    pub fn push_required_arg(&mut self, arg: TokenStream, arg_push: TokenStream) {
        self.required_args.push(arg);
        self.required_arg_pushes.push(arg_push);
    }

    pub fn get_struct_name(model_name: &str) -> Ident {
        format_ident!("{}Actions", model_name.to_case(Case::Pascal))
    }
}

pub fn generate(root: &Root) -> Vec<TokenStream> {
    root.ast.as_ref().unwrap().models.iter().map(|model| {
        let outputs = Outputs::new(&model);
        let mut data_struct = DataStruct::new(&model);
        let mut order_by_params = OrderByParams::new(&model);
        let mut pagination_params = PaginationParams::new(&model);
        let mut with_params = WithParams::new(&model);
        let mut query_structs = QueryStructs::new(&model);
        let mut set_params = SetParams::new(&model.name);
        let mut where_params = WhereParams::new(&model.name);
        let mut actions = Actions::new(&model.name);

        let model_name_pascal_string = model.name.to_case(Case::Pascal);
        let model_name_pascal = format_ident!("{}", &model_name_pascal_string);

        let model_create_one = format_ident!("{}CreateOne", model_name_pascal_string);
        let model_find_first = format_ident!("{}FindFirst", model_name_pascal_string);
        let model_find_unique = format_ident!("{}FindUnique", model_name_pascal_string);
        let model_find_many = format_ident!("{}FindMany", model_name_pascal_string);
        let model_update_unique = format_ident!("{}UpdateUnique", model_name_pascal_string);
        let model_update_many = format_ident!("{}UpdateMany", model_name_pascal_string);
        let model_upsert_one = format_ident!("{}UpsertOne", model_name_pascal_string);
        let model_delete = format_ident!("{}Delete", model_name_pascal_string);

        let set_params_enum = &set_params.enum_name.clone();
        let where_params_enum = &where_params.enum_name.clone();
        let unique_where_params_enum = &where_params.unique_enum_name.clone();
        let with_params_enum = &with_params.enum_name.clone();
        let order_by_params_enum = &order_by_params.enum_name.clone();
        let pagination_params_enum = &pagination_params.cursor_enum_name.clone();
        let outputs_fn = &outputs.fn_name.clone();

        for op in Document::operators() {
            let variant_name = format_ident!("{}", op.name.to_case(Case::Pascal));
            let op_string = op.name;

            where_params.add_variant(
                quote!(#variant_name(Vec<#where_params_enum>)),
                quote! {
                    Self::#variant_name(value) => Field {
                        name: #op_string.into(),
                        list: true,
                        wrap_list: true,
                        fields: Some(value.into_iter().map(|f| f.to_field()).collect()),
                        ..Default::default()
                    }
                },
            );
        }

        // TODO: Compound indexes
        // for unique in &model.indexes {
        //     field_structs.push()
        //     methods.push(quote! {
        //         pub fn #unique(mut self, value: #unique_where_params_enum) -> Self {
        //                 self.where_params.push(value.into());
        //                 self
        //             }
        //         });
        //     })
        // }

        for field in &model.fields {
            let mut field_query_struct =
                FieldQueryStructs::new(&model_name_pascal_string, &field.name);
            let field_query_struct_name = &field_query_struct.struct_name.clone();

            let field_string = &field.name;
            let field_snake = format_ident!("{}", field.name.to_case(Case::Snake));
            let field_pascal = format_ident!("{}", field.name.to_case(Case::Pascal));
            let field_type_string = field.field_type.value();
            let field_type = field.field_type.tokens();

            let field_set_struct = set_params.field_set_struct(&field.name);
            let field_link_struct = set_params.field_link_struct(&field.name);

            if field.kind.is_relation() {
                let link_variant = SetParams::field_link_variant(&field.name);
                let unlink_variant = SetParams::field_unlink_variant(&field.name);

                let relation_outputs_fn = Outputs::get_fn_name(&field_type_string);
                let relation_data_struct = DataStruct::get_struct_name(&field_type_string);
                let relation_where_enum = WhereParams::get_enum_ident(&field.field_type.value());
                let relation_where_unique_enum =
                    WhereParams::get_unique_enum_ident(&field.field_type.value());

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

                    where_params.add_variant(
                        quote!(#variant_name(Vec<#relation_where_enum>)),
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

                    field_query_struct.push_query(quote! {
                        pub fn #method_name_snake(&self, value: Vec<#relation_where_enum>) -> #where_params_enum {
                            #where_params_enum::#variant_name(value)
                        }
                    });
                }

                // Relation actions eg. Fetch, Link, Unlink
                if field.is_list {
                    field_query_struct.push_query(quote! {
                        pub fn fetch(&self, params: Vec<#relation_where_enum>) -> #with_params_enum {
                            #with_params_enum::#field_pascal(params)
                        }

                        pub fn link<T: From<#field_link_struct>>(&self, params: Vec<#relation_where_unique_enum>) -> T {
                            #field_link_struct(params).into()
                        }

                        pub fn unlink(&self, params: Vec<#relation_where_unique_enum>) -> #set_params_enum {
                            #set_params_enum::#unlink_variant(params)
                        }
                    });

                    field_query_struct.push_query_struct(quote! {
                        pub struct #field_link_struct(Vec<#relation_where_unique_enum>);

                        impl From<#field_link_struct> for #set_params_enum {
                            fn from(value: #field_link_struct) -> Self {
                                Self::#link_variant(value.0)
                            }
                        }
                    });

                    set_params.add_variant(
                        quote!(#link_variant(Vec<#relation_where_unique_enum>)),
                        quote! {
                            Self::#link_variant(where_params) => Field {
                                name: #field_string.into(),
                                fields: Some(vec![
                                    Field {
                                        name: "connect".into(),
                                        fields: Some(transform_equals(
                                            where_params
                                                .into_iter()
                                                .map(|param| Into::<#relation_where_enum>::into(param)
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

                    set_params.add_variant(
                        quote!(#unlink_variant(Vec<#relation_where_unique_enum>)),
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
                                                .map(|param| Into::<#relation_where_enum>::into(param)
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

                    with_params.add_variant(
                        quote!(#field_pascal(Vec<#relation_where_enum>)),
                        quote! {
                            Self::#field_pascal(where_params) => Output {
                                name: #field_string.into(),
                                outputs: #relation_outputs_fn(),
                                inputs: if where_params.len() > 0 {
                                    vec![Input {
                                        name: "where".into(),
                                        fields: where_params
                                            .into_iter()
                                            .map(|param| Into::<#relation_where_enum>::into(param)
                                                .to_field())
                                            .collect(),
                                        ..Default::default()
                                    }]
                                } else { vec![] },
                                ..Default::default()
                            }
                        },
                    );

                    data_struct.add_relation(
                        quote! {
                           #[serde(rename = #field_string)]
                           #field_snake: Option<Vec<#relation_data_struct>>
                        },
                        quote! {
                            pub fn #field_snake(&self) -> Result<&Vec<#relation_data_struct>, String> {
                                match self.#field_snake.as_ref() {
                                    Some(v) => Ok(v),
                                    None => Err(#relation_data_access_error.to_string()),
                                }
                            }
                        },
                    );
                } else {
                    field_query_struct.push_query(quote! {
                        pub fn fetch(&self) -> #with_params_enum {
                            #with_params_enum::#field_pascal
                        }

                        pub fn link<T: From<#field_link_struct>>(&self, value: #relation_where_unique_enum) -> T {
                            #field_link_struct(value).into()
                        }
                    });
                    
                    field_query_struct.push_query_struct(quote! {
                        pub struct #field_link_struct(#relation_where_unique_enum);

                        impl From<#field_link_struct> for #set_params_enum {
                            fn from(value: #field_link_struct) -> Self {
                                Self::#link_variant(value.0)
                            }
                        }
                    });

                    set_params.add_variant(
                        quote!(#link_variant(#relation_where_unique_enum)),
                        quote! {
                            Self::#link_variant(where_param) => Field {
                                name: #field_string.into(),
                                fields: Some(vec![
                                    Field {
                                        name: "connect".into(),
                                        fields: Some(transform_equals(vec![
                                            Into::<#relation_where_enum>::into(where_param).to_field()
                                        ])),
                                        ..Default::default()
                                    }
                                ]),
                                ..Default::default()
                            }
                        }
                    );

                    if !field.is_required {
                        field_query_struct.push_query(quote! {
                            pub fn unlink(&self) -> #set_params_enum {
                                #set_params_enum::#unlink_variant
                            }
                        });

                        set_params.add_variant(
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
                    
                    with_params.add_variant(
                        quote!(#field_pascal),
                        quote! {
                            Self::#field_pascal => Output {
                                name: #field_string.into(),
                                outputs: #relation_outputs_fn(),
                                ..Default::default()
                            }
                        },
                    );

                    data_struct.add_relation(
                        quote! {
                            #[serde(rename = #field_string)]
                            #field_snake: Option<Box<#relation_data_struct>>
                        },
                        quote! {
                            pub fn #field_snake(&self) -> Result<&#relation_data_struct, String> {
                                match self.#field_snake.as_ref() {
                                    Some(v) => Ok(v),
                                    None => Err(#relation_data_access_error.to_string()),
                                }
                            }
                        },
                    );
                };

                if field.required_on_create() {
                    actions.push_required_arg(
                        quote!(#field_snake: #field_link_struct,),
                        quote!(input_fields.push(#set_params_enum::from(#field_snake).to_field());),
                    );
                }
            }
            // Scalar actions
            else {
                let set_variant = SetParams::field_set_variant(&field.name);

                if !field.prisma {
                    let (set_variant_type, field_content) = if field.is_list {
                        (
                            quote!(Vec<#field_type>),
                            quote!(fields: Some(value.iter().map(|f| f.to_field()).collect())),
                        )
                    } else {
                        (
                            field_type.clone(),
                            quote!(value: Some(serde_json::to_value(value).unwrap())),
                        )
                    };

                    field_query_struct.push_query(quote! {
                        pub fn set<T: From<#field_set_struct>>(&self, value: #set_variant_type) -> T {
                            #field_set_struct(value).into()
                        }
                    });

                    field_query_struct.push_query_struct(quote! {
                        pub struct #field_set_struct(#set_variant_type);
                        impl From<#field_set_struct> for #set_params_enum {
                            fn from(value: #field_set_struct) -> Self {
                                Self::#set_variant(value.0)
                            }
                        }
                    });

                    set_params.add_variant(
                        quote!(#set_variant(#field_type)),
                        quote! {
                            Self::#set_variant(value) => Field {
                                name: #field_string.into(),
                                value: Some(serde_json::to_value(value).unwrap()),
                                ..Default::default()
                            }
                        },
                    );

                    let equals_variant_name = format_ident!("{}Equals", &field_pascal);
                    let equals_variant = quote!(#equals_variant_name(#set_variant_type));

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

                    match field.is_unique || field.is_id {
                        true => {
                            where_params.add_unique_variant(
                                equals_variant,
                                match_arm,
                                quote! {
                                    #unique_where_params_enum::#equals_variant_name(value) => Self::#equals_variant_name(value)
                                }
                            );
                            field_query_struct.push_query(quote! {
                                pub fn equals<T: From<#unique_where_params_enum>>(&self, value: #set_variant_type) -> T {
                                    #unique_where_params_enum::#equals_variant_name(value).into()
                                }
                            })
                        }
                        false => {
                            where_params.add_variant(equals_variant, match_arm);
                            field_query_struct.push_query(quote! {
                                pub fn equals(&self, value: #set_variant_type) -> #where_params_enum {
                                    #where_params_enum::#equals_variant_name(value).into()
                                }
                            });
                        }
                    };

                    // TODO: Optional Equals

                    // Pagination
                    field_query_struct.push_query(quote! {
                        pub fn order(&self, direction: Direction) -> #order_by_params_enum {
                            #order_by_params_enum::#field_pascal(direction)
                        }

                        pub fn cursor(&self, cursor: #field_type) -> #pagination_params_enum {
                            #pagination_params_enum::#field_pascal(cursor)
                        }
                    });

                    data_struct.add_field(match (field.is_list, field.is_required) {
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

                // TODO: Optionals
                // if !field.is_required && !field.is_list && !field.prisma {
                //     field_struct_methods.push(quote!{
                //         pub fn set_optional(&self, value: Option<#field_set_arg>) -> #model_set_param {
                //             #model_set_param::#field_name_pascal()
                //         }
                //     })
                // }

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

                        let method_name = format_ident!("{}", method.name.to_case(Case::Snake));

                        if method.is_list {
                            field_query_struct.push_query(quote! {
                                pub fn #method_name(&self, value: Vec<#typ>) -> #set_params_enum {
                                    #set_params_enum::#field_pascal(value)
                                }
                            });
                        } else {
                            field_query_struct.push_query(quote! {
                                pub fn #method_name(&self, value: #typ) -> #set_params_enum {
                                    #set_params_enum::#field_pascal(value)
                                }
                            });
                        }

                        // TODO: IfPresent
                    }
                }

                order_by_params.add_variant(
                    quote!(#field_pascal(Direction)),
                    quote! {
                        Self::#field_pascal(direction) => Field {
                            name: #field_string.into(),
                            value: Some(serde_json::to_value(direction).unwrap()),
                            ..Default::default()
                        }
                    },
                );

                pagination_params.add_variant(
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
                    actions.push_required_arg(
                        quote!(#field_snake: #field_set_struct,),
                        quote!(input_fields.push(#set_params_enum::from(#field_snake).to_field());),
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
                    // TODO: Deprecated warning
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

                    where_params.add_variant(
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

                    field_query_struct.push_query(quote! {
                        pub fn #method_name(&self, value: #typ) -> #where_params_enum {
                            #where_params_enum::#variant_name(value)
                        }
                    });

                    // TODO: IfPresent
                }
            }

            query_structs.add_field(
                quote! {
                    pub fn #field_snake() -> #field_query_struct_name {
                        #field_query_struct_name {}
                    }
                }, 
                field_query_struct
            );
        }

        let Actions {
            required_args,
            required_arg_pushes,
            struct_name: actions_struct,
        } = &actions;
        let DataStruct { name: data_struct_name, .. } = &data_struct;
        let WithParams { with_fn, .. } = &with_params;
        let OrderByParams { order_by_fn, .. } = &order_by_params;
        let PaginationParams { pagination_fns, .. } = &pagination_params;
        let Outputs { fn_name: outputs_fn_name, .. } = &outputs;

        let data_struct = data_struct.quote();
        let with_params = with_params.quote();
        let set_params = set_params.quote();
        let order_by_params = order_by_params.quote();
        let pagination_params = pagination_params.quote();
        let outputs_fn = outputs.quote();
        let query_structs = query_structs.quote();
        let where_params = where_params.quote();

        quote! {
            #outputs_fn

            #data_struct

            #query_structs

            #with_params

            #set_params

            #order_by_params

            #pagination_params

            #where_params
            
            pub struct #model_find_many<'a> {
                query: Query<'a>,
                order_by_params: Vec<#order_by_params_enum>,
                with_params: Vec<#with_params_enum>
            }

            impl<'a> #model_find_many<'a> {
                pub async fn exec(self) -> QueryResult<Vec<#data_struct_name>> {
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

                pub fn delete(self) -> #model_delete<'a> {
                    #model_delete {
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

                pub fn update(mut self, params: Vec<#set_params_enum>) -> #model_update_many<'a> {
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

                    #model_update_many {
                        query: Query {
                            operation: "mutation".into(),
                            method: "updateMany".into(),
                            ..self.query
                        },
                        with_params: vec![]
                    }
                }

                #order_by_fn

                #with_fn

                #pagination_fns
            }

            pub struct #model_find_first<'a> {
                query: Query<'a>,
                order_by_params: Vec<#order_by_params_enum>,
                with_params: Vec<#with_params_enum>
            }

            impl<'a> #model_find_first<'a> {
                pub async fn exec(self) -> QueryResult<#data_struct_name> {
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

            pub struct #model_find_unique<'a> {
                query: Query<'a>,
                with_params: Vec<#with_params_enum>
            }

            impl<'a> #model_find_unique<'a> {
                pub async fn exec(self) -> QueryResult<#data_struct_name> {
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

                pub fn delete(self) -> #model_delete<'a> {
                    #model_delete {
                        query: Query {
                            operation: "mutation".into(),
                            method: "deleteOne".into(),
                            model: #model_name_pascal_string.into(),
                            ..self.query
                        }
                    }
                }

                pub fn update(mut self, params: Vec<#set_params_enum>) -> #model_update_unique<'a> {
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

                    #model_update_unique {
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

            pub struct #model_create_one<'a> {
                query: Query<'a>,
                with_params: Vec<#with_params_enum>
            }

            impl<'a> #model_create_one<'a> {
                pub async fn exec(self) -> QueryResult<#data_struct_name> {
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

            pub struct #model_update_unique<'a> {
                query: Query<'a>,
                with_params: Vec<#with_params_enum>
            }

            impl<'a> #model_update_unique<'a> {
                pub async fn exec(self) -> QueryResult<#data_struct_name> {
                    self.query.perform().await
                }

                #with_fn
            }

            pub struct #model_update_many<'a> {
                query: Query<'a>,
                with_params: Vec<#with_params_enum>
            }

            impl<'a> #model_update_many<'a> {
                pub async fn exec(self) -> QueryResult<Vec<#data_struct_name>> {
                    self.query.perform().await
                }

                #with_fn
            }

            pub struct #model_upsert_one<'a> {
                query: Query<'a>,
            }

            impl<'a> #model_upsert_one<'a> {
                pub async fn exec(self) -> QueryResult<#data_struct_name> {
                    self.query.perform().await
                }

                pub fn create(
                    mut self,
                    #(#required_args)*
                    params: Vec<#set_params_enum>
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

                pub fn update(mut self, params: Vec<UserSetParam>) -> Self {
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

            pub struct #model_delete<'a> {
                query: Query<'a>
            }

            impl<'a> #model_delete<'a> {
                pub async fn exec(self) -> QueryResult<isize> {
                    self.query.perform::<DeleteResult>().await.map(|r| r.count)
                }
            }

            pub struct #actions_struct<'a> {
                client: &'a PrismaClient,
            }

            impl<'a> #actions_struct<'a> {
                // TODO: Dedicated unique field
                pub fn find_unique(&self, param: #unique_where_params_enum) -> #model_find_unique {
                    let param: #where_params_enum = param.into();
                    let fields = transform_equals(vec![param.to_field()]);

                    let query = Query {
                        ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                        name: String::new(),
                        operation: "query".into(),
                        method: "findUnique".into(),
                        model: #model_name_pascal_string.into(),
                        outputs: #outputs_fn_name(),
                        inputs: vec![Input {
                            name: "where".into(),
                            fields,
                            ..Default::default()
                        }]
                    };

                    #model_find_unique {
                        query,
                        with_params: vec![]
                    }
                }

                pub fn find_first(&self, params: Vec<#where_params_enum>) -> #model_find_first {
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
                        outputs: #outputs_fn_name(),
                        inputs
                    };

                    #model_find_first {
                        query,
                        order_by_params: vec![],
                        with_params: vec![]
                    }
                }

                pub fn find_many(&self, params: Vec<#where_params_enum>) -> #model_find_many {
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
                        outputs: #outputs_fn_name(),
                        inputs
                    };

                    #model_find_many {
                        query,
                        order_by_params: vec![],
                        with_params: vec![]
                    }
                }

                pub fn create_one(&self, #(#required_args)* params: Vec<#set_params_enum>) -> #model_create_one {
                    let mut input_fields = params.into_iter().map(|p| p.to_field()).collect::<Vec<_>>();

                    #(#required_arg_pushes)*

                    let query = Query {
                        ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                        name: String::new(),
                        operation: "mutation".into(),
                        method: "createOne".into(),
                        model: #model_name_pascal_string.into(),
                        outputs: #outputs_fn_name(),
                        inputs: vec![Input {
                            name: "data".into(),
                            fields: input_fields,
                            ..Default::default()
                        }]
                    };

                    #model_create_one {
                        query,
                        with_params: vec![]
                    }
                }

                pub fn upsert_one(&self, param: #unique_where_params_enum) -> #model_upsert_one {
                    let param: #where_params_enum = param.into();
                    let fields = transform_equals(vec![param.to_field()]);

                    let query = Query {
                        ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                        name: String::new(),
                        operation: "mutation".into(),
                        method: "upsertOne".into(),
                        model: #model_name_pascal_string.into(),
                        outputs: #outputs_fn_name(),
                        inputs: vec![Input {
                            name: "where".into(),
                            fields,
                            ..Default::default()
                        }]
                    };

                    #model_upsert_one { query }
                }
            }
        }
    }).collect()
}
