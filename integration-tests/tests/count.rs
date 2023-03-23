use crate::db::*;
use crate::utils::*;

async fn create_posts(client: &PrismaClient) -> TestResult {
    client
        .post()
        .create_many(vec![
            post::create_unchecked("Hi from Prisma!".to_string(), true, vec![]),
            post::create_unchecked("Hi from Prisma!".to_string(), true, vec![]),
            post::create_unchecked("Hi from Prisma!".to_string(), false, vec![]),
        ])
        .exec()
        .await?;

    Ok(())
}

#[tokio::test]
async fn basic() -> TestResult {
    let client = client().await;

    create_posts(&client).await?;

    let count = client.post().count(vec![]).exec().await?;

    assert_eq!(count, 3);

    cleanup(client).await
}

#[tokio::test]
async fn no_results() -> TestResult {
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
async fn where_() -> TestResult {
    let client = client().await;

    create_posts(&client).await?;

    let published_count = client
        .post()
        .count(vec![post::published::equals(true)])
        .exec()
        .await?;
    assert_eq!(published_count, 2);

    let unpublished_count = client
        .post()
        .count(vec![post::published::equals(false)])
        .exec()
        .await?;
    assert_eq!(unpublished_count, 1);

    cleanup(client).await
}

#[tokio::test]
async fn take() -> TestResult {
    let client = client().await;

    create_posts(&client).await?;

    let count = client.post().count(vec![]).take(1).exec().await?;

    assert_eq!(count, 1);

    cleanup(client).await
}

#[tokio::test]
async fn skip() -> TestResult {
    let client = client().await;

    create_posts(&client).await?;

    let count = client.post().count(vec![]).skip(1).exec().await?;

    assert_eq!(count, 2);

    cleanup(client).await
}
