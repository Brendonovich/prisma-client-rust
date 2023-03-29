use proc_macro2::Ident;
use quote::quote;
use syn::{
    bracketed, parse::Parse, parse_macro_input, punctuated::Punctuated, ItemStruct, Path, Token,
};

struct PartialUncheckedInput {
    model_module: Path,
    data: ItemStruct,
    selection: Punctuated<Ident, Token![,]>,
}

impl Parse for PartialUncheckedInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            model_module: input.parse()?,
            data: input.parse()?,
            selection: {
                let content;
                bracketed!(content in input);
                Punctuated::<Ident, Token![,]>::parse_terminated(&content)?
            },
        })
    }
}

pub fn proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let PartialUncheckedInput {
        model_module,
        data,
        selection,
    } = parse_macro_input!(input as PartialUncheckedInput);

    let fields = data
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
                #ident: Option<#ty>
            }
        });

    let specta_attrs = cfg!(feature = "specta").then(|| {
        quote! {
            #[derive(::prisma_client_rust::specta::Type)]
            #[specta(crate = "prisma_client_rust::specta")]
        }
    });

    let ident = &data.ident;

    let selection = selection.iter().collect::<Vec<_>>();

    quote! {
        #[derive(serde::Deserialize)]
        #specta_attrs
        #[allow(unused)]
        pub struct #ident {
           #(#fields),*
        }

        impl #ident {
            pub fn to_params(self) -> Vec<#model_module::UncheckedSetParam> {
                [
                    #(self.#selection.map(#model_module::#selection::set)),*
                ].into_iter().flatten().collect()
            }
        }
    }
    .into()
}
