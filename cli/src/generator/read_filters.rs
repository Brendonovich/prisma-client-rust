use super::prelude::*;

pub fn generate_module(args: &GenerateArgs) -> TokenStream {
    let read_filters = args.read_filters.iter().map(|filter| {
        let name = format_ident!("{}Filter", &filter.name);

        let method_tokens = filter.methods.iter().map(|method| {
            let variant_name = format_ident!("{}", method.name.to_case(Case::Pascal));
            let method_action_string = &method.action;

            let value_as_prisma_value = method
                .base_type
                .to_prisma_value(&format_ident!("value"), method.is_list);
            let typ = method.type_tokens();

            (
                quote!(#variant_name(#typ)),
                quote! {
                    Self::#variant_name(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![
                        (#method_action_string.to_string(), #value_as_prisma_value)
                    ])
                },
            )
        });

        let method_variants = method_tokens.clone().map(|(v, _)| v);
        let method_matches = method_tokens.clone().map(|(_, m)| m);

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
            use super::*;

            #(#read_filters)*
        }
    }
}
