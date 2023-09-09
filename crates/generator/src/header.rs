use proc_macro2::TokenStream;
use quote::quote;
use std::path::*;

use crate::GenerateArgs;

fn find_migrations_path(schema_path: &PathBuf) -> PathBuf {
    schema_path
        .parent()
        .map(|p| p.join("migrations"))
        .filter(|p| p.exists())
        .expect("Migrations folder not found! Please create one in the same folder as your schema.prisma file.")
}

pub fn generate(args: &GenerateArgs) -> TokenStream {
    let database_string = &args.engine_dmmf.datasources[0].provider;

    let pcr = quote!(::prisma_client_rust);

    let schema_path_str = &args.engine_dmmf.schema_path;
    let schema_path = schema_path_str
        .parse()
        .expect("Failed to parse schema path!");

    let migrations_include = cfg!(feature = "migrations")
        .then(|| {
            let migrations_path = find_migrations_path(&schema_path);
            let migrations_path = migrations_path.to_str().expect("Invalid migrations path");

            quote!(
                use #pcr::migrations::include_dir;
                pub static MIGRATIONS_DIR: &#pcr::migrations::include_dir::Dir =
                    &#pcr::migrations::include_dir::include_dir!(#migrations_path);
            )
        })
        .unwrap_or_default();

    quote! {
        pub static DATAMODEL_STR: &'static str = include_str!(#schema_path_str);
        static DATABASE_STR: &'static str = #database_string;

        #migrations_include
    }
}
