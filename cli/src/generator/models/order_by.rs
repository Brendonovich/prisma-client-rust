use crate::generator::prelude::*;

pub fn fetch_builder_fn(model_name_snake: &Ident) -> TokenStream {
    quote! {
        pub fn order_by(mut self, param: #model_name_snake::OrderByParam) -> Self {
            self.0 = self.0.order_by(param);
            self
        }
    }
}

pub fn enum_definition(model: &dml::Model) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let (variants, into_pv_arms): (Vec<_>, Vec<_>) = model
        .scalar_fields()
        .map(|field| {
            let field_name_snake = snake_ident(&field.name);
            let field_name_pascal = pascal_ident(&field.name);

            (
                quote!(#field_name_pascal(#pcr::Direction)),
                quote! {
                    Self::#field_name_pascal(direction) => (
                        #field_name_snake::NAME.to_string(),
                        #pcr::PrismaValue::String(direction.to_string())
                    )
                },
            )
        })
        .unzip();

    quote! {
        #[derive(Clone)]
        pub enum OrderByParam {
            #(#variants),*
        }

        impl Into<(String, #pcr::PrismaValue)> for OrderByParam {
            fn into(self) -> (String, #pcr::PrismaValue) {
                match self {
                    #(#into_pv_arms),*
                }
            }
        }
    }
}
