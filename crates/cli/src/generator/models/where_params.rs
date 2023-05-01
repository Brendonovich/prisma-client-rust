use prisma_client_rust_sdk::prisma::prisma_models::{
    walkers::{ModelWalker, ScalarFieldWalker},
    FieldArity,
};

use crate::generator::{models::field, prelude::*};

use super::SomethingThatNeedsFieldModules;

pub struct Operator {
    pub name: &'static str,
    pub action: &'static str,
    pub list: bool,
}

static OPERATORS: &'static [Operator] = &[
    Operator {
        name: "Not",
        action: "NOT",
        list: false,
    },
    Operator {
        name: "Or",
        action: "OR",
        list: true,
    },
    Operator {
        name: "And",
        action: "AND",
        list: false,
    },
];

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
                            Self::#field_pascal(super::_prisma::read_filters::#filter_enum::Equals(arg))
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
                        Self::#field_pascal(super::_prisma::read_filters::#filter_enum::Equals(#value))
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

                #pcr::SerializedWhereInput::new(name.to_string(), value.into())
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

pub fn model_data(
    model: ModelWalker,
    args: &GenerateArgs,
    module_path: &TokenStream,
) -> SomethingThatNeedsFieldModules {
    let pcr = quote!(::prisma_client_rust);

    let mut entries = vec![];

    entries.extend(OPERATORS.iter().map(|op| {
        let variant_name = pascal_ident(&op.name);
        let op_action = &op.action;

        let value = match op.list {
            true => quote! {
                #pcr::SerializedWhereValue::List(
                    value
                        .into_iter()
                        .map(#pcr::WhereInput::serialize)
                        .map(|p| #pcr::PrismaValue::Object(vec![p.into()]))
                        .collect()
                )
            },
            false => quote! {
                #pcr::SerializedWhereValue::Object(
                    ::prisma_client_rust::merge_fields(
                        value
                            .into_iter()
                            .map(#pcr::WhereInput::serialize)
                            .map(Into::into)
                            .collect()
                    )
                )
            },
        };

        Variant::BaseVariant {
            definition: quote!(#variant_name(Vec<WhereParam>)),
            match_arm: quote! {
                Self::#variant_name(value) => (
                    #op_action,
                    #value,
                )
            },
        }
    }));

    let compound_field_accessors = unique_field_combos(model).iter().flat_map(|fields| {
        if fields.len() == 1 {
            let field = fields[0];

            let read_filter = args.read_filter(
                field
            ).unwrap();

            entries.push(Variant::unique(field, read_filter, module_path));

            None
        } else {
            let variant_name_string = fields.iter().map(|f| pascal_ident(f.name()).to_string()).collect::<String>();
            let variant_name = format_ident!("{}Equals", &variant_name_string);

            let variant_data_names = fields.iter().map(|f| snake_ident(f.name())).collect::<Vec<_>>();

            let ((field_defs, field_types), (prisma_values, field_names_snake)):
                ((Vec<_>, Vec<_>), (Vec<_>, Vec<_>)) = fields.into_iter().map(|field| {
                let field_type = match field.ast_field().arity {
                    FieldArity::List | FieldArity::Required => field.type_tokens(module_path),
                    FieldArity::Optional => field.scalar_field_type().to_tokens(module_path, &FieldArity::Required, field.db)
                }.unwrap();

                let field_name_snake = snake_ident(field.name());

                (
                    (quote!(#field_name_snake: #field_type), field_type),
                    (field.scalar_field_type().to_prisma_value(&field_name_snake, &FieldArity::Required), field_name_snake)
                )
            }).unzip();

            let field_names_joined = fields.iter().map(|f| f.name()).collect::<Vec<_>>().join("_");

            entries.extend([
                Variant::BaseVariant {
                    definition: quote!(#variant_name(#(#field_types),*)),
                    match_arm: quote! {
                        Self::#variant_name(#(#field_names_snake),*) => (
                            #field_names_joined,
                            #pcr::SerializedWhereValue::Object(vec![#((#variant_data_names::NAME.to_string(), #prisma_values)),*])
                        )
                    },
                },
                Variant::CompoundUniqueVariant {
                    field_names_string: variant_name_string.clone(),
                    variant_data_destructured: field_names_snake.clone(),
                    variant_data_types: field_types
                }
            ]);

            let accessor_name = snake_ident(&variant_name_string);

            Some(quote! {
                pub fn #accessor_name<T: From<UniqueWhereParam>>(#(#field_defs),*) -> T {
                    UniqueWhereParam::#variant_name(#(#field_names_snake),*).into()
                }
            })
        }
    }).collect::<TokenStream>();

    let (field_stuff, field_where_param_entries): (_, Vec<_>) = model
        .fields()
        .filter(|f| f.ast_field().field_type.as_unsupported().is_none())
        .map(|f| field::module(f, args, module_path))
        .unzip();

    entries.extend(field_where_param_entries.into_iter().flatten());

    let collated_entries = collate_entries(entries);

    SomethingThatNeedsFieldModules {
        data: quote! {
            #compound_field_accessors
            #collated_entries
        },
        fields: field_stuff,
    }
}

pub fn unique_field_combos(model: ModelWalker) -> Vec<Vec<ScalarFieldWalker>> {
    let mut combos = model
        .indexes()
        .filter(|f| f.is_unique())
        .map(|unique| {
            unique
                .fields()
                .filter_map(|field| {
                    model
                        .scalar_fields()
                        .find(|mf| mf.field_id() == field.field_id())
                })
                .collect()
        })
        .collect::<Vec<_>>();

    if let Some(primary_key) = model.primary_key() {
        let primary_key_is_also_unique = model.indexes().any(|i| {
            primary_key.contains_exactly_fields(
                i.fields()
                    .map(|f| f.as_scalar_field())
                    .flatten()
                    .collect::<Vec<_>>()
                    .into_iter(),
            )
        });

        if !primary_key_is_also_unique {
            combos.push(
                primary_key
                    .fields()
                    .filter_map(|field| {
                        model
                            .scalar_fields()
                            .find(|mf| mf.field_id() == field.field_id())
                    })
                    .collect(),
            );
        }
    }

    combos
}
