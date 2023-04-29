use prisma_models::{walkers::ScalarFieldWalker, FieldArity};
use psl::{
    builtin_connectors,
    datamodel_connector::Connector,
    parser_database::{ParserDatabase, ScalarFieldType, ScalarType},
    ValidatedSchema,
};
use std::sync::Arc;

use dmmf::{DataModelMetaFormat, DmmfInputField, DmmfInputType, DmmfSchema, TypeLocation};
use proc_macro2::TokenStream;

use crate::{dmmf::EngineDMMF, prelude::*};

pub struct GenerateArgs {
    pub schema: Arc<ValidatedSchema>,
    pub engine_dmmf: EngineDMMF,
    pub dmmf: DataModelMetaFormat,
    pub read_filters: Vec<Filter>,
    pub write_filters: Vec<Filter>,
    pub connector: &'static dyn Connector,
}

impl GenerateArgs {
    pub fn new(
        schema: Arc<ValidatedSchema>,
        dmmf: DataModelMetaFormat,
        engine_dmmf: EngineDMMF,
    ) -> Self {
        let scalars = {
            let mut scalars = Vec::new();
            for scalar in dmmf.schema.input_object_types.get("prisma").unwrap() {
                for field in &scalar.fields {
                    for input in &field.input_types {
                        if let TypeLocation::Scalar = input.location {
                            let name = &input.typ;

                            if scalars.iter().any(|s| s == name) {
                                continue;
                            }

                            scalars.push(name.to_string());
                        }
                    }
                }
            }
            scalars
        };

        let read_filters = {
            let mut filters = vec![];

            for scalar in &scalars {
                let scalar = match scalar.as_str() {
                    "Boolean" => "Bool".to_string(),
                    n => n.to_string(),
                };

                let combinations = [
                    scalar.to_string() + "ListFilter",
                    scalar.to_string() + "NullableListFilter",
                    scalar.to_string() + "Filter",
                    scalar.to_string() + "NullableFilter",
                ];

                for c in combinations {
                    let p = match dmmf.schema.find_input_type(&c) {
                        Some(p) => p,
                        None => continue,
                    };

                    let mut fields = vec![];

                    for field in &p.fields {
                        if let Some(method) = input_field_as_method(field, &schema.db) {
                            fields.push(method);
                        }
                    }

                    let mut s = scalar.clone();

                    // checking for both is invalid - fields can be list or null but not both
                    // TODO: make this more typesafe to correspond with fields
                    if p.name.contains("List") {
                        s += "List";
                    } else if p.name.contains("Nullable") {
                        s += "Nullable";
                    }

                    filters.push(Filter {
                        name: s,
                        methods: fields,
                    })
                }
            }

            for e in schema.db.walk_enums() {
                let combinations = [
                    "Enum".to_string() + &e.ast_enum().name.name + "Filter",
                    "Enum".to_string() + &e.ast_enum().name.name + "NullableFilter",
                ];

                for c in combinations {
                    let p = match dmmf.schema.find_input_type(&c) {
                        Some(t) => t,
                        None => continue,
                    };

                    let mut fields = vec![];

                    for field in &p.fields {
                        if let Some(method) = input_field_as_method(field, &schema.db) {
                            fields.push(method);
                        }
                    }

                    let mut name = e.ast_enum().name.name.clone();

                    // checking for both is invalid - fields can be list or null but not both
                    // TODO: make this more typesafe to correspond with fields
                    if p.name.contains("List") {
                        name += "List";
                    } else if p.name.contains("Nullable") {
                        name += "Nullable";
                    }

                    filters.push(Filter {
                        name,
                        methods: fields,
                    });
                }
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

            for scalar in &scalars {
                let combinations = [
                    scalar.clone() + "FieldUpdateOperationsInput",
                    "Nullable".to_string() + scalar + "FieldUpdateOperationsInput",
                ];

                for c in combinations {
                    let p = match dmmf.schema.find_input_type(&c) {
                        Some(p) => p,
                        None => continue,
                    };

                    let mut fields = vec![];

                    for field in &p.fields {
                        if field.name == "set" {
                            continue;
                        }

                        if let Some((type_name, is_list)) = {
                            let mut ret = None;
                            for input_type in &field.input_types {
                                match input_type.location {
                                    TypeLocation::Scalar if input_type.typ != "Null" => {
                                        ret = Some((input_type.typ.clone(), input_type.is_list))
                                    }
                                    _ => {}
                                }
                            }
                            ret
                        } {
                            fields.push(Method::new(
                                pascal_ident(&field.name).to_string(),
                                field.name.clone(),
                                ScalarType::try_from_str(&type_name)
                                    .map(ScalarFieldType::BuiltInScalar)
                                    .unwrap_or_else(|| {
                                        ScalarFieldType::Enum(
                                            schema.db.find_enum(&type_name).unwrap().id,
                                        )
                                    }),
                                is_list,
                                field.is_nullable,
                            ));
                        }
                    }
                    filters.push(Filter {
                        name: scalar.clone(),
                        methods: fields,
                    });
                }
            }

            for model in schema.db.walk_models() {
                for field in model.fields() {
                    let p = match dmmf.schema.find_input_type(
                        &(model.name().to_string() + "Update" + field.name() + "Input"),
                    ) {
                        Some(p) => p,
                        None => continue,
                    };

                    let mut fields = vec![];

                    if let Some(scalar_name) = {
                        let mut scalar_name = None;

                        for field in &p.fields {
                            if field.name == "set" {
                                for input_type in &field.input_types {
                                    match input_type.location {
                                        TypeLocation::Scalar if input_type.typ != "null" => {
                                            scalar_name = Some(input_type.typ.clone() + "List");
                                        }
                                        _ => {}
                                    }
                                }

                                continue;
                            }

                            if let Some((type_name, is_list)) = {
                                let mut ret = None;

                                for input_type in &field.input_types {
                                    match input_type.location {
                                        TypeLocation::Scalar if input_type.typ != "null" => {
                                            ret = Some((input_type.typ.clone(), input_type.is_list))
                                        }
                                        _ => {}
                                    }
                                }

                                ret
                            } {
                                fields.push(Method::new(
                                    pascal_ident(&field.name).to_string(),
                                    field.name.clone(),
                                    ScalarFieldType::BuiltInScalar(
                                        ScalarType::try_from_str(&type_name).unwrap(),
                                    ),
                                    is_list,
                                    field.is_nullable,
                                ));
                            };
                        }

                        scalar_name
                    } {
                        filters.push(Filter {
                            name: scalar_name,
                            methods: fields,
                        })
                    }
                }
            }

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
            write_filters,
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
            ScalarFieldType::BuiltInScalar(typ) => match typ.as_str() {
                "Boolean" => "Bool".to_string(),
                n => n.to_string(),
            },
            ScalarFieldType::Enum(e) => field.db.walk(e).name().to_string(),
            _ => return None,
        };

        self.read_filters
            .iter()
            .find(|f| f.name == format!("{base}{postfix}"))
    }

    pub fn write_filter(&self, field: ScalarFieldWalker) -> Option<&Filter> {
        match &field.scalar_field_type() {
            ScalarFieldType::BuiltInScalar(typ) => {
                let mut typ = typ.as_str().to_string();

                if field.ast_field().arity.is_list() {
                    typ += "List";
                }

                self.write_filters.iter().find(|f| f.name == typ)
            }
            _ => None,
        }
    }
}

trait DmmfSchemaExt {
    fn find_input_type(&self, name: &str) -> Option<&DmmfInputType>;
}

impl DmmfSchemaExt for DmmfSchema {
    fn find_input_type(&self, name: &str) -> Option<&DmmfInputType> {
        self.input_object_types
            .get("prisma")
            .and_then(|t| t.iter().find(|i| i.name == name))
    }
}

#[derive(Clone, Debug)]
pub struct Method {
    pub name: String,
    pub action: String,
    pub is_list: bool,
    pub is_nullable: bool,
    pub base_type: ScalarFieldType,
}

impl Method {
    fn new(
        name: String,
        action: String,
        base_type: ScalarFieldType,
        is_list: bool,
        is_nullable: bool,
    ) -> Self {
        Method {
            name,
            action,
            is_list,
            is_nullable,
            base_type,
        }
    }

