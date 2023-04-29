use prisma_client_rust_sdk::prisma::{
    prisma_models::walkers::ModelWalker, psl::parser_database::ScalarFieldType,
};

use crate::generator::prelude::*;

use super::required_fields;

fn create_unchecked(model: ModelWalker) -> Option<TokenStream> {
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
        pub fn create_unchecked(#(#names: #types,)* _params: Vec<UncheckedSetParam>)
            -> (#(#types,)* Vec<UncheckedSetParam>) {
            (#(#names,)* _params)
        }
    })
}

fn create(model: ModelWalker) -> Option<TokenStream> {
    let model_name_snake = snake_ident(model.name());

    let (names, (types, push_wrappers)): (Vec<_>, (Vec<_>, Vec<_>)) = required_fields(model)?
        .into_iter()
        .map(|field| {
            (
                snake_ident(field.inner.name()),
                (field.typ, field.push_wrapper),
            )
        })
        .unzip();

    Some(quote! {
        pub struct CreateData {
            #(pub #names: #types,)*
            pub _params: Vec<SetParam>
        }

        impl CreateData {
            pub fn to_query<'a>(self, db: &'a PrismaClient) -> Create<'a> {
                db.#model_name_snake()
                    .create(
                        #(self.#names,)*
                        self._params
                    )
            }

            pub fn to_params(mut self) -> Vec<SetParam> {
                self._params.extend([
                    #(#names::#push_wrappers(self.#names)),*
                ]);

                self._params
            }
        }

        pub fn create(#(#names: #types,)* _params: Vec<SetParam>)
            -> CreateData {
            CreateData {
                #(#names,)*
                _params
            }
        }
    })
}

pub fn model_fns(model: ModelWalker) -> TokenStream {
    let create_unchecked = create_unchecked(model);
    let create = create(model);

    quote! {
        #create

        #create_unchecked
    }
}
