use prisma_client_rust_sdk::prisma::{
    prisma_models::walkers::ModelWalker, psl::parser_database::ScalarFieldType,
};

use crate::generator::prelude::*;

pub fn scalar_selections_fn(model: ModelWalker, module_path: &TokenStream) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let selections = model.scalar_fields().flat_map(|field| {
        let field_name_snake = snake_ident(field.name());

        Some(match field.scalar_field_type() {
            ScalarFieldType::CompositeType(id) => {
                let comp_type = model.db.walk(id);

                let comp_type_name_snake = snake_ident(comp_type.name());

                quote! {
                    #pcr::Selection::new(#field_name_snake::NAME, None, [], super::#comp_type_name_snake::scalar_selections())
                }
            }
            _ => {
                field.type_tokens(module_path)?;
                quote!(#pcr::sel(#field_name_snake::NAME))
            }
        })
    });

    quote! {
        fn scalar_selections() -> Vec<::prisma_client_rust::Selection> {
            vec![#(#selections),*]
        }
    }
}

pub fn struct_definition(model: ModelWalker, module_path: &TokenStream) -> TokenStream {
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
