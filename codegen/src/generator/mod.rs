pub mod types;
pub use types::*;
pub mod codegen;
pub mod dmmf;

use prisma_client_rust_core::binaries::{
    self, bindata, fetch_engine, global_cache_dir, platform::binary_platform_name,
};
use std::fs;
use std::fs::File;
use std::io::Write as IoWrite;
use std::path::Path;

pub fn run(input: &mut Root) {
    let targets = &input.generator.binary_targets;

    for target in targets {
        let binary_name = if target.value == "native" {
            binary_platform_name()
        } else {
            target.value.to_string()
        };

        fetch_engine(global_cache_dir(), "query-engine".to_string(), binary_name).unwrap();
    }

    let output = &input.generator.output.value;

    fs::create_dir_all(output).unwrap();

    let mut file =
        File::create(&Path::new(output).join("mod.rs")).expect("Failed to open file for codegen");

    input.engine_modules = if input
        .generator
        .config
        .disable_rust_binaries
        .as_ref()
        .map(|x| x != "true")
        .unwrap_or(true)
    {
        Some(generate_query_engine_files(
            &targets.iter().map(|t| t.value.to_string()).collect(),
            output,
        ))
    } else {
        None
    };

    let client = codegen::generate_prisma_module(input);

    file.write(client.as_bytes()).unwrap();
}

fn generate_query_engine_files(binary_targets: &Vec<String>, output_dir: &str) -> Vec<String> {
    let mut res = Vec::new();

    for binary_target in binary_targets {
        let binary_name = if binary_target == "native" {
            binary_platform_name()
        } else {
            binary_target.into()
        };

        let engine_path =
            binaries::get_engine_path(binaries::global_cache_dir(), "query-engine", &binary_name);

        // let pt = if binary_name.contains("debian") || binary_name.contains("rhel") {
        //     "linux".into()
        // } else {
        //     binary_name
        // };

        let module_name = format!("query_engine_{}_gen", binary_name);
        let filename = format!("{}.rs", module_name);
        let to = Path::new(output_dir).join(filename);

        bindata::write_file(
            &binary_name.replace("-", "_"),
            &engine_path,
            to.to_str().unwrap(),
        );

        res.push(module_name);
    }

    res
}
