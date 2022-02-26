pub mod types;
pub use types::*;
pub mod codegen;
pub mod dmmf;

use prisma_client_rust::binaries::{
    fetch_engine, global_cache_dir, platform::binary_platform_name,
};
use std::env;
use std::fs::File;
use std::io::Write as IoWrite;

fn add_defaults(input: &mut Root) -> &mut Root {
    match input.generator.config.get("package") {
        Some(_) => {}
        None => {
            input
                .generator
                .config
                .insert("package".to_owned(), "db".to_owned());
        }
    };

    input
}

pub fn run(input: &mut Root) {
    add_defaults(input);

    let mut targets = input.generator.binary_targets.clone();

    targets.push("native".to_string());
    targets.push("linux".to_string());

    for name in targets {
        let binary_name = if name == "native" {
            binary_platform_name()
        } else {
            name
        };

        fetch_engine(global_cache_dir(), "query-engine".to_string(), binary_name).unwrap();
    }

    let mut file = File::create(env::current_dir().unwrap().join("prisma.rs"))
        .expect("Failed to open file for codegen");

    let client = codegen::generate_prisma_client(input);

    file.write(client.as_bytes());
}
