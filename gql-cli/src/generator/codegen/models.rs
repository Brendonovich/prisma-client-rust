use convert_case::{Case, Casing};
use datamodel::dml::Field;
use quote::{__private::TokenStream, format_ident, quote};
use syn::Ident;

use crate::generator::{GeneratorArgs, Root};

pub fn generate(args: &GeneratorArgs) -> Vec<TokenStream> {
    let client_prefix_ident: TokenStream = args.client_module_prefix.parse().unwrap();

    args.dml
        .models
        .iter()
        .map(|model| {
            let model_name_string = &model.name;
            let model_name_snake = format_ident!("{}", model.name.to_case(Case::Snake));
            let model_name_pascal_string = model.name.to_case(Case::Pascal);

            let data_struct = quote!(#client_prefix_ident::#model_name_snake::Data);

            let field_structs = model.fields.iter().filter_map(|field| -> Option<_> {
                match field {
                    Field::ScalarField(field) => {
                        let field_name_snake_string = field.name.to_case(Case::Snake);
                        let field_name_pascal = format_ident!("{}", field.name.to_case(Case::Pascal));
                        let field_name_static = format_ident!("{}_FIELD_NAME", field.name.to_case(Case::UpperSnake));

                        Some(quote! {
                            pub static #field_name_static: &'static str = #field_name_snake_string;

                            pub struct #field_name_pascal;

                            impl BackingModelField for #field_name_pascal {
                                type BackingModel = #data_struct;
                                type Type = String;

                                fn name() -> &'static str {
                                    #field_name_static
                                }

                                fn cell() -> FieldDef {
                                    static CELL: FieldDef = OnceCell::new();
                                    &CELL
                                }
                            }
                        })
                    }
                    // Field::RelationField(field) => Some(())
                    _ => None,
                }
            }).collect::<Vec<_>>();

            let expose_resolvers = model.fields.iter().filter_map(|field| match field {
                Field::ScalarField(field) => {
                    let field_name_pascal = format_ident!("{}", field.name.to_case(Case::Pascal));
                    let field_name_snake = format_ident!("{}", field.name.to_case(Case::Snake));
                    
                    Some(quote! {
                        let field_def = <#field_name_pascal as BackingModelField>::def();
                        
                        if def.exposed_fields.contains(field_def.name) {
                            object.push((field_def.name.to_string(), self.#field_name_snake.into()));
                        }
                    })
                },
                _ => None
            }).collect::<Vec<_>>();

            let cell_type =
                quote!(&'static OnceCell<SchemaObject<#data_struct>>);

            quote! {
                mod #model_name_snake {
                    impl BackingModel for #data_struct {
                        fn name() -> &'static str {
                            #model_name_pascal_string
                        }
                        fn cell() -> #cell_type {
                            static CELL: #cell_type = OnceCell::new();
                            &CELL
                        }
                    }

                    impl Resolvable for #data_struct {
                        type T = Self;

                        fn resolve(self) -> ResolvableResult<Self> {
                            
                            ResolvableResult::Future((Self::def().resolve)(self))
                        }

                        fn resolve_exposed(self, object: &mut Vec<(String, Value)>, selected: &[String]) {
                            let def = <#data_struct as BackingModel>::def();

                            #(#expose_resolvers)*
                        }
                    }

                    #(#field_structs)*
                }
            }
        })
        .collect()
}
