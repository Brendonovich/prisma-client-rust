mod actions;
mod client;
mod models;
mod query;

use super::Root;

pub fn generate_prisma_client(root: &Root) -> String {
    let mut client = client::generate_client(root);

    client.extend(actions::generate_actions(&root.dmmf.datamodel.models));
    client.extend(models::generate_models(&root.dmmf.datamodel.models));
    client.extend(query::generate_queries(&root.dmmf.datamodel.models));

    client.to_string()
}
