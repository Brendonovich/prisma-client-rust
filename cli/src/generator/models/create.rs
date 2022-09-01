use crate::generator::prelude::*;

pub fn model_fn(model: &dml::Model) -> TokenStream {
    let required_fields = super::required_fields(model);

    let required_field_names = required_fields
        .iter()
        .map(|field| snake_ident(field.name()));
    let required_field_types = required_fields.iter().map(|field| &field.typ);

    let args = {
        let required_field_names = required_field_names.clone();
        let required_field_types = required_field_types.clone();

        quote!(#(#required_field_names: #required_field_types,)* _params: Vec<SetParam>)
    };

    quote! {
        pub fn create(#args) -> (#(#required_field_types,)* Vec<SetParam>) {
            (#(#required_field_names,)* _params)
        }
    }
}
