mod actions;
mod create;
mod data;
mod field;
mod include_select;
mod order_by;
mod pagination;
mod partial;
mod set_params;
mod types;
mod where_params;
mod with_params;

use include_select::*;
use prisma_client_rust_sdk::prelude::*;

use std::ops::Deref;

use self::where_params::Variant;

pub struct Operator {
    pub name: &'static str,
    pub action: &'static str,
    pub list: bool,
}

static OPERATORS: &'static [Operator] = &[
    Operator {
        name: "Not",
        action: "NOT",
        list: false,
    },
    Operator {
        name: "Or",
        action: "OR",
        list: true,
    },
    Operator {
        name: "And",
        action: "AND",
        list: false,
    },
];

pub struct RequiredField<'a> {
    pub wrapped_param: TokenStream,
    pub typ: TokenStream,
    pub field: &'a dml::Field,
}

impl Deref for RequiredField<'_> {
    type Target = dml::Field;
    fn deref(&self) -> &Self::Target {
        self.field
    }
}

pub fn required_fields<'a>(model: &'a dml::Model) -> Option<Vec<RequiredField<'a>>> {
    model
        .fields()
        .filter(|field| match field {
            dml::Field::ScalarField(scalar_field) => {
                !model.scalar_field_has_relation(scalar_field) && field.required_on_create()
            }
            dml::Field::RelationField(_) => field.required_on_create(),
            dml::Field::CompositeField(_) => field.required_on_create(),
        })
        .map(|field| {
            Some({
                let field_name_snake = snake_ident(&field.name());

                let typ = match field {
                    dml::Field::ScalarField(_) => field.type_tokens(&quote!(super))?,
                    dml::Field::RelationField(relation_field) => {
                        let relation_model_name_snake =
                            snake_ident(&relation_field.relation_info.referenced_model);

                        quote!(super::#relation_model_name_snake::UniqueWhereParam)
                    }
                    dml::Field::CompositeField(cf) => {
                        let type_snake = snake_ident(&cf.composite_type);

                        quote!(super::#type_snake::Create)
                    }
                };

                let push_wrapper = match field {
                    dml::Field::ScalarField(_) => Some(quote!(set)),
                    dml::Field::RelationField(_) => Some(quote!(connect)),
                    dml::Field::CompositeField(_) => Some(quote!(set)),
                };

                RequiredField {
                    field,
                    wrapped_param: push_wrapper
                        .map(|wrapper| quote!(#field_name_snake::#wrapper(#field_name_snake)))
                        .unwrap_or_else(|| quote!(#field_name_snake.into())),
                    typ,
                }
            })
        })
        .collect()
}

pub fn unique_field_combos(model: &dml::Model) -> Vec<Vec<&dml::Field>> {
    let mut combos = model
        .indices
        .iter()
        .filter(|i| matches!(i.tpe, dml::IndexType::Unique))
        .map(|unique| {
            unique
                .fields
                .iter()
                .filter_map(|field| model.fields.iter().find(|mf| mf.name() == &field.path[0].0))
                .collect()
        })
        .collect::<Vec<_>>();

    if let Some(primary_key) = &model.primary_key {
        // if primary key is marked as unique, skip primary key handling
        let primary_key_also_unique =
            primary_key.fields.len() == 1 && !model.field_is_unique(&primary_key.fields[0].name);

        // TODO: understand why i wrote this
        let primary_key_idk = !model
            .indices
            .iter()
            .filter(|i| i.tpe == dml::IndexType::Unique)
            .any(|i| {
                i.fields
                    .iter()
                    .map(|f| f.path[0].0.as_str())
                    .collect::<Vec<_>>()
                    == primary_key
                        .fields
                        .iter()
                        .map(|f| f.name.as_str())
                        .collect::<Vec<_>>()
            });

        if primary_key_also_unique || primary_key_idk {
            combos.push(
                primary_key
                    .fields
                    .iter()
                    .filter_map(|field| {
                        model
                            .fields
                            .iter()
                            .find(|mf| mf.name() == field.name.as_str())
                    })
                    .collect(),
            );
        }
    }

    combos
}

pub fn modules(args: &GenerateArgs, module_path: &TokenStream) -> Vec<TokenStream> {
    let pcr = quote!(::prisma_client_rust);

    args.dml.models.iter().map(|model| {
        let mut where_params_entries = vec![];

        let model_name = &model.name;
        let model_name_snake = snake_ident(model_name);

        where_params_entries.extend(OPERATORS.iter().map(|op| {
            let variant_name = pascal_ident(&op.name);
            let op_action = &op.action;

            let value = match op.list {
                true => quote! {
                    #pcr::SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(#pcr::WhereInput::serialize)
                            .map(Into::into)
                            .map(|v| vec![v])
                            .map(#pcr::PrismaValue::Object)
                            .collect()
                    )
                },
                false => quote! {
                    #pcr::SerializedWhereValue::Object(
                        ::prisma_client_rust::merge_fields(
                            value
                                .into_iter()
                                .map(#pcr::WhereInput::serialize)
                                .map(Into::into)
                                .collect()
                        )
                    )
                },
            };

            Variant::BaseVariant {
                definition: quote!(#variant_name(Vec<WhereParam>)),
                match_arm: quote! {
                    Self::#variant_name(value) => (
                        #op_action,
                        #value,
                    )
                },
            }
        }));

        let compound_field_accessors = unique_field_combos(&model).iter().flat_map(|fields| {
            if fields.len() == 1 {
                let field = fields[0];

                let read_filter = args.read_filter(field.as_scalar_field().unwrap()).unwrap();

                where_params_entries.push(Variant::unique(field, read_filter, module_path));

                None
            } else {
                let variant_name_string = fields.iter().map(|f| pascal_ident(f.name()).to_string()).collect::<String>();
                let variant_name = format_ident!("{}Equals", &variant_name_string);

                let variant_data_names = fields.iter().map(|f| snake_ident(f.name())).collect::<Vec<_>>();

                let ((field_defs, field_types), (prisma_values, field_names_snake)):
                    ((Vec<_>, Vec<_>), (Vec<_>, Vec<_>)) = fields.into_iter().map(|field| {
                    let field_type = match field.arity() {
                        dml::FieldArity::List | dml::FieldArity::Required => field.type_tokens(module_path),
                        dml::FieldArity::Optional => field.field_type().to_tokens(module_path, &dml::FieldArity::Required)
                    }.unwrap();

                    let field_name_snake = snake_ident(field.name());

                    (
                        (quote!(#field_name_snake: #field_type), field_type),
                        (field.field_type().to_prisma_value(&field_name_snake, &dml::FieldArity::Required).unwrap(), field_name_snake)
                    )
                }).unzip();

                let field_names_joined = fields.iter().map(|f| f.name()).collect::<Vec<_>>().join("_");

                where_params_entries.extend([
                    Variant::BaseVariant {
                        definition: quote!(#variant_name(#(#field_types),*)),
                        match_arm: quote! {
                            Self::#variant_name(#(#field_names_snake),*) => (
                                #field_names_joined,
                                #pcr::SerializedWhereValue::Object(vec![#((#variant_data_names::NAME.to_string(), #prisma_values)),*])
                            )
                        },
                    },
                    Variant::CompoundUniqueVariant {
                        field_names_string: variant_name_string.clone(),
                        variant_data_destructured: field_names_snake.clone(),
                        variant_data_types: field_types
                    }
                ]);

                let accessor_name = snake_ident(&variant_name_string);

                Some(quote! {
                    pub fn #accessor_name<T: From<UniqueWhereParam>>(#(#field_defs),*) -> T {
                        UniqueWhereParam::#variant_name(#(#field_names_snake),*).into()
                    }
                })
            }
        }).collect::<TokenStream>();

        let (field_modules, field_where_param_entries): (Vec<_>, Vec<_>) = model
            .fields
            .iter()
            .filter(|f| !f.field_type().is_unsupported())
            .map(|f| field::module(f, model, args, module_path))
            .unzip();

        where_params_entries.extend(field_where_param_entries.into_iter().flatten());

        let where_params_enums = where_params::collate_entries(where_params_entries);
        let data_struct = data::struct_definition(&model, module_path);
        let with_params_enum = with_params::enum_definition(&model);
        let set_params_enum = set_params::enum_definition(&model, args, module_path);
        let order_by_params_enum = order_by::enum_definition(&model);
        let create_fn = create::model_fns(&model);
        let select_macro = select::model_macro(model, &module_path);
        let select_params_enum = select::model_module_enum(&model, &pcr);
        let include_macro = include::model_macro(model, &module_path);
        let include_params_enum = include::model_module_enum(&model, &pcr);
        let actions_struct = actions::struct_definition(&model, args);
        let types_struct = types::struct_definition(&model, module_path);
        let partial_macro = partial::model_macro(&model, &module_path);

        quote! {
            pub mod #model_name_snake {
                use super::*;
                use super::_prisma::*;

                pub const NAME: &str = #model_name;

                #(#field_modules)*

                #compound_field_accessors

                #create_fn

                #select_macro
                #select_params_enum

                #include_macro
                #include_params_enum

                #partial_macro

                #data_struct

                #with_params_enum

                #set_params_enum

                #order_by_params_enum

                #where_params_enums

                #types_struct

                // 'static since the actions struct is only used for types

                pub type UniqueArgs = ::prisma_client_rust::UniqueArgs<Types>;
                pub type ManyArgs = ::prisma_client_rust::ManyArgs<Types>;

                pub type Count<'a> = ::prisma_client_rust::Count<'a, Types>;
                pub type Create<'a> = ::prisma_client_rust::Create<'a, Types>;
                pub type CreateMany<'a> = ::prisma_client_rust::CreateMany<'a, Types>;
                pub type FindUnique<'a> = ::prisma_client_rust::FindUnique<'a, Types>;
                pub type FindMany<'a> = ::prisma_client_rust::FindMany<'a, Types>;
                pub type FindFirst<'a> = ::prisma_client_rust::FindFirst<'a, Types>;
                pub type Update<'a> = ::prisma_client_rust::Update<'a, Types>;
                pub type UpdateMany<'a> = ::prisma_client_rust::UpdateMany<'a, Types>;
                pub type Upsert<'a> = ::prisma_client_rust::Upsert<'a, Types>;
                pub type Delete<'a> = ::prisma_client_rust::Delete<'a, Types>;
                pub type DeleteMany<'a> = ::prisma_client_rust::DeleteMany<'a, Types>;

                #actions_struct
            }
        }
    }).collect()
}
