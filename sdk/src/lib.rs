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

use proc_macro2::TokenStream;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

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

pub trait PrismaGenerator: DeserializeOwned {
    const NAME: &'static str;
    const DEFAULT_OUTPUT: &'static str;

    fn generate(self, args: GenerateArgs) -> TokenStream;

    fn erased_generate(args: GenerateArgs, config: Map<String, Value>) -> TokenStream
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
