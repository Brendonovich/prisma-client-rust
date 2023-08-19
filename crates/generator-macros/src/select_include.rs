use convert_case::{Case, Casing};
use prisma_client_rust_generator_shared::select_include::SelectableFields;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Paren,
    Field, FnArg, Ident, ItemStruct, Path, Token,
};

pub use prisma_client_rust_generator_shared::{
    select_include::Variant, Arity, FieldTuple, RelationArity,
};

#[derive(Debug)]
struct SelectionArg {
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

#[derive(Debug)]
struct SelectionFilters(TokenStream);

impl Parse for SelectionFilters {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

        Ok(Self(content.parse()?))
    }
}

impl ToTokens for SelectionFilters {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(inner) = self;
        tokens.extend(quote!((#inner)))
    }
}

#[derive(Debug)]
struct SelectionItem {
    name: Ident,
    filters: Option<SelectionFilters>,
    args: Vec<SelectionArg>,
    // We don't parse here as we don't care about subselection.
    // That gets passed on to another macro invoction.
    sub_selection: Option<(Variant, TokenStream)>,
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

#[derive(Debug)]
struct Selection(Vec<SelectionItem>);

impl Selection {
    fn iter(&self) -> impl Iterator<Item = &SelectionItem> {
        self.0.iter()
    }
}

impl IntoIterator for Selection {
    type Item = SelectionItem;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Parse for Selection {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let contents;
        braced!(contents in input);

        let mut ret = vec![];

        while !contents.is_empty() {
            ret.push(contents.parse()?);
        }

        Ok(Self(ret))
    }
}

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
    selection: Selection,
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
struct Input {
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

fn definitions(input: &Input) -> TokenStream {
    let Input {
        dollar,
        model_path,
        schema_struct,
        selectable_fields,
        macro_rules,
        ..
    } = input;

    let data_struct = {
        let mut attrs = quote! {
            #[allow(warnings)]
            #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
        };

        if cfg!(feature = "specta") {
            attrs.extend(quote! {
                #[derive(::prisma_client_rust::specta::Type)]
                #[specta(crate = "prisma_client_rust::specta")]
            });

            attrs.extend(match &macro_rules.name {
                None => quote!(#[specta(inline)]),
                Some(name) => {
                    let name_pascal = name.to_string().to_case(convert_case::Case::Pascal);

                    quote! {
                        #[specta(rename = #name_pascal)]
                    }
                }
            });
        }

        let (fields, field_modules): (Vec<_>, Vec<_>) = schema_struct
            .fields
            .iter()
            .filter_map(|field| {
                let Field {
                    attrs, ty, ident, ..
                } = &field;

                let field_in_selectables = selectable_fields
                    .iter()
                    .find(|item| Some(&item.name) == ident.as_ref());
                let field_in_selection = macro_rules
                    .selection
                    .iter()
                    .find(|item| Some(&item.name) == ident.as_ref());

                if field_in_selectables.is_some() && field_in_selection.is_none() {
                    return None;
                }

                let field = quote! {
                    #(#attrs)*
                    pub #ident: #dollar::#model_path::#ty
                };

                let field_module = field_in_selectables
                    .zip(field_in_selection.and_then(|f| f.sub_selection.as_ref()))
                    .and_then(|(field_in_selectables, (variant, sub_selection))| {
                    	let Arity::Relation(relation_model_path, _) = &field_in_selectables.arity else {
                      		return None;
                      	};

	                    let value = quote! {
	                        pub mod #ident {
	                            #dollar::#relation_model_path::#variant! {
									definitions @ #sub_selection
								}
	                        }
	                    };

                        Some(value)
                    });

                Some((field, field_module))
            })
            .unzip();

        quote! {
            #attrs
            pub struct Data {
                #(#fields),*
            }

            #(#field_modules)*
        }
    };

    quote! {
        #data_struct
    }
}

fn selection_struct(input: &Input, variant: Variant) -> TokenStream {
    let Input {
        dollar, model_path, ..
    } = input;

    let selection_type = variant.type_trait();

    quote! {
        pub struct Selection(Vec<::prisma_client_rust::Selection>);

        impl ::prisma_client_rust::#selection_type for Selection {
            type Data = Data;
            type ModelData = #dollar::#model_path::Data;

            fn to_selections(self) -> Vec<::prisma_client_rust::Selection> {
                self.0
            }
        }
    }
}

fn selection(input: &Input, variant: Variant) -> TokenStream {
    let Input {
        dollar, model_path, ..
    } = input;

    let scalar_selections = matches!(variant, Variant::Include).then(|| {
        quote! {
            <#dollar::#model_path::Types as ::prisma_client_rust::ModelTypes>::scalar_selections()
        }
    }).unwrap_or_else(|| quote!(Vec::<::prisma_client_rust::Selection>::new()));

    let selected_selections = selection_to_params(input, variant);

    quote! {
        Selection(
            #scalar_selections
                .into_iter()
                .chain([#(#selected_selections),*].into_iter().map(Into::into))
                .collect()
        )
    }
}

fn selection_to_params(input: &Input, variant: Variant) -> Vec<TokenStream> {
    let Input {
        dollar,
        model_path,
        macro_rules,
        selectable_fields,
        ..
    } = input;

    let variant_param = variant.param();
    let variant_pascal = format_ident!("{}", variant.to_string().to_case(Case::Pascal));

    macro_rules
        .selection
        .iter()
        .map(
            |SelectionItem {
                 name,
                 filters,
                 args,
                 sub_selection,
             }| {
                let Some(selectable_item) = selectable_fields
				.iter()
				.find(|field| &field.name == name) else {
					return quote_spanned!(name.span() => compile_error!("Field not found in selectable fields"))
				};

                let into = quote!(Into::<#dollar::#model_path::#variant_param>::into);
                let variant_type_path = quote!(#dollar::#model_path::#name::#variant_pascal);

                let filters = filters
                    .as_ref()
                    .map(|s| quote!(#s))
                    .unwrap_or_else(|| quote!(vec![]));

                match &selectable_item.arity {
                    Arity::Scalar => quote! {
                        #into(#variant_type_path)
                    },
                    Arity::Relation(relation_model_path, relation_arity) => {
                        match (relation_arity, sub_selection) {
                            (RelationArity::One, None) => quote! {
                                #into(#variant_type_path::Fetch)
                            },
                            (RelationArity::One, Some((selection_variant, selection))) => quote! {
                                #into(
                                    #variant_type_path::#selection_variant(
                                        #dollar::#relation_model_path::#selection_variant! {
                                            selection_to_params @ #selection
                                        }.into_iter().collect()
                                    )
                                )
                            },
                            (RelationArity::Many, None) => quote! {
                                #into(
                                    #variant_type_path::Fetch(
                                        #dollar::#relation_model_path::ManyArgs::new(
                                            #filters
                                        ) #(#args)*
                                    )
                                )
                            },
                            (RelationArity::Many, Some((selection_variant, selection))) => quote! {
                                #into(
                                    #variant_type_path::#selection_variant(
                                        #dollar::#relation_model_path::ManyArgs::new(
                                            #filters
                                        ) #(#args)*,
                                        #dollar::#relation_model_path::#selection_variant! {
                                            selection_to_params @ #selection
                                        }.into_iter().collect()
                                    )
                                )
                            },
                        }
                    }
                }
            },
        )
        .collect()
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
