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

    let count = client.post().count(vec![]).exec().await?;

    assert_eq!(count, 1);

    cleanup(client).await
}

#[tokio::test]
async fn test_count_no_results() -> TestResult {
    let client = client().await;

    let count = client
        .post()
        .count(vec![post::title::equals("lkdjkfsldkf".to_string())])
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

    let count = client.post().count(vec![]).take(1).exec().await?;

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

    let count = client.post().count(vec![]).skip(1).exec().await?;

    assert_eq!(count, 2);

    cleanup(client).await
}
