use crate::generator::prelude::*;

pub fn model_macro<'a>(model: &'a dml::Model, module_path: &TokenStream) -> TokenStream {
    let model_name_snake = snake_ident(&model.name);
    let model_name_snake_raw = snake_ident_raw(&model.name);
    let macro_name = format_ident!("_partial_{model_name_snake_raw}");

    let model_module = quote!(#module_path::#model_name_snake);

    let field_type_arms = model.scalar_fields().map(|scalar_field| {
        let field_name_snake = snake_ident(&scalar_field.name);
        let field_type = scalar_field
            .field_type
            .to_tokens(module_path, &scalar_field.arity);

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

    let deserialize_impl = {
        let field_names_str = model.fields().map(|f| f.name());

        quote! {
            #[allow(warnings)]
            enum Field {
                $($scalar_field),+,
            }

            impl<'de> ::serde::Deserialize<'de> for Field {
                fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    struct FieldVisitor;

                    impl<'de> ::serde::de::Visitor<'de> for FieldVisitor {
                        type Value = Field;

                        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                            formatter.write_str(&[
                                $(#model_module::$scalar_field::NAME),+,
                            ].into_iter().collect::<Vec<_>>().join(", "))
                        }

                        fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: ::serde::de::Error,
                        {
                            match value {
                                $(#model_module::$scalar_field::NAME => Ok(Field::$scalar_field)),*,
                                _ => Err(::serde::de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }

                    deserializer.deserialize_identifier(FieldVisitor)
                }
            }

            struct DataVisitor;

            impl<'de> ::serde::de::Visitor<'de> for DataVisitor {
                type Value = $struct_name;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str(concat!("struct ", stringify!($struct_name)))
                }

                fn visit_map<V>(self, mut map: V) -> Result<$struct_name, V::Error>
                where
                    V: ::serde::de::MapAccess<'de>,
                {
                    $(let mut $scalar_field = None;)*

                    while let Some(key) = map.next_key()? {
                        match key {
                            $(Field::$scalar_field => {
                                if $scalar_field.is_some() {
                                    return Err(::serde::de::Error::duplicate_field(
                                        #model_module::$scalar_field::NAME
                                    ));
                                }
                                $scalar_field = Some(map.next_value()?);
                            })*
                        }
                    }

                    Ok($struct_name { $($scalar_field),* })
                }
            }

            const FIELDS: &'static [&'static str] = &[#(#field_names_str),*];
            deserializer.deserialize_struct(stringify!($struct_name), FIELDS, DataVisitor)
        }
    };

    quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ($struct_name:ident {
                $($scalar_field:ident)+
            }) => {
                #[derive(PartialEq)]
                pub struct $struct_name {
                    $(pub $scalar_field: Option<#model_module::partial!(@field_type; $scalar_field)>),+
                }

                impl $struct_name {
                    #to_params_fn
                }

                impl<'de> ::serde::Deserialize<'de> for $struct_name {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: ::serde::Deserializer<'de>,
                    {
                        #deserialize_impl
                    }
                }
            };
            #(#field_type_arms)*
        }

        pub use #macro_name as partial;
    }
}
