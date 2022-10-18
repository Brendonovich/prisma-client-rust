use proc_macro2::TokenStream;
use quote::quote;
use std::env::current_dir;
use std::path::*;

use crate::generator::GenerateArgs;

static SCHEMA_PRISMA: &'static str = "schema.prisma";

fn find_schema_path() -> Result<PathBuf, ()> {
    let current_dir = current_dir().expect("Failed to get current directory");
    let current_dir = Path::new(&current_dir);

    let root_file_path = current_dir.join(SCHEMA_PRISMA);
    let nested_file_path = current_dir.join("prisma").join(SCHEMA_PRISMA);

    match root_file_path.exists() {
        true => Ok(root_file_path),
        false if nested_file_path.exists() => Ok(nested_file_path),
        _ => Err(()),
    }
}

fn find_migrations_path() -> Result<PathBuf, ()> {
    let schema_path = find_schema_path()?;

    let migrations_path = schema_path
        .parent()
        .expect("Schema path has no parent!")
        .join("migrations");

    migrations_path.exists().then(|| migrations_path).ok_or(())
}

pub fn generate(args: &GenerateArgs) -> TokenStream {
    let database_string = &args.datasources[0].provider;

    let pcr = quote!(::prisma_client_rust);

    let schema_path = find_schema_path().expect("Schema not found!");
    let schema_path = schema_path.to_str().expect("Invalid schema path");

    let migrations_include = cfg!(feature = "migrations")
        .then(|| {
            let migrations_path = find_migrations_path().expect("Migrations folder not found!");
            let migrations_path = migrations_path.to_str().expect("Invalid migrations path");

            quote!(
                use #pcr::migrations::include_dir;
                pub static MIGRATIONS_DIR: &#pcr::migrations::include_dir::Dir =
                    &#pcr::migrations::include_dir::include_dir!(#migrations_path);
            )
        })
        .unwrap_or_default();

    quote! {
        #![allow(warnings, unused)]

        pub static DATAMODEL_STR: &'static str = include_str!(#schema_path);
        static DATABASE_STR: &'static str = #database_string;

        #migrations_include

        pub async fn new_client() -> Result<PrismaClient, #pcr::NewClientError> {
            PrismaClient::_builder().build().await
        }

        // adapted from https://github.com/polytope-labs/prisma-client-rs/blob/0dec2a67081e78b42700f6a62f414236438f84be/codegen/src/prisma.rs.template#L182
        pub async fn new_client_with_url(url: &str) -> Result<PrismaClient, #pcr::NewClientError> {
            PrismaClient::_builder().with_url(url.to_string()).build().await
        }
    }
}
