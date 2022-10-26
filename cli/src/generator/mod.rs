mod client;
mod enums;
mod header;
mod internal_enums;
mod models;
mod read_filters;

use prisma_client_rust_sdk::prelude::*;

fn default_module_path() -> String {
    "prisma".to_string()
}

#[derive(serde::Deserialize)]
pub struct PrismaClientRustGenerator {
    #[serde(default = "default_module_path")]
    module_path: String,
}

impl PrismaGenerator for PrismaClientRustGenerator {
    const NAME: &'static str = "Prisma Client Rust";
    const DEFAULT_OUTPUT: &'static str = "../src/prisma.rs";

    fn generate(self, args: GenerateArgs) -> TokenStream {
        let header = header::generate(&args);

        let models = models::generate(
            &args,
            self.module_path.parse().expect("Invalid module path"),
        );

        let internal_enums = internal_enums::generate(&args);
        let client = client::generate(&args);

        let use_query_mode = match &args.connector {
            #[cfg(feature = "postgresql")]
            c if c.is_provider(datamodel::builtin_connectors::POSTGRES.name()) => true,
            #[cfg(feature = "mongodb")]
            c if c.is_provider(datamodel::builtin_connectors::MONGODB.name()) => true,
            _ => false,
        }
        .then(|| {
            quote!(
                pub use _prisma::QueryMode;
            )
        });
        let read_filters_module = read_filters::generate_module(&args);
        let enums = enums::generate(&args);

        quote! {
            #header

            #(#models)*

            pub mod _prisma {
                #client
                #internal_enums
                #read_filters_module
            }

            pub use _prisma::PrismaClient;
            #use_query_mode

            #enums
        }
    }
}