    pub fn type_tokens(&self, prefix: &TokenStream, db: &ParserDatabase) -> Option<TokenStream> {
        self.base_type.to_tokens(prefix, &self.arity(), db)
    }

    pub fn arity(&self) -> FieldArity {
        if self.is_list {
            FieldArity::List
        } else if self.is_nullable {
            FieldArity::Optional
        } else {
            FieldArity::Required
        }
    }
}

#[derive(Debug)]
pub struct Filter {
    pub name: String,
    pub methods: Vec<Method>,
}

/// Gets a method definition from an input field.
fn input_field_as_method(field: &DmmfInputField, db: &ParserDatabase) -> Option<Method> {
    field.input_types.iter().find(|input_type|
        matches!(input_type.location, TypeLocation::Scalar | TypeLocation::EnumTypes if input_type.typ != "Null")
    ).map(|input_type| {
        let type_name = input_type.typ.clone();
        let is_list = input_type.is_list;

        // if let None = db.find_enum(&type_name) {
        //     panic!("bruh: {type_name}");
        // }

        Method::new(
            // 'in' is a reserved keyword in Rust
            match field.name.as_str() {
                "in" => "InVec".to_string(),
                "notIn" => "NotInVec".to_string(),
                name => pascal_ident(name).to_string(),
            },
            field.name.clone(),
            ScalarType::try_from_str(&type_name)
                .map(ScalarFieldType::BuiltInScalar)
                .unwrap_or_else(|| ScalarFieldType::Enum(db.find_enum(&type_name).unwrap().id)),
            is_list,
            field.is_nullable,
        )
    })
}
