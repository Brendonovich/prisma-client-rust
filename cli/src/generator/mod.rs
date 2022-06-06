mod client;
mod enums;
mod header;
mod internal_enums;
mod models;

use prisma_client_rust_sdk::{GenerateArgs, PrismaGenerator};
use quote::quote;

pub struct PrismaClientRustGenerator {}

impl PrismaGenerator for PrismaClientRustGenerator {
    const NAME: &'static str = "Prisma Client Rust";
    const DEFAULT_OUTPUT: &'static str = "./prisma.rs";

    fn generate(args: GenerateArgs) -> String {
        let mut header = header::generate(&args);

        header.extend(models::generate(&args));

        let internal_enums = internal_enums::generate(&args);
        let client = client::generate(&args);

        header.extend(quote! {
            pub mod _prisma {
                #client
                #internal_enums
            }

            pub use _prisma::PrismaClient;
        });

        header.extend(enums::generate(&args));

        header.to_string()
    }
}
