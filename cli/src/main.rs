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
use generator::ast::AST;
use serde_json;
use serde_path_to_error;
use std::{
    default::Default,
    env,
    io::{stderr, stdin, BufRead, BufReader, Write},
};

fn main() {
    let args = env::args();

    if args.len() > 1 {
        let args = args.skip(1).collect::<Vec<_>>();
        let command: &str = &args[0];

        match command {
            "prefetch" => {
                prisma_cli::main(&vec!["-v".into()]);
                return;
            }
            _ => prisma_cli::main(&args),
        }

        return;
    }

    if let Err(_) = std::env::var("PRISMA_GENERATOR_INVOCATION") {
        println!("This command is only meant to be invoked internally. Please run the following instead:");
        println!("`prisma-client-rust <command>`");
        println!("e.g.");
        println!("`prisma-client-rust generate`");

        std::process::exit(1);
    }

    invoke_prisma();
}

fn invoke_prisma() -> Result<(), ()> {
    loop {
        let mut content = String::new();
        BufReader::new(stdin()).read_line(&mut content);

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
                    Ok(mut params) => {
                        let ast = AST::new(&params.dmmf);
                        params.ast = Some(ast);

                        generator::run(&params);
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

        return match input.method.as_str() {
            "generate" => Ok(()),
            _ => continue,
        };
    }
}
