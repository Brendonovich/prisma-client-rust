use prisma_client_rust_sdk::prisma::prisma_models::walkers::ModelWalker;

use crate::generator::prelude::*;

use super::where_params;

pub fn fetch_builder_fns(model: ModelWalker) -> TokenStream {
    let unique_input = where_params::where_unique_input_ident(model);

    quote! {
        pub fn skip(mut self, value: i64) -> Self {
            self.0 = self.0.skip(value);
            self
        }

        pub fn take(mut self, value: i64) -> Self {
            self.0 = self.0.take(value);
            self
        }

        pub fn cursor(mut self, value: #unique_input) -> Self {
            self.0 = self.0.cursor(value.into());
            self
        }
    }
}
