use crate::generator::prelude::*;

use super::required_fields;

fn create_unchecked(model: &dml::Model) -> TokenStream {
    let scalar_fields = model.required_scalar_fields();
    let scalar_fields = scalar_fields.iter();

    let scalar_field_names = scalar_fields
        .clone()
        .map(|f| snake_ident(f.name()))
        .collect::<Vec<_>>();

    let scalar_field_types = scalar_fields
        .clone()
        .map(|f| f.type_tokens(quote!()))
        .collect::<Vec<_>>();

    quote! {
        pub fn create_unchecked(#(#scalar_field_names: #scalar_field_types,)* _params: Vec<SetParam>)
            -> (#(#scalar_field_types,)* Vec<SetParam>) {
            (#(#scalar_field_names,)* _params)
        }
    }
}

fn create(model: &dml::Model) -> TokenStream {
    let required_fields = required_fields(model);

    let required_field_names = required_fields
        .iter()
        .map(|field| snake_ident(field.name()))
        .collect::<Vec<_>>();
    let required_field_types = required_fields
        .iter()
        .map(|field| &field.typ)
        .collect::<Vec<_>>();

    quote! {
        pub fn create(#(#required_field_names: #required_field_types,)* _params: Vec<SetParam>)
            -> (#(#required_field_types,)* Vec<SetParam>) {
            (#(#required_field_names,)* _params)
        }
    }
}

pub fn model_fns(model: &dml::Model) -> TokenStream {
    let create_unchecked = create_unchecked(model);
    let create = create(model);

    quote! {
        #create

        #create_unchecked
    }
}
