use prisma_models::{walkers::ScalarFieldWalker, FieldArity};
use psl::{
    builtin_connectors,
    datamodel_connector::Connector,
    parser_database::{ScalarFieldType, ScalarType},
    ValidatedSchema,
};
use std::collections::HashSet;

use dmmf::{DataModelMetaFormat, DmmfInputField, DmmfInputType, DmmfSchema, TypeLocation};
use proc_macro2::TokenStream;

use crate::{dmmf::EngineDMMF, prelude::*};

pub struct GenerateArgs<'a> {
    pub schema: &'a ValidatedSchema,
    pub engine_dmmf: EngineDMMF,
    pub dmmf: &'a DataModelMetaFormat,
    pub read_filters: Vec<Filter<'a>>,
    pub write_params: Vec<Filter<'a>>,
    pub connector: &'static dyn Connector,
}

impl<'a> GenerateArgs<'a> {
    pub fn new(
        schema: &'a ValidatedSchema,
        dmmf: &'a DataModelMetaFormat,
        engine_dmmf: EngineDMMF,
    ) -> Self {
        let scalars = dmmf
            .schema
            .input_object_types
            .get("prisma")
            .unwrap()
            .iter()
            .flat_map(|scalar| {
                scalar.fields.iter().flat_map(|field| {
                    field.input_types.iter().flat_map(|input| {
                        matches!(input.location, TypeLocation::Scalar)
                            .then(|| ScalarType::try_from_str(&input.typ))
                            .flatten()
                    })
                })
            })
            .collect::<HashSet<_>>();

        let read_filters = {
            let mut filters = vec![];

            for scalar in &scalars {
                let possible_filters = [
                    scalar.to_dmmf_string() + "ListFilter",
                    scalar.to_dmmf_string() + "NullableListFilter",
                    scalar.to_dmmf_string() + "Filter",
                    scalar.to_dmmf_string() + "NullableFilter",
                ];

                filters.extend(possible_filters.iter().filter_map(|filter| {
                    let filter_type = dmmf.schema.find_input_type(&filter)?;

                    let mut s = scalar.as_str().to_string();

                    // checking for both is invalid - fields can be list or null but not both
                    // TODO: make this more typesafe to correspond with fields
                    if filter_type.name.contains("List") {
                        s += "List";
                    } else if filter_type.name.contains("Nullable") {
                        s += "Nullable";
                    }

                    Some(Filter {
                        name: s,
                        fields: filter_type.fields.iter().collect(),
                    })
                }));
            }

            for enm in schema.db.walk_enums() {
                let possible_filters = [
                    "Enum".to_string() + &enm.ast_enum().name.name + "Filter",
                    "Enum".to_string() + &enm.ast_enum().name.name + "NullableFilter",
                ];

                filters.extend(possible_filters.iter().filter_map(|filter| {
                    let filter_type = dmmf.schema.find_input_type(&filter)?;

                    let mut name = enm.ast_enum().name.name.clone();

                    // checking for both is invalid - fields can be list or null but not both
                    // TODO: make this more typesafe to correspond with fields
                    if filter_type.name.contains("List") {
                        name += "List";
                    } else if filter_type.name.contains("Nullable") {
                        name += "Nullable";
                    }

                    Some(Filter {
                        name,
                        fields: filter_type.fields.iter().collect(),
                    })
                }));
            }

            // for i in 0..dml.models.len() {
            //     let m = &dml.models[i];
            //     let p =
            //         match schema.find_input_type(&(m.name.to_string() + "OrderByRelevanceInput")) {
            //             Some(p) => p,
            //             None => continue,
            //         };

            //     let mut methods = vec![];

            //     for field in &p.fields {
            //         if let Some(method) = input_field_as_method(field) {
            //             methods.push(method);
            //         }
            //     }

            //     filters.push(Filter {
            //         name: m.name.clone(),
            //         methods,
            //     });

            //     dml.models[i]
            //         .fields
            //         .push(Field::ScalarField(ScalarField::new(
            //             "relevance",
            //             FieldArity::Optional,
            //             FieldType::Enum(p.name.clone()),
            //         )));
            // }

            filters
        };

        let write_filters = {
            let mut filters = vec![];

            filters.extend(scalars.iter().flat_map(|scalar| {
                if matches!(scalar, ScalarType::Json) {
                    return vec![Filter {
                        name: "Json".to_string(),
                        fields: vec![],
                    }];
                }

                let possible_inputs = [
                    format!("{}FieldUpdateOperationsInput", scalar.to_dmmf_string()),
                    format!(
                        "Nullable{}FieldUpdateOperationsInput",
                        scalar.to_dmmf_string()
                    ),
                ];

                possible_inputs
                    .into_iter()
                    .filter_map(|input| {
                        let input_type = dmmf.schema.find_input_type(&input)?;

                        let mut s = scalar.as_str().to_string();

                        if input_type.name.contains("List") {
                            s += "List";
                        } else if input_type.name.contains("Nullable") {
                            s += "Nullable";
                        }

                        Some(Filter {
                            name: s,
                            fields: input_type
                                .fields
                                .iter()
                                .filter_map(|field| {
                                    field.input_types.iter().find(|input_type| match input_type
                                        .location
                                    {
                                        TypeLocation::Scalar if input_type.typ != "Null" => true,
                                        _ => false,
                                    })?;

                                    Some(field)
                                })
                                .collect(),
                        })
                    })
                    .collect()
            }));

            filters.extend(schema.db.walk_enums().flat_map(|enm| {
                let possible_inputs = [
                    format!("Enum{}FieldUpdateOperationsInput", enm.name()),
                    format!("NullableEnum{}FieldUpdateOperationsInput", enm.name()),
                ];

                possible_inputs.into_iter().filter_map(move |input| {
                    let input_type = dmmf.schema.find_input_type(&input)?;

                    let mut name = enm.name().to_string();

                    if input_type.name.contains("List") {
                        name += "List";
                    } else if input_type.name.contains("Nullable") {
                        name += "Nullable";
                    }

                    Some(Filter {
                        name,
                        fields: input_type
                            .fields
                            .iter()
                            .filter_map(|field| {
                                field.input_types.iter().find(|input_type| {
                                    match input_type.location {
                                        TypeLocation::Scalar if input_type.typ != "Null" => true,
                                        _ => true,
                                    }
                                })?;

                                Some(field)
                            })
                            .collect(),
                    })
                })
            }));

            filters.extend(schema.db.walk_models().flat_map(|model| {
                model
                    .fields()
                    .filter_map(|field| {
                        let input_type = dmmf.schema.find_input_type(&format!(
                            "{}Update{}Input",
                            model.name(),
                            field.name()
                        ))?;

                        let mut fields = vec![];

                        let scalar_name = {
                            let mut scalar_name = None;

                            fields.extend(input_type.fields.iter().filter_map(|field| {
                                if field.name == "set" {
                                    for input_type in &field.input_types {
                                        match input_type.location {
                                            TypeLocation::Scalar if input_type.typ != "null" => {
                                                scalar_name = Some(input_type.typ.clone() + "List");
                                            }
                                            _ => {}
                                        }
                                    }
                                }

                                field
                                    .input_types
                                    .iter()
                                    .find(|input_type| match input_type.location {
                                        TypeLocation::Scalar if input_type.typ != "null" => true,
                                        _ => false,
                                    })
                                    .map(|_| field)
                            }));

                            scalar_name
                        }?;

                        Some(Filter {
                            name: scalar_name,
                            fields,
                        })
                    })
                    .collect::<Vec<_>>()
            }));

            filters
        };

        use builtin_connectors::*;
        let connector = match &engine_dmmf.datasources[0].provider {
            #[cfg(feature = "sqlite")]
            p if SQLITE.is_provider(p) => SQLITE,
            #[cfg(feature = "postgresql")]
            p if POSTGRES.is_provider(p) => POSTGRES,
            #[cfg(feature = "postgresql")]
            p if COCKROACH.is_provider(p) => COCKROACH,
            #[cfg(feature = "mssql")]
            p if MSSQL.is_provider(p) => MSSQL,
            #[cfg(feature = "mysql")]
            p if MYSQL.is_provider(p) => MYSQL,
            #[cfg(feature = "mongodb")]
            p if MONGODB.is_provider(p) => MONGODB,
            p => panic!(
                "Database provider {p} is not available. Have you enabled its Cargo.toml feature?"
            ),
        };

        Self {
            schema,
            dmmf,
            engine_dmmf,
            read_filters,
            write_params: write_filters,
            connector,
        }
    }

