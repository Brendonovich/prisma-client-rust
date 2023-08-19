mod definitions;
mod selection;

use prisma_client_rust_generator_shared::select_include::SelectableFields;
use proc_macro2::TokenStream;
use quote::*;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    *,
};

use definitions::*;
use selection::*;

pub use prisma_client_rust_generator_shared::{
    select_include::Variant, Arity, FieldTuple, RelationArity,
};

#[derive(Debug)]
enum SectionOrName {
    Section(Section),
    Name(Ident),
}

impl Parse for SectionOrName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(Token![@]) {
            let value = input.parse()?;
            input.parse::<Token![@]>()?;

            Ok(Self::Section(value))
        } else if input.peek(Ident) {
            Ok(Self::Name(input.parse()?))
        } else {
            Err(input.error("expected section or name"))
        }
    }
}

#[derive(Debug)]
struct MacroRulesInput {
    section: Option<Section>,
    // TODO: merge with name to have more accurate types
    fn_args: Option<Vec<FnArg>>,
    name: Option<Ident>,
    pub selection: Selection,
}

impl Parse for MacroRulesInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            section: {
                if input.peek(Ident) && input.peek2(Token![@]) {
                    let value = input.parse()?;
                    input.parse::<Token![@]>()?;
                    Some(value)
                } else {
                    None
                }
            },
            fn_args: {
                if input.peek(Paren) {
                    let content;
                    parenthesized!(content in input);

                    let args = Punctuated::<FnArg, Token![,]>::parse_terminated(&content)?
                        .into_iter()
                        .collect();

                    input.parse::<Token![=>]>()?;

                    Some(args)
                } else {
                    None
                }
            },
            name: input.parse()?,
            selection: input.parse()?,
        })
    }
}

#[derive(Debug)]
enum Section {
    Definitions,
    SelectionToParams,
}

impl Parse for Section {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value = input.parse::<Ident>()?;

        match value.to_string().as_str() {
            "definitions" => Ok(Self::Definitions),
            "selection_to_params" => Ok(Self::SelectionToParams),
            _ => Err(input.error("expected 'definitions' or 'selection_to_params'")),
        }
    }
}

#[derive(Debug)]
pub struct Input {
    dollar: Ident,
    model_path: Path,
    schema_struct: ItemStruct,
    selectable_fields: SelectableFields,
    macro_rules: MacroRulesInput,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            dollar: input.parse()?,
            model_path: {
                input.parse::<Token![,]>()?;
                input.parse()?
            },
            schema_struct: {
                input.parse::<Token![,]>()?;
                input.parse()?
            },
            selectable_fields: {
                input.parse::<Token![,]>()?;

                input.parse()?
            },
            macro_rules: {
                input.parse::<Token![,]>()?;

                let contents;
                braced!(contents in input);
                contents.parse()?
            },
        })
    }
}

pub fn proc_macro(input: proc_macro::TokenStream, variant: Variant) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Input);

    let Input { macro_rules, .. } = &input;

    match &macro_rules.section {
        Some(Section::Definitions) => definitions(&input),
        Some(Section::SelectionToParams) => {
            let params = selection_to_params(&input, variant);
            quote!([#(#params),*])
        }
        None => {
            let definitions = definitions(&input);
            let selection_struct = selection_struct(&input, variant);
            let selection = selection(&input, variant);

            let variant_ident = format_ident!("{variant}");

            match &macro_rules.name {
                Some(module_name) => {
                    let fn_args = macro_rules
                        .fn_args
                        .as_ref()
                        .map(|fn_args| {
                            let iter = fn_args.iter();
                            quote!(#(#iter),*)
                        })
                        .unwrap_or_default();

                    quote! {
                        #[allow(warnings)]
                        pub mod #module_name {
                            use super::*;

                            #definitions
                            #selection_struct

                            pub fn #variant_ident(#fn_args) -> Selection {
                                #selection
                            }
                        }
                    }
                }
                None => {
                    quote! {{
                        #definitions
                        #selection_struct
                        #selection
                    }}
                }
            }
        }
    }
    .into()
}

struct FactoryInput {
    internal_name: Ident,
    public_name: Ident,
    // model_path & fields
    rest: TokenStream,
}

impl Parse for FactoryInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            internal_name: input.parse()?,
            public_name: {
                input.parse::<Token![,]>()?;
                input.parse()?
            },
            rest: {
                input.parse::<Token![,]>()?;
                input.parse()?
            },
        })
    }
}

// factory means rustfmt can work!
pub fn proc_macro_factory(
    input: proc_macro::TokenStream,
    variant: Variant,
) -> proc_macro::TokenStream {
    let FactoryInput {
        internal_name,
        public_name,
        rest,
    } = parse_macro_input!(input as FactoryInput);

    quote! {
        #[macro_export]
        macro_rules! #internal_name {
            ($($input:tt)+) => {
                ::prisma_client_rust::macros::#variant! {
                    $crate,
                    #rest,
                    { $($input)+ }
                }
            };
        }
        pub use #internal_name as #public_name;
    }
    .into()
}
