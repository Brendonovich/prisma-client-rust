use prisma_client_rust_sdk::prisma::prisma_models::walkers::CompositeTypeWalker;

use crate::prelude::*;

pub fn struct_definition(ty: CompositeTypeWalker) -> TokenStream {
    let fields = ty.fields().flat_map(|field| {
        let field_name_str = field.name();
        let field_name_snake = snake_ident(field.name());
        let field_ty = field.type_tokens(&quote!())?;

        Some(quote! {
            #[serde(rename = #field_name_str)]
            pub #field_name_snake: #field_ty
        })
    });

    let specta_derive = cfg!(feature = "specta").then(|| {
        let ty_name_pascal_str = pascal_ident(ty.name()).to_string();

        quote! {
            #[derive(::prisma_client_rust::specta::Type)]
            #[specta(rename = #ty_name_pascal_str, crate = prisma_client_rust::specta)]
        }
    });

    quote! {
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
        #specta_derive
        pub struct Data {
            #(#fields),*
        }
    }
}
