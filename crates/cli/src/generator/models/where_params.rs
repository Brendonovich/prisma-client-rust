use prisma_client_rust_sdk::prisma::prisma_models::{walkers::ScalarFieldWalker, FieldArity};

use crate::generator::prelude::*;

#[derive(Debug)]
pub enum Variant {
    BaseVariant {
        definition: TokenStream,
        match_arm: TokenStream,
    },
    UniqueVariant {
        field_name: String,
        field_required_type: TokenStream,
        read_filter_name: String,
        optional: bool,
    },
    CompoundUniqueVariant {
        field_names_string: String,
        variant_data_destructured: Vec<Ident>,
        variant_data_types: Vec<TokenStream>,
    },
}

impl Variant {
    pub fn unique(
        field: ScalarFieldWalker,
        read_filter: &Filter,
        module_path: &TokenStream,
    ) -> Self {
        Self::UniqueVariant {
            field_name: field.name().to_string(),
            field_required_type: field
                .scalar_field_type()
                .to_tokens(module_path, &FieldArity::Required, field.db)
                .unwrap(),
            read_filter_name: read_filter.name.to_string(),
            optional: field.ast_field().arity.is_optional(),
        }
    }
}

pub fn collate_entries(entries: Vec<Variant>) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let (variants, to_serialized_where): (Vec<_>, Vec<_>) = entries
        .iter()
        .filter_map(|e| match e {
            Variant::BaseVariant {
                definition,
                match_arm,
            } => Some((definition.clone(), Some(match_arm))),
            _ => None,
        })
        .unzip();

    let (optional_unique_impls, (unique_variants, unique_to_where_arms)): (Vec<_>, (Vec<_>, Vec<_>)) = entries.iter().filter_map(|e| match e {
        Variant::UniqueVariant {
            field_name,
            field_required_type,
            read_filter_name,
            optional,
        } => {
            let field_pascal = pascal_ident(field_name);
            let field_snake = snake_ident(field_name);

            let variant_name = format_ident!("{}Equals", &field_pascal);
            let filter_enum = format_ident!("{}Filter", &read_filter_name);

            let optional_unique_impls = optional.then(|| {
                quote!{
                    impl ::prisma_client_rust::FromOptionalUniqueArg<#field_snake::Set> for WhereParam {
                        type Arg = Option<#field_required_type>;

                        fn from_arg(arg: Self::Arg) -> Self where Self: Sized {
                            Self::#field_pascal(_prisma::read_filters::#filter_enum::Equals(arg))
                        }
                    }

                    impl ::prisma_client_rust::FromOptionalUniqueArg<#field_snake::Set> for UniqueWhereParam {
                        type Arg = #field_required_type;

                        fn from_arg(arg: Self::Arg) -> Self where Self: Sized {
                            Self::#variant_name(arg)
                        }
                    }
                }
            });

            let value = optional.then(|| quote!(Some(value))).unwrap_or_else(|| quote!(value));

            Some((
                optional_unique_impls,
                (
                    quote!(#variant_name(#field_required_type)),
                    quote!(UniqueWhereParam::#variant_name(value) =>
                        Self::#field_pascal(_prisma::read_filters::#filter_enum::Equals(#value))
                    ),
                )
            ))
        }
        Variant::CompoundUniqueVariant { field_names_string, variant_data_destructured, variant_data_types } => {
            let variant_name = format_ident!("{}Equals", field_names_string);

            Some((
                None,
                (
                    quote!(#variant_name(#(#variant_data_types),*)),
                    quote!(UniqueWhereParam::#variant_name(#(#variant_data_destructured),*)
                        => Self::#variant_name(#(#variant_data_destructured),*)
                    )
                )
            ))
        }
        _ => None,
    }).unzip();

    quote! {
        #[derive(Clone)]
        pub enum WhereParam {
            #(#variants),*
        }

        impl #pcr::WhereInput for WhereParam {
            fn serialize(self) -> #pcr::SerializedWhereInput {
                let (name, value) = match self {
                    #(#to_serialized_where),*
                };

                #pcr::SerializedWhereInput::new(name, value.into())
            }
        }

        #[derive(Clone)]
        pub enum UniqueWhereParam {
            #(#unique_variants),*
        }

        impl From<UniqueWhereParam> for WhereParam {
            fn from(value: UniqueWhereParam) -> Self {
                match value {
                    #(#unique_to_where_arms),*
                }
            }
        }

        #(#optional_unique_impls)*

        impl From<#pcr::Operator<Self>> for WhereParam {
            fn from(op: #pcr::Operator<Self>) -> Self {
                match op {
                    #pcr::Operator::Not(value) => Self::Not(value),
                    #pcr::Operator::And(value) => Self::And(value),
                    #pcr::Operator::Or(value) => Self::Or(value),
                }
            }
        }
    }
}
