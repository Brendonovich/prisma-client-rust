mod client;
mod enums;
mod header;
mod internal_enums;
mod models;
pub(crate) mod prelude;
mod read_filters;

use prelude::*;
use serde::{Deserialize, Serialize};

fn default_module_path() -> String {
    "prisma".to_string()
}

#[derive(Deserialize)]
pub struct PrismaClientRustGenerator {
    #[serde(default = "default_module_path")]
    module_path: String,
}

#[derive(Debug, Serialize, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse module_path")]
    InvalidModulePath,
}

impl PrismaGenerator for PrismaClientRustGenerator {
    const NAME: &'static str = "Prisma Client Rust";
    const DEFAULT_OUTPUT: &'static str = "./prisma.rs";

    type Error = Error;

    fn generate(self, args: GenerateArgs) -> Result<String, Self::Error> {
        let mut header = header::generate(&args);

        header.extend(models::generate(
            &args,
            self.module_path
                .parse()
                .map_err(|_| Error::InvalidModulePath)?,
        ));

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

        header.extend(quote! {
            pub mod _prisma {
                #client
                #internal_enums
                #read_filters_module
            }

            pub use _prisma::PrismaClient;
            #use_query_mode
        });

        header.extend(enums::generate(&args));

        Ok(header.to_string())
    }
}
