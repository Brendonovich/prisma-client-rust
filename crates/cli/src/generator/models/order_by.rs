use std::collections::BTreeMap;

use prisma_client_rust_sdk::prisma::{
    dmmf::TypeLocation,
    prisma_models::{
        walkers::{ModelWalker, RefinedFieldWalker},
        FieldArity,
    },
    psl::parser_database::ScalarFieldType,
};

use crate::generator::prelude::*;

use super::SomethingThatNeedsFieldModules;

pub fn fetch_builder_fn(model_name_snake: &Ident) -> TokenStream {
    quote! {
        pub fn order_by(mut self, param: #model_name_snake::OrderByWithRelationParam) -> Self {
            self.0 = self.0.order_by(param);
            self
        }
    }
}

pub fn model_data(model: ModelWalker, args: &GenerateArgs) -> SomethingThatNeedsFieldModules {
    let pcr = quote!(::prisma_client_rust);

    let fields = model
        .fields()
        .flat_map(|field| {
            let field_name_pascal = pascal_ident(field.name());

            let field_fn = match field.refine() {
                RefinedFieldWalker::Scalar(scalar_field) => {
                    match scalar_field.scalar_field_type() {
                        ScalarFieldType::BuiltInScalar(_) | ScalarFieldType::Enum(_) => {
                            quote! {
                            	pub struct Order(pub SortOrder);

                             	impl From<Order> for super::OrderByWithRelationParam {
                              		fn from(Order(v): Order) -> Self {
							  			Self::#field_name_pascal(v)
							  		}
	                            }

								impl From<Order> for super::OrderByRelationAggregateParam{
									fn from(Order(v): Order) -> Self {
						  				Self::#field_name_pascal(v)
							  		}
	                            }

                                pub fn order<T: From<Order>>(direction: SortOrder) -> T {
                                    Order(direction).into()
                                }
                            }
                        }
                        ScalarFieldType::CompositeType(id) => {
                            let composite_type_snake = snake_ident(model.db.walk(id).name());

                            if field.ast_field().arity.is_list() {
                                return None;
                            }

                            let variant_type = quote!(Vec<#composite_type_snake::OrderByParam>);

                            quote! {
                                pub fn order(direction: #variant_type) -> OrderByParam {
                                    OrderByParam::#field_name_pascal(direction)
                                }
                            }
                        }
                        ScalarFieldType::Unsupported(_) => return None,
                    }
                }
                RefinedFieldWalker::Relation(relation_field) => {
                    let relation_model_name_str = relation_field.related_model().name();
                    let relation_model_name_snake = snake_ident(relation_model_name_str);

                    let variant_type =
                        quote!(#relation_model_name_snake::OrderByRelationAggregateParam);

                    match relation_field.referential_arity() {
                        FieldArity::List => quote! {
                            pub fn order(direction: Vec<#variant_type>) -> super::OrderByWithRelationParam {
                                super::OrderByWithRelationParam::#field_name_pascal(direction)
                            }
                        },
                        _ => return None,
                    }
                }
            };

            Some((field.name().to_string(), field_fn))
        })
        .collect::<BTreeMap<_, _>>();

    let order_by_relation_aggregate_param = args
        .dmmf
        .schema
        .find_input_type(&format!("{}OrderByWithAggregationInput", model.name()))
        .map(|input_type| {
            let (variants, into_pv_arms): (Vec<_>, Vec<_>) = input_type
                .fields
                .iter()
                .flat_map(|field| {
                    let field_name_str = &field.name;
                    let field_name_pascal = pascal_ident(&field.name);

                    let typ = &field.input_types[0];
                    let typ =
                        typ.to_tokens(&quote!(super), &FieldArity::Required, &args.schema.db)?;

                    Some((
                        quote!(#field_name_pascal(#typ)),
                        quote! {
                            Self::#field_name_pascal(param) => (
                                #field_name_str,
                                param.into()
                            )
                        },
                    ))
                })
                .unzip();

            quote! {
                #[derive(Clone)]
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
            }
        });

    let order_by_with_relation_param = args
        .dmmf
        .schema
        .find_input_type(&format!("{}OrderByWithRelationInput", model.name()))
        .map(|input_type| {
            let (variants, into_pv_arms): (Vec<_>, Vec<_>) = input_type
                .fields
                .iter()
                .flat_map(|field| {
                    let field_name_str = &field.name;
                    let field_name_pascal = pascal_ident(&field.name);

                    let typ_ref = &field.input_types[0];
                    let typ = typ_ref.to_tokens(
                        &quote!(super),
                        &FieldArity::Required,
                        &args.schema.db,
                    )?;

                    let pv = match &typ_ref.location {
                        TypeLocation::EnumTypes => quote!(param.into()),
                        TypeLocation::Scalar => quote!(param.into()),
                        TypeLocation::InputObjectTypes => quote! {
                            #pcr::PrismaValue::Object(
                                param.into_iter().map(Into::into).collect()
                            )
                        },
                        _ => return None,
                    };

                    Some((
                        quote!(#field_name_pascal(#typ)),
                        quote! {
                            Self::#field_name_pascal(param) => (
                                #field_name_str,
                                #pv
                            )
                        },
                    ))
                })
                .unzip();

            quote! {
                #[derive(Clone)]
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
            }
        });

    SomethingThatNeedsFieldModules {
        data: quote! {
            #order_by_with_relation_param
            #order_by_relation_aggregate_param
        },
        fields,
    }
}
