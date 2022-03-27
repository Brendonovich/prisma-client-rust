mod client;
mod enums;
mod model;

use super::{Root, ast::AST};

pub fn generate_prisma_client(root: &AST) -> String {
    let mut client = client::generate(root);

    for model in &root.dmmf.datamodel.models {
        client.extend(model::generate(model));
    }

    for e in &root.dmmf.datamodel.enums {
        client.extend(enums::generate(e))
    }

    client.to_string()
}
