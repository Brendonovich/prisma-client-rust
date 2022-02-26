use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};

use crate::generator::dmmf::{Document, Method, Model, Type};

pub fn generate_queries(models: &Vec<Model>) -> TokenStream {
    models.iter().map(|model| generate_query(model)).collect()
}

fn generate_query(model: &Model) -> TokenStream {
    let name_pascal = format_ident!("{}", model.name.to_case(Case::Pascal));
    let model_where_param = format_ident!("{}WhereParam", name_pascal);

    let model_field_structs = model
        .fields
        .iter()
        .map(|field| {
            let field_struct_name =
                format_ident!("{}{}", name_pascal, field.name.to_case(Case::Pascal));
            let field_type = format_ident!("{}", field.field_type.value());
            let field_name_pascal = field.name.to_case(Case::Pascal);

            let field_struct_fns = if field.kind.is_relation() {
                let methods = if field.is_list {
                    vec![
                        Method {
                            name: "Some".into(),
                            action: "some".into(),
                        },
                        Method {
                            name: "Every".into(),
                            action: "every".into(),
                        },
                    ]
                } else {
                    vec![Method {
                        name: "Where".into(),
                        action: "is".into(),
                    }]
                };

                methods
                    .iter()
                    .map(|m| {
                        let variant_name = format_ident!(
                            "{}{}",
                            field_name_pascal,
                            m.action.to_case(Case::Pascal)
                        );
                        let method_name = format_ident!("{}", m.action);
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
                            m.action.to_case(Case::Pascal)
                        );
                        let method_name = format_ident!("{}", m.action);

                        quote! {
                            pub fn #method_name(&self, value: #field_type) -> #model_where_param {
                                #model_where_param::#variant_name(value)
                            }
                        }
                    })
                    .collect::<Vec<_>>()
            };

            quote! {
                pub struct #field_struct_name {}

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

        impl #name_pascal {
            #(#model_query_struct_methods)*
            #(#model_operator_struct_methods)*
        }

        #(#model_field_structs)*
    }
}