    pub fn read_filter(&self, field: ScalarFieldWalker) -> Option<&Filter> {
        let postfix = match field.ast_field().arity {
            FieldArity::List => "List",
            FieldArity::Optional => "Nullable",
            _ => "",
        };

        let base = match field.scalar_field_type() {
            ScalarFieldType::BuiltInScalar(typ) => typ.as_str(),
            ScalarFieldType::Enum(e) => field.db.walk(e).name(),
            _ => return None,
        };

        self.read_filters
            .iter()
            .find(|f| f.name == format!("{base}{postfix}"))
    }

    pub fn write_param(&self, field: ScalarFieldWalker) -> Option<&Filter> {
        let postfix = match field.ast_field().arity {
            FieldArity::List => "List",
            FieldArity::Optional => "Nullable",
            _ => "",
        };

        let base = match field.scalar_field_type() {
            ScalarFieldType::BuiltInScalar(typ) => typ.as_str(),
            ScalarFieldType::Enum(e) => field.db.walk(e).name(),
            _ => return None,
        };

        self.write_params
            .iter()
            .find(|f| f.name == format!("{base}{postfix}"))
    }
}

pub trait DmmfSchemaExt {
    fn find_input_type(&self, name: &str) -> Option<&DmmfInputType>;
}

impl DmmfSchemaExt for DmmfSchema {
    fn find_input_type(&self, name: &str) -> Option<&DmmfInputType> {
        self.input_object_types
            .get("prisma")
            .and_then(|t| t.iter().find(|i| i.name == name))
    }
}

