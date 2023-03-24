use crate::generator::prelude::*;

use super::required_fields;

fn create_unchecked(model: &dml::Model, module_path: &TokenStream) -> Option<TokenStream> {
    let (scalar_field_names, scalar_field_types): (Vec<_>, Vec<_>) = model
        .required_scalar_fields()
        .iter()
        .map(|f| Some((snake_ident(f.name()), f.type_tokens(module_path)?)))
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .unzip();

    Some(quote! {
        pub fn create_unchecked(#(#scalar_field_names: #scalar_field_types,)* _params: Vec<UncheckedSetParam>)
            -> (#(#scalar_field_types,)* Vec<UncheckedSetParam>) {
            (#(#scalar_field_names,)* _params)
        }
    })
}

fn create(model: &dml::Model, module_path: &TokenStream) -> Option<TokenStream> {
    let (required_field_names, required_field_types): (Vec<_>, Vec<_>) =
        required_fields(model, module_path)?
            .iter()
            .map(|field| (snake_ident(field.name()), field.typ.clone()))
            .unzip();

    Some(quote! {
        pub fn create(#(#required_field_names: #required_field_types,)* _params: Vec<SetParam>)
            -> (#(#required_field_types,)* Vec<SetParam>) {
            (#(#required_field_names,)* _params)
        }
    })
}

pub fn model_fns(model: &dml::Model, module_path: &TokenStream) -> TokenStream {
    let create_unchecked = create_unchecked(model, module_path);
    let create = create(model, module_path);

    quote! {
        #create

        #create_unchecked
    }
}
