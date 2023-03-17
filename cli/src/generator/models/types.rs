use crate::generator::prelude::*;

pub fn scalar_selections_fn(model: &dml::Model) -> TokenStream {
    let scalar_fields_snake = model.scalar_fields().map(|f| snake_ident(&f.name));

    quote! {
        fn scalar_selections() -> Vec<::prisma_client_rust::Selection> {
            [#(#scalar_fields_snake::NAME),*]
                .into_iter()
                .map(::prisma_client_rust::sel)
                .collect()
        }
    }
}

pub fn struct_definition(model: &dml::Model) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let scalar_selections_fn = scalar_selections_fn(model);

    quote! {
        #[derive(Clone)]
        pub struct Types;

        impl #pcr::ModelTypes for Types {
            type Data = Data;
            type Where = WhereParam;
            type Set = SetParam;
            type With = WithParam;
            type OrderBy = OrderByParam;
            type Cursor = UniqueWhereParam;

            const MODEL: &'static str = NAME;

            #scalar_selections_fn
        }
    }
}
