use crate::generator::prelude::*;
use prisma_client_rust_sdk::{
    prisma_datamodel::datamodel_connector::ConnectorCapability, GenerateArgs,
};

use super::required_fields;

pub fn create_args_params_pushes(model: &dml::Model) -> Vec<TokenStream> {
    let required_fields = required_fields(model);

    required_fields
        .iter()
        .map(|field| {
            let field_name_snake = snake_ident(field.name());
            let push_wrapper = &field.push_wrapper;

            quote!(_params.push(#push_wrapper(#field_name_snake)))
        })
        .collect()
}

pub fn create_fn(model: &dml::Model) -> TokenStream {
    let model_name_str = &model.name;

    let required_fields = required_fields(model);

    let required_field_names = required_fields
        .iter()
        .map(|field| snake_ident(field.name()));
    let required_field_types = required_fields.iter().map(|field| &field.typ);

    let create_args_params_pushes = create_args_params_pushes(model);

    quote! {
        pub fn create(self, #(#required_field_names: #required_field_types,)* mut _params: Vec<SetParam>) -> Create<'a> {
            #(#create_args_params_pushes;)*

            Create::new(
                self.client._new_query_context(),
                ::prisma_client_rust::QueryInfo::new(#model_name_str, _outputs()),
                _params
            )
        }
    }
}

pub fn create_many_fn(model: &dml::Model) -> TokenStream {
    let model_name_str = &model.name;

    let scalar_field_names = model
        .required_scalar_fields()
        .iter()
        .map(|f| snake_ident(f.name()))
        .collect::<Vec<_>>();
    let scalar_field_names2 = scalar_field_names.clone();

    let scalar_field_types = model
        .required_scalar_fields()
        .iter()
        .map(|f| f.type_tokens())
        .collect::<Vec<_>>();

    quote! {
        pub fn create_many(self, data: Vec<(#(#scalar_field_types,)* Vec<SetParam>)>) -> CreateMany<'a> {
            let data = data.into_iter().map(|(#(#scalar_field_names2,)* mut _params)| {
                #(_params.push(#scalar_field_names::set(#scalar_field_names));)*

                _params
            }).collect();

            CreateMany::new(
                self.client._new_query_context(),
                ::prisma_client_rust::QueryInfo::new(#model_name_str, _outputs()),
                data
            )
        }
    }
}

pub fn upsert_fn(model: &dml::Model) -> TokenStream {
    let model_name_str = &model.name;

    let required_fields = required_fields(model);

    let create_args_names_snake = required_fields
        .iter()
        .map(|field| snake_ident(field.name()));
    let create_args_typs = required_fields.iter().map(|field| &field.typ);
    let create_args_params_pushes = create_args_params_pushes(model);

    quote! {
        pub fn upsert(self, _where: UniqueWhereParam, (#(#create_args_names_snake,)* mut _params): (#(#create_args_typs,)* Vec<SetParam>), _update: Vec<SetParam>) -> Upsert<'a> {
            #(#create_args_params_pushes;)*

            Upsert::new(
                self.client._new_query_context(),
                ::prisma_client_rust::QueryInfo::new(#model_name_str, _outputs()),
                _where.into(),
                _params,
                _update
            )
        }
    }
}

pub fn struct_definition(model: &dml::Model, args: &GenerateArgs) -> TokenStream {
    let model_name_str = &model.name;

    let create_fn = create_fn(model);
    let upsert_fn = upsert_fn(model);

    let create_many_fn = (args
        .connector
        .capabilities()
        .contains(&ConnectorCapability::CreateMany))
    .then(|| create_many_fn(model));

    let pcr = quote!(::prisma_client_rust);

    quote! {
        pub struct Actions<'a> {
            pub client: &'a PrismaClient,
        }

        impl<'a> Actions<'a> {
            pub fn find_unique(self, _where: UniqueWhereParam) -> FindUnique<'a> {
                FindUnique::new(
                    self.client._new_query_context(),
                    #pcr::QueryInfo::new(#model_name_str, _outputs()),
                    _where.into()
                )
            }

            pub fn find_first(self, _where: Vec<WhereParam>) -> FindFirst<'a> {
                FindFirst::new(
                    self.client._new_query_context(),
                    #pcr::QueryInfo::new(#model_name_str, _outputs()),
                    _where
                )
            }

            pub fn find_many(self, _where: Vec<WhereParam>) -> FindMany<'a> {
                FindMany::new(
                    self.client._new_query_context(),
                    #pcr::QueryInfo::new(#model_name_str, _outputs()),
                    _where
                )
            }

            #create_fn

            #create_many_fn

            pub fn update(self, _where: UniqueWhereParam, _params: Vec<SetParam>) -> Update<'a> {
                Update::new(
                    self.client._new_query_context(),
                    #pcr::QueryInfo::new(#model_name_str, _outputs()),
                    _where.into(),
                    _params,
                    vec![]
                )
            }

            pub fn update_many(self, _where: Vec<WhereParam>, _params: Vec<SetParam>) -> UpdateMany<'a> {
                UpdateMany::new(
                    self.client._new_query_context(),
                    #pcr::QueryInfo::new(#model_name_str, _outputs()),
                    _where,
                    _params,
                )
            }

            #upsert_fn

            pub fn delete(self, _where: UniqueWhereParam) -> Delete<'a> {
                Delete::new(
                    self.client._new_query_context(),
                    #pcr::QueryInfo::new(#model_name_str, _outputs()),
                    _where.into(),
                    vec![]
                )
            }

            pub fn delete_many(self, _where: Vec<WhereParam>) -> DeleteMany<'a> {
                DeleteMany::new(
                    self.client._new_query_context(),
                    #pcr::QueryInfo::new(#model_name_str, _outputs()),
                    _where.into()
                )
            }

            pub fn count(self, _where: Vec<WhereParam>) -> Count<'a> {
                Count::new(
                    self.client._new_query_context(),
                    #pcr::QueryInfo::new(#model_name_str, _outputs()),
                    vec![]
                )
            }
        }
    }
}
