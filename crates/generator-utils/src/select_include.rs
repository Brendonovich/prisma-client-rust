use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Token,
};

use crate::FieldTuple;

mod kw {
    syn::custom_keyword!(select);
    syn::custom_keyword!(include);
}

#[derive(Debug, Clone, Copy)]
pub enum Variant {
    Select,
    Include,
}

impl Variant {
    pub fn type_trait(&self) -> Ident {
        format_ident!("{}Type", self.to_string().to_case(Case::Pascal))
    }

    pub fn param(&self) -> Ident {
        format_ident!("{}Param", self.to_string().to_case(Case::Pascal))
    }
}

impl Parse for Variant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::select) {
            input.parse::<kw::select>().map(|_| Self::Select)
        } else if input.peek(kw::include) {
            input.parse::<kw::include>().map(|_| Self::Include)
        } else {
            Err(input.error("expected 'select' or 'include'"))
        }
    }
}

impl core::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Select => "select",
            Self::Include => "include",
        };

        write!(f, "{}", s)
    }
}

impl ToTokens for Variant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("{self}");
        tokens.extend(quote!(#ident));
    }
}

#[derive(Debug)]
pub struct SelectableFields(pub Vec<FieldTuple>);

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
