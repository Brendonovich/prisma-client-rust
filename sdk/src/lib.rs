mod args;
mod binaries;
mod casing;
mod dmmf;
mod extensions;
mod jsonrpc;
mod keywords;
mod prisma_cli;
mod runtime;
mod utils;

use runtime::{run_generator, GeneratorMetadata};

pub use args::GenerateArgs;
pub use casing::*;
pub use extensions::*;
pub use datamodel as prisma_datamodel;

pub trait PrismaGenerator {
    const NAME: &'static str;
    const DEFAULT_OUTPUT: &'static str;

    fn generate(args: GenerateArgs) -> String;
}

pub fn execute<G: PrismaGenerator>(args: &Vec<String>) {
    run_generator(
        args,
        GeneratorMetadata::new(G::generate, G::NAME, G::DEFAULT_OUTPUT),
    );
}
