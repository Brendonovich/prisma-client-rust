use crate::db::{new_client, PrismaClient};
use prisma_client_rust::queries::QueryError;

pub type TestResult = Result<(), QueryError>;

pub async fn client() -> PrismaClient {
    let client = new_client().await.unwrap();

    client.category().delete_many(vec![]).exec().await.unwrap();
    client.post().delete_many(vec![]).exec().await.unwrap();
    client.profile().delete_many(vec![]).exec().await.unwrap();
    client.user().delete_many(vec![]).exec().await.unwrap();
    client.types().delete_many(vec![]).exec().await.unwrap();

    client
}

pub async fn cleanup(client: PrismaClient) -> TestResult {
    client.category().delete_many(vec![]).exec().await.unwrap();
    client.post().delete_many(vec![]).exec().await.unwrap();
    client.profile().delete_many(vec![]).exec().await.unwrap();
    client.user().delete_many(vec![]).exec().await.unwrap();
    client.types().delete_many(vec![]).exec().await.unwrap();

    Ok(())
}
