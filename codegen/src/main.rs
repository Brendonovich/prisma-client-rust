use prisma_client_rust::{cli, generator};
use std::env;
use std::io;
use std::io::{BufReader, BufRead, Write};
use std::default::Default;
use serde_json;
use serde_json::{json, Value};
use serde_path_to_error;
use prisma_client_rust::jsonrpc::{Request, Response};
use prisma_client_rust::jsonrpc::methods::{ManifestResponse, Manifest};
use prisma_client_rust::generator::Root;
use std::any::Any;
use serde::Serialize;
use std::iter::Map;

fn main(){
    let args = env::args().skip(1);

    if args.len() > 0 {
        cli::main(&args.collect());
    }

    invoke_prisma();
}

fn invoke_prisma() -> Result<(), ()> {
    loop {
        let mut content = String::new();
        BufReader::new(io::stdin()).read_line(&mut content);

        let input: Request = serde_json::from_str(&content).unwrap();

        let value = match input.method.as_str() {
            "getManifest" => serde_json::to_value(ManifestResponse {
                manifest: Manifest {
                    default_output: "prisma.rs".to_string(),
                    pretty_name: "Prisma Client Rust".to_string(),
                    ..Default::default()
                }
            }
        ).unwrap(),
            "generate" => {
                let params_str = input.params.to_string();

                let deserializer =
                    &mut serde_json::Deserializer::from_str(&params_str);

                let result: Result<Root, _> =
                    serde_path_to_error::deserialize(deserializer);

                match result {
                    Ok(mut params) => {
                        println!("Generating");
                        generator::run(&mut params);
                    }
                    Err(err) => {
                        panic!("{}", err);
                    }
                };

                serde_json::to_value(json!(null)).unwrap()
            },
            _ => panic!()
        };


        let response = Response {
            jsonrpc: "2.0".to_string(),
            id: input.id,
            result: value
        };

        let mut bytes = serde_json::to_vec(&response)
            .expect("Could not marshal json data for reply");

        bytes.push(b'\n');

        let bytes_arr = bytes.as_ref();

        io::stderr().by_ref().write(bytes_arr).unwrap();
    }

    Ok(())
}