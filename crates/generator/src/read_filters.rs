use super::prelude::*;

pub fn generate_module(args: &GenerateArgs) -> TokenStream {
    let read_filters = args.read_filters.iter().map(|filter| {
        let name = format_ident!("{}Filter", &filter.name);

        let (method_variants, method_matches): (Vec<_>, Vec<_>) = filter
            .fields
            .iter()
            .flat_map(|field| {
                let action_str = &field.name;
                let action_sanitised_str = match field.name.as_str() {
                    "in" => "inVec",
                    "notIn" => "notInVec",
                    n => n,
                };

                let variant_name = pascal_ident(&action_sanitised_str);

                let value_ident = format_ident!("value");

                let value_as_prisma_value = field.to_prisma_value(&value_ident);

                let typ = field.type_tokens(&quote!(super::super::));

                // https://github.com/Brendonovich/prisma-client-rust/issues/297
                if filter.name == "JsonNullable" && field.name == "equals" {
                    Some((
                        quote!(#variant_name(Option<#typ>)),
                        quote! {
                            Self::#variant_name(#value_ident) =>
                                ::prisma_client_rust::SerializedWhereValue::Object(
                                    vec![(
                                        #action_str.to_string(),
                                        #value_ident.map(|#value_ident| #value_as_prisma_value)
                                            .unwrap_or(::prisma_client_rust::PrismaValue::Null)
                                    )]
                                )
                        },
                    ))
                } else {
                    Some((
                        quote!(#variant_name(#typ)),
                        quote! {
                            Self::#variant_name(#value_ident) =>
                                ::prisma_client_rust::SerializedWhereValue::Object(
                                    vec![(
                                        #action_str.to_string(),
                                        #value_as_prisma_value
                                    )]
                                )
                        },
                    ))
                }
            })
            .unzip();

        quote! {
            #[derive(Debug, Clone)]
            pub enum #name {
                #(#method_variants),*
            }

            impl Into<::prisma_client_rust::SerializedWhereValue> for #name {
                fn into(self) -> ::prisma_client_rust::SerializedWhereValue {
                    match self {
                        #(#method_matches),*
                    }
                }
            }
        }
    });

    quote! {
        pub mod read_filters {
            use super::*;

            #(#read_filters)*
        }
    }
}
