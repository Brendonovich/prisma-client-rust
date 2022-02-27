use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};

use crate::generator::dmmf::{Document, Method, Model, Type};

pub fn generate_queries(models: &Vec<Model>) -> TokenStream {
    models.iter().map(|model| generate_query(model)).collect()
}

fn generate_query(model: &Model) -> TokenStream {
    let name_pascal = format_ident!("{}", model.name.to_case(Case::Pascal));
    let model_where_param = format_ident!("{}WhereParam", name_pascal);
   
    let model_set_enum_ident = format_ident!("{}SetParam", name_pascal);
    let model_set_enum_variants = { 
        let model_set_enum = model.fields.iter().map(|field| {
            let field_ident = format_ident!("{}", field.name.to_case(Case::Pascal));
            let field_type_ident = format_ident!("{}", field.field_type.value());
            if field.kind.include_in_struct() {
                quote! { #field_ident(#field_type_ident) }
            } else {
                let relation_where_param = format_ident!("{}WhereParam", field_type_ident);
                if field.is_list {
                    quote! { #field_ident(Vec<#relation_where_param>)}
                } else {
                    quote! { #field_ident(#relation_where_param)}
                }
            }
        }).collect::<Vec<_>>();
        
        let model_set_enum_match_arms = model.fields.iter().map(|field| {
            let field_ident = format_ident!("{}", field.name.to_case(Case::Pascal));
            let field_name = field.name.to_case(Case::Snake);
            
            if field.kind.include_in_struct() {
                quote! { 
                    Self::#field_ident(value) => Field { 
                        name: #field_name.into(),
                        value: Some(value.into()),
                        ..Default::default()
                    }
                }
            } else {
                if field.is_list {
                    quote! {
                        Self::#field_ident(value) => Field {
                            name: #field_name.into(),
                            fields: Some(vec![
                                Field {
                                    name: "connect".into(),
                                    fields: Some(builder::transform_equals(
                                        value.into_iter().map(|item| item.field()).collect()
                                    )),
                                    list: true,
                                    wrap_list: true,
                                    ..Default::default()
                                }
                            ]),
                            ..Default::default()
                        }
                    }
                } else {
                    quote! {
                        Self::#field_ident(value) => Field {
                            name: #field_name.into(),
                            fields: Some(vec![
                                Field {
                                    name: "connect".into(),
                                    fields: Some(builder::transform_equals(vec![
                                        value.field()
                                    ])),
                                    ..Default::default()
                                }
                            ]),
                            ..Default::default()
                        }
                    }
                }
            }
        }).collect::<Vec<_>>();

        quote! {
            pub enum #model_set_enum_ident {
                #(#model_set_enum),*
            }
            
            impl #model_set_enum_ident {
                pub fn field(self) -> Field {
                    match self {
                        #(#model_set_enum_match_arms),*
                    }
                }
            }
        }
    };
    
    let model_field_structs = model
        .fields
        .iter()
        .map(|field| { 
            let field_struct_name =
                format_ident!("{}{}", name_pascal, field.name.to_case(Case::Pascal));
            let field_type = format_ident!("{}", field.field_type.value());
            let field_name_pascal = field.name.to_case(Case::Pascal);
            let field_ident_pascal = format_ident!("{}", &field_name_pascal);

            let mut field_struct_fns = if field.kind.is_relation() {
                let methods = field.relation_methods();

                methods
                    .iter()
                    .map(|m| {
                        let variant_name = format_ident!(
                            "{}{}",
                            field_name_pascal,
                            m.name.to_case(Case::Pascal)
                        );
                        let method_name = format_ident!("{}", m.name.to_case(Case::Snake));
                        let value_type = format_ident!("{}WhereParam", field_type);

                        quote! {
                            pub fn #method_name(&self, value: Vec<#value_type>) -> #model_where_param {
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
                            field_name_pascal,
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
             
            let field_set_struct_ident = format_ident!("{}Set{}", name_pascal, field.name.to_case(Case::Pascal));

            let field_set_struct = if field.kind.is_relation() {
                let relation_where_param = format_ident!("{}WhereParam", field_type);
                
                if field.is_list {
                    field_struct_fns.push(quote! {
                        pub fn link<T: From<#field_set_struct_ident>>(&self, value: Vec<#relation_where_param>) -> T {
                            #field_set_struct_ident(value).into()
                        }
                    });
                    
                    Some(quote! {
                        pub struct #field_set_struct_ident(Vec<#relation_where_param>);
                        
                        impl From<#field_set_struct_ident> for #model_set_enum_ident {
                            fn from(value: #field_set_struct_ident) -> Self {
                                Self::#field_ident_pascal(value.0.into_iter().map(|v| v.into()).collect())
                            }
                        }
                    })
                } else {
                    field_struct_fns.push(quote! {
                        pub fn link<T: From<#field_set_struct_ident>>(&self, value: #relation_where_param) -> T {
                            #field_set_struct_ident(value).into()
                        }
                    });
                    
                    Some(quote! {
                        pub struct #field_set_struct_ident(#relation_where_param);
                        
                        impl From<#field_set_struct_ident> for #model_set_enum_ident {
                            fn from(value: #field_set_struct_ident) -> Self {
                                Self::#field_ident_pascal(value.0)
                            }
                        }
                    })
                }
            } else  { 
                field_struct_fns.push(quote! {
                    pub fn set<T: From<#field_set_struct_ident>>(&self, value: #field_type) -> T {
                        #field_set_struct_ident(value).into()
                    }
                });
                
                Some(quote! { 
                    pub struct #field_set_struct_ident(#field_type);
                
                    impl From<#field_set_struct_ident> for #model_set_enum_ident {
                        fn from(value: #field_set_struct_ident) -> Self {
                            Self::#field_ident_pascal(value.0)
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
        .collect::<Vec<_>>();

    let model_query_struct_methods = model
        .fields
        .iter()
        .map(|field| {
            let field_method_name = format_ident!("{}", field.name.to_case(Case::Snake));
            let field_struct_name = format_ident!(
                "{}{}",
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

    let model_operator_struct_methods = Document::operators()
        .iter()
        .map(|op| {
            let field_method_name = format_ident!("{}", op.name.to_case(Case::Snake));
            let op_name_ident = format_ident!("{}", op.name.to_case(Case::Pascal));

            quote! {
                pub fn #field_method_name(params: Vec<#model_where_param>) -> #model_where_param {
                    #model_where_param::#op_name_ident(params)
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        pub struct #name_pascal {}
        
        #model_set_enum_variants

        impl #name_pascal {
            #(#model_query_struct_methods)*
            #(#model_operator_struct_methods)*
        }

        #(#model_field_structs)*
    }
}
