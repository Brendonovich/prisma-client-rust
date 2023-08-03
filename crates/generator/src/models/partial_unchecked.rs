use prisma_client_rust_sdk::prisma::prisma_models::walkers::ModelWalker;

use crate::prelude::*;

pub fn r#macro(model: ModelWalker, module_path: &TokenStream) -> TokenStream {
    let model_name_snake = snake_ident(model.name());
    let model_name_snake_raw = snake_ident_raw(model.name());
    let macro_name = format_ident!("_partial_unchecked_{model_name_snake_raw}");

    let model_module = quote!(#module_path #model_name_snake);

    let struct_fields = model.scalar_fields().map(|scalar_field| {
        let field_name_str = scalar_field.name();
        let field_name_snake = snake_ident(field_name_str);

        let arity = scalar_field.ast_field().arity;

        let double_option_attrs = arity.is_optional().then(|| {
            quote! {
                #[serde(default, with = "::prisma_client_rust::serde::double_option")]
            }
        });

        quote! {
            #[serde(rename = #field_name_str)]
            #double_option_attrs
            pub #field_name_snake: #module_path #model_name_snake::#field_name_snake::Type
        }
    });

    quote! {
        ::prisma_client_rust::macros::partial_unchecked_factory!(
            #macro_name,
            #model_module,
            struct Data {
                #(#struct_fields),*
            }
         );
    }
}
