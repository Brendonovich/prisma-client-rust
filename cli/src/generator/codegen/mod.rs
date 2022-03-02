mod client;
mod model;

use super::Root;

pub fn generate_prisma_client(root: &Root) -> String {
    let mut client = client::generate(root);

    for model in &root.dmmf.datamodel.models {
        client.extend(model::generate(model));
    }

    client.to_string()
}
