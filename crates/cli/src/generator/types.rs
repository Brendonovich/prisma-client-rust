use prisma_client_rust_sdk::prisma::prisma_models::FieldArity;

use crate::generator::prelude::*;

pub fn types(args: &GenerateArgs) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let types = args
        .dmmf
        .schema
        .input_object_types
        .get("prisma")
        .unwrap()
        .iter()
        .filter(|t| t.name.contains("OrderBy"))
        .map(|t| {
            let is_struct = t.fields.iter().any(|f| f.is_required);
            let type_name = format_ident!("{}", &t.name);

            if is_struct {
                quote! {
                    pub struct #type_name {

                    }
                }
            } else {
                let (variants, into_pv_arms): (Vec<_>, Vec<_>) = t
                    .fields
                    .iter()
                    .flat_map(|field| {
                        let field_name_str = &field.name;
                        let field_name_pascal = pascal_ident(&field.name);

                        let input_type = &field.input_types[0];

                        let typ = input_type.to_tokens(
                            &quote!(),
                            &FieldArity::Required,
                            &args.schema.db,
                        )?;

                        let value_ident = format_ident!("value");
                        let value = input_type.wrap_pv(&value_ident, quote!(#value_ident.into()));

                        Some((
                            quote!(#field_name_pascal(#typ)),
                            quote! {
                                Self::#field_name_pascal(#value_ident) => (
                                    #field_name_str,
                                    #value
                                )
                            },
                        ))
                    })
                    .unzip();

                quote! {
                    #[derive(Clone)]
                    pub enum #type_name {
                        #(#variants),*
                    }

                    impl Into<(String, #pcr::PrismaValue)> for #type_name {
                        fn into(self) -> (String, #pcr::PrismaValue) {
                            let (k, v) = match self {
                                #(#into_pv_arms),*
                            };

                            (k.to_string(), v)
                        }
                    }
                }
            }
        });

    quote! {
        #(#types)*
    }
}
