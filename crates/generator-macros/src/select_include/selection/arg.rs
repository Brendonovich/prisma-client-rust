use proc_macro2::TokenStream;
use quote::*;
use syn::{
    parse::{Parse, ParseStream},
    *,
};

#[derive(Debug)]
pub struct SelectionArg {
    name: Ident,
    values: TokenStream,
}

impl Parse for SelectionArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![.]>()?;

        Ok(Self {
            name: input.parse()?,
            values: {
                let content;
                parenthesized!(content in input);

                content.parse()?
            },
        })
    }
}

impl ToTokens for SelectionArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, values } = self;
        tokens.extend(quote!(.#name(#values)))
    }
}
