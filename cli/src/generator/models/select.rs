use crate::generator::prelude::*;

pub fn generate_macro(model: &dml::Model, module_path: &TokenStream) -> TokenStream {
    let model_name_snake = format_ident!("{}", model.name.to_case(Case::Snake));
    let macro_name = format_ident!("_select_{}", model_name_snake);
    
    let filters_pattern_produce = quote!(($($filters:tt)+)$(.$arg:ident($($arg_params:tt)*))*);
    let filters_pattern_consume = quote!(($($filters)+)$(.$arg($($arg_params)*))*);
    
    let selections_pattern_produce = quote!({$($selections:tt)+});
    let selections_pattern_consume = quote!({$($selections)+});
    
    let selection_pattern_produce = quote!($field:ident $(#filters_pattern_produce)? $(#selections_pattern_produce)?);
    let selection_pattern_consume = quote!($field $(#filters_pattern_consume)? $(#selections_pattern_consume)?);
    
    let field_type_impls = model.fields.iter().map(|field| {
        let field_name_snake = format_ident!("{}", field.name().to_case(Case::Snake));
        let field_type = field.field_type().to_tokens();
        let field_type = match field.field_type() {
            dml::FieldType::Relation(_) => quote!(crate::prisma::#field_type),
            _ => field_type
        };
        let field_type = match field.arity() {
            dml::FieldArity::Required => field_type,
            dml::FieldArity::Optional => quote!(Option<#field_type>),
            dml::FieldArity::List => quote!(Vec<#field_type>),
        };

        let selection_type_impl = field.as_relation_field().map(|_| {
            let field_type = quote!(#field_name_snake::Data);
            let field_type = match field.arity() {
                dml::FieldArity::Required => field_type,
                dml::FieldArity::Optional => quote!(Option<#field_type>),
                dml::FieldArity::List => quote!(Vec<#field_type>),
            };
            
            quote!((@field_type; #field_name_snake #selections_pattern_produce) => { #field_type };)
        });
        
        quote! {
            #selection_type_impl
            (@field_type; #field_name_snake) => { #field_type };
        }
    });
    
    let field_module_impls = model.fields.iter().filter_map(|f| f.as_relation_field()).map(|field| {
        let field_name_snake = format_ident!("{}", field.name.to_case(Case::Snake));
        let relation_model_name_snake = format_ident!("{}", field.relation_info.to.to_case(Case::Snake));
        
        quote! {
            (@field_module; #field_name_snake #selections_pattern_produce) => {
                $crate::#module_path::#relation_model_name_snake::select!(@definitions; $($selections)+);
            };
        }
    });
    
    let select_field_to_selection_impls = model.fields.iter().map(|field| {
        let field_string = field.name();
        let field_name_snake = format_ident!("{}", field.name().to_case(Case::Snake));
        
        match field.field_type() {
            dml::FieldType::Relation(relation) =>{
                let relation_model_name_snake = format_ident!("{}", relation.to.to_case(Case::Snake));
                
                quote! {
                    (@select_field_to_selection; #field_name_snake $(#filters_pattern_produce)? #selections_pattern_produce) => {{
                        #[allow(warnings)]
                        let mut selection = ::prisma_client_rust::query_core::Selection::builder(#field_string);
                        $(
                            let args = $crate::#module_path::#relation_model_name_snake::ManyArgs::new #filters_pattern_consume;
                            selection.set_arguments(args.to_graphql().0);
                        )?
                        selection.nested_selections($crate::#module_path::#relation_model_name_snake::select!(
                            @select_fields_to_selections;
                            $($selections)+
                        ));
                        selection.build()
                    }};
                    (@select_field_to_selection; #field_name_snake $(#filters_pattern_produce)?) => {{
                        #[allow(warnings)]
                        let mut selection = ::prisma_client_rust::query_core::Selection::builder(#field_string);
                        $(
                            let args = $crate::#module_path::#relation_model_name_snake::ManyArgs::new #filters_pattern_consume;
                            selection.set_arguments(args.to_graphql().0);
                        )?
                        selection.nested_selections($crate::#module_path::#relation_model_name_snake::_outputs());
                        selection.build()
                    }};
                }
            },
            _ => quote! {
                (@select_field_to_selection; #field_name_snake) => {
                    ::prisma_client_rust::query_core::Selection::builder(#field_string).build()
                };
            }
        }
    });
    
    let fields_enum_variants = model.fields.iter().map(|f| {
        let i = format_ident!("{}", f.name().to_case(Case::Snake));
        quote!(#i)
    });

    let specta_derive = cfg!(feature = "specta").then_some(quote! {
        #[derive(prisma_client_rust::specta::Type)]
        #[specta(crate = "prisma_client_rust::specta", inline)]
    });
    
    quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ($(#selection_pattern_produce)+) => {{
                $crate::#module_path::#model_name_snake::select!(@definitions; $(#selection_pattern_consume)+);
                
                Select($crate::#module_path::#model_name_snake::select!(@select_fields_to_selections; $(#selection_pattern_consume)+))
            }};
            (@definitions; $(#selection_pattern_produce)+) => {
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
                
                #[derive(::serde::Deserialize)]
                #specta_derive
                #[allow(warnings)]
                pub struct Data {
                    $($field: $crate::#module_path::#model_name_snake::select!(@field_type; $field $(#selections_pattern_consume)?),)+
                }

                $($(pub mod $field {
                    $crate::#module_path::#model_name_snake::select!(@field_module; $field #selections_pattern_consume);
                })?)+

                pub struct Select(pub Vec<::prisma_client_rust::query_core::Selection>);

                impl ::prisma_client_rust::select::SelectType<$crate::#module_path::#model_name_snake::Data> for Select {
                    type Data = Data;
                    
                    fn to_selections(self) -> Vec<::prisma_client_rust::query_core::Selection> {
                        self.0
                    }
                }
            };
            
            #(#field_type_impls)*
            (@field_type; $field:ident $($tokens:tt)*) => { compile_error!(stringify!(Cannot select field nonexistent field $field on model #model_name_snake)) };
            
            #(#field_module_impls)*
            (@field_module; $($tokens:tt)*) => {};
            
            (@select_fields_to_selections; $(#selection_pattern_produce)+) => {
                vec![$($crate::#module_path::#model_name_snake::select!(
                    @select_field_to_selection;
                    #selection_pattern_consume
                )),+]
            };
            
            #(#select_field_to_selection_impls)*
            (@select_field_to_selection; $($tokens:tt)*) => { ::prisma_client_rust::query_core::Selection::builder("").build() };
        }
        pub use #macro_name as select;
    }
}
