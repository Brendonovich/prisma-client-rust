use std::{path::Path, process::Command, sync::Arc};

use convert_case::Case;
use datamodel::{datamodel_connector::ConnectorCapabilities, dml::Datamodel, Configuration};
use dmmf::{from_precomputed_parts, DataModelMetaFormat};
use prisma_models::InternalDataModelBuilder;
use query_core::schema_builder;

use crate::{args::GenerateArgs, casing::Casing, keywords::is_reserved_keyword, GeneratorError};

pub fn rustfmt(path: &Path) {
    Command::new("rustfmt")
        .arg("--edition=2021")
        .arg(path.to_str().unwrap())
        .output()
        .ok();
}

/// Validates that names of models, fields and enums do not overlap with reserved Rust keywords.
pub fn validate_names(args: &GenerateArgs) -> Result<(), GeneratorError> {
    let mut errors = vec![];
    // ensure that model and field names are not conflicting with keywords
    for model in &args.dml.models {
        if is_reserved_keyword(&model.name.to_case(Case::Snake)) {
            errors.push(format!(
                "> Model '{}' produces reserved keyword '{}' and must be changed",
                model.name,
                model.name.to_case(Case::Snake)
            ));
        }

        for field in &model.fields {
            if is_reserved_keyword(&field.name().to_case(Case::Snake)) {
                errors.push(format!(
                    "> Field '{}' of model '{}' produces reserved keyword '{}' and must be changed",
                    field.name(),
                    model.name,
                    field.name().to_case(Case::Snake)
                ));
            }
        }
    }

    for e in &args.dml.enums {
        if is_reserved_keyword(&e.name.to_case(Case::Pascal)) {
            errors.push(format!(
                "> Enum '{}' produces reserved keyword '{}' and must be changed",
                e.name,
                e.name.to_case(Case::Pascal)
            ));
        }
    }

    match errors.len() {
        0 => Ok(()),
        _ => Err(GeneratorError::ReservedNames(errors.join("\n"))),
    }
}

pub fn build_schema(datamodel: &Datamodel, configuration: &Configuration) -> DataModelMetaFormat {
    let datasource = configuration.datasources.first();

    let capabilities = datasource
        .map(|ds| ds.capabilities())
        .unwrap_or_else(ConnectorCapabilities::empty);

    let referential_integrity = datasource
        .map(|ds| ds.referential_integrity())
        .unwrap_or_default();

    let internal_data_model = InternalDataModelBuilder::from(datamodel).build("".into());

    let query_schema = Arc::new(schema_builder::build(
        internal_data_model,
        true,
        capabilities,
        configuration.preview_features().iter().collect(),
        referential_integrity,
    ));

    // std::fs::write(
    //     "schema.graphql",
    //     GraphQLSchemaRenderer::render(query_schema.clone()),
    // );

    from_precomputed_parts(datamodel, query_schema)
}
