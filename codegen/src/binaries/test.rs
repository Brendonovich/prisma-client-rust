use datamodel::parse_datamodel;
use inflector::Inflector;
use prisma_models::dml::Field;
use prisma_models::DatamodelConverter;
use query_core::{BuildMode, QuerySchemaBuilder, SupportedCapabilities};
use query_engine::dmmf::schema::TypeKind;
use query_engine::dmmf::{
    render_dmmf,
    schema::{DMMFOutputType, DMMFTypeInfo},
};
use serde::Serialize;
use serde_json::{json, Value};
use std::{fs, path::PathBuf, sync::Arc};

#[derive(Debug, Serialize, Clone)]
struct TypeName {
    render: String,
    actual: String,
}
#[derive(Debug, Serialize)]
struct Enum {
    name: String,
    default: String,
    variants: Vec<TypeName>,
}

#[derive(Debug, Serialize, Clone)]
struct TypeField {
    is_required: bool,
    r#type: String,
    name: TypeName,
}

#[derive(Debug, Serialize)]
struct Type {
    name: String,
    fields: Vec<TypeField>,
}

/// Generates the client.
pub fn write_to_dir(datamodel: &str, path: PathBuf) {
    let model_str = fs::read_to_string(PathBuf::from(datamodel))
        .expect("failed to read .prisma file");
    fs::write(path, generate(&model_str))
        .expect("Error while writing to prisma.rs");
}

