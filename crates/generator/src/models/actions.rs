use crate::{
    models::create::InputObjects,
    prelude::{prisma::psl::datamodel_connector, *},
};
use prisma_client_rust_sdk::{
    prisma::{dmmf::TypeLocation, prisma_models::walkers::ModelWalker},
    GenerateArgs,
};

use super::required_fields;

pub fn create_fn(model: ModelWalker, args: &GenerateArgs) -> Option<TokenStream> {
    args.dmmf
        .schema
        .find_input_type(&format!("{}CreateInput", model.name()))
        .map(|input_type| {
	        let ((names_snake, names_pascal), types): ((Vec<_>, Vec<_>), Vec<_>) = model
	            .fields()
	            .filter_map(|field| {
	                input_type
	                    .fields
	                    .iter()
	                    .filter(|f| f.is_required)
	                    .find(|f| f.name == field.name())
	            })
	            .filter_map(|field| {
	                let field_name_pascal = pascal_ident(&field.name);
	                let field_name_snake = snake_ident(&field.name);

	                let type_ref = &field.input_types[0];
	                let typ = match type_ref.location {
	                    TypeLocation::InputObjectTypes => {
	                        let obj = type_ref.typ.parse::<InputObjects>().unwrap();

	                        quote!(super::#obj)
	                    }
	                    _ => type_ref.to_tokens(&quote!(super::), &field.arity(), &args.schema.db)?,
	                };

	                Some(((field_name_snake, field_name_pascal), typ))
	            })
	            .unzip();

            quote! {
	            pub fn create(self, #(#names_snake: #types,)* mut _params: Vec<CreateParam>) -> CreateQuery<'a> {
	                _params.extend([
	                    #(CreateParam::#names_pascal(#names_snake)),*
	                ]);

	                CreateQuery::new(
	                    self.client,
	                    _params
	                )
	            }
            }
        })
}

pub fn create_unchecked_fn(model: ModelWalker, args: &GenerateArgs) -> Option<TokenStream> {
    args.dmmf
        .schema
        .find_input_type(&format!("{}UncheckedCreateInput", model.name()))
        .map(|input_type| {
	        let (names, types): (Vec<_>, Vec<_>) = input_type
	            .fields
	            .iter()
	            .filter(|f| f.is_required)
	            .filter_map(|field| {
	                let field_name_snake = snake_ident(&field.name);

	                let type_ref = &field.input_types[0];
	                let typ = match type_ref.location {
	                    TypeLocation::InputObjectTypes => {
	                        let ident = format_ident!(
	                            "{}",
	                            type_ref.typ.parse::<InputObjects>().unwrap().to_string()
	                        );

	                        quote!(#field_name_snake::#ident)
	                    }
	                    _ => {
	                        type_ref.to_tokens(&quote!(super::), &field.arity(), &args.schema.db)?
	                    }
	                };

	                Some((field_name_snake, typ))
	            })
	            .unzip();

        	quote! {
	            pub fn create_unchecked(self, #(#names: #types,)* mut _params: Vec<CreateUncheckedParam>) -> CreateUncheckedQuery<'a> {
	                _params.extend([
	                    #(#names::set(#names)),*
	                ]);

	                CreateUncheckedQuery::new(
	                    self.client,
	                    _params
	                )
	            }
         	}
        })
}

pub fn create_many_fn(model: ModelWalker) -> Option<TokenStream> {
    model
        .scalar_fields()
        .all(|scalar_field| !scalar_field.required_on_create() || !scalar_field.is_unsupported())
        .then(|| {
            quote! {
                pub fn create_many(self, data: Vec<CreateUnchecked>) -> CreateManyQuery<'a> {
                    let data = data.into_iter().map(CreateUnchecked::to_params).collect();

                    CreateManyQuery::new(
                        self.client,
                        data
                    )
                }
            }
        })
}

pub fn upsert_fn(model: ModelWalker) -> Option<TokenStream> {
    // necessary to check whether CreateData is even available
    let _ = required_fields(model)?;

    Some(quote! {
        pub fn upsert(
            self,
             _where: UniqueWhereParam,
             _create: Create,
             _update: Vec<SetParam>
        ) -> UpsertQuery<'a> {
            UpsertQuery::new(
                self.client,
                _where,
                _create.to_params(),
                _update
            )
        }
    })
}

pub fn mongo_raw_fns() -> Option<TokenStream> {
    cfg!(feature = "mongodb").then(|| {
        quote! {
            pub fn find_raw<T: ::prisma_client_rust::Data>(self) -> FindRawQuery<'a, T> {
                FindRawQuery::new(self.client)
            }

            pub fn aggregate_raw<T: ::prisma_client_rust::Data>(self) -> AggregateRawQuery<'a, T> {
                AggregateRawQuery::new(self.client)
            }
        }
    })
}

pub fn struct_definition(model: ModelWalker, args: &GenerateArgs) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let create_fn = create_fn(model, args);
    let create_unchecked_fn = create_unchecked_fn(model, args);
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
            pub fn find_unique(self, _where: UniqueWhereParam) -> FindUniqueQuery<'a> {
                FindUniqueQuery::new(
                    self.client,
                    _where
                )
            }

            pub fn find_first(self, _where: Vec<WhereParam>) -> FindFirstQuery<'a> {
                FindFirstQuery::new(
                    self.client,
                    _where
                )
            }

            pub fn find_many(self, _where: Vec<WhereParam>) -> FindManyQuery<'a> {
                FindManyQuery::new(
                    self.client,
                    _where
                )
            }

            #create_fn
            #create_unchecked_fn

            #create_many_fn

            pub fn update(self, _where: UniqueWhereParam, _params: Vec<SetParam>) -> UpdateQuery<'a> {
                UpdateQuery::new(
                    self.client,
                    _where,
                    _params,
                    vec![]
                )
            }

            pub fn update_unchecked(self, _where: UniqueWhereParam, _params: Vec<UncheckedSetParam>) -> UpdateUncheckedQuery<'a> {
                UpdateUncheckedQuery::new(
                    self.client,
                    _where,
                    _params.into_iter().map(Into::into).collect(),
                    vec![]
                )
            }

            pub fn update_many(self, _where: Vec<WhereParam>, _params: Vec<SetParam>) -> UpdateManyQuery<'a> {
                UpdateManyQuery::new(
                    self.client,
                    _where,
                    _params,
                )
            }

            #upsert_fn

            pub fn delete(self, _where: UniqueWhereParam) -> DeleteQuery<'a> {
                DeleteQuery::new(
                    self.client,
                    _where,
                    vec![]
                )
            }

            pub fn delete_many(self, _where: Vec<WhereParam>) -> DeleteManyQuery<'a> {
                DeleteManyQuery::new(
                    self.client,
                    _where
                )
            }

            pub fn count(self, _where: Vec<WhereParam>) -> CountQuery<'a> {
                CountQuery::new(
                    self.client,
                    _where
                )
            }

            #monogo_raw_fns
        }
    }
}
