mod data;

use prisma_client_rust_sdk::prelude::*;

pub struct SetParam {
    variant: TokenStream,
    into_pv_arm: TokenStream,
}

pub fn field_set_params(
    field: &dml::CompositeTypeField,
    module_path: &TokenStream,
) -> Option<Vec<SetParam>> {
    let field_name_snake = snake_ident(&field.name);
    let field_name_pascal = pascal_ident(&field.name);
    let field_type = field.type_tokens(module_path)?;

    let set_variant = {
        let variant_name = format_ident!("Set{field_name_pascal}");
        let converter = field.type_prisma_value(&format_ident!("value"))?;

        SetParam {
            variant: quote!(#variant_name(#field_type)),
            into_pv_arm: quote! {
                SetParam::#variant_name(value) => (
                    #field_name_snake::NAME.to_string(),
                    #converter
                )
            },
        }
    };

    Some(vec![set_variant])
}

pub fn field_module(field: &dml::CompositeTypeField, module_path: &TokenStream) -> TokenStream {
    let field_name_str = &field.name;
    let field_name_snake = snake_ident(&field.name);
    let field_name_pascal = pascal_ident(&field.name);

    let field_type = field.type_tokens(module_path);

    let set_variant_name = format_ident!("Set{field_name_pascal}");

    quote! {
        pub mod #field_name_snake {
            use super::super::*;
            use super::SetParam;

            pub const NAME: &str = #field_name_str;

            pub fn set(val: #field_type) -> SetParam {
                SetParam::#set_variant_name(val)
            }
        }
    }
}

pub fn scalar_selections_fn(
    comp_type: &dml::CompositeType,
    module_path: &TokenStream,
) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let selections = comp_type.fields.iter().flat_map(|field| {
        let field_name_snake = snake_ident(&field.name);

        Some(match &field.r#type {
            dml::CompositeTypeFieldType::Scalar(_, _) | dml::CompositeTypeFieldType::Enum(_) => {
                field.type_tokens(module_path)?;
                quote!(#pcr::sel(#field_name_snake::NAME))
            }
            dml::CompositeTypeFieldType::CompositeType(field_comp_type) => {
                let field_comp_type_snake = snake_ident(&field_comp_type);

                quote! {
                    #pcr::Selection::new(
                        #field_name_snake::NAME,
                        None,
                        [],
                        super::#field_comp_type_snake::scalar_selections()
                    )
                }
            }
            dml::CompositeTypeFieldType::Unsupported(_) => return None,
        })
    });

    quote! {
        pub fn scalar_selections() -> Vec<::prisma_client_rust::Selection> {
            vec![#(#selections),*]
        }
    }
}

pub fn generate(args: &GenerateArgs, module_path: &TokenStream) -> Vec<TokenStream> {
    let pcr = quote!(::prisma_client_rust);

    args.dml
        .composite_types()
        .map(|comp_type| {
            let ty_name_snake = snake_ident(&comp_type.name);

            let scalar_selections_fn = scalar_selections_fn(&comp_type, module_path);

            let field_modules = comp_type
                .fields
                .iter()
                .map(|f| field_module(f, module_path));

            let data_struct = data::struct_definition(&comp_type, module_path);

            let set_param = {
                let (variants, into_pv_arms): (Vec<_>, Vec<_>) = comp_type
                    .fields
                    .iter()
                    .flat_map(|f| field_set_params(f, module_path))
                    .flatten()
                    .map(|p| (p.variant, p.into_pv_arm))
                    .unzip();

                quote! {
                    #[derive(Clone)]
                    pub enum SetParam {
                        #(#variants),*
                    }

                    impl From<SetParam> for (String, #pcr::PrismaValue) {
                        fn from(v: SetParam) -> Self {
                            match v {
                                #(#into_pv_arms),*
                            }
                        }
                    }
                }
            };

            let create = comp_type
                .fields
                .iter()
                .filter(|f| f.required_on_create())
                .map(|field| Some((snake_ident(&field.name), field.type_tokens(module_path)?)))
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

                        pub fn create<T: From<Create>>(
                            #(#required_field_names: #required_field_types,)*
                            _params: Vec<SetParam>
                        ) -> T {
                            Create {
                                #(#required_field_names,)*
                                _params
                            }.into()
                        }
                    }
                });

            quote! {
                pub mod #ty_name_snake {
                    use super::*;
                    use super::_prisma::*;

                    #scalar_selections_fn

                    #(#field_modules)*

                    #data_struct

                    #set_param

                    #create
                }
            }
        })
        .collect()
}
