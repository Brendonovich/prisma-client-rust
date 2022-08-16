use crate::generator::prelude::*;

pub fn fetch_builder_fn(model_name_snake: &Ident) -> TokenStream {
    quote! {
        pub fn order_by(mut self, param: impl Into<#model_name_snake::OrderByParam>) -> Self {
            self.0 = self.0.order_by(param);
            self
        }
    }
}

pub fn enum_definition(model: &dml::Model) -> TokenStream {
    let variants = model.scalar_fields().map(|field| {
        let field_name_pascal = pascal_ident(&field.name);
        quote!(#field_name_pascal(Direction))
    });

    let into_pv_arms = model.scalar_fields().map(|field| {
        let field_name_str = &field.name;
        let field_name_pascal = pascal_ident(field_name_str);

        quote! {
            Self::#field_name_pascal(direction) => (
                #field_name_str.to_string(),
                PrismaValue::String(direction.to_string())
            )
        }
    });

    quote! {
        #[derive(Clone)]
        pub enum OrderByParam {
            #(#variants),*
        }

        impl Into<(String, PrismaValue)> for OrderByParam {
            fn into(self) -> (String, PrismaValue) {
                match self {
                    #(#into_pv_arms),*
                }
            }
        }
    }
}
