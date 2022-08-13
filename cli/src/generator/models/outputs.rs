use crate::generator::prelude::*;

pub fn model_fn(model: &dml::Model) -> TokenStream {
    let outputs = model
        .scalar_fields()
        .map(|f| &f.name);

    quote! {
        pub fn _outputs() -> Vec<Selection> {
            [#(#outputs),*]
                .into_iter()
                .map(|o| {
                    let builder = Selection::builder(o);
                    builder.build()
                })
                .collect()
        }
    }
}

