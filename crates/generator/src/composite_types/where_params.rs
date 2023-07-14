use prisma_client_rust_sdk::prisma::prisma_models::walkers::CompositeTypeWalker;

use crate::prelude::*;

use super::CompositeTypeModulePart;

pub fn module_part(comp_type: CompositeTypeWalker) -> CompositeTypeModulePart {
    let pcr = quote!(::prisma_client_rust);

    let ((variants, match_arms), fields): ((Vec<_>, Vec<_>), _) = comp_type
        .fields()
        .map(|field| {
            let field_name_snake = snake_ident(field.name());
            let field_name_pascal = pascal_ident(field.name());

            let field_type = field.type_tokens(&quote!());
            let value_ident = format_ident!("value");
            let value_to_pv = field.type_prisma_value(&value_ident);

            let where_variant_name = format_ident!("{field_name_pascal}Equals");

            (
                (
                    quote!(#where_variant_name(#field_type)),
                    quote! {
                        Self::#where_variant_name(#value_ident) => (
                            #field_name_snake::NAME,
                            #value_to_pv
                        )
                    },
                ),
                (
                    field.name().to_string(),
                    quote! {
                        pub fn equals(val: #field_type) -> WhereParam {
                            WhereParam::#where_variant_name(val)
                        }
                    },
                ),
            )
        })
        .unzip();

    CompositeTypeModulePart {
        data: quote! {
            #[derive(Debug, Clone)]
            pub enum WhereParam {
                #(#variants),*
            }

            impl #pcr::WhereInput for WhereParam {
                fn serialize(self) -> #pcr::SerializedWhereInput {
                    let (name, value) = match self {
                        #(#match_arms),*
                    };

                    #pcr::SerializedWhereInput::new(name.to_string(), #pcr::SerializedWhereValue::Value(value))
                }
            }
        },
        fields,
    }
}
