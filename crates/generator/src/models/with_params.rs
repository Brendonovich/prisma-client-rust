use prisma_client_rust_sdk::prisma::prisma_models::{
    walkers::{ModelWalker, RelationFieldWalker},
    FieldArity,
};

use crate::prelude::*;

use super::{order_by, pagination, ModelModulePart};

pub fn builder_fn(field: RelationFieldWalker) -> TokenStream {
    let relation_model_name_snake = snake_ident(field.related_model().name());

    quote! {
        pub fn with(mut self, params: impl Into<#relation_model_name_snake::WithParam>) -> Self {
            self.0 = self.0.with(params.into());
            self
        }
    }
}

fn enum_variant(field: RelationFieldWalker) -> TokenStream {
    let field_name_pascal = pascal_ident(field.name());
    let relation_model_name_snake = snake_ident(field.related_model().name());

    let args = match field.ast_field().arity {
        FieldArity::List => quote!(ManyArgs),
        _ => quote!(UniqueArgs),
    };

    quote!(#field_name_pascal(super::#relation_model_name_snake::#args))
}

fn into_selection_arm(field: RelationFieldWalker) -> TokenStream {
    let field_name_snake = snake_ident(field.name());
    let field_name_pascal = pascal_ident(field.name());
    let relation_model_name_snake = snake_ident(field.related_model().name());

    let pcr = quote!(::prisma_client_rust);

    let body = match field.ast_field().arity {
        FieldArity::List => quote! {
            let (arguments, mut nested_selections) = args.to_graphql();
            nested_selections.extend(<super::#relation_model_name_snake::Types as #pcr::ModelTypes>::scalar_selections());

            #pcr::Selection::new(
                #field_name_snake::NAME,
                None,
                arguments,
                nested_selections
            )
        },
        _ => quote! {
            let mut selections = <super::#relation_model_name_snake::Types as #pcr::ModelTypes>::scalar_selections();
            selections.extend(args.with_params.into_iter().map(Into::<#pcr::Selection>::into));

            #pcr::Selection::new(
                #field_name_snake::NAME,
                None,
                [],
                selections
            )
        },
    };

    quote! {
        Self::#field_name_pascal(args) => {
            #body
        }
    }
}

pub fn model_data(model: ModelWalker) -> ModelModulePart {
    let variants = model.relation_fields().map(enum_variant);
    let into_selection_arms = model.relation_fields().map(into_selection_arm);

    let fields = model
        .relation_fields()
        .map(|field| {
            let field_name_pascal = pascal_ident(field.name());

            let relation_model_name_snake = snake_ident(field.related_model().name());

            let order_by_fn = order_by::fetch_builder_fn(&relation_model_name_snake);
            let pagination_fns = pagination::fetch_builder_fns(&relation_model_name_snake);
            let with_fn = builder_fn(field);

            let body = match field.referential_arity() {
                FieldArity::List => {
                    quote! {
                        pub struct Fetch(pub #relation_model_name_snake::ManyArgs);

                        impl Fetch {
                            #with_fn

                            #order_by_fn

                            #pagination_fns
                        }

                        impl From<Fetch> for WithParam {
                            fn from(Fetch(v): Fetch) -> Self {
                                WithParam::#field_name_pascal(v)
                            }
                        }

                        pub fn fetch(params: Vec<#relation_model_name_snake::WhereParam>) -> Fetch {
                            Fetch(#relation_model_name_snake::ManyArgs::new(params))
                        }
                    }
                }
                _ => {
                    quote! {
                        pub struct Fetch(pub #relation_model_name_snake::UniqueArgs);

                        impl Fetch {
                            #with_fn
                        }

                        impl From<Fetch> for WithParam {
                            fn from(Fetch(v): Fetch) -> Self {
                                WithParam::#field_name_pascal(v)
                            }
                        }

                        pub fn fetch() -> Fetch {
                            Fetch(#relation_model_name_snake::UniqueArgs::new())
                        }
                    }
                }
            };

            (field.name().to_string(), body)
        })
        .collect();

    ModelModulePart {
        data: quote! {
            #[derive(Debug, Clone)]
            pub enum WithParam {
                #(#variants),*
            }

            impl Into<::prisma_client_rust::Selection> for WithParam {
                fn into(self) -> ::prisma_client_rust::Selection {
                    match self {
                        #(#into_selection_arms),*
                    }
                }
            }
        },
        fields,
    }
}
