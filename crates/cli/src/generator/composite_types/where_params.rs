use crate::generator::prelude::*;

pub struct Variant {
    pub definition: TokenStream,
    pub match_arm: TokenStream,
}

pub fn model_enum(entries: Vec<Variant>) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let (variants, match_arms): (Vec<_>, Vec<_>) = entries
        .into_iter()
        .map(|v| (v.definition, v.match_arm))
        .unzip();

    quote! {
        #[derive(Clone)]
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
    }
}
