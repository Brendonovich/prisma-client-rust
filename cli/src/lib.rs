mod binaries;
mod generator;
mod prisma_cli;

use prisma_client_rust_sdk::*;
use std::env;

use generator::PrismaClientRustGenerator;

pub fn run() {
    let args = env::args();

    let args = args.skip(1).collect::<Vec<_>>();

    if std::env::var("PRISMA_GENERATOR_INVOCATION").is_err() {
        prisma_cli::main(&args);
        return;
    }

    PrismaClientRustGenerator::run();
}
