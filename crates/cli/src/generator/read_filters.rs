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

                let pv = match action_str {
                    // "equals" => quote! {
                    //     ::prisma_client_rust::SerializedWhereValue::Value(
                    //          #value_as_prisma_value
                    //     )
                    // },
                    _ => quote! {
                        ::prisma_client_rust::SerializedWhereValue::Object(
                            vec![(
                                #action_str.to_string(),
                                #value_as_prisma_value
                            )]
                        )
                    },
                };

                Some((
                    quote!(#variant_name(#typ)),
                    quote! {
                        Self::#variant_name(#value_ident) => #pv
                    },
                ))
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
