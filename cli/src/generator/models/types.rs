use crate::generator::prelude::*;

pub fn scalar_selections_fn(model: &dml::Model) -> TokenStream {
    let scalar_fields = model
        .scalar_fields()
        .filter(|f| !f.field_type.is_unsupported())
        .map(|f| &f.name);

    quote! {
        fn scalar_selections() -> Vec<::prisma_client_rust::Selection> {
            [#(#scalar_fields),*]
                .into_iter()
                .map(::prisma_client_rust::sel)
                .collect()
        }
    }
}

pub fn struct_definition(model: &dml::Model) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let model_name_str = &model.name;
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

            const MODEL: &'static str = #model_name_str;

            #scalar_selections_fn
        }
    }
}
