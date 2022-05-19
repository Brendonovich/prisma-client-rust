use convert_case::{Case, Casing};
use datamodel::dml::{Model, Field, FieldType, ScalarType, FieldArity};
use quote::{__private::TokenStream, format_ident, quote};
use syn::Ident;

use crate::generator::{
    GeneratorArgs,
};

pub struct Operator {
    pub name: &'static str,
    pub action: &'static str
}

static OPERATORS: &'static [Operator; 3] = &[
    Operator { name: "Not", action: "NOT" },
    Operator { name: "Or", action: "OR" },
    Operator { name: "And", action: "AND" },
];

pub struct RelationMethod {
    name: &'static str,
    action: &'static str
}

trait FieldExt {
    fn type_tokens(&self) -> TokenStream;
    
    fn type_prisma_value(&self, var: &Ident) -> TokenStream;
    
    fn type_query_value(&self, var: &Ident) -> TokenStream;
    
    fn relation_methods(&self) -> &'static [&'static str];
    
    fn required_on_create(&self) -> bool;
}

impl FieldExt for Field {
    fn type_tokens(&self) -> TokenStream {
        match self.field_type() {
            FieldType::Enum(name) => {
                let name = format_ident!("{}", name.to_case(Case::Pascal));
                quote!(#name)
            },
            FieldType::Relation(info) => {
                let model = format_ident!("{}", info.to.to_case(Case::Snake));
                quote!(#model::Data)
            },
            FieldType::Scalar(typ, _, _) => typ.to_tokens(),
            _ => unimplemented!()
        }
    }
    
    fn type_prisma_value(&self, var: &Ident) -> TokenStream {
        match self.field_type() {
            FieldType::Scalar(typ, _, _) => typ.to_prisma_value(var),
            _ => unimplemented!()
        }
    }
    
    fn type_query_value(&self, var: &Ident) -> TokenStream {
        if self.arity().is_list() {
            let converter = self.type_prisma_value(&format_ident!("v"));
            quote!(QueryValue::List(#var.into_iter().map(|v| #converter.into).collect()))
        } else {
            let t = self.type_prisma_value(var);
            
            quote!(#t.into())
        }
    }

    fn relation_methods(&self) -> &'static [&'static str] {
        if self.arity().is_list() {
            &["some", "every", "none"]
        } else {
            &["is", "is_not"]
        }
    }

    fn required_on_create(&self) -> bool {
        self.arity().is_required() && !self.is_updated_at() && self.default_value().is_none()
    }
}

trait FieldTypeExt {
    fn to_tokens(&self) -> TokenStream;
    fn to_prisma_value(&self, var: &Ident) -> TokenStream;
    fn to_query_value(&self, var: &Ident, is_list: bool) -> TokenStream;
}

impl FieldTypeExt for FieldType {
    fn to_tokens(&self) -> TokenStream {
        match self {
            FieldType::Enum(name) => {
                let name = format_ident!("{}", name.to_case(Case::Pascal));
                quote!(#name)
            },
            FieldType::Relation(info) => {
                let model = format_ident!("{}", info.to.to_case(Case::Snake));
                quote!(#model::Data)
            },
            FieldType::Scalar(typ, _, _) => typ.to_tokens(),
            _ => unimplemented!()
        }
    }
    
    fn to_prisma_value(&self, var: &Ident) -> TokenStream {
        match self {
            FieldType::Scalar(typ, _, _) => typ.to_prisma_value(var),
            _ => unimplemented!()
        }
    }
    
    fn to_query_value(&self, var: &Ident, is_list: bool) -> TokenStream {
        if is_list {
            let converter = self.to_prisma_value(&format_ident!("v"));
            quote!(QueryValue::List(#var.into_iter().map(|v| #converter.into).collect()))
        } else {
            let t = self.to_prisma_value(var);
            
            quote!(#t.into())
        }
    }
}

trait ScalarTypeExt {
    fn to_tokens(&self) -> TokenStream;
    fn to_prisma_value(&self, var: &Ident) -> TokenStream;
    fn to_query_value(&self, var: &Ident, is_list: bool) -> TokenStream;
}

impl ScalarTypeExt for ScalarType {
    fn to_tokens(&self) -> TokenStream {
        match self {
            ScalarType::Int => quote!(i32),
            ScalarType::BigInt => quote!(i64),
            ScalarType::Float | ScalarType::Decimal => quote!(f64),
            ScalarType::Boolean => quote!(bool),
            ScalarType::String => quote!(String),
            ScalarType::Json => quote!(serde_json::Value),
            ScalarType::DateTime => quote!(chrono::DateTime<chrono::FixedOffset>),
            ScalarType::Bytes => quote!(Vec<u8>),
        }
    }
    
    fn to_prisma_value(&self, var: &Ident) -> TokenStream {
        match self {
            ScalarType::Int => quote!(PrismaValue::Int(#var as i64)),
            ScalarType::BigInt => quote!(PrismaValue::BigInt(#var)),
            ScalarType::Float | ScalarType::Decimal => quote!(PrismaValue::Float(bigdecimal::BigDecimal::from_f64(#var).unwrap().normalized())),
            ScalarType::Boolean => quote!(PrismaValue::Boolean(#var)),
            ScalarType::String => quote!(PrismaValue::String(#var)),
            ScalarType::Json => quote!(PrismaValue::Json(serde_json::to_string(&#var).unwrap())),
            ScalarType::DateTime => quote!(PrismaValue::DateTime(#var)),
            ScalarType::Bytes => quote!(PrismaValue::Bytes(#var)),
        }
    }
    
    fn to_query_value(&self, var: &Ident, is_list: bool) -> TokenStream {
        if is_list {
            let converter = self.to_prisma_value(&format_ident!("v"));
            quote!(QueryValue::List(#var.into_iter().map(|v| #converter.into).collect()))
        } else {
            let t = self.to_prisma_value(var);
            
            quote!(#t.into())
        }
    }
}

struct Outputs {
    outputs: Vec<String>
}

impl Outputs {
    pub fn new(model: &Model) -> Self {
        Self {
            outputs: model
                .fields
                .iter()
                .filter(|f| f.required_on_create())
                .map(|f| f.name().to_string())
                .collect()
        }
    }

    pub fn quote(&self) -> TokenStream {
        let Self { outputs } = self;

        quote! {
            pub fn _outputs() -> Vec<Selection> {
                [#(#outputs),*]
                    .into_iter()
                    .map(|o| {
                        let builder = Selection::builder(o);
                        builder.build()
                    })
                    .collect()
            }
        }
    }
}

struct WithParams {
    variants: Vec<TokenStream>,
    match_arms: Vec<TokenStream>,
    from_args: Vec<TokenStream>
}

impl WithParams {
    pub fn new() -> Self {
        Self {
            variants: vec![],
            match_arms: vec![],
            from_args: vec![]
        }
    }
    
    fn with_fn(module: &Ident) -> TokenStream {
        quote! {
            pub fn with(mut self, params: impl Into<#module::WithParam>) -> Self {
                self.args = self.args.with(params.into());
                self
            }
        }
    }
    
    fn add_single_variant(&mut self, field_name: &str, model_module: &Ident, variant_name: &Ident) {
        self.variants.push(quote!(#variant_name(super::#model_module::Args)));
        self.match_arms.push(quote! {
            Self::#variant_name(args) => {
                let mut selections = super::#model_module::_outputs();
                selections.extend(args.with_params.into_iter().map(Into::<Selection>::into));
                
                let mut builder = Selection::builder(#field_name);
                builder.nested_selections(selections);
                builder.build()
            }
        });
    }
    
    fn add_many_variant(&mut self, field_name: &str, model_module: &Ident, variant_name: &Ident) {
        self.variants.push(quote!(#variant_name(super::#model_module::FindManyArgs)));
        self.match_arms.push(quote! {
            Self::#variant_name(args) => {
                let FindManySelectionArgs {
                    mut nested_selections,
                    arguments
                } = args.into();
                nested_selections.extend(super::#model_module::_outputs());
                
                let mut builder = Selection::builder(#field_name);
                builder.nested_selections(nested_selections)
                    .set_arguments(arguments);
                builder.build()
            }
        });
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            variants,
            match_arms,
            from_args,
            ..
        } = self;

        quote! {
            pub enum WithParam {
                #(#variants),*
            }
            
            impl Into<Selection> for WithParam {
                fn into(self) -> Selection {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
            
            #(#from_args)*
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
            
            impl Into<(String, QueryValue)> for SetParam {
                fn into(self) -> (String, QueryValue) {
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
    variants: Vec<TokenStream>,
    match_arms: Vec<TokenStream>,
}

impl OrderByParams {
    pub fn new() -> Self {
        Self {
            variants: vec![],
            match_arms: vec![],
        }
    }
    
    fn order_by_fn(module: &Ident) -> TokenStream {
        quote! {
            pub fn order_by(mut self, param: #module::OrderByParam) -> Self {
                self.args = self.args.order_by(param);
                self
            }
        }
    }

    fn add_variant(&mut self, field_name: &str, variant_name: &Ident) {
        self.variants.push(quote!(#variant_name(Direction)));
        self.match_arms.push(quote! {
            Self::#variant_name(direction) => (
                #field_name.to_string(), 
                QueryValue::String(direction.to_string())
            ) 
        });
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
            
            impl Into<(String, QueryValue)> for OrderByParam {
                fn into(self) -> (String, QueryValue) {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        }
    }
}

struct PaginationParams {
    cursor_variants: Vec<TokenStream>,
    cursor_match_arms: Vec<TokenStream>,
}

impl PaginationParams {
    pub fn new() -> Self {
        Self {
            cursor_variants: vec![],
            cursor_match_arms: vec![],
        }
    }
    
    pub fn pagination_fns(module: &Ident) -> TokenStream {
        quote! {
            pub fn skip(mut self, value: i64) -> Self {
                self.args = self.args.skip(value);
                self
            }

            pub fn take(mut self, value: i64) -> Self {
                self.args = self.args.take(value);
                self
            }

            pub fn cursor(mut self, value: impl Into<#module::Cursor>) -> Self {
                self.args = self.args.cursor(value.into());
                self
            }
        }
    }

    pub fn add_variant(&mut self, field: &Field) {
        let field_name = field.name();
        let rust_type = field.type_tokens();
        let variant_name = format_ident!("{}Cursor", field_name.to_case(Case::Pascal));
        let prisma_value = field.type_prisma_value(&format_ident!("cursor"));
        
        self.cursor_variants.push(quote!(#variant_name(#rust_type)));
        
        self.cursor_match_arms.push(quote! {
            Self::#variant_name(cursor) => (
                #field_name.to_string(),
                #prisma_value.into()
            )
        });
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
            
            impl Into<(String, QueryValue)> for Cursor {
                fn into(self) -> (String, QueryValue) {
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
    pub to_query_value: Vec<TokenStream>,
    pub unique_variants: Vec<TokenStream>,
    pub from_unique_match_arms: Vec<TokenStream>,
    pub from_optional_uniques: Vec<TokenStream>
}

impl WhereParams {
    pub fn new() -> Self {
        Self {
            variants: vec![],
            to_query_value: vec![],
            unique_variants: vec![],
            from_unique_match_arms: vec![],
            from_optional_uniques: vec![]
        }
    }

    pub fn add_variant(&mut self, variant: TokenStream, match_arm: TokenStream) {
        self.variants.push(variant);
        self.to_query_value.push(match_arm);
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
            to_query_value,
            unique_variants,
            from_unique_match_arms,
            from_optional_uniques
        } = self;

        quote! {
            pub enum WhereParam {
                #(#variants),*
            }
            
            impl Into<SerializedWhere> for WhereParam {
                fn into(self) -> SerializedWhere {
                    match self {
                        #(#to_query_value),*
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
}

impl DataStruct {
    pub fn new() -> Self {
        Self {
            fields: vec![],
        }
    }

    pub fn add_field(&mut self, field: TokenStream) {
        self.fields.push(field);
    }

    pub fn quote(&self) -> TokenStream {
        let Self {
            fields,
        } = self;

        quote! {
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct Data {
                #(#fields),*
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

pub fn generate(args: &GeneratorArgs) -> Vec<TokenStream> {
    args.dml.models.iter().map(|model| {
        let model_outputs = Outputs::new(&model);
        let mut model_data_struct = DataStruct::new();
        let mut model_order_by_params = OrderByParams::new();
        let mut model_pagination_params = PaginationParams::new();
        let mut model_with_params = WithParams::new();
        let mut model_query_module = ModelQueryModules::new();
        let mut model_set_params = SetParams::new();
        let mut model_where_params = WhereParams::new();
        let mut model_actions = Actions::new();
        
        let model_name_string = &model.name;
        let model_name_snake = format_ident!("{}", model.name.to_case(Case::Snake));
 
        for op in OPERATORS {
            let variant_name = format_ident!("{}", op.name.to_case(Case::Pascal));
            let op_action = &op.action;

            model_where_params.add_variant(
                quote!(#variant_name(Vec<WhereParam>)),
                quote! {
                    Self::#variant_name(value) => (
                        #op_action.to_string(),
                        SerializedWhereValue::List(
                            value
                                .into_iter()
                                .map(|v| {
                                    QueryValue::Object(
                                        transform_equals(
                                            vec![Into::<SerializedWhere>::into(v)].into_iter(),
                                        )
                                        .into_iter()
                                        .collect(),
                                    )
                                })
                                .collect(),
                        ),
                    )
                },
            );
        }

        for unique in &model.indices {
            let variant_name_string = unique.fields.iter().map(|f| f.path[0].0.to_case(Case::Pascal)).collect::<String>();
            let variant_name = format_ident!("{}Equals", &variant_name_string);
            let accessor_name = format_ident!("{}", &variant_name_string.to_case(Case::Snake));
            
            let mut variant_data_as_types = vec![];
            let mut variant_data_as_args = vec![];
            let mut variant_data_as_destructured = vec![];
            let mut variant_data_as_query_values = vec![];
            let variant_data_names = unique.fields.iter().map(|f| f.path[0].0.to_string()).collect::<Vec<_>>();
            
            for field in &unique.fields {
                let model_field = model.fields.iter().find(|mf| mf.name() == &field.path[0].0).unwrap();
                let field_type = model_field.type_tokens();
                
                let field_name_snake = format_ident!("{}", field.path[0].0.to_case(Case::Snake));
                
                let field_type = match model_field.arity().is_list() {
                    true => quote!(Vec<#field_type>),
                    false => quote!(#field_type),
                };
                
                variant_data_as_args.push(quote!(#field_name_snake: #field_type));
                variant_data_as_types.push(field_type);
                variant_data_as_destructured.push(quote!(#field_name_snake));
                variant_data_as_query_values.push(model_field.type_query_value(&field_name_snake));
            }

            let field_name_string = unique.fields.iter().map(|f| f.path[0].0.to_string()).collect::<Vec<_>>().join("_");

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
                    Self::#variant_name(#(#variant_data_as_destructured),*) => (
                        #field_name_string.to_string(),
                        SerializedWhereValue::Object(vec![#((#variant_data_names.to_string(), #variant_data_as_query_values)),*])
                    )
                },
                quote! {
                    UniqueWhereParam::#variant_name(#(#variant_data_as_destructured),*) => Self::#variant_name(#(#variant_data_as_destructured),*)
                },
                quote!(#variant_name(#(#variant_data_as_types),*)),
            );
        }

        for root_field in &model.fields {
            let mut field_query_module =
                FieldQueryModule::new(&root_field);

            let field_string = root_field.name();
            let field_snake = format_ident!("{}", field_string.to_case(Case::Snake));
            let field_pascal = format_ident!("{}", field_string.to_case(Case::Pascal));
            let field_type = root_field.type_tokens();
            
            match root_field {
                Field::RelationField(field) => {
                    let link_variant = SetParams::field_link_variant(field_string);
                    let unlink_variant = SetParams::field_unlink_variant(field_string);
                    
                    let relation_type_snake = format_ident!("{}", field.relation_info.to.to_case(Case::Snake));
                    
                    for method in root_field.relation_methods() {
                        let method_action_string = method.to_case(Case::Camel);
                        let variant_name = format_ident!("{}{}", &field_pascal, method.to_case(Case::Pascal));
                        let method_name_snake = format_ident!("{}", method.to_case(Case::Snake));
                        
                        model_where_params.add_variant(
                            quote!(#variant_name(Vec<super::#relation_type_snake::WhereParam>)),
                            quote! {
                                Self::#variant_name(value) => (
                                    #field_string.to_string(),
                                    SerializedWhereValue::Object(vec![(
                                        #method_action_string.to_string(),
                                        QueryValue::Object(
                                            transform_equals(
                                                value
                                                    .into_iter()
                                                    .map(Into::<SerializedWhere>::into)    
                                            )
                                            .into_iter()
                                            .collect()
                                        ),
                                    )])
                                )
                            },
                        );
                        
                        field_query_module.add_method(quote! {
                            pub fn #method_name_snake(value: Vec<#relation_type_snake::WhereParam>) -> WhereParam {
                                WhereParam::#variant_name(value)
                            }
                        });
                    }
                    
                    let with_fn = WithParams::with_fn(&relation_type_snake);
                    
                    if field.arity.is_list() {
                        let order_by_fn = OrderByParams::order_by_fn(&relation_type_snake);
                        let pagination_fns = PaginationParams::pagination_fns(&relation_type_snake);
                                                                    
                        field_query_module.add_method(quote! {
                            pub struct Fetch {
                                args: #relation_type_snake::FindManyArgs
                            }
                            
                            impl Fetch {
                                #with_fn
                                
                                #order_by_fn
                                
                                #pagination_fns
                            }
                            
                            impl From<Fetch> for WithParam {
                                fn from(fetch: Fetch) -> Self {
                                    WithParam::#field_pascal(fetch.args)
                                }
                            }
                            
                            pub fn fetch(params: Vec<#relation_type_snake::WhereParam>) -> Fetch {
                                Fetch {
                                    args: #relation_type_snake::FindManyArgs::new(params)
                                }
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
                        
                        // Link variant
                        model_set_params.add_variant(
                            quote!(#link_variant(Vec<super::#relation_type_snake::UniqueWhereParam>)),
                            quote! {
                                SetParam::#link_variant(where_params) => (
                                    #field_string.to_string(),
                                    QueryValue::Object(
                                        vec![(
                                            "connect".to_string(),
                                            QueryValue::Object(
                                                transform_equals(
                                                    where_params
                                                        .into_iter()
                                                        .map(Into::<super::#relation_type_snake::WhereParam>::into)
                                                        .map(Into::into)
                                                )
                                                .into_iter()
                                                .collect()
                                            )
                                        )]
                                        .into_iter()
                                        .collect()
                                    )
                                )
                            }
                        );

                        // Unlink variant
                        model_set_params.add_variant(
                            quote!(#unlink_variant(Vec<super::#relation_type_snake::UniqueWhereParam>)),
                            quote! {
                                SetParam::#unlink_variant(where_params) => (
                                    #field_string.to_string(),
                                    QueryValue::Object(
                                        vec![(
                                            "disconnect".to_string(),
                                            QueryValue::Object(
                                                transform_equals( 
                                                    where_params
                                                        .into_iter()
                                                        .map(Into::<super::#relation_type_snake::WhereParam>::into)
                                                        .map(Into::into)
                                                )
                                                .into_iter()
                                                .collect()
                                            )
                                        )]
                                        .into_iter()
                                        .collect()
                                    )
                                )
                            },
                        );
                        
                        model_with_params.add_many_variant(
                            field_string,
                            &relation_type_snake,
                            &field_pascal
                        );

                        model_data_struct.add_field(
                            quote! {
                                #[serde(
                                    rename = #field_string,
                                    skip_serializing_if = "Result::is_err",
                                    default = "prisma_client_rust::serde::default_field_not_fetched",
                                    with = "prisma_client_rust::serde::required_relation"
                                )]
                                pub #field_snake: RelationResult<Vec<super::#relation_type_snake::Data>>
                            }
                        );
                    } else {
                        field_query_module.add_method(quote! {
                            pub struct Fetch {
                                args: #relation_type_snake::Args
                            }
                            
                            impl Fetch {
                                #with_fn
                            }
                            
                            impl From<Fetch> for WithParam {
                                fn from(fetch: Fetch) -> Self {
                                    WithParam::#field_pascal(fetch.args)
                                }
                            }
                            
                            pub fn fetch() -> Fetch {
                                Fetch {
                                    args: #relation_type_snake::Args::new()
                                }
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
                                SetParam::#link_variant(where_param) => (
                                    #field_string.to_string(),
                                    QueryValue::Object(
                                        vec![(
                                            "connect".to_string(),
                                            QueryValue::Object(
                                                transform_equals(
                                                    vec![where_param]
                                                        .into_iter()
                                                        .map(Into::<super::#relation_type_snake::WhereParam>::into)
                                                        .map(Into::into)
                                                )
                                                .into_iter()
                                                .collect()
                                            )
                                        )]
                                        .into_iter()
                                        .collect()
                                    )
                                )
                            }
                        );
                        
                        // Only allow unlink if field is not required
                        if !field.arity.is_optional() {
                            field_query_module.add_method(quote! {
                                pub fn unlink() -> SetParam {
                                    SetParam::#unlink_variant
                                }
                            });

                            model_set_params.add_variant(
                                quote!(#unlink_variant),
                                quote! {
                                    SetParam::#unlink_variant => (
                                        #field_string.to_string(),
                                        QueryValue::Object(
                                            vec![(
                                                "disconnect".to_string(),
                                                QueryValue::Boolean(true)
                                            )]
                                            .into_iter()
                                            .collect()
                                        )
                                    )
                                },
                            );
                        }
                        
                        model_with_params.add_single_variant(
                            field_string,
                            &relation_type_snake,
                            &field_pascal
                        );
                        
                        let struct_field_type = if !field.arity.is_optional() {
                            quote!(Box<super::#relation_type_snake::Data>)
                        } else {
                            quote!(Option<Box<super::#relation_type_snake::Data>>)
                        };
                        
                        let serde_attr = if !field.arity.is_optional() {
                            quote! {
                                #[serde(
                                    rename = #field_string,
                                    skip_serializing_if = "Result::is_err",
                                    default = "prisma_client_rust::serde::default_field_not_fetched",
                                    with = "prisma_client_rust::serde::required_relation"
                                )]
                            }
                        } else {
                            quote! {
                               #[serde(
                                    rename = #field_string,
                                    skip_serializing_if = "Result::is_err", 
                                    default = "prisma_client_rust::serde::default_field_not_fetched",
                                    with = "prisma_client_rust::serde::optional_single_relation"
                                )]
                            }
                        };
                        
                        model_data_struct.add_field(
                            quote! {
                                #serde_attr
                                pub #field_snake: RelationResult<#struct_field_type>
                            }
                        );
                    }
                    
                    if root_field.required_on_create() {
                        model_actions.push_required_arg(
                            quote!(#field_snake: #field_snake::Link,),
                            quote!(params.push(#field_snake.into());),
                       );
                    }
                },
                Field::ScalarField(field) => {
                    let field_set_variant = SetParams::field_set_variant(field_string);
                    
                    let converter = root_field.type_query_value(&format_ident!("value"));
                    
                    let (field_set_variant_type, field_content) = match field.arity {
                        FieldArity::List => (
                            quote!(Vec<#field_type>),
                            converter,
                        ),
                        FieldArity::Required => (quote!(#field_type), converter),
                        FieldArity::Optional => (quote!(Option<#field_type>), quote!(value.map(|value| #converter).unwrap_or(QueryValue::Null)))
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
                            SetParam::#field_set_variant(value) => (
                                #field_string.to_string(),
                                #field_content
                            )
                        },
                    );

                    let equals_variant_name = format_ident!("{}Equals", &field_pascal);
                    let equals_variant = quote!(#equals_variant_name(#field_set_variant_type));
                    let type_as_query_value = root_field.type_query_value(&format_ident!("value"));
                    
                    let type_as_query_value = if !root_field.arity().is_optional() {
                        type_as_query_value
                    } else {
                        quote!(value.map(|value| #type_as_query_value).unwrap_or(QueryValue::Null))
                    };

                    let match_arm = quote! {
                        Self::#equals_variant_name(value) => (
                            #field_string.to_string(),
                            SerializedWhereValue::Object(vec![("equals".to_string(), #type_as_query_value)])
                        )
                    };
                    
                    match (model.field_is_primary(field_string), model.field_is_unique(field_string), !root_field.arity().is_optional())  {
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
                    });
                    
                    if model.field_is_primary(field_string) || model.field_is_unique(field_string) {
                        field_query_module.add_method(quote! {
                            pub fn cursor(cursor: #field_type) -> Cursor {
                                Cursor::#field_pascal(cursor)
                            }
                        });
                    }

                    model_data_struct.add_field(match (root_field.arity().is_list(), !root_field.arity().is_optional()) {
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
                    
                    if let Some(read_type) = args.read_filter(&field) {
                        for method in &read_type.methods {
                            let typ = method.typ.to_tokens();

                            let method_name = format_ident!("{}", method.name.to_case(Case::Snake));
                            let variant_name =
                                format_ident!("{}{}", &field_pascal, method.name.to_case(Case::Pascal));
                            let method_action_string = &method.action;

                            let field_name = field.name.to_string();

                            let (typ, value_as_query_value) = if method.is_list {
                                let prisma_value_converter = method.typ.to_prisma_value(&format_ident!("v"));
                                
                                (
                                    quote!(Vec<#typ>),
                                    quote! {
                                        QueryValue::List(
                                            value
                                                .into_iter()
                                                .map(|v| #prisma_value_converter.into())
                                                .collect()
                                        )
                                    },
                                )
                            } else {
                                let as_prisma_value = method.typ.to_prisma_value(&format_ident!("value"));
                                (
                                    typ,
                                    quote!(#as_prisma_value.into()),
                                )
                            };

                            model_where_params.add_variant(
                                quote!(#variant_name(#typ)),
                                quote! {
                                    Self::#variant_name(value) => (
                                        #field_name.to_string(),
                                        SerializedWhereValue::Object(vec![(#method_action_string.to_string(), #value_as_query_value)])
                                    )
                                },
                            );

                            field_query_module.add_method(quote! {
                                pub fn #method_name(value: #typ) -> WhereParam {
                                    WhereParam::#variant_name(value)
                                }
                            });
                        }
                    }

                    if let Some(write_type) = args.write_filter(&field) {
                        for method in &write_type.methods {
                            let typ = method.typ.to_tokens();

                            let method_name_snake = format_ident!("{}", method.name.to_case(Case::Snake));

                            let typ = if method.is_list {
                                quote!(Vec<#typ>)
                            } else { typ };
                            
                            let query_value_converter = method.typ.to_query_value(&format_ident!("value"), method.is_list);

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
                                    SetParam::#variant_name(value) => (
                                        #field_string.to_string(),
                                        QueryValue::Object(
                                            vec![(
                                                #method_action.to_string(),
                                                #query_value_converter
                                            )]
                                                .into_iter()
                                                .collect()
                                        )
                                    )
                                }
                            );
                        }
                    }

                    model_order_by_params.add_variant(field_string, &field_pascal);

                    model_pagination_params.add_variant(&root_field);

                    if root_field.required_on_create() {
                        model_actions.push_required_arg(
                            quote!(#field_snake: #field_snake::Set,),
                            quote!(params.push(#field_snake.into());),
                        );
                    }
                },
                _ => todo!()
            };
            
            std::fs::write("./bruh.rs", format!("{:?}", &args.schema));
            

            model_query_module.add_field_module(field_query_module);
        }

        let Actions {
            required_args,
            required_arg_pushes,
            ..
        } = &model_actions;
        
        let with_fn = WithParams::with_fn(&model_name_snake);
        let order_by_fn = OrderByParams::order_by_fn(&model_name_snake);
        let pagination_fns = PaginationParams::pagination_fns(&model_name_snake);

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

                pub type FindManyArgs = prisma_client_rust::FindManyArgs<WhereParam, WithParam, OrderByParam, Cursor>;
                    
                pub struct FindMany<'a> {
                    ctx: QueryContext<'a>,
                    args: FindManyArgs,
                }

                impl<'a> FindMany<'a> {
                    pub async fn exec(self) -> QueryResult<Vec<Data>> {
                        let Self { ctx, args } = self;
                        ctx.execute(args.to_operation(#model_name_string, _outputs()))
                            .await
                    }

                    pub fn delete(self) -> DeleteMany<'a> {
                        let Self {
                            ctx,
                            args
                        } = self;
                        
                        DeleteMany {
                            ctx,
                            args: DeleteManyArgs::new(args.where_params),
                        }
                    }

                    pub fn update(mut self, params: Vec<SetParam>) -> UpdateMany<'a> {
                        let Self {
                            ctx,
                            args
                        } = self;
                        
                        UpdateMany {
                            ctx,
                            args: UpdateManyArgs::new(args.where_params, params),
                        }
                    }

                    #with_fn

                    #order_by_fn

                    #pagination_fns
                }
                
                pub type FindFirstArgs = prisma_client_rust::FindFirstArgs<WhereParam, WithParam, OrderByParam, Cursor>;

                pub struct FindFirst<'a> {
                    ctx: QueryContext<'a>,
                    args: FindFirstArgs,
                }

                impl<'a> FindFirst<'a> {
                    pub async fn exec(self) -> QueryResult<Option<Data>> {
                        let Self { ctx, args } = self;
                        ctx.execute(args.to_operation(#model_name_string, _outputs()))
                            .await
                    }

                    #with_fn

                    #order_by_fn

                    #pagination_fns
                }
                
                pub type Args = prisma_client_rust::Args<WithParam>;
                pub type FindUniqueArgs = prisma_client_rust::FindUniqueArgs<WhereParam, WithParam>;
                
                pub struct FindUnique<'a> {
                    ctx: QueryContext<'a>,
                    args: FindUniqueArgs,
                }

                impl<'a> FindUnique<'a> {
                    pub async fn exec(self) -> QueryResult<Option<Data>> {
                        let Self { ctx, args } = self;
                        ctx.execute(args.to_operation(#model_name_string, _outputs()))
                            .await
                    }

                    pub fn delete(self) -> Delete<'a> {
                        let Self {
                            ctx,
                            args
                        } = self;
                        
                        let FindUniqueArgs {
                            where_param,
                            with_params
                        } = args;
                        
                        Delete { ctx, args: DeleteArgs::new(where_param, with_params) }
                    }

                    #with_fn

                    pub fn update(mut self, params: Vec<SetParam>) -> Update<'a> {
                        let Self {
                            ctx,
                            args
                        } = self;
                        
                        let FindUniqueArgs {
                            where_param,
                            with_params
                        } = args;
                        
                        Update {
                            ctx,
                            args: UpdateArgs::new(where_param, params, with_params)
                        }
                    }
                }
                
                pub type CreateArgs = prisma_client_rust::CreateArgs<SetParam, WithParam>;

                pub struct Create<'a> {
                    ctx: QueryContext<'a>,
                    args: CreateArgs,
                }

                impl<'a> Create<'a> {
                    pub async fn exec(self) -> QueryResult<Data> {
                        let Self { ctx, args } = self;
                        ctx.execute(args.to_operation(#model_name_string, _outputs()))
                            .await
                    }

                    #with_fn
                }

                pub type UpdateArgs = prisma_client_rust::UpdateArgs<WhereParam, SetParam, WithParam>;

                pub struct Update<'a> {
                    ctx: QueryContext<'a>,
                    args: UpdateArgs
                }

                impl<'a> Update<'a> {
                    pub async fn exec(self) -> QueryResult<Option<Data>> {
                        let Self {
                            ctx,
                            args,
                        } = self;
                        
                        let result = ctx.execute(args.to_operation(#model_name_string, _outputs()))
                            .await;
                        
                        match result {
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
                
                pub type UpdateManyArgs = prisma_client_rust::UpdateManyArgs<WhereParam, SetParam>;

                pub struct UpdateMany<'a> {
                    ctx: QueryContext<'a>,
                    args: UpdateManyArgs
                }

                impl<'a> UpdateMany<'a> {
                    pub async fn exec(self) -> QueryResult<i64> {
                        let Self { ctx, args } = self;
                        ctx.execute(args.to_operation(#model_name_string)).await.map(|res: BatchResult| res.count)
                    }
                }
                
                pub type UpsertArgs = prisma_client_rust::UpsertArgs<WhereParam, SetParam, WithParam>;

                pub struct Upsert<'a> {
                    ctx: QueryContext<'a>,
                    args: UpsertArgs
                }

                impl<'a> Upsert<'a> {
                    pub async fn exec(self) -> QueryResult<Data> {
                        let Self { ctx, args } = self;
                        ctx.execute(args.to_operation(#model_name_string, _outputs()))
                            .await
                    }

                    pub fn create(
                        mut self,
                        #(#required_args)*
                        mut params: Vec<SetParam>
                    ) -> Self {
                        #(#required_arg_pushes)*

                        self.args = self.args.create(params);

                        self
                    }

                    pub fn update(mut self, params: Vec<SetParam>) -> Self {
                        self.args = self.args.update(params);
                        
                        self
                    }
                }
                
                pub type DeleteArgs = prisma_client_rust::DeleteArgs<WhereParam, WithParam>;
                
                pub struct Delete<'a> {
                    ctx: QueryContext<'a>,
                    args: DeleteArgs
                }

                impl<'a> Delete<'a> {
                    pub async fn exec(self) -> QueryResult<Option<Data>> {
                        let Self { ctx, args } = self;
                        
                        let result = ctx.execute(args.to_operation(#model_name_string, _outputs())).await;
                        
                        match result {
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
                
                pub type DeleteManyArgs = prisma_client_rust::DeleteManyArgs<WhereParam>;
                
                pub struct DeleteMany<'a> {
                    ctx: QueryContext<'a>,
                    args: DeleteManyArgs
                }

                impl<'a> DeleteMany<'a> {
                    pub async fn exec(self) -> QueryResult<i64> {
                        let Self { ctx, args } = self;
                        ctx.execute(args.to_operation(#model_name_string)).await.map(|res: BatchResult| res.count)
                    }
                }

                pub struct Actions<'a> {
                    pub client: &'a PrismaClient,
                }

                impl<'a> Actions<'a> {
                    pub fn create(&self, #(#required_args)* mut params: Vec<SetParam>) -> Create {
                        #(#required_arg_pushes)*

                        Create {
                            ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                            args: CreateArgs::new(params)
                        }
                    }

                    pub fn find_unique(&self, param: UniqueWhereParam) -> FindUnique {
                        FindUnique {
                            ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                            args: FindUniqueArgs::new(param.into()),
                        }
                    }

                    pub fn find_first(&self, params: Vec<WhereParam>) -> FindFirst {
                        FindFirst {
                            ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                            args: FindFirstArgs::new(params),
                        }
                    }

                    pub fn find_many(&self, params: Vec<WhereParam>) -> FindMany {
                        FindMany {
                            ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                            args: FindManyArgs::new(params),
                        }
                    }

                    pub fn upsert(&self, param: UniqueWhereParam) -> Upsert {
                        Upsert { 
                            ctx: QueryContext::new(&self.client.executor, self.client.query_schema.clone()),
                            args: UpsertArgs::new(param.into()),
                        }
                    }
                }
            }
        }
    }).collect()
}
