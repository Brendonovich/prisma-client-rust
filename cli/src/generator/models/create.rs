use crate::generator::prelude::*;

use super::required_fields;

fn create_unchecked(model: &dml::Model) -> TokenStream {
    let (scalar_field_names, scalar_field_types): (Vec<_>, Vec<_>) = model
        .required_scalar_fields()
        .iter()
        .map(|f| (snake_ident(f.name()), f.type_tokens(quote!())))
        .unzip();

    quote! {
        pub fn create_unchecked(#(#scalar_field_names: #scalar_field_types,)* _params: Vec<SetParam>)
            -> (#(#scalar_field_types,)* Vec<SetParam>) {
            (#(#scalar_field_names,)* _params)
        }
    }
}

fn create(model: &dml::Model) -> TokenStream {
    let (required_field_names, required_field_types): (Vec<_>, Vec<_>) = required_fields(model)
        .iter()
        .map(|field| (snake_ident(field.name()), field.typ.clone()))
        .unzip();

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
