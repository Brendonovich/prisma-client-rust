use crate::generator::prelude::*;

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

        pub fn cursor(mut self, value: impl Into<#model_name_snake::Cursor>) -> Self {
            self.0 = self.0.cursor(value.into());
            self
        }
    }
}

pub fn field_cursor_type(field: &dml::Field, model: &dml::Model) -> Option<TokenStream> {
    (field.is_scalar_field()
        && (model.field_is_primary(field.name()) || model.field_is_unique(field.name())))
    .then(|| {
        let field_base_type = field.field_type().to_tokens();

        match field.arity() {
            dml::FieldArity::List => quote!(Vec<#field_base_type>),
            _ => field_base_type,
        }
    })
}
pub fn cursor_enum_definition(model: &dml::Model) -> TokenStream {
    let cursor_types = model
        .fields()
        .filter_map(|field| field_cursor_type(field, model).map(|typ| (field, typ)));

    let cursor_variants = cursor_types.clone().map(|(field, cursor_type)| {
        let field_name_pascal = pascal_ident(field.name());

        quote!(#field_name_pascal(#cursor_type))
    });

    let cursor_into_pv_arms = cursor_types.clone().map(|(field, _)| {
        let field_name_pascal = pascal_ident(field.name());
        let field_name_str = field.name();

        let cursor_ident = format_ident!("cursor");

        let prisma_value = field.type_prisma_value(&cursor_ident);

        quote! {
            Self::#field_name_pascal(#cursor_ident) => (
                #field_name_str.to_string(),
                #prisma_value
            )
        }
    });

    quote! {
        #[derive(Clone)]
        pub enum Cursor {
            #(#cursor_variants),*
        }

        impl Into<(String, ::prisma_client_rust::PrismaValue)> for Cursor {
            fn into(self) -> (String, ::prisma_client_rust::PrismaValue) {
                match self {
                    #(#cursor_into_pv_arms),*
                }
            }
        }
    }
}
