use super::prelude::*;

pub fn enum_name(filter: &Filter) -> Ident {
    format_ident!("{}Param", &filter.name)
}

pub fn generate_module(args: &GenerateArgs) -> TokenStream {
    let write_params = args.write_params.iter().map(|write_param| {
        let name = enum_name(write_param);

        let (method_variants, method_matches): (Vec<_>, Vec<_>) = write_param
            .fields
            .iter()
            .flat_map(|field| {
                let typ = field.type_tokens(&quote!(super::super::));
                let action = &field.name;

                let prisma_value_converter = field.to_prisma_value(&format_ident!("value"));

                let method_name_pascal = pascal_ident(&field.name);

                // Set doesn't use 'set' as the action name.
                let prisma_value = if action == "set" {
                    prisma_value_converter
                } else {
                    quote! {
                        ::prisma_client_rust::PrismaValue::Object(vec![(
                            #action.to_string(),
                            #prisma_value_converter
                        )])
                    }
                };

                Some((
                    quote!(#method_name_pascal(#typ)),
                    quote!(Self::#method_name_pascal(value) => #prisma_value),
                ))
            })
            .unzip();

        quote! {
            #[derive(Debug, Clone)]
            pub enum #name {
                #(#method_variants),*
            }

            impl Into<::prisma_client_rust::PrismaValue> for #name {
                fn into(self) -> ::prisma_client_rust::PrismaValue {
                    match self {
                        #(#method_matches),*
                    }
                }
            }
        }
    });

    quote! {
        pub mod write_params {
            #(#write_params)*
        }
    }
}
