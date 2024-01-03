use std::collections::BTreeMap;

use prisma_client_rust_sdk::prisma::{
    dmmf::TypeLocation,
    prisma_models::{walkers::ModelWalker, FieldArity},
};

use crate::prelude::*;

use super::ModelModulePart;

pub fn fetch_builder_fn(model_name_snake: &Ident) -> TokenStream {
    quote! {
        pub fn order_by(mut self, param: #model_name_snake::OrderByWithRelationParam) -> Self {
            self.0 = self.0.order_by(param);
            self
        }
    }
}

pub fn model_data(model: ModelWalker, args: &GenerateArgs) -> ModelModulePart {
    let pcr = quote!(::prisma_client_rust);

    let (order_by_relation_aggregate_param, aggregate_field_stuff) = args
        .dmmf
        .schema
        .find_input_type(&format!("{}OrderByRelationAggregateInput", model.name()))
        .map(|input_type| {
            let ((variants, into_pv_arms), field_things): ((Vec<_>, Vec<_>), Vec<_>) = input_type
                .fields
                .iter()
                .flat_map(|field| {
                    let field_name_str = &field.name;
                    let field_name_pascal = pascal_ident(&field.name);

                    let typ = &field.input_types[0];
                    let typ =
                        typ.to_tokens(&quote!(super::), &FieldArity::Required, &args.schema.db)?;

                    Some((
                        (
                            quote!(#field_name_pascal(#typ)),
                            quote! {
                                Self::#field_name_pascal(param) => (
                                    #field_name_str,
                                    param.into()
                                )
                            },
                        ),
                        (
                            field_name_str,
                            (
                                typ,
                                quote! {
                                    impl From<Order> for super::OrderByRelationAggregateParam {
                                        fn from(Order(v): Order) -> Self {
                                            Self::#field_name_pascal(v)
                                        }
                                    }
                                },
                            ),
                        ),
                    ))
                })
                .unzip();

            (
                quote! {
                    #[derive(Debug, Clone)]
                    pub enum OrderByRelationAggregateParam {
                        #(#variants),*
                    }

                    impl Into<(String, #pcr::PrismaValue)> for OrderByRelationAggregateParam {
                        fn into(self) -> (String, #pcr::PrismaValue) {
                            let (k, v) = match self {
                                #(#into_pv_arms),*
                            };

                            (k.to_string(), v)
                        }
                    }
                },
                field_things,
            )
        })
        .unwrap_or_default();

    let (order_by_with_relation_param, relation_field_stuff) = args
        .dmmf
        .schema
        .find_input_type(&format!("{}OrderByWithRelationInput", model.name()))
        .map(|input_type| {
            let ((variants, into_pv_arms), field_stuff): ((Vec<_>, Vec<_>), Vec<_>) = input_type
                .fields
                .iter()
                .flat_map(|field| {
                    let field_name_str = &field.name;
                    let field_name_pascal = pascal_ident(&field.name);

                    let typ_ref = &field.input_types[0];
                    let typ = typ_ref.to_tokens(
                        &quote!(super::),
                        &FieldArity::Required,
                        &args.schema.db,
                    )?;

                    let pv = match &typ_ref.location {
                        TypeLocation::EnumTypes | TypeLocation::Scalar => quote!(param.into()),
                        TypeLocation::InputObjectTypes => quote! {
                            #pcr::PrismaValue::Object(
                                param.into_iter().map(Into::into).collect()
                            )
                        },
                        _ => return None,
                    };

                    Some((
                        (
                            quote!(#field_name_pascal(#typ)),
                            quote! {
                                Self::#field_name_pascal(param) => (
                                    #field_name_str,
                                    #pv
                                )
                            },
                        ),
                        (
                            field_name_str,
                            (
                                typ_ref.to_tokens(
                                    &quote!(),
                                    &FieldArity::Required,
                                    &args.schema.db,
                                )?,
                                quote! {
                                    impl From<Order> for super::OrderByWithRelationParam {
                                        fn from(Order(v): Order) -> Self {
                                            Self::#field_name_pascal(v)
                                        }
                                    }
                                },
                            ),
                        ),
                    ))
                })
                .unzip();

            (
                quote! {
                    #[derive(Debug, Clone)]
                    pub enum OrderByWithRelationParam {
                       #(#variants),*
                    }

                    impl Into<(String, #pcr::PrismaValue)> for OrderByWithRelationParam {
                        fn into(self) -> (String, #pcr::PrismaValue) {
                            let (k, v) = match self {
                                #(#into_pv_arms),*
                            };

                            (k.to_string(), v)
                        }
                    }
                },
                field_stuff,
            )
        })
        .unwrap_or_default();

    ModelModulePart {
        data: quote! {
            #order_by_with_relation_param
            #order_by_relation_aggregate_param
        },
        fields: aggregate_field_stuff
            .into_iter()
            .chain(relation_field_stuff)
            .fold(BTreeMap::new(), |mut acc, (name, data)| {
                let entry = acc.entry(name.to_string()).or_insert_with(|| vec![]);
                entry.push(data);
                acc
            })
            .into_iter()
            .map(|(name, data)| {
                let Some(typ) = data
                    .iter()
                    .find_map(|(typ, _)| (typ.to_string() == data[0].0.to_string()).then_some(typ))
                else {
                    panic!();
                };

                let impls = data.iter().map(|(_, impls)| impls);

                (
                    name,
                    quote! {
                        pub struct Order(#typ);

                        pub fn order<T: From<Order>>(v: #typ) -> T {
                            Order(v).into()
                        }

                        #(#impls)*
                    },
                )
            })
            .collect(),
    }
}
