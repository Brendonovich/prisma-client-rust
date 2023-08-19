use prisma_client_rust_generator_shared::select_include::Variant;
use proc_macro2::TokenStream;
use quote::*;
use syn::{
    parse::{Parse, ParseStream},
    token::Paren,
    *,
};

use super::*;

#[derive(Debug)]
pub struct SelectionItem {
    pub name: Ident,
    pub filters: Option<SelectionFilters>,
    pub args: Vec<SelectionArg>,
    // We don't parse here as we don't care about subselection.
    // That gets passed on to another macro invoction.
    pub sub_selection: Option<(Variant, TokenStream)>,
}

impl Parse for SelectionItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            filters: {
                if input.peek(Paren) {
                    Some(input.parse()?)
                } else {
                    None
                }
            },
            args: {
                let mut ret = vec![];

                while input.peek(Token![.]) {
                    ret.push(input.parse()?);
                }

                ret
            },
            sub_selection: {
                if input.peek(Token![:]) {
                    input.parse::<Token![:]>()?;

                    let variant = input.parse()?;

                    let content;
                    braced!(content in input);
                    // parse separately to re-wrap in braces for Selection::parse
                    let content = content.parse::<TokenStream>()?;

                    Some((variant, quote!({ #content })))
                } else {
                    None
                }
            },
        })
    }
}
