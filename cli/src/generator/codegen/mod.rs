mod client;
mod enums;
mod header;
mod internal_enums;
mod models;

use crate::keywords::{is_reserved_name, UnderscoreSafeCasing};

use super::GeneratorArgs;
use convert_case::{Case, Casing};
use quote::quote;

pub fn generate_prisma_client(root: &GeneratorArgs) -> String {
    validate_names(root);

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

fn validate_names(args: &GeneratorArgs) {
    // ensure that model and field names are not conflicting with keywords
    for model in &args.dml.models {
        if is_reserved_name(&model.name.to_case_safe(Case::Snake)) {
            panic!(
                "Model '{}' produces reserved keyword '{}' and must be changed",
                model.name,
                model.name.to_case(Case::Snake)
            );
        }

        for field in &model.fields {
            if is_reserved_name(&field.name().to_case_safe(Case::Snake)) {
                panic!(
                    "Field '{}' of model '{}' produces reserved keyword '{}' and must be changed",
                    field.name(),
                    model.name,
                    field.name().to_case(Case::Snake)
                );
            }
        }
    }

    for e in &args.dml.enums {
        if is_reserved_name(&e.name.to_case_safe(Case::Pascal)) {
            panic!(
                "Enum '{}' produces reserved keyword '{}' and must be changed",
                e.name,
                e.name.to_case(Case::Pascal)
            );
        }
    }
}
