use crate::generator::prelude::*;

pub fn model_fn(model: &dml::Model) -> TokenStream {
    let outputs = model.scalar_fields().map(|f| &f.name);

    quote! {
        pub fn outputs() -> Vec<::prisma_client_rust::Selection> {
            [#(#outputs),*]
                .into_iter()
                .map(|o| {
                    let builder = ::prisma_client_rust::Selection::builder(o);
                    builder.build()
                })
                .collect()
        }
    }
}
