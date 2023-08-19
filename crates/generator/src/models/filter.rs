use prisma_client_rust_generator_shared::select_include::SelectableFields;
use prisma_client_rust_sdk::prisma::prisma_models::walkers::ModelWalker;

use crate::prelude::*;

pub fn r#macro(model: ModelWalker, module_path: &TokenStream) -> TokenStream {
    let model_name_snake = snake_ident(model.name());

    let name = format_ident!("_{}_filter", model.name().to_case(Case::Snake, true));

    let selectable_fields = SelectableFields::new(model.fields(), module_path);

    quote! {
        ::prisma_client_rust::macros::filter_factory!(
            #name,
            #module_path #model_name_snake,
            #selectable_fields
        );
    }
}
