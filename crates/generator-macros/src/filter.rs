use prisma_client_rust_generator_shared::{select_include::SelectableFields, Arity};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, Path, Token,
};

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
    fn parse(input: ParseStream) -> syn::Result<Self> {
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
    dollar_crate: Ident,
    module_path: Path,
    fields: SelectableFields,
    filter: Punctuated<Filter, Token![,]>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            dollar_crate: input.parse()?,
            module_path: {
                input.parse::<Token![,]>()?;
                input.parse()?
            },
            fields: {
                input.parse::<Token![,]>()?;
                input.parse()?
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
        dollar_crate,
        module_path: model_path,
        fields,
        filter,
    } = parse_macro_input!(input as Input);

    let items = filter
        .into_iter()
        .map(|Filter { field, methods }| {
            let Some(field) = fields
                .iter()
                .find(|schema_field| schema_field.name == field)
            else {
                let all_fields = fields
                    .iter()
                    .map(|field| format!("'{}'", field.name.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ");

                let error =
                    format!("Field '{field}' not found. Available fields are {all_fields}.");

                return quote_spanned!(field.span() => compile_error!(#error));
            };

            let field_name = &field.name;

            match &field.arity {
                Arity::Scalar => {
                    let methods = methods.into_iter().map(|Method { name, value }| {
                        quote! {
                            #dollar_crate::#model_path::#field_name::#name(#value)
                        }
                    });

                    quote!(#(#methods),*)
                }
                Arity::Relation(related_model_path, _) => {
                    let methods = methods.into_iter().map(|Method { name, value }| {
                        quote! {
                            #dollar_crate::#model_path::#field_name::#name(
                                #dollar_crate::#related_model_path::filter! #value
                            )
                        }
                    });

                    quote!(#(#methods),*)
                }
            }
        })
        .collect::<Vec<_>>();

    quote!(vec![#(#items),*]).into()
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
                    $crate,
                    #rest,
                    { $($inner)+ }
                )
            };
        }
        pub use #name as filter;
    }
    .into()
}
