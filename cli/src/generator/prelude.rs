pub use datamodel::dml;
pub use prisma_client_rust_sdk::*;
pub use proc_macro2::*;
pub use quote::*;
pub use syn::Ident;

pub fn snake_ident(name: &str) -> Ident {
    format_ident!("{}", name.to_case(Case::Snake))
}

pub fn pascal_ident(name: &str) -> Ident {
    format_ident!("{}", name.to_case(Case::Pascal))
}
