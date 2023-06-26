use prisma_client_rust_sdk::{prelude::*, prisma::prisma_models::walkers::CompositeTypeWalker};

use super::CompositeTypeModulePart;

pub fn create_fn(comp_type: CompositeTypeWalker) -> Option<TokenStream> {
    comp_type
        .fields()
        .filter(|f| f.required_on_create())
        .map(|field| {
            Some((
                snake_ident(field.name()),
                field.type_tokens(&quote!(super::))?,
            ))
        })
        .collect::<Option<Vec<_>>>()
        .map(|v| {
            let (required_field_names, required_field_types): (Vec<_>, Vec<_>) =
                v.into_iter().unzip();

            let required_fields_wrapped = required_field_names
                .iter()
                .map(|name_snake| quote!(#name_snake::set(self.#name_snake)));

            quote! {
                #[derive(Debug, Clone)]
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

pub fn module_part(comp_type: CompositeTypeWalker) -> CompositeTypeModulePart {
    let ((variants, into_pv_arms), fields): ((Vec<_>, Vec<_>), _) = comp_type
        .fields()
        .flat_map(|field| {
            let field_name_snake = snake_ident(field.name());
            let field_name_pascal = pascal_ident(field.name());
            let field_type = field.type_tokens(&quote!(super::))?;

            let variant_name = format_ident!("Set{field_name_pascal}");
            let converter = field.type_prisma_value(&format_ident!("value"))?;

            Some((
                (
                    quote!(#variant_name(#field_type)),
                    quote! {
                        SetParam::#variant_name(value) => (
                            #field_name_snake::NAME,
                            #converter
                        )
                    },
                ),
                (
                    field.name().to_string(),
                    quote! {
                        pub fn set(val: #field_type) -> SetParam {
                            SetParam::#variant_name(val)
                        }
                    },
                ),
            ))
        })
        .unzip();

    CompositeTypeModulePart {
        data: quote! {
           #[derive(Debug, Clone)]
           pub enum SetParam {
               #(#variants),*
           }

           impl From<SetParam> for (String, ::prisma_client_rust::PrismaValue) {
               fn from(v: SetParam) -> Self {
                   let (k, v) = match v {
                       #(#into_pv_arms),*
                   };

                   (k.to_string(), v)
               }
           }
        },
        fields,
    }
}
