use crate::generator::prelude::{prisma::psl::datamodel_connector, *};
use prisma_client_rust_sdk::GenerateArgs;

use super::required_fields;

pub fn create_fn(model: &dml::Model, module_path: &TokenStream) -> Option<TokenStream> {
    let (names, (types, wrapped_params)): (Vec<_>, (Vec<_>, Vec<_>)) =
        required_fields(model, module_path)?
            .into_iter()
            .map(|field| (snake_ident(field.name()), (field.typ, field.wrapped_param)))
            .unzip();

    Some(quote! {
        pub fn create(self, #(#names: #types,)* mut _params: Vec<SetParam>) -> Create<'a> {
            _params.extend([
                #(#wrapped_params),*
            ]);

            Create::new(
                self.client,
                _params
            )
        }
    })
}

pub fn create_unchecked_fn(model: &dml::Model, module_path: &TokenStream) -> Option<TokenStream> {
    let (names, types): (Vec<_>, Vec<_>) = model
        .required_scalar_fields()
        .iter()
        .map(|f| Some((snake_ident(f.name()), f.type_tokens(module_path)?)))
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .unzip();

    Some(quote! {
        pub fn create_unchecked(self, #(#names: #types,)* mut _params: Vec<UncheckedSetParam>) -> Create<'a> {
            _params.extend([
                #(#names::set(#names)),*
            ]);

            Create::new(
                self.client,
                _params.into_iter().map(Into::into).collect()
            )
        }
    })
}

pub fn create_many_fn(model: &dml::Model, module_path: &TokenStream) -> Option<TokenStream> {
    let scalar_field_names = model
        .required_scalar_fields()
        .iter()
        .map(|f| snake_ident(f.name()))
        .collect::<Vec<_>>();
    let scalar_field_names2 = scalar_field_names.clone();

    let scalar_field_types = model
        .required_scalar_fields()
        .iter()
        .map(|f| f.type_tokens(module_path))
        .collect::<Option<Vec<_>>>()?;

    Some(quote! {
        pub fn create_many(self, data: Vec<(#(#scalar_field_types,)* Vec<UncheckedSetParam>)>) -> CreateMany<'a> {
            let data = data.into_iter().map(|(#(#scalar_field_names2,)* mut _params)| {
                _params.extend([
                    #(#scalar_field_names::set(#scalar_field_names)),*
                ]);

                _params
            }).collect();

            CreateMany::new(
                self.client,
                data
            )
        }
    })
}

pub fn upsert_fn(model: &dml::Model, module_path: &TokenStream) -> Option<TokenStream> {
    let (names, (types, wrapped_params)): (Vec<_>, (Vec<_>, Vec<_>)) =
        required_fields(model, module_path)?
            .into_iter()
            .map(|field| (snake_ident(field.name()), (field.typ, field.wrapped_param)))
            .unzip();

    Some(quote! {
        pub fn upsert(
            self,
             _where: UniqueWhereParam,
              (#(#names,)* mut _params): (#(#types,)* Vec<SetParam>),
               _update: Vec<SetParam>
        ) -> Upsert<'a> {
            _params.extend([
                #(#wrapped_params),*
            ]);

            Upsert::new(
                self.client,
                _where.into(),
                _params,
                _update
            )
        }
    })
}

pub fn struct_definition(
    model: &dml::Model,
    args: &GenerateArgs,
    module_path: &TokenStream,
) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let create_fn = create_fn(model, module_path);
    let create_unchecked_fn = create_unchecked_fn(model, module_path);
    let upsert_fn = upsert_fn(model, module_path);

    let create_many_fn = (args
        .connector
        .capabilities()
        .contains(&datamodel_connector::ConnectorCapability::CreateMany))
    .then(|| create_many_fn(model, module_path));

    quote! {
        #[derive(Clone)]
        pub struct Actions<'a> {
            pub client: &'a #pcr::PrismaClientInternals,
        }

        impl<'a> Actions<'a> {
            pub fn find_unique(self, _where: UniqueWhereParam) -> FindUnique<'a> {
                FindUnique::new(
                    self.client,
                    _where.into()
                )
            }

            pub fn find_first(self, _where: Vec<WhereParam>) -> FindFirst<'a> {
                FindFirst::new(
                    self.client,
                    _where
                )
            }

            pub fn find_many(self, _where: Vec<WhereParam>) -> FindMany<'a> {
                FindMany::new(
                    self.client,
                    _where
                )
            }

            #create_fn
            #create_unchecked_fn

            #create_many_fn

            pub fn update(self, _where: UniqueWhereParam, _params: Vec<SetParam>) -> Update<'a> {
                Update::new(
                    self.client,
                    _where.into(),
                    _params,
                    vec![]
                )
            }

            pub fn update_unchecked(self, _where: UniqueWhereParam, _params: Vec<UncheckedSetParam>) -> Update<'a> {
                Update::new(
                    self.client,
                    _where.into(),
                    _params.into_iter().map(Into::into).collect(),
                    vec![]
                )
            }

            pub fn update_many(self, _where: Vec<WhereParam>, _params: Vec<UncheckedSetParam>) -> UpdateMany<'a> {
                UpdateMany::new(
                    self.client,
                    _where,
                    _params,
                )
            }

            #upsert_fn

            pub fn delete(self, _where: UniqueWhereParam) -> Delete<'a> {
                Delete::new(
                    self.client,
                    _where.into(),
                    vec![]
                )
            }

            pub fn delete_many(self, _where: Vec<WhereParam>) -> DeleteMany<'a> {
                DeleteMany::new(
                    self.client,
                    _where
                )
            }

            pub fn count(self, _where: Vec<WhereParam>) -> Count<'a> {
                Count::new(
                    self.client,
                    _where
                )
            }
        }
    }
}
