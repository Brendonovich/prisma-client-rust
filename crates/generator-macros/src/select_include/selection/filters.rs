use proc_macro2::TokenStream;
use quote::*;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
};

#[derive(Debug)]
pub struct SelectionFilters(TokenStream);

impl Parse for SelectionFilters {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

        Ok(Self(content.parse()?))
    }
}

impl ToTokens for SelectionFilters {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(inner) = self;
        tokens.extend(quote!((#inner)))
    }
}
