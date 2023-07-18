mod binaries;
mod prisma_cli;

use std::env;

pub fn run() {
    let args = env::args();

    let args = args.skip(1).collect::<Vec<_>>();

    if std::env::var("PRISMA_GENERATOR_INVOCATION").is_err() {
        prisma_cli::main(&args);
        return;
    }

    prisma_client_rust_generator::run();
}
