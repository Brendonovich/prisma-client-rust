pub mod binaries;
pub mod generator;
pub mod jsonrpc;
pub mod prisma_cli;

use crate::{
    generator::Root,
    jsonrpc::{
        methods::{Manifest, ManifestResponse},
        Request, Response,
    },
};
use datamodel::datamodel_connector::ConnectorCapabilities;
use generator::GeneratorArgs;
use prisma_models::InternalDataModelBuilder;
use query_core::{schema_builder, BuildMode, QuerySchemaRef, QuerySchemaRenderer};
use request_handlers::dmmf::schema::DmmfQuerySchemaRenderer;
use serde_json;
use serde_path_to_error;
use std::{
    default::Default,
    env,
    io::{stderr, stdin, BufRead, BufReader, Write},
    sync::Arc,
};

pub fn run() {
    let args = env::args();

    let args = args.skip(1).collect::<Vec<_>>();

    execute(&args);
}

pub fn execute(args: &Vec<String>) {
    if args.len() > 0 {
        prisma_cli::main(args);
        return;
    }

    if let Err(_) = std::env::var("PRISMA_GENERATOR_INVOCATION") {
        println!(
            "This command is only meant to be invoked internally. Please specify a command to run."
        );

        std::process::exit(1);
    }

    loop {
        let mut content = String::new();
        BufReader::new(stdin())
            .read_line(&mut content)
            .expect("Failed to read prisma cli output");

        let input: Request = serde_json::from_str(&content).unwrap();

        let value = match input.method.as_str() {
            "getManifest" => serde_json::to_value(ManifestResponse {
                manifest: Manifest {
                    default_output: "prisma.rs".to_string(),
                    pretty_name: "Prisma Client Rust".to_string(),
                    ..Default::default()
                },
            })
            .unwrap(),
            "generate" => {
                let params_str = input.params.to_string();

                let deserializer = &mut serde_json::Deserializer::from_str(&params_str);

                let result: Result<Root, _> = serde_path_to_error::deserialize(deserializer);

                match result {
                    Ok(params) => {
                        let datamodel = datamodel::parse_datamodel(&params.datamodel).unwrap();

                        let config = datamodel::parse_configuration(&params.datamodel).unwrap();
                        let datasource = config.subject.datasources.first();

                        let capabilities = datasource
                            .map(|ds| ds.capabilities())
                            .unwrap_or_else(ConnectorCapabilities::empty);

                        let referential_integrity = datasource
                            .map(|ds| ds.referential_integrity())
                            .unwrap_or_default();

                        let internal_data_model =
                            InternalDataModelBuilder::from(&datamodel.subject).build("".into());

                        let query_schema: QuerySchemaRef = Arc::new(schema_builder::build(
                            internal_data_model,
                            BuildMode::Modern,
                            true,
                            capabilities,
                            config.subject.preview_features().iter().collect(),
                            referential_integrity,
                        ));

                        let (schema, _) = DmmfQuerySchemaRenderer::render(query_schema);

                        generator::run(GeneratorArgs::new(
                            datamodel.subject,
                            schema,
                            params.datamodel,
                            params.generator.output.value.clone(),
                        ));
                    }
                    Err(err) => {
                        panic!("{}", err);
                    }
                };

                serde_json::Value::Null
            }
            _ => panic!(),
        };

        let response = Response {
            jsonrpc: "2.0".to_string(),
            id: input.id,
            result: value,
        };

        let mut bytes =
            serde_json::to_vec(&response).expect("Could not marshal json data for reply");

        bytes.push(b'\n');

        let bytes_arr = bytes.as_ref();

        stderr().by_ref().write(bytes_arr).unwrap();

        match input.method.as_str() {
            "generate" => break,
            _ => continue,
        };
    }
}
