use crate::db::{new_client, PrismaClient};
use prisma_client_rust::queries::Error;

pub type TestResult = Result<(), Error>;

pub async fn client() -> PrismaClient {
    let client = new_client().await.unwrap();

    client
        .category()
        .find_many(vec![])
        .delete()
        .exec()
        .await
        .unwrap();
    client
        .post()
        .find_many(vec![])
        .delete()
        .exec()
        .await
        .unwrap();
    client
        .profile()
        .find_many(vec![])
        .delete()
        .exec()
        .await
        .unwrap();
    client
        .user()
        .find_many(vec![])
        .delete()
        .exec()
        .await
        .unwrap();

    client
}

pub async fn cleanup(client: PrismaClient) -> TestResult {
    client
        .category()
        .find_many(vec![])
        .delete()
        .exec()
        .await
        .unwrap();
    client
        .post()
        .find_many(vec![])
        .delete()
        .exec()
        .await
        .unwrap();
    client
        .profile()
        .find_many(vec![])
        .delete()
        .exec()
        .await
        .unwrap();
    client
        .user()
        .find_many(vec![])
        .delete()
        .exec()
        .await
        .unwrap();
    client
        .types()
        .find_many(vec![])
        .delete()
        .exec()
        .await
        .unwrap();

    Ok(())
}
