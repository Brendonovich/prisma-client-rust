use quote::{__private::TokenStream, format_ident, quote};

use crate::generator::{GeneratorArgs};

pub fn generate(_root: &GeneratorArgs) -> TokenStream {
    let gql_rs_ident = format_ident!("gql_rs");

    quote! {
        #![allow(warnings, unused)]

        use #gql_rs_ident::*;
    }
}
