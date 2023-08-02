mod client;
mod composite_types;
mod enums;
mod header;
mod internal_enums;
mod models;
mod read_filters;
mod write_params;

use prisma_client_rust_sdk::prelude::*;
use serde::Serialize;

fn default_module_path() -> String {
    "prisma".to_string()
}

#[derive(serde::Deserialize)]
pub struct Generator {
    #[serde(default = "default_module_path")]
    module_path: String,
}

#[derive(Debug, Serialize, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse module_path")]
    InvalidModulePath,
}

impl PrismaGenerator for Generator {
    const NAME: &'static str = "Prisma Client Rust";
    const DEFAULT_OUTPUT: &'static str = "../src/prisma.rs";

    type Error = Error;

    fn generate(self, args: GenerateArgs) -> Result<Module, Self::Error> {
        let header = header::generate(&args);

        let module_path = {
            let provided: TokenStream = self
                .module_path
                .parse()
                .map_err(|_| Error::InvalidModulePath)?;

            quote!(#provided::)
        };

        let enums = enums::generate(&args);

        let mut module = Module::new(
            "client",
            quote! {
                #header

                pub use _prisma::*;

                #enums
            },
        );

        let client = client::generate(&args);
        let internal_enums = internal_enums::generate(&args);
        let read_filters_module = read_filters::generate_module(&args);
        let write_params_module = write_params::generate_module(&args);

        module.add_submodule(Module::new(
            "_prisma",
            quote! {
                #client
                #internal_enums
                #read_filters_module
                #write_params_module
            },
        ));

        models::modules(&args, &module_path)
            .into_iter()
            .for_each(|model| module.add_submodule(model));
        composite_types::modules(&args, &module_path)
            .into_iter()
            .for_each(|ct| module.add_submodule(ct));

        Ok(module)
    }
}

pub fn run() {
    Generator::run();
}
