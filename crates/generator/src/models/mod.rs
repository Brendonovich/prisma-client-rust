mod actions;
mod create;
mod data;
mod filter;
mod include_select;
mod order_by;
mod pagination;
mod partial_unchecked;
mod set_params;
mod types;
mod where_params;
mod with_params;

use std::collections::BTreeMap;

use include_select::*;
use prisma_client_rust_sdk::{
    prelude::*,
    prisma::{
        prisma_models::walkers::{FieldWalker, ModelWalker, RefinedFieldWalker},
        psl::parser_database::ScalarFieldType,
    },
};

pub struct RequiredField<'a> {
    pub push_wrapper: TokenStream,
    pub typ: TokenStream,
    pub inner: FieldWalker<'a>,
}

pub fn required_fields<'a>(model: ModelWalker<'a>) -> Option<Vec<RequiredField<'a>>> {
    model
        .fields()
        .filter(|field| match field.refine() {
            RefinedFieldWalker::Scalar(scalar_field) => match scalar_field.scalar_field_type() {
                ScalarFieldType::CompositeType(_) => field.required_on_create(),
                _ => !model.scalar_field_has_relation(scalar_field) && field.required_on_create(),
            },
            RefinedFieldWalker::Relation(_) => field.required_on_create(),
        })
        .map(|field| {
            Some({
                let typ = match field.refine() {
                    RefinedFieldWalker::Scalar(scalar_field) => {
                        match scalar_field.scalar_field_type() {
                            ScalarFieldType::CompositeType(id) => {
                                let comp_type = model.db.walk(id);

                                let type_snake = snake_ident(comp_type.name());

                                quote!(super::#type_snake::Create)
                            }
                            _ => field.type_tokens(&quote!(super::))?,
                        }
                    }
                    RefinedFieldWalker::Relation(relation_field) => {
                        let relation_model_name_snake =
                            snake_ident(relation_field.related_model().name());

                        quote!(super::#relation_model_name_snake::UniqueWhereParam)
                    }
                };

                let push_wrapper = match field.refine() {
                    RefinedFieldWalker::Scalar(_) => quote!(set),
                    RefinedFieldWalker::Relation(_) => quote!(connect),
                };

                RequiredField {
                    inner: field,
                    push_wrapper,
                    typ,
                }
            })
        })
        .collect()
}

pub fn modules(args: &GenerateArgs, module_path: &TokenStream) -> Vec<Module> {
    let pcr = quote!(::prisma_client_rust);

    args.schema
        .db
        .walk_models()
        .map(|model| {
            let model_name = model.name();

            let actions_struct = actions::struct_definition(model, args);

            let (field_stuff, field_modules) = ModelModulePart::combine(vec![
                data::model_data(model),
                where_params::model_data(model, args, module_path),
                order_by::model_data(model, args),
                with_params::model_data(model),
                set_params::model_data(model, args),
                select::model_data(model, &module_path),
                include::model_data(model, &module_path),
            ]);

            let create_types = create::types(model);
            let types_struct = types::r#struct(model, module_path);
            let data_struct = data::r#struct(model);
            let partial_unchecked_macro = partial_unchecked::r#macro(model, &module_path);
            let filter_macro = filter::r#macro(model, module_path);

            let mongo_raw_types = cfg!(feature = "mongodb").then(|| quote! {
	            pub type FindRawQuery<'a, T: #pcr::Data> = #pcr::FindRaw<'a, Types, T>;
	            pub type AggregateRawQuery<'a, T: #pcr::Data> = #pcr::AggregateRaw<'a, Types, T>;
	        });

            let mut module = Module::new(
                model.name(),
                quote! {
                    use super::_prisma::*;

                    pub const NAME: &str = #model_name;

                    #field_stuff
                    #create_types
                    #types_struct
                    #data_struct
                    #partial_unchecked_macro
                    #filter_macro

                    pub type UniqueArgs = #pcr::UniqueArgs<Types>;
                    pub type ManyArgs = #pcr::ManyArgs<Types>;

                    pub type CountQuery<'a> = #pcr::Count<'a, Types>;
                    pub type CreateQuery<'a> = #pcr::Create<'a, Types>;
                    pub type CreateUncheckedQuery<'a> = #pcr::CreateUnchecked<'a, Types>;
                    pub type CreateManyQuery<'a> = #pcr::CreateMany<'a, Types>;
                    pub type FindUniqueQuery<'a> = #pcr::FindUnique<'a, Types>;
                    pub type FindManyQuery<'a> = #pcr::FindMany<'a, Types>;
                    pub type FindFirstQuery<'a> = #pcr::FindFirst<'a, Types>;
                    pub type UpdateQuery<'a> = #pcr::Update<'a, Types>;
                    pub type UpdateUncheckedQuery<'a> = #pcr::UpdateUnchecked<'a, Types>;
                    pub type UpdateManyQuery<'a> = #pcr::UpdateMany<'a, Types>;
                    pub type UpsertQuery<'a> = #pcr::Upsert<'a, Types>;
                    pub type DeleteQuery<'a> = #pcr::Delete<'a, Types>;
                    pub type DeleteManyQuery<'a> = #pcr::DeleteMany<'a, Types>;

                    #mongo_raw_types

                    #actions_struct
                },
            );

            field_modules
                .into_iter()
                .for_each(|field| module.add_submodule(field));

            module
        })
        .collect()
}

pub struct ModelModulePart {
    data: TokenStream,
    fields: BTreeMap<String, TokenStream>,
}

impl ModelModulePart {
    pub fn combine(parts: Vec<Self>) -> (TokenStream, Vec<Module>) {
        let (data, fields): (Vec<_>, Vec<_>) =
            parts.into_iter().map(|p| (p.data, p.fields)).unzip();

        let field_stuff = fields
            .into_iter()
            .flat_map(|p| p.into_iter())
            .fold(BTreeMap::new(), |mut acc, (k, v)| {
                let entry = acc.entry(k).or_insert_with(|| vec![]);
                entry.push(v);
                acc
            })
            .into_iter()
            .map(|(field_name_str, data)| {
                Module::new(&field_name_str, quote! {
                    use super::super::{_prisma::*, *};
                    use super::{WhereParam, UniqueWhereParam, WithParam, SetParam, UncheckedSetParam};

					          pub const NAME: &str = #field_name_str;

                    #(#data)*
                })
            }).collect();

        (quote!(#(#data)*), field_stuff)
    }
}
