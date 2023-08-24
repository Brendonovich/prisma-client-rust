mod args;
mod casing;
pub mod dmmf;
mod extensions;
mod jsonrpc;
mod keywords;
mod runtime;
mod shared_config;
mod utils;

use std::path::{Path, PathBuf};

use proc_macro2::TokenStream;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};
use thiserror::Error;

use runtime::GeneratorMetadata;

pub use args::GenerateArgs;
pub use casing::*;
pub use extensions::*;
pub use quote::quote;

use crate::prelude::snake_ident;

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
pub type GenerateResult = Result<Module, GeneratorError>;

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

pub struct Module {
    pub name: String,
    pub contents: TokenStream,
    pub submodules: Vec<Module>,
}

impl Module {
    pub fn new(name: &str, contents: TokenStream) -> Self {
        Self {
            name: name.to_string(),
            contents,
            submodules: vec![],
        }
    }

    pub fn add_submodule(&mut self, submodule: Module) {
        self.submodules.push(submodule);
    }

    pub fn flatten(&self) -> TokenStream {
        let contents = &self.contents;

        let submodule_contents = self
            .submodules
            .iter()
            .map(|sm| {
                let name = snake_ident(&sm.name);
                let contents = sm.flatten();

                quote! {
                    pub mod #name {
                        #contents
                    }
                }
            })
            .collect::<Vec<_>>();

        quote! {
            #contents

            #(#submodule_contents)*
        }
    }

    pub fn get_all_paths(&self, parent_path: &Path) -> Vec<PathBuf> {
        if self.submodules.len() > 0 {
            [parent_path.join("mod.rs")]
                .into_iter()
                .chain(self.submodules.iter().flat_map(|sm| {
                    sm.get_all_paths(&parent_path.join(&sm.name.to_case(Case::Snake, true)))
                }))
                .collect()
        } else {
            vec![parent_path.with_extension("rs")]
        }
    }
}

pub trait PrismaGenerator: DeserializeOwned {
    const NAME: &'static str;
    const DEFAULT_OUTPUT: &'static str;

    type Error: Serialize + std::error::Error;

    fn generate(self, args: GenerateArgs) -> Result<Module, Self::Error>;

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
