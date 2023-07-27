mod filter;
mod partial_unchecked;

#[proc_macro]
pub fn to_pascal_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let converted = convert_case::Casing::to_case(&input.to_string(), convert_case::Case::Pascal);

    proc_macro::TokenTree::Literal(proc_macro::Literal::string(&converted)).into()
}
#[proc_macro]
pub fn partial_unchecked(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    partial_unchecked::proc_macro(input)
}

#[proc_macro]
pub fn filter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    filter::proc_macro(input)
}

#[proc_macro]
pub fn filter_factory(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    filter::proc_macro_factory(input)
}
