use crate::db::*;
use crate::utils::*;

use prisma_client_rust::bigdecimal::BigDecimal;
use std::str::FromStr;

#[tokio::test]
async fn create() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create(
            "Hi from Prisma!".to_string(),
            true,
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
        .create("Brendan".to_string(), vec![])
        .exec()
        .await?;

    assert_eq!(user.name, "Brendan");

    cleanup(client).await
}

#[tokio::test]
async fn unique_violation() -> TestResult {
    let client = client().await;

    let user = client
        .user()
        .create(
            "Brendan".to_string(),
            vec![user::id::set("user-1".to_string())],
        )
        .exec()
        .await?;

    assert_eq!(user.name, "Brendan");
    assert_eq!(user.id, "user-1");

    let user = client
        .user()
        .create(
            "Brendan".to_string(),
            vec![user::id::set("user-1".to_string())],
        )
        .exec()
        .await;

    assert!(user.is_err());

    cleanup(client).await
}

#[tokio::test]
async fn set_none() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create("Post".to_string(), false, vec![post::desc::set(None)])
        .exec()
        .await?;

    assert_eq!(post.desc, None);

    cleanup(client).await
}

#[tokio::test]
async fn unchecked() -> TestResult {
    let client = client().await;

    let user = client
        .user()
        .create("Brendan".to_string(), vec![])
        .exec()
        .await?;

    let file_path = client
        .file_path()
        .create_unchecked(0, "".to_string(), user.id, vec![])
        .include(file_path::include!({ user }))
        .exec()
        .await?;

    assert_eq!(&file_path.user.name, &user.name);

    cleanup(client).await
}

#[tokio::test]
async fn from_struct() -> TestResult {
    let client = client().await;

    let post = post::Create {
        title: "Hi from Prisma!".to_string(),
        published: true,
        _params: vec![post::desc::set(Some(
            "Prisma is a database toolkit that makes databases easy.".to_string(),
        ))],
    }
    .to_query(&client)
    .exec()
    .await?;

    assert_eq!(post.title, "Hi from Prisma!");
    assert_eq!(
        post.desc,
        Some("Prisma is a database toolkit that makes databases easy.".to_string())
    );
    assert_eq!(post.published, true);

    let user = user::Create {
        name: "Brendan".to_string(),
        _params: vec![],
    }
    .to_query(&client)
    .exec()
    .await?;

    assert_eq!(user.name, "Brendan");

    cleanup(client).await
}
