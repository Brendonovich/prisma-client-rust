mod generator;

use std::env;

use generator::PrismaClientRustGenerator;
use prisma_client_rust_sdk::execute;

pub fn run() {
    let args = env::args();

    let args = args.skip(1).collect::<Vec<_>>();

    execute::<PrismaClientRustGenerator>(&args);
}
