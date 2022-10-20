mod args;
mod binaries;
mod casing;
pub mod dmmf;
mod extensions;
mod jsonrpc;
mod keywords;
mod prisma_cli;
mod runtime;
mod utils;

use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use runtime::{run_generator, GeneratorMetadata};

pub use args::GenerateArgs;
pub use casing::*;
pub use extensions::*;

pub mod prisma {
    pub use datamodel;
    pub use dmmf;
    pub use prisma_models;
    pub use query_core;
    pub use request_handlers;
}

pub trait PrismaGenerator: DeserializeOwned {
    const NAME: &'static str;
    const DEFAULT_OUTPUT: &'static str;

    fn generate(self, args: GenerateArgs) -> String;

    fn erased_generate(args: GenerateArgs, config: Map<String, Value>) -> String
    where
        Self: Sized,
    {
        serde_json::from_value::<Self>(Value::Object(config))
            .unwrap()
            .generate(args)
    }
}

pub fn execute<G: PrismaGenerator>(args: &Vec<String>) {
    run_generator(
        GeneratorMetadata::new(G::erased_generate, G::NAME, G::DEFAULT_OUTPUT),
        args,
    );
}
