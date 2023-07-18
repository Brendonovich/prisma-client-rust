mod data;
mod order_by;
mod set_params;
mod where_params;

use std::collections::BTreeMap;

use prisma_client_rust_sdk::{
    prelude::*,
    prisma::{prisma_models::walkers::CompositeTypeWalker, psl::parser_database::ScalarFieldType},
};

pub fn scalar_selections_fn(
    comp_type: CompositeTypeWalker,
    module_path: &TokenStream,
) -> TokenStream {
    let pcr = quote!(::prisma_client_rust);

    let selections = comp_type.fields().flat_map(|field| {
        let field_name_snake = snake_ident(field.name());

        Some(match field.r#type() {
            ScalarFieldType::BuiltInScalar(_) | ScalarFieldType::Enum(_) => {
                field.type_tokens(module_path)?;
                quote!(#pcr::sel(#field_name_snake::NAME))
            }
            ScalarFieldType::CompositeType(id) => {
                let comp_type = comp_type.db.walk(id);

                let field_comp_type_snake = snake_ident(comp_type.name());

                quote! {
                    #pcr::Selection::new(
                        #field_name_snake::NAME,
                        None,
                        [],
                        super::#field_comp_type_snake::scalar_selections()
                    )
                }
            }
            ScalarFieldType::Unsupported(_) => return None,
        })
    });

    quote! {
        pub fn scalar_selections() -> Vec<::prisma_client_rust::Selection> {
            vec![#(#selections),*]
        }
    }
}

pub fn modules(args: &GenerateArgs, module_path: &TokenStream) -> Vec<Module> {
    args.schema
        .db
        .walk_composite_types()
        .map(|comp_type| {
            let scalar_selections_fn = scalar_selections_fn(comp_type, module_path);

            let data_struct = data::struct_definition(comp_type);
            let order_by_enum = order_by::enum_definition(comp_type, args);
            let create_fn = set_params::create_fn(comp_type);

            let parts = CompositeTypeModulePart::combine(vec![
                set_params::module_part(comp_type),
                where_params::module_part(comp_type),
            ]);

            Module::new(
                comp_type.name(),
                quote! {
                    use super::*;
                    use super::_prisma::*;

                    #scalar_selections_fn

                    #parts

                    #data_struct

                    #create_fn

                    #order_by_enum
                },
            )
        })
        .collect()
}

pub struct CompositeTypeModulePart {
    data: TokenStream,
    fields: BTreeMap<String, TokenStream>,
}

impl CompositeTypeModulePart {
    pub fn combine(parts: Vec<Self>) -> TokenStream {
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
                let field_name_snake = snake_ident(&field_name_str);

                quote! {
                    pub mod #field_name_snake {
                        use super::super::*;
                        use super::{SetParam, WhereParam};

                        pub const NAME: &str = #field_name_str;

                        #(#data)*
                    }
                }
            });

        quote! {
            #(#data)*

            #(#field_stuff)*
        }
    }
}
