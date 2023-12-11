mod arg;
mod filters;
mod item;

use convert_case::*;
use prisma_client_rust_generator_shared::{select_include::Variant, Arity, RelationArity};
use proc_macro2::TokenStream;
use quote::*;
use syn::{
    braced,
    parse::{Parse, ParseStream},
};

pub use arg::*;
pub use filters::*;
pub use item::*;

use super::Input;

pub fn selection(input: &Input, variant: Variant) -> TokenStream {
    let Input {
        dollar, model_path, ..
    } = input;

    let scalar_selections = matches!(variant, Variant::Include).then(|| {
		quote! {
			<#dollar::#model_path::Types as ::prisma_client_rust::ModelTypes>::scalar_selections()
		}
	}).unwrap_or_else(|| quote!(Vec::<::prisma_client_rust::Selection>::new()));

    let selected_selections = selection_to_params(input, variant);

    quote! {
        Selection(
            #scalar_selections
                .into_iter()
                .chain([#(#selected_selections),*].into_iter().map(Into::into))
                .collect()
        )
    }
}

pub fn selection_to_params(input: &Input, variant: Variant) -> Vec<TokenStream> {
    let Input {
        dollar,
        model_path,
        macro_rules,
        selectable_fields,
        ..
    } = input;

    let variant_param = variant.param();
    let variant_pascal = format_ident!("{}", variant.to_string().to_case(Case::Pascal));

    macro_rules
        .selection
        .iter()
        .map(
            |SelectionItem {
                 name,
                 filters,
                 args,
                 sub_selection,
             }| {
                let Some(selectable_item) = selectable_fields
					.iter()
					.find(|field| &field.name == name) else {
						return quote_spanned!(name.span() => compile_error!("Field not found in selectable fields"))
					};

                let into = quote!(Into::<#dollar::#model_path::#variant_param>::into);
                let variant_type_path = quote!(#dollar::#model_path::#name::#variant_pascal);

                let filters = filters
                    .as_ref()
                    .map(|s| quote!(#s))
                    .unwrap_or_else(|| quote!(vec![]));

                match &selectable_item.arity {
                    Arity::Scalar => quote! {
                        #into(#variant_type_path)
                    },
                    Arity::Relation(relation_model_path, relation_arity) => {
                        match (relation_arity, sub_selection) {
                            (RelationArity::Many, None) => quote! {
                                #into(
                                    #variant_type_path::Fetch(
                                        #dollar::#relation_model_path::ManyArgs::new(
                                            #filters
                                        ) #(#args)*
                                    )
                                )
                            },
                            (RelationArity::Many, Some((selection_variant, selection))) => quote! {
                                #into(
                                    #variant_type_path::#selection_variant(
                                        #dollar::#relation_model_path::ManyArgs::new(
                                            #filters
                                        ) #(#args)*,
                                        #dollar::#relation_model_path::#selection_variant! {
                                            selection_to_params @ #selection
                                        }.into_iter().collect()
                                    )
                                )
                            },
                            (RelationArity::One | RelationArity::Optional, None) => quote! {
                                #into(#variant_type_path::Fetch)
                            },
                            (
                                RelationArity::One | RelationArity::Optional,
                                Some((selection_variant, selection)),
                            ) => quote! {
                                #into(
                                    #variant_type_path::#selection_variant(
                                        #dollar::#relation_model_path::#selection_variant! {
                                            selection_to_params @ #selection
                                        }.into_iter().collect()
                                    )
                                )
                            },
                        }
                    }
                }
            },
        )
        .collect()
}

pub fn selection_struct(input: &Input, variant: Variant) -> TokenStream {
    let Input {
        dollar, model_path, ..
    } = input;

    let selection_type = variant.type_trait();

    quote! {
        pub struct Selection(Vec<::prisma_client_rust::Selection>);

        impl ::prisma_client_rust::#selection_type for Selection {
            type Data = Data;
            type ModelData = #dollar::#model_path::Data;

            fn to_selections(self) -> Vec<::prisma_client_rust::Selection> {
                self.0
            }
        }
    }
}

#[derive(Debug)]
pub struct Selection(Vec<SelectionItem>);

impl Selection {
    pub fn iter(&self) -> impl Iterator<Item = &SelectionItem> {
        self.0.iter()
    }
}

impl IntoIterator for Selection {
    type Item = SelectionItem;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Parse for Selection {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let contents;
        braced!(contents in input);

        let mut ret = vec![];

        while !contents.is_empty() {
            ret.push(contents.parse()?);
        }

        Ok(Self(ret))
    }
}