fn generate(model_str: &str) -> String {
    let model = parse_datamodel(&model_str).unwrap();
    let internal_model = DatamodelConverter::convert(&model).build("".into());
    let cap = SupportedCapabilities::empty();
    let schema_builder = QuerySchemaBuilder::new(&internal_model, &cap, BuildMode::Modern, false);
    let query_schema = Arc::new(schema_builder.build());
    let dmmf = render_dmmf(&model, query_schema);
    let mut tt = tinytemplate::TinyTemplate::new();

    tt.add_template("client", include_str!("./prisma.rs.template"))
        .unwrap();

    let models = model
        .models
        .into_iter()
        .map(|m| {
            m.fields
                .into_iter()
                .filter_map(|f| {
                    if f.field_type.is_relation() {
                        Some(f)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    let enums = dmmf
        .schema
        .enums
        .into_iter()
        .map(|enu| {
            let variants = enu
                .values
                .iter()
                .map(|v| TypeName {
                    render: v.to_class_case(),
                    actual: v.clone(),
                })
                .collect::<Vec<_>>();

            Enum {
                name: enu.name,
                default: enu.values[0].to_class_case(),
                variants,
            }
        })
        .collect::<Vec<_>>();

    let inputs = dmmf
        .schema
        .input_types
        .iter()
        .map(|typ| {
            let fields = typ
                .fields
                .iter()
                .map(|field| {
                    let name = if field.name == "where" {
                        "filter".to_owned()
                    } else {
                        field.name.to_snake_case()
                    };

                    let is_relation = is_relation(&models, &field.name);

                    TypeField {
                        is_required: field.input_type.is_required,
                        name: TypeName {
                            render: name,
                            actual: field.name.clone(),
                        },
                        r#type: format_to_rust_type(
                            &field.input_type,
                            is_relation,
                            typ.name.contains("UpdateInput"),
                        ),
                    }
                })
                .collect::<Vec<_>>();

            Type {
                name: typ.name.to_pascal_case(),
                fields,
            }
        })
        .collect::<Vec<_>>();

    let (query, mutation) = (dmmf.schema.root_query_type, dmmf.schema.root_mutation_type);

    let (outputs, others): (Vec<DMMFOutputType>, Vec<DMMFOutputType>) =
        dmmf.schema.output_types.into_iter().partition(|typ| {
            if typ.name == query || typ.name == mutation || typ.name.contains("Aggregate") {
                false
            } else {
                true
            }
        });

    let outputs = outputs
        .iter()
        .map(|typ| {
            let fields = typ
                .fields
                .iter()
                .map(|field| {
                    let is_relation = is_relation(&models, &field.name);
                    TypeField {
                        is_required: field.output_type.is_required,
                        name: TypeName {
                            render: field.name.to_snake_case(),
                            actual: field.name.clone(),
                        },
                        r#type: format_to_rust_type(&field.output_type, is_relation, false),
                    }
                })
                .collect::<Vec<_>>();

            Type {
                name: typ.name.to_pascal_case(),
                fields,
            }
        })
        .collect::<Vec<_>>();

    let operations: Vec<Value> = others
        .iter()
        .filter_map(|typ| build_operation(typ))
        .collect();

    let data = json!({
        "operations": operations,
        "inputs": inputs,
        "outputs": outputs,
        "enums": enums,
        "datamodel": model_str,
    });

    tt.render("client", &data).unwrap()
}

fn is_relation(models: &Vec<Field>, name: &str) -> bool {
    models
        .iter()
        .filter_map(|f| {
            if name.contains(&f.name) {
                Some(())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .len()
        > 0
}

/// converts DMMFTypeInfo to a rust type
fn format_to_rust_type(typ: &DMMFTypeInfo, needs_box: bool, is_update: bool) -> String {
    let formatted = match typ.typ.as_str() {
        // graphql scalar types.
        "Int" => "i32",
        "Float" => "f64",
        "DateTime" => "DateTime<Utc>",
        "Boolean" => "bool",
        _ => &typ.typ,
    };

    let formatted = match typ.kind {
        TypeKind::Object if needs_box => format!("Box<{}>", formatted),
        _ => formatted.to_string(),
    };

    let formatted = if typ.is_list {
        format!("Vec<{}>", formatted)
    } else {
        formatted
    };

    if !typ.is_required && is_update {
        format!("Option<Option<{}>>", formatted)
    } else if !typ.is_required {
        format!("Option<{}>", formatted)
    } else {
        formatted
    }
}

fn build_operation(out: &DMMFOutputType) -> Option<Value> {
    let operation = out.name.to_lowercase();

    if operation.contains("aggregate") {
        return None;
    }

    let (methods, outputs) = out.fields.iter().fold(
        (Vec::new(), Vec::new()),
        |(mut methods, mut outputs), field| {
            let mut arg_name = if field.name.contains("aggregate") {
                format!(
                    ", data: FindMany{}Args",
                    field.name.replace("aggregate", "")
                )
            } else {
                format!(", data: {}", format_arg_name(&field.name))
            };

            let only = field.args.len() == 1;
            let args = if field.name.contains("aggregate") {
                out.fields
                    .iter()
                    .find(|f| f.name == format!("findMany{}", field.name.replace("aggregate", "")))
                    .unwrap()
                    .args
                    .iter()
                    .map(|arg| TypeField {
                        is_required: arg.input_type.is_required,
                        name: TypeName {
                            render: match arg.name.as_str() {
                                "where" => "filter".to_owned(),
                                "orderBy" => "order_by".to_owned(),
                                _ => arg.name.clone(),
                            },
                            actual: arg.name.clone(),
                        },
                        r#type: format_to_rust_type(&arg.input_type, false, false),
                    })
                    .collect::<Vec<_>>()
            } else {
                field
                    .args
                    .iter()
                    .map(|arg| TypeField {
                        is_required: arg.input_type.is_required,
                        name: TypeName {
                            render: match arg.name.as_str() {
                                "where" => "filter".to_owned(),
                                "orderBy" => "order_by".to_owned(),
                                _ => arg.name.clone(),
                            },
                            actual: arg.name.clone(),
                        },
                        r#type: format_to_rust_type(&arg.input_type, false, false),
                    })
                    .collect::<Vec<_>>()
            };

            if only {
                let a = args.first().unwrap();
                arg_name = format!(", {}: {}", a.name.render, a.r#type);
            } else if !field.name.contains("aggregate") {
                let output = Type {
                    name: format_arg_name(&field.name),
                    fields: args.clone(),
                };
                outputs.push(output);
            }

            let use_batch = field.name.contains("deleteMany")
                || field.name.contains("updateMany")
                || field.name.contains("aggregate");

            let generics = if !use_batch { "<T>" } else { "" };

            let return_ty = if use_batch {
                "BatchPayload"
            } else if field.name.contains("findOne") {
                "T"
            } else if field.name.contains("findMany") {
                "Vec<T>"
            } else {
                "T"
            };

            let query_name = if field.name.contains("aggregate") {
                format!("{} {{ count", field.name)
            } else {
                field.name.clone()
            };

            let query = if field.name.contains("aggregate") {
                String::from(r#""}""#)
            } else if use_batch {
                String::from(r#""{ count }""#)
            } else {
                String::from("T::query()")
            };

            let method = json!({
                "fn_name": format_method_name(field.name.clone()),
                "query_name": query_name,
                "data_name": field.name,
                "args": args,
                "only": only,
                "arg": arg_name,
                "generics": generics,
                "is_batch": use_batch,
                "query": query,
                "return": return_ty
            });

            methods.push(method);

            (methods, outputs)
        },
    );

    Some(json!({
        "name": operation,
        "methods": methods,
        "outputs": outputs,
    }))
}

fn format_arg_name(name: &str) -> String {
    format!("{}Args", name.to_pascal_case())
}

/// formats method name from
/// `findManyUser` to `users`
/// `findOneUser` to `user`
/// `deleteOneUser` to `delete_user`, updateOneUser` to `update_user`,
/// `deleteManyUser` to `delete_users`, updateManyUser` to `update_users`,
fn format_method_name(name: String) -> String {
    if name.contains("findMany") {
        return name
            .replace("findMany", " ")
            .to_snake_case()
            .to_lowercase()
            .to_plural();
    }

    if name.contains("findOne") {
        return name.replace("findOne", "").to_snake_case().to_lowercase();
    }

    if name.contains("One") {
        return name.replace("One", " ").to_snake_case().to_lowercase();
    }

    name.replace("Many", " ")
        .to_snake_case()
        .to_lowercase()
        .to_plural()
}

#[cfg(test)]
mod test {
    #[test]
    fn generate_client() {
        let out = super::generate(
            r##"
datasource pg {
	provider = "mysql"
	url = "mysql://root:prisma@localhost:3306/default@default"
}

model User {
    id String @id @default(cuid())
}
"##,
        );
        println!("{}", out);
    }
}
