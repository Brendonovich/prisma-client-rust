use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, Token,
};

enum Arity {
    Scalar,
    Relation(Ident),
}

impl Parse for Arity {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;

        Ok(match ident.to_string().as_str() {
            "Scalar" => Self::Scalar,
            "Relation" => Self::Relation({
                let content;
                parenthesized!(content in input);
                content.parse()?
            }),
            _ => {
                return Err(syn::Error::new_spanned(
                    ident,
                    "expected `Scalar` or `Relation`",
                ))
            }
        })
    }
}

struct FieldTuple {
    name: Ident,
    arity: Arity,
}

impl Parse for FieldTuple {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
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

struct Method {
    name: Ident,
    value: TokenStream,
}

impl Parse for Method {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            value: {
                input.parse::<Token![:]>()?;
                input.parse()?
            },
        })
    }
}

struct Filter {
    field: Ident,
    methods: Punctuated<Method, Token![,]>,
}

impl Parse for Filter {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            field: input.parse()?,
            methods: {
                input.parse::<Token![:]>()?;

                let content;
                braced!(content in input);

                Punctuated::parse_terminated(&content)?
            },
        })
    }
}

struct Input {
    model_name: Ident,
    fields: Punctuated<FieldTuple, Token![,]>,
    filter: Punctuated<Filter, Token![,]>,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            model_name: input.parse()?,
            fields: {
                input.parse::<Token![,]>()?;

                let content;
                bracketed!(content in input);
                Punctuated::parse_terminated(&content)?
            },
            filter: {
                input.parse::<Token![,]>()?;

                let content;
                braced!(content in input);

                Punctuated::parse_terminated(&content)?
            },
        })
    }
}

pub fn proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input {
        model_name,
        fields,
        filter,
    } = parse_macro_input!(input as Input);

    let items = filter
        .into_iter()
        .map(|Filter { field, methods }| {
            let Some(field) = fields.iter().find(|schema_field| schema_field.name == field) else {
            	return quote_spanned!(
             		field.span() => compile_error!("expected field to be one of the model's fields")
            	)
            };

            let field_name = &field.name;

            match &field.arity {
                Arity::Scalar => {
                    let methods = methods.into_iter().map(
                        |Method { name, value }| quote!(#model_name::#field_name::#name(#value)),
                    );

                    quote!(#(#methods),*)
                }
                Arity::Relation(related_model) => {
                    let methods = methods.into_iter().map(
						|Method { name, value }| quote!(#model_name::#field_name::#name(#related_model::filter! #value)),
					);

                    quote!(#(#methods),*)
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        vec![
            #(#items),*
        ]
    }
    .into()
}

// factory means rustfmt can work!
pub fn proc_macro_factory(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    struct FactoryInput {
        name: Ident,
        rest: TokenStream,
    }

    impl Parse for FactoryInput {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                name: input.parse()?,
                rest: {
                    input.parse::<Token![,]>()?;
                    input.parse()?
                },
            })
        }
    }

    let FactoryInput { name, rest } = parse_macro_input!(input as FactoryInput);

    quote! {
        #[macro_export]
        macro_rules! #name {
            ($($inner:tt)+) => {
                ::prisma_client_rust::macros::filter!(
                    #rest,
                    { $($inner)+ }
                )
            };
        }
        pub use #name as filter;
    }
    .into()
}
