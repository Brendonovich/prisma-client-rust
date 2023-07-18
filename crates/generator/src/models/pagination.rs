use crate::prelude::*;

pub fn fetch_builder_fns(model_name_snake: &Ident) -> TokenStream {
    quote! {
        pub fn skip(mut self, value: i64) -> Self {
            self.0 = self.0.skip(value);
            self
        }

        pub fn take(mut self, value: i64) -> Self {
            self.0 = self.0.take(value);
            self
        }

        pub fn cursor(mut self, value: #model_name_snake::UniqueWhereParam) -> Self {
            self.0 = self.0.cursor(value.into());
            self
        }
    }
}
