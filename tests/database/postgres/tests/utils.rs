use crate::db::PrismaClient;
use prisma_client_rust::QueryError;

pub type TestResult = Result<(), QueryError>;

pub async fn client() -> PrismaClient {
    let client = PrismaClient::_builder().build().await.unwrap();

    // client
    //     ._batch(vec![client.post().delete_many(vec![])])
    //     .await
    //     .unwrap();

    client
}

pub async fn cleanup(client: PrismaClient) -> TestResult {
    // client
    //     ._batch(vec![client.post().delete_many(vec![])])
    //     .await
    //     .unwrap();

    Ok(())
}
