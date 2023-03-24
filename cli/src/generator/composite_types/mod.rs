mod data;

use prisma_client_rust_sdk::prelude::*;

pub fn generate(args: &GenerateArgs, module_path: &TokenStream) -> Vec<TokenStream> {
    args.dml
        .composite_types()
        .map(|ty| {
            let ty_name_snake = snake_ident(&ty.name);

            let data_struct = data::struct_definition(&ty);

            quote! {
                pub mod #ty_name_snake {
                    use super::*;
                    use super::_prisma::*;

                    #data_struct
                }
            }
        })
        .collect()
}
