use crate::generator::prelude::*;

pub fn model_macro<'a>(model: &'a dml::Model, module_path: &TokenStream) -> TokenStream {
    let model_name_snake = snake_ident(&model.name);
    let model_name_snake_raw = snake_ident_raw(&model.name);
    let macro_name = format_ident!("_partial_{model_name_snake_raw}");

    let model_module = quote!($crate::#module_path::#model_name_snake);

    let field_type_arms = model.scalar_fields().map(|scalar_field| {
        let field_name_snake = snake_ident(&scalar_field.name);
        let field_type = scalar_field
            .field_type
            .to_tokens(quote!(crate::#module_path::), &scalar_field.arity);

        quote! {
            (@field_type; #field_name_snake) => { #field_type };
        }
    });

    let to_params_fn = {
        quote! {
            pub fn to_params(self) -> Vec<#model_module::SetParam> {
                [
                    $(self.$scalar_field.map(#model_module::$scalar_field::set)),+
                ].into_iter().flatten().collect()
            }
        }
    };

    quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ($vis:vis struct $struct_name:ident {
                $($scalar_field:ident)+
            }) => {
                $vis struct $struct_name {
                    $(pub $scalar_field: Option<#model_module::partial!(@field_type; $scalar_field)>),+
                }

                impl $struct_name {
                    #to_params_fn
                }
            };
            #(#field_type_arms)*
        }

        pub use #macro_name as partial;
    }
}
