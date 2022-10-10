use crate::db::*;
use crate::utils::*;

/// Using create_many with SQLite is currently unstable
/// Enabling the CLI's `sqlite-create-many` is unsafe and
/// could cause problems

#[tokio::test]
async fn basic() -> TestResult {
    let client = client().await;

    let data = vec![
        post::create_unchecked(
            "Hi from Prisma!".to_string(),
            true,
            vec![post::desc::set(Some(
                "Prisma is a database toolkit that makes databases easy.".to_string(),
            ))],
        );
        1000
    ];

    let posts_count = client.post().create_many(data).exec().await?;

    assert_eq!(posts_count, 1000);

    cleanup(client).await
}

#[tokio::test]
async fn skip_duplicates() -> TestResult {
    let client = client().await;

    let data = vec![
        post::create_unchecked(
            "Hi from Prisma!".to_string(),
            true,
            vec![post::id::set("0".to_string())],
        );
        1000
    ];

    let posts_count = client
        .post()
        .create_many(data)
        .skip_duplicates()
        .exec()
        .await?;

    assert_eq!(posts_count, 1);

    cleanup(client).await
}
