mod args;
mod casing;
pub mod dmmf;
mod extensions;
mod jsonrpc;
mod keywords;
mod runtime;
mod utils;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};
use thiserror::Error;

use runtime::GeneratorMetadata;

pub use args::GenerateArgs;
pub use casing::*;
pub use extensions::*;
pub use quote::quote;

pub mod prisma {
    pub use dmmf;
    pub use prisma_models;
    pub use psl;
    pub use query_core;
    pub use request_handlers;
}

pub mod prelude {
    pub use super::{args::*, prisma::*, *};
    pub use proc_macro2::*;
    pub use quote::*;
    pub use syn::Ident;

    pub fn ident(name: &str) -> Ident {
        format_ident!("{name}")
    }

    pub fn snake_ident(name: &str) -> Ident {
        format_ident!("{}", name.to_case(Case::Snake, false))
    }

    pub fn snake_ident_raw(name: &str) -> Ident {
        format_ident!("{}", name.to_case(Case::Snake, true))
    }

    pub fn pascal_ident(name: &str) -> Ident {
        format_ident!("{}", name.to_case(Case::Pascal, false))
    }
}

pub type GenerateFn = fn(GenerateArgs, Map<String, Value>) -> GenerateResult;
pub type GenerateResult = Result<String, GeneratorError>;

#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error("Schema contains invalid names \n{0}")]
    ReservedNames(String),
    #[error("Failed to create client file: {0}")]
    FileCreate(std::io::Error),
    #[error("Failed to write generated client to file: {0}")]
    FileWrite(std::io::Error),
    #[error("Failed to deserialize generator arguments: {0}")]
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

    fn run() {
        GeneratorMetadata::new(Self::erased_generate, Self::NAME, Self::DEFAULT_OUTPUT).run();
    }
}
