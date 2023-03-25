use crate::generator::prelude::*;

pub fn enum_definition(comp_type: &dml::CompositeType) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let (variants, into_pv_arms): (Vec<_>, Vec<_>) = comp_type
        .fields
        .iter()
        .flat_map(|field| {
            let field_name_snake = snake_ident(&field.name);
            let field_name_pascal = pascal_ident(&field.name);

            if field.arity.is_list() {
                return None;
            }

            Some(match &field.r#type {
                dml::CompositeTypeFieldType::CompositeType(cf) => {
                    let composite_type_snake = snake_ident(&cf);

                    (
                        quote!(#field_name_pascal(Vec<super::#composite_type_snake::OrderByParam>)),
                        quote! {
                            Self::#field_name_pascal(params) => (
                                #field_name_snake::NAME.to_string(),
                                #pcr::PrismaValue::Object(
                                    params
                                         .into_iter()
                                         .map(Into::into)
                                         .collect()
                                )
                            )
                        },
                    )
                }
                _ => (
                    quote!(#field_name_pascal(#pcr::Direction)),
                    quote! {
                        Self::#field_name_pascal(direction) => (
                            #field_name_snake::NAME.to_string(),
                            #pcr::PrismaValue::String(direction.to_string())
                        )
                    },
                ),
            })
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
