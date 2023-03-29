use crate::generator::prelude::*;

use super::required_fields;

fn create_unchecked(model: &dml::Model) -> Option<TokenStream> {
    let (names, types): (Vec<_>, Vec<_>) = model
        .fields()
        .filter_map(|field| {
            let name_snake = snake_ident(field.name());

            if !field.required_on_create() {
                return None;
            }

            Some((
                name_snake,
                match field {
                    dml::Field::RelationField(_) => return None,
                    dml::Field::CompositeField(cf) => {
                        let comp_type_snake = snake_ident(&cf.composite_type);
                        quote!(super::#comp_type_snake::Create)
                    }
                    dml::Field::ScalarField(_) => field.type_tokens(&quote!(super))?,
                },
            ))
        })
        .unzip();

    Some(quote! {
        pub fn create_unchecked(#(#names: #types,)* _params: Vec<SetParam>)
            -> (#(#types,)* Vec<SetParam>) {
            (#(#names,)* _params)
        }
    })
}

fn create(model: &dml::Model) -> Option<TokenStream> {
    let (required_field_names, required_field_types): (Vec<_>, Vec<_>) = required_fields(model)?
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

pub fn model_fns(model: &dml::Model) -> TokenStream {
    let create_unchecked = create_unchecked(model);
    let create = create(model);

    quote! {
        #create

        #create_unchecked
    }
}
