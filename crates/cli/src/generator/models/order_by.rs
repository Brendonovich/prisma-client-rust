use prisma_client_rust_sdk::prisma::{
    prisma_models::walkers::ModelWalker, psl::parser_database::ScalarFieldType,
};

use crate::generator::prelude::*;

pub fn fetch_builder_fn(model_name_snake: &Ident) -> TokenStream {
    quote! {
        pub fn order_by(mut self, param: #model_name_snake::OrderByParam) -> Self {
            self.0 = self.0.order_by(param);
            self
        }
    }
}

pub fn enum_definition(model: ModelWalker) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let (variants, into_pv_arms): (Vec<_>, Vec<_>) = model
        .scalar_fields()
        .flat_map(|field| {
            let field_name_snake = snake_ident(field.name());
            let field_name_pascal = pascal_ident(field.name());

            Some(match field.scalar_field_type() {
                ScalarFieldType::BuiltInScalar(_) | ScalarFieldType::Enum(_) => (
                    quote!(#field_name_pascal(#pcr::Direction)),
                    quote! {
                        Self::#field_name_pascal(direction) => (
                            #field_name_snake::NAME,
                            #pcr::PrismaValue::String(direction.to_string())
                        )
                    },
                ),
                ScalarFieldType::CompositeType(id) => {
                    let composite_type_snake = snake_ident(model.db.walk(id).name());

                    if field.ast_field().arity.is_list() {
                        return None;
                    }

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
                ScalarFieldType::Unsupported(_) => return None,
            })
        })
        .unzip();

    quote! {
        #[derive(Clone)]
        pub enum OrderByParam {
            #(#variants),*
        }

        impl Into<(&'static str, #pcr::PrismaValue)> for OrderByParam {
            fn into(self) -> (&'static str, #pcr::PrismaValue) {
                match self {
                    #(#into_pv_arms),*
                }
            }
        }
    }
}
