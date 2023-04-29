use super::prelude::*;

pub fn generate_module(args: &GenerateArgs) -> TokenStream {
    let read_filters = args.read_filters.iter().map(|filter| {
        let name = format_ident!("{}Filter", &filter.name);

        let (method_variants, method_matches): (Vec<_>, Vec<_>) = filter
            .methods
            .iter()
            .map(|method| {
                let variant_name = pascal_ident(&method.name);
                let method_action_string = &method.action;

                let value_ident = format_ident!("value");

                let value_as_prisma_value = method
                    .base_type
                    .to_prisma_value(&value_ident, &method.arity());

                let typ = method
                    .type_tokens(&quote!(super::super), &args.schema.db)
                    .unwrap();

                (
                    quote!(#variant_name(#typ)),
                    quote! {
                        Self::#variant_name(#value_ident) =>
                            ::prisma_client_rust::SerializedWhereValue::Object(
                            vec![(
                                #method_action_string.to_string(),
                                #value_as_prisma_value
                            )
                        ])
                    },
                )
            })
            .unzip();

        quote! {
            #[derive(Clone)]
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
            #(#read_filters)*
        }
    }
}
