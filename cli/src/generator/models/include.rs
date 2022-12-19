use crate::generator::prelude::*;

pub fn generate_macro(model: &dml::Model, module_path: &TokenStream) -> TokenStream {
    let model_name_pascal_str = model.name.to_case(Case::Pascal);
    let model_name_snake = snake_ident(&model.name);
    let macro_name = format_ident!("_include_{}", model_name_snake);
    
    let filters_pattern_produce = quote!(($($filters:tt)+)$(.$arg:ident($($arg_params:tt)*))*);
    let filters_pattern_consume = quote!(($($filters)+)$(.$arg($($arg_params)*))*);
    
    let selections_pattern_produce = quote!(: $selection_mode:ident {$($selections:tt)+});
    let selections_pattern_consume = quote!(: $selection_mode {$($selections)+});
    
    let selection_pattern_produce = quote!($field:ident $(#filters_pattern_produce)? $(#selections_pattern_produce)?);
    let selection_pattern_consume = quote!($field $(#filters_pattern_consume)? $(#selections_pattern_consume)?);
    
    let field_type_impls = model.relation_fields().map(|field| {
        let field_name_snake = snake_ident(&field.name);
        let relation_model_name_snake = snake_ident(&field.relation_info.to);

        let field_type = quote!(crate::#module_path::#relation_model_name_snake::Data);
        let field_type_base = match &field.arity {
            dml::FieldArity::Required => field_type,
            dml::FieldArity::Optional => quote!(Option<#field_type>),
            dml::FieldArity::List => quote!(Vec<#field_type>),
        };
        
        let field_type = quote!(#field_name_snake::Data);
        let field_type_selected = match &field.arity {
            dml::FieldArity::Required => field_type,
            dml::FieldArity::Optional => quote!(Option<#field_type>),
            dml::FieldArity::List => quote!(Vec<#field_type>),
        };
        
        quote! {
            (@field_type; #field_name_snake #selections_pattern_produce) => { #field_type_selected };
            (@field_type; #field_name_snake) => { #field_type_base };
        }
    });
    
    let field_module_impls = model.relation_fields().map(|field| {
        let field_name_snake = snake_ident(&field.name);
        let relation_model_name_snake = snake_ident(&field.relation_info.to);
        
        quote! {
            (@field_module; #field_name_snake #selections_pattern_produce) => {
                $crate::#module_path::#relation_model_name_snake::include!(@definitions; ; $($selections)+);
            };
        }
    });
    
    let selection_field_to_selection_param_impls = model.relation_fields().map(|field| {
        let field_name_snake = snake_ident(&field.name);
        let relation_model_name_snake = snake_ident(&field.relation_info.to);

        match field.arity {
            dml::FieldArity::List => {
                quote! {
                    (@selection_field_to_selection_param; #field_name_snake $(#filters_pattern_produce)? #selections_pattern_produce) => {{
                        Into::<$crate::#module_path::#model_name_snake::IncludeParam>::into(
                            $crate::#module_path::#model_name_snake::#field_name_snake::Include::$selection_mode(
                                $crate::#module_path::#relation_model_name_snake::ManyArgs::new($crate::#module_path::#relation_model_name_snake::include!(
                                    @filters_to_args;
                                    $($($filters)+)?
                                )) $($(.$arg($($arg_params)*))*)?,
                                $crate::#module_path::#relation_model_name_snake::include!(
                                    @selections_to_include_params;
                                    #selections_pattern_consume
                                ).into_iter().collect()
                            )
                        )
                    }};
                    (@selection_field_to_selection_param; #field_name_snake $(#filters_pattern_produce)?) => {{
                        Into::<$crate::#module_path::#model_name_snake::IncludeParam>::into(
                            $crate::#module_path::#model_name_snake::#field_name_snake::Include::Fetch(
                                $crate::#module_path::#relation_model_name_snake::ManyArgs::new($crate::#module_path::#relation_model_name_snake::include!(
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
                    Into::<$crate::#module_path::#model_name_snake::IncludeParam>::into(
                        $crate::#module_path::#model_name_snake::#field_name_snake::Include::$selection_mode(
                            $crate::#module_path::#relation_model_name_snake::include!(
                                @selections_to_include_params;
                                #selections_pattern_consume
                            ).into_iter().collect()
                        )
                    )
                }};
                (@selection_field_to_selection_param; #field_name_snake $(#filters_pattern_produce)?) => {{
                    Into::<$crate::#module_path::#model_name_snake::IncludeParam>::into(
                        $crate::#module_path::#model_name_snake::#field_name_snake::Include::Fetch
                    )
                }};
            }
        }
    });

    let data_struct_scalar_fields = model.fields().filter_map(|f| {
        let field_name_snake = snake_ident(f.name());
        let field_type = f.type_tokens(quote!(crate::#module_path::));

        f.as_scalar_field().map(|_| {
            quote!(pub #field_name_snake: #field_type)
        })
    });

    let fields_enum_variants = model.relation_fields().map(|f| {
        let i = snake_ident(&f.name);
        quote!(#i)
    });

    let specta = quote!(prisma_client_rust::rspc::internal::specta);

    let specta_impl = cfg!(feature = "rspc").then(|| {
        let object_scalar_fields = model.fields().filter_map(|f| {
            let field_name_str = f.name();
            let field_type = f.type_tokens(quote!());

            f.as_scalar_field().map(|_| 
                quote!(#specta::ObjectField {
                    name: #field_name_str.to_string(),
                    optional: false,
                    ty: <#field_type as #specta::Type>::reference(
                        #specta::DefOpts {
                            parent_inline: false,
                            type_map: _opts.type_map
                        },
                        &[]
                    )
                },)
            )
        });

        quote! {
            impl #specta::Type for Data {
                const NAME: &'static str = $crate::#module_path::#model_name_snake::include!(@specta_type_name; $($module_name)?);

                fn inline(_opts: #specta::DefOpts, _: &[#specta::DataType]) -> #specta::DataType {
                    use ::prisma_client_rust::convert_case::Casing;

                    #specta::DataType::Object(#specta::ObjectType {
                        name: Self::NAME.to_case(::prisma_client_rust::convert_case::Case::Pascal),
                        tag: None,
                        generics: vec![],
                        fields: vec![#(#object_scalar_fields)* $(#specta::ObjectField {
                            name: $crate::#module_path::#model_name_snake::include!(@field_serde_name; $field).to_string(),
                            optional: false,
                            ty: <$crate::#module_path::#model_name_snake::include!(@field_type; $field $(#selections_pattern_consume)?) as #specta::Type>::reference(
                                #specta::DefOpts {
                                    parent_inline: false,
                                    type_map: _opts.type_map
                                },
                                &[]
                            )
                        }),*],
                        type_id: None
                    })
                }

                $crate::#module_path::#model_name_snake::include!(@specta_reference_body; $($module_name)?);
            }
        }
    });

    let specta_macro_arms = cfg!(feature = "rspc").then(|| {
        quote! {
            (@specta_reference_body; $name:ident) =>  {
                fn reference(opts: #specta::DefOpts, _: &[#specta::DataType]) -> #specta::DataType {
                    use ::prisma_client_rust::convert_case::Casing;

                    if !opts.type_map.contains_key(Self::NAME) {
                        Self::definition(#specta::DefOpts {
                            parent_inline: false,
                            type_map: opts.type_map
                        });
                    }

                    #specta::DataType::Reference {
                        name: Self::NAME.to_case(::prisma_client_rust::convert_case::Case::Pascal),
                        generics: vec![],
                        type_id: std::any::TypeId::of::<Self>()
                    }
                }

                fn definition(opts: #specta::DefOpts) -> #specta::DataType {
                    if !opts.type_map.contains_key(Self::NAME) {
                        opts.type_map.insert(Self::NAME, #specta::DataType::Object(#specta::ObjectType {
                            name: "PLACEHOLDER".to_string(),
                            generics: vec![],
                            fields: vec![],
                            tag: None,
                            type_id: Some(std::any::TypeId::of::<Self>())
                        }));

                        let def = Self::inline(#specta::DefOpts {
                            parent_inline: false,
                            type_map: opts.type_map
                        }, &[]);

                        opts.type_map.insert(Self::NAME, def.clone());
                    }

                    opts.type_map.get(Self::NAME).unwrap().clone()
                }
            };
            (@specta_reference_body;) => {
                fn reference(_opts: #specta::DefOpts, _: &[#specta::DataType]) -> #specta::DataType {
                    Self::inline(_opts, &[])
                }

                fn definition(_opts: #specta::DefOpts) -> #specta::DataType {
                    unreachable!()
                }
            };

            (@specta_type_name; $name:ident) => {
                stringify!($name)
            };
            (@specta_type_name;) => { "Data" };
        }
    });

    let all_fields_str = model.relation_fields().map(|f| f.name.to_case(Case::Snake)).collect::<Vec<_>>().join(", ");

    let field_serde_names = model.fields().map(|f| {
        let field_name_str = f.name();
        let field_name_snake = snake_ident(f.name());

        quote!((@field_serde_name; #field_name_snake) => { #field_name_str };)
    });

    let scalar_field_names_snake = model.scalar_fields().map(|f| snake_ident(&f.name)).collect::<Vec<_>>();

    let deserialize_impl = {
        let field_names_str = model.fields().map(|f| f.name());

        quote! {
            #[allow(warnings)]
            enum Field {
                $($field),+,
                #(#scalar_field_names_snake),*
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
                            formatter.write_str(concat!(
                                $($crate::#module_path::#model_name_snake::include!(@field_serde_name; $field), ", "),+,
                                #($crate::#module_path::#model_name_snake::include!(@field_serde_name; #scalar_field_names_snake), ", "),*
                            ))
                        }

                        fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: ::serde::de::Error,
                        {
                            match value {
                                $($crate::#module_path::#model_name_snake::include!(@field_serde_name; $field) => Ok(Field::$field)),*,
                                #($crate::#module_path::#model_name_snake::include!(@field_serde_name; #scalar_field_names_snake) => Ok(Field::#scalar_field_names_snake)),*,
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
                    #(let mut #scalar_field_names_snake = None;)*

                    while let Some(key) = map.next_key()? {
                        match key {
                            #(Field::#scalar_field_names_snake => {
                                if #scalar_field_names_snake.is_some() {
                                    return Err(::serde::de::Error::duplicate_field($crate::#module_path::#model_name_snake::include!(@field_serde_name; #scalar_field_names_snake)));
                                }
                                #scalar_field_names_snake = Some(map.next_value()?);
                            })*
                            $(Field::$field => {
                                if $field.is_some() {
                                    return Err(::serde::de::Error::duplicate_field($crate::#module_path::#model_name_snake::include!(@field_serde_name; $field)));
                                }
                                $field = Some(map.next_value()?);
                            })*
                        }
                    }
                    
                    $(let $field = $field.ok_or_else(|| serde::de::Error::missing_field($crate::#module_path::#model_name_snake::include!(@field_serde_name; $field)))?;)*
                    #(let #scalar_field_names_snake = #scalar_field_names_snake.ok_or_else(|| serde::de::Error::missing_field($crate::#module_path::#model_name_snake::include!(@field_serde_name; #scalar_field_names_snake)))?;)*

                    Ok(Data { #(#scalar_field_names_snake),*, $($field),* })
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
                    #(stringify!(#scalar_field_names_snake)),*
                ].len()
            )?;
            $(state.serialize_field($crate::#module_path::#model_name_snake::include!(@field_serde_name; $field), &self.$field)?;)*
            #(state.serialize_field($crate::#module_path::#model_name_snake::include!(@field_serde_name; #scalar_field_names_snake), &self.#scalar_field_names_snake)?;)*
            state.end()
        }
    };

    quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ($(($($func_arg:ident: $func_arg_ty:ty),+) =>)? $module_name:ident { $(#selection_pattern_produce)+ }) => {
                #[allow(warnings)]
                pub mod $module_name {
                    $crate::#module_path::#model_name_snake::include!(@definitions; $module_name; $(#selection_pattern_consume)+);

                    pub struct Include(Vec<::prisma_client_rust::Selection>);

                    impl ::prisma_client_rust::include::IncludeType for Include {
                        type Data = Data;
                        type ModelData = $crate::#module_path::#model_name_snake::Data;
                        
                        fn to_selections(self) -> Vec<::prisma_client_rust::Selection> {
                            self.0
                        }
                    }

                    use super::*;

                    pub fn include($($($func_arg:$func_arg_ty),+)?) -> Include {
                        let mut selections = <$crate::#module_path::#model_name_snake::Actions as ::prisma_client_rust::ModelActions>::scalar_selections();

                        selections.extend(
                            $crate::#module_path::#model_name_snake::include!(
                                @selections_to_include_params; : include
                                { $(#selection_pattern_consume)+ }
                            )
                            .into_iter()
                            .map(|p| p.to_selection())
                        );

                        Include(selections)
                    }
                }
            };
            ({ $(#selection_pattern_produce)+ }) => {{
                $crate::#module_path::#model_name_snake::include!(@definitions; ; $(#selection_pattern_consume)+);
                
                pub struct Include(Vec<::prisma_client_rust::Selection>);

                impl ::prisma_client_rust::include::IncludeType for Include {
                    type Data = Data;
                    type ModelData = $crate::#module_path::#model_name_snake::Data;
                    
                    fn to_selections(self) -> Vec<::prisma_client_rust::Selection> {
                        self.0
                    }
                }

                Include({
                    let mut selections = <$crate::#module_path::#model_name_snake::Actions as ::prisma_client_rust::ModelActions>::scalar_selections();

                    selections.extend(
                        $crate::#module_path::#model_name_snake::include!(
                            @selections_to_include_params; : include
                            { $(#selection_pattern_consume)+ }
                        )
                        .into_iter()
                        .map(|p| p.to_selection())
                    );

                    selections
                })
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

                #[allow(warnings)]
                #[derive(std::fmt::Debug, Clone)]
                pub struct Data {
                    #(#data_struct_scalar_fields,)*
                    $(pub $field: $crate::#module_path::#model_name_snake::include!(@field_type; $field $(#selections_pattern_consume)?),)+
                }

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

                #specta_impl

                $($(pub mod $field {
                    $crate::#module_path::#model_name_snake::$selection_mode!(@field_module; $field #selections_pattern_consume);
                })?)+
            };
            
            #(#field_type_impls)*
            (@field_type; $field:ident $($tokens:tt)*) => { compile_error!(stringify!(Cannot include nonexistent relation $field on model #model_name_pascal_str, available relations are #all_fields_str)) };
            
            #(#field_module_impls)*
            (@field_module; $($tokens:tt)*) => {};

            #(#selection_field_to_selection_param_impls)*
            (@selection_field_to_selection_param; $($tokens:tt)*) => { compile_error!(stringify!($($tokens)*)) }; // ::prisma_client_rust::Selection::builder("").build() };

            (@selections_to_include_params; : $macro_name:ident {$(#selection_pattern_produce)+}) => {
                [ $($crate::#module_path::#model_name_snake::$macro_name!(@selection_field_to_selection_param; #selection_pattern_consume),)+]
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
        pub use #macro_name as include;
    }
}

pub fn field_module_enum(field: &dml::Field, pcr: &TokenStream) -> TokenStream {
    let field_name_pascal = pascal_ident(field.name());
    let field_name_str = field.name();

    match field {
        dml::Field::RelationField(relation_field) => {
            let relation_model_name_snake = snake_ident(&relation_field.relation_info.to);

            match field.arity() {
                dml::FieldArity::List => quote! {
                    pub enum Include {
                        Select(#relation_model_name_snake::ManyArgs, Vec<#relation_model_name_snake::SelectParam>),
                        Include(#relation_model_name_snake::ManyArgs, Vec<#relation_model_name_snake::IncludeParam>),
                        Fetch(#relation_model_name_snake::ManyArgs)
                    }

                    impl Into<super::IncludeParam> for Include {
                        fn into(self) -> super::IncludeParam {
                            super::IncludeParam::#field_name_pascal(self)
                        }
                    }

                    impl Include {
                        pub fn to_selection(self) -> #pcr::Selection {
                            let mut selection = #pcr::Selection::builder(#field_name_str);

                            match self {
                                Self::Select(args, selections) => {
                                    selection.set_arguments(args.to_graphql().0);
                                    selection.nested_selections(selections.into_iter().map(|s| s.to_selection()).collect());
                                },
                                Self::Include(args, selections) => {
                                    selection.set_arguments(args.to_graphql().0);

                                    let mut nested_selections = <#relation_model_name_snake::Actions as #pcr::ModelActions>::scalar_selections();
                                    nested_selections.extend(selections.into_iter().map(|s| s.to_selection()));
                                    selection.nested_selections(nested_selections);
                                },
                                Self::Fetch(args) => {
                                    selection.set_arguments(args.to_graphql().0);
                                    selection.nested_selections(<#relation_model_name_snake::Actions as #pcr::ModelActions>::scalar_selections());
                                }
                            }

                            selection.build()
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
                    pub enum Include {
                        Select(Vec<#relation_model_name_snake::SelectParam>),
                        Include(Vec<#relation_model_name_snake::IncludeParam>),
                        Fetch
                    }

                    impl Into<super::IncludeParam> for Include {
                        fn into(self) -> super::IncludeParam {
                            super::IncludeParam::#field_name_pascal(self)
                        }
                    }

                    impl Include {
                        pub fn to_selection(self) -> #pcr::Selection {
                            let mut selection = #pcr::Selection::builder(#field_name_str);

                            match self {
                                Self::Select(selections) => {
                                    selection.nested_selections(selections.into_iter().map(|s| s.to_selection()).collect());
                                },
                                Self::Include(selections) => {
                                    let mut nested_selections = <#relation_model_name_snake::Actions as #pcr::ModelActions>::scalar_selections();
                                    nested_selections.extend(selections.into_iter().map(|s| s.to_selection()));
                                    selection.nested_selections(nested_selections);
                                },
                                Self::Fetch => {
                                    selection.nested_selections(<#relation_model_name_snake::Actions as #pcr::ModelActions>::scalar_selections());
                                }
                            }

                            selection.build()
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
        },
        dml::Field::ScalarField(_) => quote! {
            pub struct Include;

            impl Into<super::IncludeParam> for Include {
                fn into(self) -> super::IncludeParam {
                    super::IncludeParam::#field_name_pascal(self)
                }
            }

            impl Include {
                pub fn to_selection(self) -> #pcr::Selection {
                    #pcr::Selection::builder(#field_name_str).build()
                }
            }
        },
        dml::Field::CompositeField(_) => todo!()
    }
}

pub fn model_module_enum(model: &dml::Model, pcr: &TokenStream) -> TokenStream {
    let variants = model.fields().map(|field| {
        let field_name_snake = snake_ident(field.name());
        let field_name_pascal = pascal_ident(field.name());

        quote!(#field_name_pascal(#field_name_snake::Include))
    });

    let field_names_pascal = model.fields().map(|field| pascal_ident(field.name()));

    quote! {
        pub enum IncludeParam {
            #(#variants),*
        }

        impl IncludeParam {
            pub fn to_selection(self) -> #pcr::Selection {
                match self {
                    #(Self::#field_names_pascal(data) => data.to_selection()),*
                }
            }
        }
    }
}
