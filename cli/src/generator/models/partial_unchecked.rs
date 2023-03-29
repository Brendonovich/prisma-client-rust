use crate::generator::prelude::*;

pub fn model_macro<'a>(model: &'a dml::Model, module_path: &TokenStream) -> TokenStream {
    let model_name_snake = snake_ident(&model.name);
    let model_name_snake_raw = snake_ident_raw(&model.name);
    let macro_name = format_ident!("_partial_unchecked_{model_name_snake_raw}");

    let model_module = quote!(#module_path::#model_name_snake);

    let struct_fields = model.scalar_fields().map(|scalar_field| {
        let field_name_str = &scalar_field.name;
        let field_name_snake = snake_ident(&scalar_field.name);
        let field_type = scalar_field
            .field_type
            .to_tokens(module_path, &scalar_field.arity);

        let double_option_attrs = scalar_field.arity.is_optional().then(|| {
            quote! {
                #[serde(default, with = "::prisma_client_rust::serde::double_option")]
            }
        });

        quote! {
            #[serde(rename = #field_name_str)]
            #double_option_attrs
            #field_name_snake: #field_type
        }
    });

    quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ($struct_name:ident {
                $($scalar_field:ident)+
            }) => {
                ::prisma_client_rust::macros::partial_unchecked! {
                    #model_module
                    struct $struct_name {
                        #(#struct_fields),*
                    }
                    [$($scalar_field),+]
                }
            };
        }

        pub use #macro_name as partial_unchecked;
    }
}
