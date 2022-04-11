use crate::{
    db::{Post, User},
    utils::*,
};

#[tokio::test]
async fn find_unique_id_field() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create(
            Post::title().set("My post title!".to_string()),
            Post::published().set(false),
            vec![],
        )
        .exec()
        .await?;

    let found = client
        .post()
        .find_unique(Post::id().equals(post.id.clone()))
        .exec()
        .await?;
    assert_eq!(found.unwrap().id, post.id);

    cleanup(client).await
}

#[tokio::test]
async fn find_unique_no_match() -> TestResult {
    let client = client().await;

    let found = client
        .post()
        .find_unique(Post::id().equals("sdlfskdf".to_string()))
        .exec()
        .await?;
    assert!(found.is_none());

    cleanup(client).await
}

#[tokio::test]
async fn find_unique_by_unique_field() -> TestResult {
    let client = client().await;

    let user = client
        .user()
        .create(
            User::name().set("Brendan".to_string()),
            vec![User::email().set(Some("brendonovich@outlook.com".to_string()))],
        )
        .exec()
        .await?;

    let found = client
        .user()
        .find_unique(User::email().equals("brendonovich@outlook.com".to_string()))
        .exec()
        .await?
        .unwrap();
    assert_eq!(found.id, user.id);

    let found = client
        .user()
        .find_unique(User::email().equals("unknown".to_string()))
        .exec()
        .await?;
    assert!(found.is_none());

    cleanup(client).await
}
