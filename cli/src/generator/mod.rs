mod client;
mod enums;
mod header;
mod internal_enums;
mod models;
mod prelude;

use prelude::*;
use serde::Deserialize;

fn default_module_path() -> String {
    "prisma".to_string()
}

#[derive(Deserialize)]
pub struct PrismaClientRustGenerator {
    #[serde(default = "default_module_path")]
    module_path: String,
}

impl PrismaGenerator for PrismaClientRustGenerator {
    const NAME: &'static str = "Prisma Client Rust";
    const DEFAULT_OUTPUT: &'static str = "./prisma.rs";

    fn generate(self, args: GenerateArgs) -> String {
        let mut header = header::generate(&args);

        header.extend(models::generate(
            &args,
            self.module_path.parse().expect("Invalid module path"),
        ));

        let internal_enums = internal_enums::generate(&args);
        let client = client::generate(&args);

        let use_query_mode = match &args.connector {
            #[cfg(feature = "postgresql")]
            c if c.is_provider(prisma_datamodel::builtin_connectors::POSTGRES.name()) => true,
            #[cfg(feature = "mongodb")]
            c if c.is_provider(prisma_datamodel::builtin_connectors::MONGODB.name()) => true,
            _ => false,
        }
        .then(|| {
            quote!(
                pub use _prisma::QueryMode;
            )
        });

        header.extend(quote! {
            pub mod _prisma {
                #client
                #internal_enums
            }

            pub use _prisma::PrismaClient;
            #use_query_mode
        });

        header.extend(enums::generate(&args));

        header.to_string()
    }
}
