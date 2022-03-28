mod client;
mod enums;
mod models;

use super::Root;

pub fn generate_prisma_client(root: &Root) -> String {
    let mut client = client::generate(root);

    client.extend(models::generate(root));
    client.extend(enums::generate(root));

    client.to_string()
}
