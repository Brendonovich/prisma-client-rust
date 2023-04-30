use prisma_client_rust_sdk::prisma::{
    prisma_models::{
        walkers::{FieldWalker, ModelWalker, RefinedFieldWalker, ScalarFieldWalker},
        FieldArity,
    },
    psl::parser_database::ScalarFieldType,
};

use crate::generator::prelude::*;

enum Variant {
    Select,
    Include,
}

impl core::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Select => "select",
            Self::Include => "include",
        };

        write!(f, "{}", s)
    }
}

impl Variant {
    fn type_trait(&self) -> Ident {
        format_ident!("{}Type", pascal_ident(&self.to_string()))
    }

    fn param(&self) -> Ident {
        format_ident!("{}Param", pascal_ident(&self.to_string()))
    }
}

fn model_macro<'a>(
    model: ModelWalker<'a>,
    module_path: &TokenStream,
    variant: Variant,
    // Fields that should always be included
    base_fields: impl Iterator<Item = ScalarFieldWalker<'a>> + Clone,
    // Fields that can be picked from
    selection_fields: impl Iterator<Item = FieldWalker<'a>> + Clone,
) -> TokenStream {
    let model_name_pascal_str = pascal_ident(model.name()).to_string();
    let model_name_snake = snake_ident(model.name());
    let model_name_snake_raw = snake_ident_raw(model.name());
    let macro_name = format_ident!("_{variant}_{model_name_snake_raw}");

    let model_module = quote!(#module_path::#model_name_snake);

    let selection_type = variant.type_trait();
    let selection_param = variant.param();

    let variant_ident = format_ident!("{variant}");
    let variant_pascal = pascal_ident(&variant.to_string());

    let filters_pattern_produce = quote!(($($filters:tt)+)$(.$arg:ident($($arg_params:tt)*))*);
    let filters_pattern_consume = quote!(($($filters)+)$(.$arg($($arg_params)*))*);

    let selections_pattern_produce = quote!(: $selection_mode:ident {$($selections:tt)+});
    let selections_pattern_consume = quote!(: $selection_mode {$($selections)+});

    let selection_pattern_produce =
        quote!($field:ident $(#filters_pattern_produce)? $(#selections_pattern_produce)?);
    let selection_pattern_consume =
        quote!($field $(#filters_pattern_consume)? $(#selections_pattern_consume)?);

    let field_type_impls = selection_fields.clone().map(|field| {
        let field_name_snake = snake_ident(field.name());
        let field_type = field.type_tokens(module_path);

        let selection_type_impl = matches!(field.refine(), RefinedFieldWalker::Relation(_)).then(|| {
            let field_type = field
                .ast_field()
                .arity
                .wrap_type(&quote!(#field_name_snake::Data));

            quote!((@field_type; #field_name_snake #selections_pattern_produce) => { #field_type };)
        });

        quote! {
            #selection_type_impl
            (@field_type; #field_name_snake) => { #field_type };
        }
    });

    let field_module_impls = model.relation_fields().map(|field| {
        let field_name_snake = snake_ident(field.name());
        let relation_model_name_snake = snake_ident(field.related_model().name());

        quote! {
            (@field_module; #field_name_snake #selections_pattern_produce) => {
                #module_path::#relation_model_name_snake::#variant_ident!(@definitions; ; $($selections)+);
            };
        }
    });

    let selection_field_to_selection_param_impls = selection_fields.clone().map(|field| {
        let field_name_snake = snake_ident(field.name());

        let field_module = quote!(#model_module::#field_name_snake);

        match field.refine() {
            RefinedFieldWalker::Relation(relation_field) =>{
                let relation_model_name_snake = snake_ident(relation_field.related_model().name());

                let relation_model_module = quote!(#module_path::#relation_model_name_snake);

                match relation_field.ast_field().arity {
                    FieldArity::List => {
                        quote! {
                            (@selection_field_to_selection_param; #field_name_snake $(#filters_pattern_produce)? #selections_pattern_produce) => {{
                                Into::<#model_module::#selection_param>::into(
                                    #field_module::#variant_pascal::$selection_mode(
                                        #relation_model_module::ManyArgs::new(#module_path::#relation_model_name_snake::#variant_ident!(
                                            @filters_to_args;
                                            $($($filters)+)?
                                        )) $($(.$arg($($arg_params)*))*)?,
                                        #relation_model_module::select!(
                                            @selections_to_params;
                                            #selections_pattern_consume
                                        ).into_iter().collect()
                                    )
                                )
                            }};
                            (@selection_field_to_selection_param; #field_name_snake $(#filters_pattern_produce)?) => {{
                                Into::<#model_module::#selection_param>::into(
                                    #field_module::#variant_pascal::Fetch(
                                        #relation_model_module::ManyArgs::new(#module_path::#relation_model_name_snake::#variant_ident!(
                                            @filters_to_args;
                                            $($($filters)+)?
                                        )) $($(.$arg($($arg_params)*))*)?
                                    ),
                                )
                            }};
                        }
                    },
                    _ => quote! {
                        (@selection_field_to_selection_param; #field_name_snake $(#filters_pattern_produce)? #selections_pattern_produce) => {{
                            Into::<#model_module::#selection_param>::into(
                                #field_module::#variant_pascal::$selection_mode(
                                    #relation_model_module::select!(
                                        @selections_to_params;
                                        #selections_pattern_consume
                                    ).into_iter().collect()
                                )
                            )
                        }};
                        (@selection_field_to_selection_param; #field_name_snake $(#filters_pattern_produce)?) => {{
                            Into::<#model_module::#selection_param>::into(
                                #field_module::#variant_pascal::Fetch
                            )
                        }};
                    }
                }
            },
            RefinedFieldWalker::Scalar(scalar_field) => match scalar_field.scalar_field_type() {
                ScalarFieldType::CompositeType(_) => quote!(),
                _ => {
                    quote! {
                        (@selection_field_to_selection_param; #field_name_snake) => {
                            Into::<#model_module::#selection_param>::into(
                                #field_module::#variant_pascal
                            )
                        };
                    }
                }
            }
        }
    });

    let data_struct_scalar_fields = base_fields.clone().map(|f| {
        let field_name_snake = snake_ident(f.name());
        let field_type = f.type_tokens(module_path);

        let specta_rename = cfg!(feature = "specta").then(|| {
            quote!(#[specta(rename_from_path = #module_path::#model_name_snake::#field_name_snake::NAME)])
        });

        quote! {
            #specta_rename
            pub #field_name_snake: #field_type
        }
    });

    let fields_enum_variants = selection_fields.clone().map(|f| {
        let i = snake_ident(f.name());
        quote!(#i)
    });

    let field_serde_names = model
        .fields()
        .filter(|f| f.ast_field().field_type.as_unsupported().is_none())
        .map(|f| {
            let field_name_str = f.name();
            let field_name_snake = snake_ident(f.name());

            quote!((@field_serde_name; #field_name_snake) => { #field_name_str };)
        });

    let base_field_names_snake = base_fields
        .clone()
        .map(|f| snake_ident(f.name()))
        .collect::<Vec<_>>();

    let deserialize_impl = {
        let field_names_str = model.fields().map(|f| f.name());

        quote! {
            #[allow(warnings)]
            enum Field {
                $($field),+,
                #(#base_field_names_snake),*
            }

            impl<'de> ::serde::Deserialize<'de> for Field {
                fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    struct FieldVisitor;

                    impl<'de> ::serde::de::Visitor<'de> for FieldVisitor {
                        type Value = Field;

                        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                            formatter.write_str(&[
                                $(#model_module::$field::NAME),+,
                                #(#model_module::#base_field_names_snake::NAME),*
                            ].into_iter().collect::<Vec<_>>().join(", "))
                        }

                        fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: ::serde::de::Error,
                        {
                            match value {
                                $(#model_module::$field::NAME => Ok(Field::$field)),*,
                                #(#model_module::#base_field_names_snake::NAME => Ok(Field::#base_field_names_snake),)*
                                _ => Err(::serde::de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }

                    deserializer.deserialize_identifier(FieldVisitor)
                }
            }

            struct DataVisitor;

            impl<'de> ::serde::de::Visitor<'de> for DataVisitor {
                type Value = Data;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("struct Data")
                }

                fn visit_map<V>(self, mut map: V) -> Result<Data, V::Error>
                where
                    V: ::serde::de::MapAccess<'de>,
                {
                    $(let mut $field = None;)*
                    #(let mut #base_field_names_snake = None;)*

                    while let Some(key) = map.next_key()? {
                        match key {
                            #(Field::#base_field_names_snake => {
                                if #base_field_names_snake.is_some() {
                                    return Err(::serde::de::Error::duplicate_field(
                                        #model_module::#base_field_names_snake::NAME
                                    ));
                                }
                                #base_field_names_snake = Some(map.next_value()?);
                            })*
                            $(Field::$field => {
                                if $field.is_some() {
                                    return Err(::serde::de::Error::duplicate_field(
                                        #model_module::$field::NAME
                                    ));
                                }
                                $field = Some(map.next_value()?);
                            })*
                        }
                    }

                    $(let $field = $field.ok_or_else(||
                        serde::de::Error::missing_field(#model_module::$field::NAME)
                    )?;)*
                    #(let #base_field_names_snake = #base_field_names_snake.ok_or_else(||
                        serde::de::Error::missing_field(#model_module::#base_field_names_snake::NAME)
                    )?;)*

                    Ok(Data { #(#base_field_names_snake,)* $($field),* })
                }
            }

            const FIELDS: &'static [&'static str] = &[#(#field_names_str),*];
            deserializer.deserialize_struct("Data", FIELDS, DataVisitor)
        }
    };

    let serialize_impl = {
        quote! {
            use ::serde::ser::SerializeStruct;

            let mut state = serializer.serialize_struct(
                "Data",
                [
                    $(stringify!($field),)+
                    #(stringify!(#base_field_names_snake)),*
                ].len()
            )?;
            $(state.serialize_field(#model_module::$field::NAME, &self.$field)?;)*
            #(state.serialize_field(#model_module::#base_field_names_snake::NAME, &self.#base_field_names_snake)?;)*
            state.end()
        }
    };

    let all_fields_str = selection_fields
        .clone()
        .map(|f| snake_ident(f.name()).to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let data_struct_attrs = quote! {
        #[allow(warnings)]
        #[derive(std::fmt::Debug, Clone)]
    };

    let specta_macro_arms = cfg!(feature = "specta").then(|| {
        let data_struct_attrs = quote! {
            #data_struct_attrs
            #[derive(::prisma_client_rust::specta::Type)]
            #[specta(
                rename_from_path = SPECTA_TYPE_NAME,
                crate = "prisma_client_rust::specta"
            )]
        };

        quote! {
            (@specta_data_struct; $struct:item;) => {
                #data_struct_attrs
                #[specta(inline)]
                $struct
            };
            (@specta_data_struct; $struct:item; $name:ident) => {
                #data_struct_attrs
                $struct
            };
        }
    });

    let data_struct = {
        let specta_rename = cfg!(feature = "specta").then(|| {
            quote!(#[specta(rename_from_path =
                #model_module::$field::NAME
            )])
        });

        quote! {
            pub struct Data {
                #(#data_struct_scalar_fields,)*
                $(
                    #specta_rename
                    pub $field: #model_module::#variant_ident!(@field_type; $field $(#selections_pattern_consume)?),
                )+
            }
        }
    };
    let data_struct = cfg!(feature = "specta")
        .then(|| {
            quote! {
                const SPECTA_TYPE_NAME: &'static str = prisma_client_rust::macros::to_pascal_case!(
                    $($module_name)?
                );
                #model_module::#variant_ident!(@specta_data_struct; #data_struct; $($module_name)?);
            }
        })
        .unwrap_or(quote!( #data_struct_attrs #data_struct ));

    let selection = {
        let scalar_selections = matches!(variant, Variant::Include).then(||
            quote! {
                <#module_path::#model_name_snake::Types as ::prisma_client_rust::ModelTypes>::scalar_selections()
            }
        );

        quote!(Selection(
            [
                #module_path::#model_name_snake::#variant_ident!(
                    @selections_to_params; : #variant_ident
                    { $(#selection_pattern_consume)+ }
                )
                    .into_iter()
                    .map(|p| p.to_selection())
                    .collect::<Vec<_>>(),
                #scalar_selections
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
        ))
    };

    let selection_struct = quote! {
        pub struct Selection(Vec<::prisma_client_rust::Selection>);

        impl ::prisma_client_rust::#selection_type for Selection {
            type Data = Data;
            type ModelData = #model_module::Data;

            fn to_selections(self) -> Vec<::prisma_client_rust::Selection> {
                self.0
            }
        }
    };

    quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ($(($($func_arg:ident: $func_arg_ty:ty),+) =>)? $module_name:ident { $(#selection_pattern_produce)+ }) => {
                #[allow(warnings)]
                pub mod $module_name {
                    #model_module::#variant_ident!(@definitions; $module_name; $(#selection_pattern_consume)+);

                    use super::*;

                    #selection_struct

                    pub fn #variant_ident($($($func_arg:$func_arg_ty),+)?) -> Selection {
                        #selection
                    }
                }
            };
            ({ $(#selection_pattern_produce)+ }) => {{
                #model_module::#variant_ident!(@definitions; ; $(#selection_pattern_consume)+);

                #selection_struct

                #selection
            }};
            (@definitions; $($module_name:ident)?; $(#selection_pattern_produce)+) => {
                #[allow(warnings)]
                enum Fields {
                    #(#fields_enum_variants),*
                }

                #[allow(warnings)]
                impl Fields {
                    fn selections() {
                        $(let _ = Fields::$field;)+
                    }
                }

                #data_struct

                impl ::serde::Serialize for Data {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: ::serde::Serializer,
                    {
                        #serialize_impl
                    }
                }

                impl<'de> ::serde::Deserialize<'de> for Data {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: ::serde::Deserializer<'de>,
                    {
                        #deserialize_impl
                    }
                }

                $($(pub mod $field {
                    #model_module::$selection_mode!(@field_module; $field #selections_pattern_consume);
                })?)+
            };

            #(#field_type_impls)*
            (@field_type; $field:ident $($tokens:tt)*) => { compile_error!(stringify!(Cannot include nonexistent relation $field on model #model_name_pascal_str, available relations are #all_fields_str)) };

            #(#field_module_impls)*
            (@field_module; $($tokens:tt)*) => {};

            #(#selection_field_to_selection_param_impls)*
            (@selection_field_to_selection_param; $($tokens:tt)*) => { compile_error!(stringify!($($tokens)*)) }; // ::prisma_client_rust::Selection::builder("").build() };

            (@selections_to_params; : $macro_name:ident {$(#selection_pattern_produce)+}) => {
                [ $(#module_path::#model_name_snake::$macro_name!(@selection_field_to_selection_param; #selection_pattern_consume),)+]
            };

            (@filters_to_args;) => {
                vec![]
            };
            (@filters_to_args; $($t:tt)*) => {
                $($t)*
            };

            #(#field_serde_names)*

            #specta_macro_arms
        }
        pub use #macro_name as #variant_ident;
    }
}

fn field_module_enum(
    field: FieldWalker,
    pcr: &TokenStream,
    variant: Variant,
) -> Option<TokenStream> {
    let field_name_pascal = pascal_ident(field.name());
    let field_name_str = field.name();

    let variant_pascal = pascal_ident(&variant.to_string());
    let variant_param = variant.param();

    Some(match field.refine() {
        RefinedFieldWalker::Relation(relation_field) => {
            let relation_model_name_snake = snake_ident(relation_field.related_model().name());

            let initial_nested_selections = match variant {
                Variant::Include => {
                    quote!(<#relation_model_name_snake::Types as #pcr::ModelTypes>::scalar_selections())
                }
                Variant::Select => quote!(vec![]),
            };

            match field.ast_field().arity {
                FieldArity::List => quote! {
                    pub enum #variant_pascal {
                        Select(#relation_model_name_snake::ManyArgs, Vec<#relation_model_name_snake::SelectParam>),
                        Include(#relation_model_name_snake::ManyArgs, Vec<#relation_model_name_snake::IncludeParam>),
                        Fetch(#relation_model_name_snake::ManyArgs)
                    }

                    impl Into<super::#variant_param> for #variant_pascal {
                        fn into(self) -> super::#variant_param {
                            super::#variant_param::#field_name_pascal(self)
                        }
                    }

                    impl #variant_pascal {
                        pub fn to_selection(self) -> #pcr::Selection {
                            let (args, selections) = match self {
                                Self::Select(args, selections) => (
                                    args.to_graphql().0,
                                    selections.into_iter().map(|s| s.to_selection()).collect()
                                ),
                                Self::Include(args, selections) => (
                                    args.to_graphql().0,
                                    {
                                        let mut nested_selections = #initial_nested_selections;
                                        nested_selections.extend(selections.into_iter().map(|s| s.to_selection()));
                                        nested_selections
                                    }
                                ),
                                Self::Fetch(args) => (
                                    args.to_graphql().0,
                                    <#relation_model_name_snake::Types as #pcr::ModelTypes>::scalar_selections()
                                )
                            };

                            #pcr::Selection::new(NAME, None, args, selections)
                        }

                        pub fn select(args: #relation_model_name_snake::ManyArgs, nested_selections: Vec<#relation_model_name_snake::SelectParam>) -> Self {
                            Self::Select(args, nested_selections)
                        }

                        pub fn include(args: #relation_model_name_snake::ManyArgs, nested_selections: Vec<#relation_model_name_snake::IncludeParam>) -> Self {
                            Self::Include(args, nested_selections)
                        }
                    }
                },
                _ => quote! {
                    pub enum #variant_pascal {
                        Select(Vec<#relation_model_name_snake::SelectParam>),
                        Include(Vec<#relation_model_name_snake::IncludeParam>),
                        Fetch
                    }

                    impl Into<super::#variant_param> for #variant_pascal {
                        fn into(self) -> super::#variant_param {
                            super::#variant_param::#field_name_pascal(self)
                        }
                    }

                    impl #variant_pascal {
                        pub fn to_selection(self) -> #pcr::Selection {
                            let selections = match self {
                                Self::Select(selections) => {
                                    selections.into_iter().map(|s| s.to_selection()).collect()
                                },
                                Self::Include(selections) => {
                                    let mut nested_selections = #initial_nested_selections;
                                    nested_selections.extend(selections.into_iter().map(|s| s.to_selection()));
                                    nested_selections
                                },
                                Self::Fetch => {
                                    <#relation_model_name_snake::Types as #pcr::ModelTypes>::scalar_selections()
                                }
                            };

                            #pcr::Selection::new(#field_name_str, None, [], selections)
                        }

                        pub fn select(nested_selections: Vec<#relation_model_name_snake::SelectParam>) -> Self {
                            Self::Select(nested_selections)
                        }

                        pub fn include(nested_selections: Vec<#relation_model_name_snake::IncludeParam>) -> Self {
                            Self::Include(nested_selections)
                        }
                    }
                },
            }
        }
        RefinedFieldWalker::Scalar(_) => quote! {
            pub struct #variant_pascal;

            impl Into<super::#variant_param> for #variant_pascal {
                fn into(self) -> super::#variant_param {
                    super::#variant_param::#field_name_pascal(self)
                }
            }

            impl #variant_pascal {
                pub fn to_selection(self) -> #pcr::Selection {
                    #pcr::sel(NAME)
                }
            }
        },
    })
}

fn model_module_enum(model: ModelWalker, pcr: &TokenStream, variant: Variant) -> TokenStream {
    let variant_pascal = pascal_ident(&variant.to_string());

    let variants = model
        .fields()
        .filter(|f| !f.ast_field().field_type.as_unsupported().is_some())
        .map(|field| {
            let field_name_snake = snake_ident(field.name());
            let field_name_pascal = pascal_ident(field.name());

            quote!(#field_name_pascal(#field_name_snake::#variant_pascal))
        });

    let field_names_pascal = model
        .fields()
        .filter(|f| !f.ast_field().field_type.as_unsupported().is_some())
        .map(|field| pascal_ident(field.name()));

    let variant_param = variant.param();

    quote! {
        pub enum #variant_param {
            #(#variants),*
        }

        impl #variant_param {
            pub fn to_selection(self) -> #pcr::Selection {
                match self {
                    #(Self::#field_names_pascal(data) => data.to_selection()),*
                }
            }
        }
    }
}

pub mod include {
    use prisma_client_rust_sdk::prisma::prisma_models::walkers::{
        FieldWalker, ModelWalker, RefinedFieldWalker,
    };

    use super::*;

    pub fn model_macro(model: ModelWalker, module_path: &TokenStream) -> TokenStream {
        super::model_macro(
            model,
            module_path,
            Variant::Include,
            model
                .scalar_fields()
                .filter(|f| !f.scalar_field_type().is_unsupported())
                .collect::<Vec<_>>()
                .into_iter(),
            model
                .fields()
                .filter(|f| matches!(f.refine(), RefinedFieldWalker::Relation(_))),
        )
    }

    pub fn field_module_enum(field: FieldWalker, pcr: &TokenStream) -> Option<TokenStream> {
        super::field_module_enum(field, pcr, Variant::Include)
    }

    pub fn model_module_enum(model: ModelWalker, pcr: &TokenStream) -> TokenStream {
        super::model_module_enum(model, pcr, Variant::Include)
    }
}

pub mod select {
    use prisma_client_rust_sdk::prisma::prisma_models::walkers::{FieldWalker, ModelWalker};

    use super::*;

    pub fn model_macro(model: ModelWalker, module_path: &TokenStream) -> TokenStream {
        super::model_macro(
            model,
            module_path,
            Variant::Select,
            vec![].into_iter(),
            model
                .fields()
                .filter(|f| f.ast_field().field_type.as_unsupported().is_none()),
        )
    }

    pub fn field_module_enum(field: FieldWalker, pcr: &TokenStream) -> Option<TokenStream> {
        super::field_module_enum(field, pcr, Variant::Select)
    }

    pub fn model_module_enum(model: ModelWalker, pcr: &TokenStream) -> TokenStream {
        super::model_module_enum(model, pcr, Variant::Select)
    }
}
