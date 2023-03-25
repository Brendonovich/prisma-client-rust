use proc_macro2::Ident;
use quote::quote;
use syn::{
    bracketed, parse::Parse, parse_macro_input, punctuated::Punctuated, ItemStruct, Path, Token,
};

#[proc_macro]
pub fn to_pascal_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let converted = convert_case::Casing::to_case(&input.to_string(), convert_case::Case::Pascal);

    proc_macro::TokenTree::Literal(proc_macro::Literal::string(&converted)).into()
}

struct PartialInput {
    model_module: Path,
    data: ItemStruct,
    selection: Punctuated<Ident, Token![,]>,
}

impl Parse for PartialInput {
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

#[proc_macro]
pub fn partial(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let PartialInput {
        model_module,
        data,
        selection,
    } = parse_macro_input!(input as PartialInput);

    let fields = data
        .fields
        .iter()
        .filter(|f| selection.iter().any(|s| s == f.ident.as_ref().unwrap()))
        .map(|field| {
            let attrs = &field.attrs;
            let ident = &field.ident;
            let ty = &field.ty;

            quote! {
                #(#attrs)*
                #ident: Option<#ty>
            }
        });

    let ident = &data.ident;

    let selection = selection.iter().collect::<Vec<_>>();

    quote!(
        #[derive(serde::Deserialize)]
        #[allow(unused)]
        pub struct #ident {
           #(#fields),*
        }

        impl #ident {
            pub fn to_params(self) -> Vec<#model_module::SetParam> {
                [
                    #(self.#selection.map(#model_module::#selection::set)),*
                ].into_iter().flatten().collect()
            }
        }
    )
    .into()
}
