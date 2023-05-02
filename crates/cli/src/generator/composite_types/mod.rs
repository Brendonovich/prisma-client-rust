mod data;
mod field;
mod order_by;
mod set_params;
mod where_params;

use prisma_client_rust_sdk::{
    prelude::*,
    prisma::{prisma_models::walkers::CompositeTypeWalker, psl::parser_database::ScalarFieldType},
};

pub fn scalar_selections_fn(
    comp_type: CompositeTypeWalker,
    module_path: &TokenStream,
) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let selections = comp_type.fields().flat_map(|field| {
        let field_name_snake = snake_ident(field.name());

        Some(match field.r#type() {
            ScalarFieldType::BuiltInScalar(_) | ScalarFieldType::Enum(_) => {
                field.type_tokens(module_path)?;
                quote!(#pcr::sel(#field_name_snake::NAME))
            }
            ScalarFieldType::CompositeType(id) => {
                let comp_type = comp_type.db.walk(id);

                let field_comp_type_snake = snake_ident(comp_type.name());

                quote! {
                    #pcr::Selection::new(
                        #field_name_snake::NAME,
                        None,
                        [],
                        super::#field_comp_type_snake::scalar_selections()
                    )
                }
            }
            ScalarFieldType::Unsupported(_) => return None,
        })
    });

    quote! {
        pub fn scalar_selections() -> Vec<::prisma_client_rust::Selection> {
            vec![#(#selections),*]
        }
    }
}

pub fn modules(args: &GenerateArgs, module_path: &TokenStream) -> Vec<TokenStream> {
    args.schema
        .db
        .walk_composite_types()
        .map(|comp_type| {
            let comp_type_name_snake = snake_ident(comp_type.name());

            let scalar_selections_fn = scalar_selections_fn(comp_type, module_path);

            let (field_modules, field_where_param_entries): (Vec<_>, Vec<_>) =
                comp_type.fields().map(field::module).unzip();

            let data_struct = data::struct_definition(comp_type);
            let set_param_enum = set_params::enum_definition(comp_type);
            let order_by_enum = order_by::enum_definition(comp_type, args);
            let create_fn = set_params::create_fn(comp_type);
            let where_param = where_params::model_enum(field_where_param_entries);

            quote! {
                pub mod #comp_type_name_snake {
                    use super::*;
                    use super::_prisma::*;

                    #scalar_selections_fn

                    #(#field_modules)*

                    #data_struct

                    #where_param

                    #set_param_enum
                    #create_fn

                    #order_by_enum
                }
            }
        })
        .collect()
}
