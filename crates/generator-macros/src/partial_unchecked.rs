use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    ItemStruct, Path, Token,
};

struct Input {
    dollar_crate: Ident,
    model_module: Path,
    data_struct: ItemStruct,
    struct_name: Ident,
    selection: Punctuated<Ident, Token![,]>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            dollar_crate: input.parse()?,
            model_module: {
                input.parse::<Token![,]>()?;
                input.parse()?
            },
            data_struct: {
                input.parse::<Token![,]>()?;
                input.parse()?
            },
            struct_name: {
                input.parse::<Token![,]>()?;
                input.parse()?
            },
            selection: {
                input.parse::<Token![,]>()?;

                let content;
                bracketed!(content in input);
                Punctuated::<Ident, Token![,]>::parse_terminated(&content)?
            },
        })
    }
}

pub fn proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input {
        dollar_crate,
        model_module,
        data_struct,
        struct_name,
        selection,
    } = parse_macro_input!(input as Input);

    let fields = data_struct
        .fields
        .iter()
        .filter(|f| selection.iter().any(|s| s == f.ident.as_ref().unwrap()))
        .map(|field| {
            let attrs = &field.attrs;
            let ident = &field.ident;
            let ty = &field.ty;

            let specta_attrs = cfg!(feature = "specta").then(|| quote!(#[specta(optional)]));

            quote! {
                #(#attrs)*
                #specta_attrs
                pub #ident: Option<#dollar_crate::#ty>
            }
        });

    let specta_attrs = cfg!(feature = "specta").then(|| {
        quote! {
            #[derive(::prisma_client_rust::specta::Type)]
            #[specta(crate = prisma_client_rust::specta)]
        }
    });

    let selection = selection.iter().collect::<Vec<_>>();

    quote! {
        #[derive(serde::Deserialize)]
        #specta_attrs
        #[allow(unused)]
        pub struct #struct_name {
           #(#fields),*
        }

        impl #struct_name {
            pub fn to_params(self) -> Vec<#dollar_crate::#model_module::UncheckedSetParam> {
                [
                    #(self.#selection.map(#dollar_crate::#model_module::#selection::set)),*
                ].into_iter().flatten().collect()
            }
        }
    }
    .into()
}

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
            ($struct_name:ident {
                $($scalar_field:ident)+
            }) => {
                ::prisma_client_rust::macros::partial_unchecked!(
                    $crate,
                    #rest,
                    $struct_name,
                    [$($scalar_field),+]
                );
            }
        }
        pub use #name as partial_unchecked;
    }
    .into()
}
