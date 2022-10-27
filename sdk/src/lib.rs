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

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};
use thiserror::Error;

use runtime::{run_generator, GeneratorMetadata};

pub use args::GenerateArgs;
pub use casing::*;
pub use extensions::*;
pub use quote::quote;

pub mod prisma {
    pub use datamodel;
    pub use dmmf;
    pub use prisma_models;
    pub use query_core;
    pub use request_handlers;
}

pub mod prelude {
    pub use super::{
        prisma::{datamodel::dml, *},
        *,
    };
    pub use proc_macro2::*;
    pub use quote::*;
    pub use syn::Ident;

    pub fn ident(name: &str) -> Ident {
        format_ident!("{name}")
    }

    pub fn snake_ident(name: &str) -> Ident {
        format_ident!("{}", name.to_case(Case::Snake))
    }

    pub fn pascal_ident(name: &str) -> Ident {
        format_ident!("{}", name.to_case(Case::Pascal))
    }
}

pub type GenerateFn = fn(GenerateArgs, Map<String, Value>) -> GenerateResult;
pub type GenerateResult = Result<String, GeneratorError>;

#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error("Schema contains invalid names \n{0}")]
    ReservedNames(String),
    #[error("Failed to create client file")]
    FileCreate(std::io::Error),
    #[error("Failed to write generated client to file")]
    FileWrite(std::io::Error),
    #[error("Failed to deserialize generator arguments")]
    ArgDeserialize(serde_json::Error),
    #[error("Generator {name} failed: \n{message}")]
    InternalError { name: &'static str, message: String },
}

pub trait PrismaGenerator: DeserializeOwned {
    const NAME: &'static str;
    const DEFAULT_OUTPUT: &'static str;

    type Error: Serialize + std::error::Error;

    fn generate(self, args: GenerateArgs) -> Result<String, Self::Error>;

    fn erased_generate(args: GenerateArgs, config: Map<String, Value>) -> GenerateResult
    where
        Self: Sized,
    {
        let generator = serde_json::from_value::<Self>(Value::Object(config))
            .map_err(GeneratorError::ArgDeserialize)?;

        generator
            .generate(args)
            .map_err(|e| GeneratorError::InternalError {
                name: Self::NAME,
                message: e.to_string(),
            })
    }
}

pub fn execute<G: PrismaGenerator>(args: &Vec<String>) {
    run_generator(
        GeneratorMetadata::new(G::erased_generate, G::NAME, G::DEFAULT_OUTPUT),
        args,
    );
}