pub trait DmmfInputFieldExt {
    fn arity(&self) -> FieldArity;
    fn type_tokens(&self, prefix: &TokenStream) -> TokenStream;
    fn to_prisma_value(&self, var: &Ident) -> TokenStream;
}

impl DmmfInputFieldExt for DmmfInputField {
    fn arity(&self) -> FieldArity {
        let input_type = self
            .input_types
            .iter()
            .find(|typ| !matches!(typ.location, TypeLocation::Scalar if typ.typ == "Null"))
            .expect(&format!("No type found for field {}", self.name));

        if input_type.is_list {
            FieldArity::List
        } else if self.is_nullable {
            FieldArity::Optional
        } else {
            FieldArity::Required
        }
    }

    fn type_tokens(&self, prefix: &TokenStream) -> TokenStream {
        let input_type = self
            .input_types
            .iter()
            .find(|typ| !matches!(typ.location, TypeLocation::Scalar if typ.typ == "Null"))
            .expect(&format!("No type found for field {}", self.name));

        let arity = self.arity();

        match input_type.location {
            TypeLocation::Scalar => arity.wrap_type(
                &ScalarType::try_from_str(&input_type.typ)
                    .unwrap()
                    .to_tokens(),
            ),
            TypeLocation::EnumTypes => {
                let typ: TokenStream = input_type.typ.parse().unwrap();
                arity.wrap_type(&quote!(#prefix #typ))
            }
            TypeLocation::InputObjectTypes => {
                let typ: TokenStream = input_type.typ.parse().unwrap();
                quote!(Vec<#prefix #typ>)
            }
            _ => todo!(),
        }
    }

    fn to_prisma_value(&self, var: &Ident) -> TokenStream {
        let pv = quote!(::prisma_client_rust::PrismaValue);

        let input_type = self
            .input_types
            .iter()
            .find(|typ| !matches!(typ.location, TypeLocation::Scalar if typ.typ == "Null"))
            .expect(&format!("No type found for field {}", self.name));

        let arity = self.arity();

        match input_type.location {
            TypeLocation::Scalar => arity.wrap_pv(
                var,
                ScalarType::try_from_str(&input_type.typ)
                    .unwrap()
                    .to_prisma_value(var),
            ),
            TypeLocation::EnumTypes => arity.wrap_pv(var, quote!(#pv::Enum(#var.to_string()))),
            TypeLocation::InputObjectTypes => {
                quote!(#pv::Object(#var.into_iter().map(Into::into).collect()))
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct Filter<'a> {
    pub name: String,
    pub fields: Vec<&'a DmmfInputField>,
}
