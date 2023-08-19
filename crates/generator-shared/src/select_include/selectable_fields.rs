use prisma_client_rust_sdk::prisma::prisma_models::walkers::FieldWalker;
use proc_macro2::TokenStream;
use quote::*;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
};

use crate::FieldTuple;

#[derive(Debug)]
pub struct SelectableFields(Vec<FieldTuple>);

impl SelectableFields {
    pub fn new<'a>(
        fields: impl Iterator<Item = FieldWalker<'a>>,
        module_path: &TokenStream,
    ) -> Self {
        Self(
            fields
                .map(|field| FieldTuple::new(field, module_path))
                .collect(),
        )
    }
}

impl Parse for SelectableFields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let contents;
        bracketed!(contents in input);

        Ok(Self(
            Punctuated::<FieldTuple, Token![,]>::parse_terminated(&contents)?
                .into_iter()
                .collect(),
        ))
    }
}

impl ToTokens for SelectableFields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(fields) = self;

        tokens.extend(quote!([#(#fields),*]))
    }
}

impl SelectableFields {
    pub fn iter(&self) -> impl Iterator<Item = &FieldTuple> {
        self.0.iter()
    }
}
