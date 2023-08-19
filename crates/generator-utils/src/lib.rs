use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    Ident, Path, Token,
};

pub mod select_include;

#[derive(Debug)]
pub enum RelationArity {
    One,
    Many,
}

impl Parse for RelationArity {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;

        Ok(match ident.to_string().as_str() {
            "One" => Self::One,
            "Many" => Self::Many,
            _ => return Err(syn::Error::new_spanned(ident, "expected `One` or `Many`")),
        })
    }
}

impl ToTokens for RelationArity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::One => quote!(One),
            Self::Many => quote!(Many),
        })
    }
}

#[derive(Debug)]
pub enum Arity {
    Scalar,
    Relation(Path, RelationArity),
}

impl Parse for Arity {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;

        Ok(match ident.to_string().as_str() {
            "Scalar" => Self::Scalar,
            "Relation" => {
                let content;
                parenthesized!(content in input);

                Self::Relation(content.parse()?, {
                    content.parse::<Token![,]>()?;
                    content.parse()?
                })
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    ident,
                    "expected `Scalar` or `Relation`",
                ))
            }
        })
    }
}

impl ToTokens for Arity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Scalar => tokens.extend(quote!(Scalar)),
            Self::Relation(path, arity) => tokens.extend(quote!(Relation(#path, #arity))),
        }
    }
}

#[derive(Debug)]
pub struct FieldTuple {
    pub name: Ident,
    pub arity: Arity,
}

impl Parse for FieldTuple {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

        Ok(Self {
            name: content.parse()?,
            arity: {
                content.parse::<Token![,]>()?;
                content.parse()?
            },
        })
    }
}

impl ToTokens for FieldTuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, arity } = self;

        tokens.extend(quote!((#name, #arity)));
    }
}
