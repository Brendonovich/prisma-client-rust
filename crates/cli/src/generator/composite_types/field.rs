use super::where_params::Variant;
use prisma_client_rust_sdk::{
    prelude::*, prisma::prisma_models::walkers::CompositeTypeFieldWalker,
};

pub fn module(
    field: CompositeTypeFieldWalker,
    module_path: &TokenStream,
) -> (TokenStream, Variant) {
    let field_name_str = field.name();
    let field_name_snake = snake_ident(field.name());
    let field_name_pascal = pascal_ident(field.name());

    let field_type = field.type_tokens(module_path);
    let value_ident = format_ident!("value");
    let value_to_pv = field.type_prisma_value(&value_ident);

    let set_variant_name = format_ident!("Set{field_name_pascal}");
    let where_variant_name = format_ident!("{field_name_pascal}Equals");

    (
        quote! {
            pub mod #field_name_snake {
                use super::super::*;
                use super::{SetParam, WhereParam};

                pub const NAME: &str = #field_name_str;

                pub fn set(val: #field_type) -> SetParam {
                    SetParam::#set_variant_name(val)
                }

                pub fn equals(val: #field_type) -> WhereParam {
                    WhereParam::#where_variant_name(val)
                }
            }
        },
        Variant {
            definition: quote!(#where_variant_name(#field_type)),
            match_arm: quote! {
                Self::#where_variant_name(#value_ident) => (
                    #field_name_snake::NAME,
                    #value_to_pv
                )
            },
        },
    )
}
