use crate::generator::prelude::*;

pub fn scalar_selections_fn(model: &dml::Model, module_path: &TokenStream) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let selections = model.fields().flat_map(|field| {
        let field_name_snake = snake_ident(field.name());

        Some(match field {
            dml::Field::ScalarField(_) => {
                field.type_tokens(module_path)?;
                quote!(#pcr::sel(#field_name_snake::NAME))
            }
            dml::Field::CompositeField(composite_field) => {
                let composite_type_snake = snake_ident(&composite_field.composite_type);
                quote! {
                    #pcr::Selection::new(#field_name_snake::NAME, None, [], super::#composite_type_snake::scalar_selections())
                }
            }
            dml::Field::RelationField(_) => return None,
        })
    });

    quote! {
        fn scalar_selections() -> Vec<::prisma_client_rust::Selection> {
            vec![#(#selections),*]
        }
    }
}

pub fn struct_definition(model: &dml::Model, module_path: &TokenStream) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let scalar_selections_fn = scalar_selections_fn(model, module_path);

    quote! {
        #[derive(Clone)]
        pub struct Types;

        impl #pcr::ModelTypes for Types {
            type Data = Data;
            type Where = WhereParam;
            type UncheckedSet = UncheckedSetParam;
            type Set = SetParam;
            type With = WithParam;
            type OrderBy = OrderByParam;
            type Cursor = UniqueWhereParam;

            const MODEL: &'static str = NAME;

            #scalar_selections_fn
        }
    }
}
