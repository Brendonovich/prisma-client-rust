pub mod types;
pub use types::*;
pub mod dmmf;

use std::borrow::Borrow;
use std::{fs, env};
use crate::binaries::platform::binary_platform_name;
use crate::binaries::{fetch_engine, global_cache_dir};
use std::path::Path;
use std::fs::File;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use serde_json::Value;
use convert_case::{Case, Casing};
use tinytemplate::error::Error;
use std::ops::Deref;

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

    let mut file = File::create(
        env::current_dir()
            .unwrap()
            .join("prisma.rs")
    ).expect("Failed to open file for codegen");

    let mut tt = tinytemplate::TinyTemplate::new();

    tt.add_formatter("snake", |v: &Value, s: &mut String| {
        match v {
            Value::String(v_str) => {
                write!(s, "{}", v_str.to_case(Case::Snake));
                Ok(())
            },
            _ => Err(Error::GenericError {msg: "".to_string()})
        }
    });
    tt.add_formatter("camel", |v: &Value, s: &mut String| {
        match v {
            Value::String(v_str) => {
                write!(s, "{}", v_str.to_case(Case::Camel));
                Ok(())
            },
            _ => Err(Error::GenericError {msg: "".to_string()})
        }
    });
    tt.add_formatter("uppercamel", |v: &Value, s: &mut String| {
        match v {
            Value::String(v_str) => {
                write!(s, "{}", v_str.to_case(Case::UpperCamel));
                Ok(())
            },
            _ => Err(Error::GenericError {msg: "".to_string()})
        }
    });

    tt.add_template("client", include_str!("templates/client.rstpl")).unwrap();

    let input_json: serde_json::Value = serde_json::to_value(input).unwrap();

    file.write(tt.render("client", &input_json).unwrap().as_bytes());
}