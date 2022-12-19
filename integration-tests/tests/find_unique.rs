use crate::{db::*, utils::*};

#[tokio::test]
async fn primary_key() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create("My post title!".to_string(), false, vec![])
        .exec()
        .await?;

    let found = client
        .post()
        .find_unique(post::id::equals(post.id.clone()))
        .exec()
        .await?;
    assert_eq!(found.unwrap().id, post.id);

    cleanup(client).await
}

#[tokio::test]
async fn unique_field() -> TestResult {
    let client = client().await;

    let user = client
        .user()
        .create(
            "Brendan".to_string(),
            vec![user::email::set(Some(
                "brendonovich@outlook.com".to_string(),
            ))],
        )
        .exec()
        .await?;

    let found = client
        .user()
        .find_unique(user::email::equals("brendonovich@outlook.com".to_string()))
        .exec()
        .await?
        .unwrap();
    assert_eq!(found.id, user.id);

    let found = client
        .user()
        .find_unique(user::email::equals("unknown".to_string()))
        .exec()
        .await?;
    assert!(found.is_none());

    cleanup(client).await
}

#[tokio::test]
async fn compound() -> TestResult {
    let client = client().await;

    let user = client
        .user()
        .create(
            "Brendan".to_string(),
            vec![user::email::set(Some(
                "brendonovich@outlook.com".to_string(),
            ))],
        )
        .exec()
        .await?;

    let post = client
        .post()
        .create(
            "Title".to_string(),
            false,
            vec![post::author::connect(user::id::equals(user.id.clone()))],
        )
        .exec()
        .await?;

    let found = client
        .post()
        .find_unique(post::title_author_id(post.title, user.id))
        .exec()
        .await?;
    assert!(found.is_some());

    cleanup(client).await
}

#[tokio::test]
async fn no_match() -> TestResult {
    let client = client().await;

    let found = client
        .post()
        .find_unique(post::id::equals("sdlfskdf".to_string()))
        .exec()
        .await?;
    assert!(found.is_none());

    cleanup(client).await
}
