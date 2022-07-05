use crate::db::*;
use crate::utils::*;

#[tokio::test]
async fn test_count() -> TestResult {
    let client = client().await;

    client
        .post()
        .create("Hi from Prisma!".to_string(), true, vec![])
        .exec()
        .await?;

    let count = client.post().count().exec().await?;

    assert_eq!(count, 1);

    cleanup(client).await
}

#[tokio::test]
async fn test_count_no_results() -> TestResult {
    let client = client().await;

    let count = client
        .post()
        .find_many(vec![post::title::equals("lkdjkfsldkf".to_string())])
        .count()
        .exec()
        .await?;

    assert_eq!(count, 0);

    cleanup(client).await
}

#[tokio::test]
async fn test_take() -> TestResult {
    let client = client().await;

    client
        .post()
        .create("Hi from Prisma!".to_string(), true, vec![])
        .exec()
        .await?;

    client
        .post()
        .create("Hi from Prisma!".to_string(), true, vec![])
        .exec()
        .await?;

    client
        .post()
        .create("Hi from Prisma!".to_string(), true, vec![])
        .exec()
        .await?;

    let count = client.post().count().take(1).exec().await?;

    assert_eq!(count, 1);

    cleanup(client).await
}

#[tokio::test]
async fn test_skip() -> TestResult {
    let client = client().await;

    client
        .post()
        .create("Hi from Prisma!".to_string(), true, vec![])
        .exec()
        .await?;

    client
        .post()
        .create("Hi from Prisma!".to_string(), true, vec![])
        .exec()
        .await?;

    client
        .post()
        .create("Hi from Prisma!".to_string(), true, vec![])
        .exec()
        .await?;

    let count = client.post().count().skip(1).exec().await?;

    assert_eq!(count, 2);

    cleanup(client).await
}
