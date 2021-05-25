pub mod types;
pub use types::*;

use std::borrow::Borrow;
use std::{fs, env};
use crate::binaries::platform::binary_platform_name;
use crate::binaries::{fetch_engine, global_cache_dir};
use std::path::Path;

fn add_defaults(input: &mut Root) -> &mut Root {
    match input.generator.config.get("package") {
        Some(_) => {},
        None => { input.generator.config.insert(
            "package".to_owned(),
            "db".to_owned());
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

    let file_path = env::current_dir().unwrap().join("prisma.rs");
    fs::write( file_path, "FFFFFFFF".to_string()).unwrap();
}

