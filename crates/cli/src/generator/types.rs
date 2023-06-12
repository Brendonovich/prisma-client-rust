use std::collections::BTreeMap;

use prisma_client_rust_sdk::prisma::{dmmf::TypeLocation, prisma_models::FieldArity};

use crate::generator::prelude::*;

pub fn types(args: &GenerateArgs) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let input_object_types = args.dmmf.schema.input_object_types.get("prisma").unwrap();

    let types = input_object_types
        .iter()
        .filter(|t| {
            (t.name.contains("OrderBy")
                || t.name.ends_with("UniqueInput")
                || t.name.ends_with("WhereInput")
                || t.name.ends_with("Filter"))
                && !(t.name.ends_with("AggregatesFilter") || t.name.ends_with("AggregationFilter"))
        })
        .map(|t| {
            let is_struct = t.fields.iter().any(|f| f.is_required);
            let type_name = format_ident!("{}", &t.name);

            if is_struct {
                let ((field_names, field_types), pv_fields): ((Vec<_>, Vec<_>), Vec<_>) = t
                    .fields
                    .iter()
                    .map(|f| {
                        let field_name_snake = snake_ident(&f.name);
                        let field_name_str = &f.name;

                        let value_ident = format_ident!("value");
                        let pv = f.to_prisma_value(&value_ident, t, args);

                        (
                            (
                                field_name_snake.clone(),
                                f.type_tokens(&quote!(super::), t, args),
                            ),
                            (quote! {
                                (#field_name_str.to_string(), {
                                    let #value_ident = self.#field_name_snake;
                                    #pv
                                })
                            }),
                        )
                    })
                    .unzip();

                quote! {
                    #[derive(Clone)]
                    pub struct #type_name {
                        #(pub #field_names: #field_types),*
                    }

                    impl Into<#pcr::PrismaValue> for #type_name {
                        fn into(self) -> #pcr::PrismaValue {
                            #pcr::PrismaValue::Object(vec![
                                #(#pv_fields),*
                            ])
                        }
                    }
                }
            } else {
                let (variants, into_pv_arms): (Vec<_>, Vec<_>) = t
                    .fields
                    .iter()
                    .flat_map(|field| {
                        let field_name_str = &field.name;
                        let field_name_pascal = pascal_ident(&field.name);

                        let typ = field.type_tokens(&quote!(), t, args);

                        let value_ident = format_ident!("value");
                        let value = field.to_prisma_value(&value_ident, t, args);

                        Some((
                            field_name_str,
                            (
                                quote!(#field_name_pascal(#typ)),
                                quote! {
                                    Self::#field_name_pascal(#value_ident) => (
                                        #field_name_str,
                                        #value
                                    )
                                },
                            ),
                        ))
                    })
                    .collect::<BTreeMap<_, _>>()
                    .into_values()
                    .unzip();

                let nested_scalar_from_impl = args
                    .dmmf
                    .schema
                    .find_input_type(&format!("Nested{}", type_name))
                    .map(|typ| {
                        let name = format_ident!("{}", &typ.name);

                        let fields = typ.fields.iter().map(|f| {
                            let field_name_pascal = pascal_ident(&f.name);

                            quote!(#type_name::#field_name_pascal(v) => Self::#field_name_pascal(v))
                        });

                        quote! {
                            impl From<#type_name> for #name {
                                fn from(value: #type_name) -> Self {
                                    match value {
                                        #(#fields),*
                                    }
                                }
                            }
                        }
                    });

                quote! {
                    #[derive(Clone)]
                    pub enum #type_name {
                        #(#variants),*
                    }

                    impl Into<(String, #pcr::PrismaValue)> for #type_name {
                        fn into(self) -> (String, #pcr::PrismaValue) {
                            let (k, v) = match self {
                                #(#into_pv_arms),*
                            };

                            (k.to_string(), v)
                        }
                    }

                    #nested_scalar_from_impl
                }
            }
        });

    quote! {
        #(#types)*
    }
}
