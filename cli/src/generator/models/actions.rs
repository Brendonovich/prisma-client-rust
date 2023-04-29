use crate::generator::prelude::{prisma::psl::datamodel_connector, *};
use prisma_client_rust_sdk::{
    prisma::{prisma_models::walkers::ModelWalker, psl::parser_database::ScalarFieldType},
    GenerateArgs,
};

use super::required_fields;

pub fn create_fn(model: ModelWalker) -> Option<TokenStream> {
    let (names, (types, wrapped_params)): (Vec<_>, (Vec<_>, Vec<_>)) = required_fields(model)?
        .into_iter()
        .map(|field| {
            (
                snake_ident(field.inner.name()),
                (field.typ, field.wrapped_param),
            )
        })
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

pub fn create_unchecked_fn(model: ModelWalker) -> Option<TokenStream> {
    let (names, types): (Vec<_>, Vec<_>) = model
        .scalar_fields()
        .filter_map(|field| {
            let name_snake = snake_ident(field.name());

            if !field.required_on_create() {
                return None;
            }

            Some((
                name_snake,
                match field.scalar_field_type() {
                    ScalarFieldType::CompositeType(id) => {
                        let comp_type = model.db.walk(id);

                        let comp_type_snake = snake_ident(comp_type.name());

                        quote!(super::#comp_type_snake::Create)
                    }
                    _ => field.type_tokens(&quote!(super))?,
                },
            ))
        })
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

pub fn create_many_fn(model: ModelWalker) -> Option<TokenStream> {
    let (names, types): (Vec<_>, Vec<_>) = model
        .scalar_fields()
        .filter_map(|scalar_field| {
            let name_snake = snake_ident(scalar_field.name());

            if !scalar_field.required_on_create() {
                return None;
            }

            Some((
                name_snake,
                match scalar_field.scalar_field_type() {
                    ScalarFieldType::CompositeType(id) => {
                        let comp_type = model.db.walk(id);

                        let comp_type_snake = snake_ident(comp_type.name());

                        quote!(super::#comp_type_snake::Create)
                    }
                    _ => scalar_field.type_tokens(&quote!(super))?,
                },
            ))
        })
        .unzip();

    Some(quote! {
        pub fn create_many(self, data: Vec<(#(#types,)* Vec<UncheckedSetParam>)>) -> CreateMany<'a> {
            let data = data.into_iter().map(|(#(#names,)* mut _params)| {
                _params.extend([
                    #(#names::set(#names)),*
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

pub fn upsert_fn(model: ModelWalker) -> Option<TokenStream> {
    let (names, (types, wrapped_params)): (Vec<_>, (Vec<_>, Vec<_>)) = required_fields(model)?
        .into_iter()
        .map(|field| {
            (
                snake_ident(field.inner.name()),
                (field.typ, field.wrapped_param),
            )
        })
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

pub fn mongo_raw_fns() -> Option<TokenStream> {
    let pcr = quote!(::prisma_client_rust);

    if cfg!(not(feature = "mongodb")) {
        return None;
    };

    Some(quote! {
        pub fn find_raw<T: #pcr::Data>(self) -> #pcr::FindRaw<'a, Types, T> {
            #pcr::FindRaw::new(self.client)
        }

        pub fn aggregate_raw<T: #pcr::Data>(self) -> #pcr::AggregateRaw<'a, Types, T> {
            #pcr::AggregateRaw::new(self.client)
        }
    })
}

pub fn struct_definition(model: ModelWalker, args: &GenerateArgs) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let create_fn = create_fn(model);
    let create_unchecked_fn = create_unchecked_fn(model);
    let upsert_fn = upsert_fn(model);
    let monogo_raw_fns = mongo_raw_fns();

    let create_many_fn = (args
        .connector
        .capabilities()
        .contains(datamodel_connector::ConnectorCapability::CreateMany))
    .then(|| create_many_fn(model));

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

            pub fn update_many(self, _where: Vec<WhereParam>, _params: Vec<SetParam>) -> UpdateMany<'a> {
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

            #monogo_raw_fns
        }
    }
}
