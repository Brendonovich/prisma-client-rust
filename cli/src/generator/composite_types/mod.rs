mod data;

use prisma_client_rust_sdk::prelude::*;

pub fn generate(args: &GenerateArgs, module_path: &TokenStream) -> Vec<TokenStream> {
    args.dml
        .composite_types()
        .map(|comp_type| {
            let ty_name_snake = snake_ident(&comp_type.name);

            let data_struct = data::struct_definition(&comp_type, module_path);

            let set_param = {
                let variants = comp_type.fields.iter().map(|field| {
                    let field_name_pascal = pascal_ident(&field.name);
                    let field_type = field.type_tokens(module_path);

                    quote!(#field_name_pascal(#field_type))
                });

                quote! {
                    #[derive(Clone)]
                    pub enum SetParam {
                        #(#variants),*
                    }
                }
            };

            let set_struct = comp_type
                .fields
                .iter()
                .filter(|f| f.required_on_create())
                .map(|field| Some((snake_ident(&field.name), field.type_tokens(module_path)?)))
                .collect::<Option<Vec<_>>>()
                .map(|v| {
                    let (required_field_names, required_field_types): (Vec<_>, Vec<_>) =
                        v.into_iter().unzip();

                    quote! {
                        #[derive(Clone)]
                        pub struct Set {
                            #(pub #required_field_names: #required_field_types,)*
                            pub _params: Vec<SetParam>
                        }
                    }
                });

            quote! {
                pub mod #ty_name_snake {
                    use super::*;
                    use super::_prisma::*;

                    #data_struct

                    #set_param
                    #set_struct
                }
            }
        })
        .collect()
}
