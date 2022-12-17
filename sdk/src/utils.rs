use std::{path::Path, process::Command, sync::Arc};

use datamodel::{datamodel_connector::ConnectorCapabilities, dml::Datamodel, Configuration};
use dmmf::{from_precomputed_parts, DataModelMetaFormat};
use prisma_models::InternalDataModelBuilder;
use query_core::schema_builder;

pub fn rustfmt(path: &Path) {
    Command::new("rustfmt")
        .arg("--edition=2021")
        .arg(path.to_str().unwrap())
        .output()
        .ok();
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
