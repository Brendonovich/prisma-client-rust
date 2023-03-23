#[proc_macro]
pub fn to_pascal_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let converted = convert_case::Casing::to_case(&input.to_string(), convert_case::Case::Pascal);

    proc_macro::TokenTree::Literal(proc_macro::Literal::string(&converted)).into()
}
