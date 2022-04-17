use crate::db::*;
use crate::utils::*;

#[tokio::test]
async fn test_create() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create(
            post::title::set("Hi from Prisma!".to_string()),
            post::published::set(true),
            vec![post::desc::set(Some(
                "Prisma is a database toolkit that makes databases easy.".to_string(),
            ))],
        )
        .exec()
        .await?;

    assert_eq!(post.title, "Hi from Prisma!");
    assert_eq!(
        post.desc,
        Some("Prisma is a database toolkit that makes databases easy.".to_string())
    );
    assert_eq!(post.published, true);

    let user = client
        .user()
        .create(user::name::set("Brendan".to_string()), vec![])
        .exec()
        .await?;

    assert_eq!(user.name, "Brendan");

    cleanup(client).await
}

#[tokio::test]
async fn test_create_unique_violation() -> TestResult {
    let client = client().await;

    let user = client
        .user()
        .create(
            user::name::set("Brendan".to_string()),
            vec![user::id::set("user-1".to_string())],
        )
        .exec()
        .await?;

    assert_eq!(user.name, "Brendan");
    assert_eq!(user.id, "user-1");

    let user = client
        .user()
        .create(
            user::name::set("Brendan".to_string()),
            vec![user::id::set("user-1".to_string())],
        )
        .exec()
        .await;

    assert!(user.is_err());

    cleanup(client).await
}

#[tokio::test]
async fn test_setting_field_to_null() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create(
            post::title::set("Post".to_string()),
            post::published::set(false),
            vec![post::desc::set(None)],
        )
        .exec()
        .await?;

    assert_eq!(post.desc, None);

    cleanup(client).await
}
