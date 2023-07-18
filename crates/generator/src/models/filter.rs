use prisma_client_rust_sdk::prisma::prisma_models::walkers::{ModelWalker, RefinedFieldWalker};

use crate::prelude::*;

pub fn r#macro(model: ModelWalker, module_path: &TokenStream) -> TokenStream {
    let model_name_snake = snake_ident(model.name());

    let name = format_ident!("_{}_filter", model.name().to_case(Case::Snake, true));

    let fields = model.fields().map(|field| {
        let field_name_snake = snake_ident(field.name());

        let variant = match field.refine() {
            RefinedFieldWalker::Scalar(_) => quote!(Scalar),
            RefinedFieldWalker::Relation(relation_field) => {
                let related_model_name_snake = snake_ident(relation_field.related_model().name());

                quote!(Relation(#related_model_name_snake))
            }
        };

        quote!((#field_name_snake, #variant))
    });

    quote! {
        ::prisma_client_rust::macros::filter_factory!(
            #name,
            #model_name_snake,
            [#(#fields),*]
        );
    }
}
