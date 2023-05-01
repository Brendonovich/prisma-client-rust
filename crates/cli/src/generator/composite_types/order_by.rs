use prisma_client_rust_sdk::prisma::{
    prisma_models::walkers::CompositeTypeWalker, psl::parser_database::ScalarFieldType,
};

use crate::generator::prelude::*;

pub fn enum_definition(comp_type: CompositeTypeWalker) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let (variants, into_pv_arms): (Vec<_>, Vec<_>) = comp_type
        .fields()
        .flat_map(|field| {
            let field_name_snake = snake_ident(field.name());
            let field_name_pascal = pascal_ident(field.name());

            if field.ast_field().arity.is_list() {
                return None;
            }

            Some(match field.r#type() {
                ScalarFieldType::CompositeType(id) => {
                    let comp_type = field.db.walk(id);

                    let composite_type_snake = snake_ident(comp_type.name());

                    (
                        quote!(#field_name_pascal(Vec<super::#composite_type_snake::OrderByParam>)),
                        quote! {
                            Self::#field_name_pascal(params) => (
                                #field_name_snake::NAME,
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
                    quote!(#field_name_pascal(SortOrder)),
                    quote! {
                        Self::#field_name_pascal(direction) => (
                            #field_name_snake::NAME,
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
                let (k, v) = match self {
                    #(#into_pv_arms),*
                };

                (k.to_string(), v)
            }
        }
    }
}
