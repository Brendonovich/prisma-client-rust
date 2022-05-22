mod client;
mod enums;
mod header;
mod internal_enums;
mod models;

use super::GeneratorArgs;
use quote::quote;

pub fn generate_prisma_client(root: &GeneratorArgs) -> String {
    let mut header = header::generate(root);

    header.extend(models::generate(root));

    let internal_enums = internal_enums::generate(root);
    let client = client::generate(root);

    header.extend(quote! {
        pub mod _prisma {
            #client
            #internal_enums
        }

        pub use _prisma::PrismaClient;
    });

    header.extend(enums::generate(root));

    header.to_string()
}
