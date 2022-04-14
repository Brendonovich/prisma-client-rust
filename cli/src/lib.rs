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
        println!("This command is only meant to be invoked internally. Please run the following instead:");
        println!("`prisma-client-rust <command>`");
        println!("e.g.");
        println!("`prisma-client-rust generate`");

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

        match input.method.as_str() {
            "generate" => break,
            _ => continue,
        };
    }
}
