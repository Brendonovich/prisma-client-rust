use std::ops::Deref;

use crate::generator::prelude::*;

struct ScalarField<'a> {
    pub typ: TokenStream,
    pub name: &'a str,
    pub inner: &'a dml::ScalarField,
}

impl<'a> Deref for ScalarField<'a> {
    type Target = dml::ScalarField;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

struct RelationField<'a> {
    pub typ: TokenStream,
    pub name: &'a str,
    pub inner: &'a dml::RelationField,
}

impl<'a> Deref for RelationField<'a> {
    type Target = dml::RelationField;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

enum Field<'a> {
    Scalar(ScalarField<'a>),
    Relation(RelationField<'a>),
}

pub fn struct_definition(model: &dml::Model) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let fields = model
        .fields()
        .map(|field| match field {
            dml::Field::RelationField(relation_field) => {
                let relation_model_name_snake = snake_ident(&relation_field.relation_info.to);

                let typ = match &relation_field.arity {
                    dml::FieldArity::List => quote!(Vec<super::#relation_model_name_snake::Data>),
                    dml::FieldArity::Optional => {
                        quote!(Option<Box<super::#relation_model_name_snake::Data>>)
                    }
                    dml::FieldArity::Required => {
                        quote!(Box<super::#relation_model_name_snake::Data>)
                    }
                };

                Field::Relation(RelationField {
                    typ,
                    name: field.name(),
                    inner: &relation_field,
                })
            }
            dml::Field::ScalarField(scalar_field) => {
                let typ = field.type_tokens();

                Field::Scalar(ScalarField {
                    typ,
                    name: field.name(),
                    inner: &scalar_field,
                })
            }
            dml::Field::CompositeField(_) => panic!("Composite fields are not supported!"),
        })
        .collect::<Vec<_>>();

    let struct_fields = fields.iter().map(|field| match field {
        Field::Relation(field) => {
            let typ = &field.typ;

            let field_name_str = &field.name;
            let field_name_snake = snake_ident(field_name_str);

            let attrs = match &field.inner.arity {
                dml::FieldArity::Optional => {
                    quote! {
                        #[serde(
                            rename = #field_name_str,
                            default,
                            skip_serializing_if = "Option::is_none",
                            with = "prisma_client_rust::serde::double_option"
                        )]
                    }
                }
                _ => quote! {
                    #[serde(rename = #field_name_str)]
                },
            };

            quote! {
                #attrs
                pub #field_name_snake: Option<#typ>
            }
        }
        Field::Scalar(field) => {
            let typ = &field.typ;

            let field_name_str = &field.name;
            let field_name_snake = snake_ident(field_name_str);

            quote! {
                #[serde(rename = #field_name_str)]
                pub #field_name_snake: #typ
            }
        }
    });

    let relation_accessors = fields.iter().filter_map(|field| match field {
        Field::Relation(field) => {
            let field_name_snake = snake_ident(&field.name);
            let relation_model_name_snake = snake_ident(&field.relation_info.to);

            let typ = &field.typ;

            let access_error = quote!(#pcr::RelationNotFetchedError::new(stringify!(#field_name_snake)));

            let ret = match field.arity.is_list() {
                true => quote! {
                    pub fn #field_name_snake(&self) -> Result<&#typ, #pcr::RelationNotFetchedError> {
                        self.#field_name_snake.as_ref().ok_or(#access_error)
                    }
                },
                false => {
                    let (accessor_type, inner_map) = match field.arity.is_optional() {
                        false => (
                            quote!(&super::#relation_model_name_snake::Data),
                            None
                        ),
                        true => (
                            quote!(Option<&super::#relation_model_name_snake::Data>),
                            Some(quote!(.map(|v| v.as_ref()))),
                        ),
                    };

                    quote! {
                        pub fn #field_name_snake(&self) -> Result<#accessor_type, #pcr::RelationNotFetchedError> {
                            self.#field_name_snake.as_ref().ok_or(#access_error).map(|v| v.as_ref() #inner_map)
                        }
                    }
                }
            };

            Some(ret)
        }
        _ => None,
    });

    let specta_derive = cfg!(feature = "rspc").then(|| {
        let model_name_pascal_str = model.name.to_case(Case::Pascal);

        quote! {
            #[derive(::prisma_client_rust::rspc::Type)]
            #[specta(rename = #model_name_pascal_str, crate = "prisma_client_rust::rspc::internal::specta")]
        }
    });

    quote! {
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
        #specta_derive
        pub struct Data {
            #(#struct_fields),*
        }

        impl Data {
            #(#relation_accessors)*
        }
    }
}
