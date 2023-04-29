use prisma_client_rust_sdk::{prelude::*, prisma::prisma_models::walkers::CompositeTypeWalker};

pub fn create_fn(comp_type: CompositeTypeWalker, module_path: &TokenStream) -> Option<TokenStream> {
    comp_type
        .fields()
        .filter(|f| f.required_on_create())
        .map(|field| Some((snake_ident(field.name()), field.type_tokens(module_path)?)))
        .collect::<Option<Vec<_>>>()
        .map(|v| {
            let (required_field_names, required_field_types): (Vec<_>, Vec<_>) =
                v.into_iter().unzip();

            let required_fields_wrapped = required_field_names
                .iter()
                .map(|name_snake| quote!(#name_snake::set(self.#name_snake)));

            quote! {
                #[derive(Clone)]
                pub struct Create {
                    #(pub #required_field_names: #required_field_types,)*
                    pub _params: Vec<SetParam>
                }

                impl Create {
                    pub fn to_params(self) -> Vec<SetParam> {
                         let mut _params = self._params;
                           _params.extend([
                             #(#required_fields_wrapped),*
                         ]);
                         _params
                     }
                }

                pub fn create(
                    #(#required_field_names: #required_field_types,)*
                    _params: Vec<SetParam>
                ) -> Create {
                    Create {
                        #(#required_field_names,)*
                        _params
                    }.into()
                }
            }
        })
}

pub fn enum_definition(comp_type: CompositeTypeWalker, module_path: &TokenStream) -> TokenStream {
    let (variants, into_pv_arms): (Vec<_>, Vec<_>) = comp_type
        .fields()
        .flat_map(|field| {
            let field_name_snake = snake_ident(field.name());
            let field_name_pascal = pascal_ident(field.name());
            let field_type = field.type_tokens(module_path)?;

            let variant_name = format_ident!("Set{field_name_pascal}");
            let converter = field.type_prisma_value(&format_ident!("value"))?;

            Some((
                quote!(#variant_name(#field_type)),
                quote! {
                    SetParam::#variant_name(value) => (
                        #field_name_snake::NAME.to_string(),
                        #converter
                    )
                },
            ))
        })
        .unzip();

    quote! {
        #[derive(Clone)]
        pub enum SetParam {
            #(#variants),*
        }

        impl From<SetParam> for (String, ::prisma_client_rust::PrismaValue) {
            fn from(v: SetParam) -> Self {
                match v {
                    #(#into_pv_arms),*
                }
            }
        }
    }
}
