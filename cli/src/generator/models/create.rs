use crate::generator::prelude::*;

pub fn model_fn(model: &dml::Model) -> TokenStream {
    let scalar_field_names = model
        .required_scalar_fields()
        .iter()
        .map(|f| snake_ident(f.name()))
        .collect::<Vec<_>>();

    let scalar_field_types = model
        .required_scalar_fields()
        .iter()
        .map(|f| f.type_tokens())
        .collect::<Vec<_>>();

    let args = {
        let scalar_field_names = scalar_field_names.clone();
        let scalar_field_types = scalar_field_types.clone();

        quote!(#(#scalar_field_names: #scalar_field_types,)* _params: Vec<SetParam>)
    };

    quote! {
        pub fn create(#args) -> (#(#scalar_field_types,)* Vec<SetParam>) {
            (#(#scalar_field_names,)* _params)
        }
    }
}
