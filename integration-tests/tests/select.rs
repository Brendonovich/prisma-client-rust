use crate::{db::*, utils::*};

#[tokio::test]
async fn scalars() -> TestResult {
    let client = client().await;

    client
        .user()
        .create("Brendan".to_string(), vec![])
        .select(user::select! {
            id
            name
            email
            underscored_
        })
        .exec()
        .await?;

    cleanup(client).await
}

#[tokio::test]
async fn relations() -> TestResult {
    let client = client().await;

    client
        .user()
        .create("Brendan".to_string(), vec![])
        .select(user::select! {
            profile
            posts
            favourite_posts
        })
        .exec()
        .await?;

    cleanup(client).await
}

#[tokio::test]
async fn relations_nested() -> TestResult {
    let client = client().await;

    client
        .user()
        .create("Brendan".to_string(), vec![])
        .select(user::select! {
            profile {
                user_id
                bio
                city
            }
            posts {
                id
                author
                desc
            }
        })
        .exec()
        .await?;

    cleanup(client).await
}

#[tokio::test]
async fn many_relation_args() -> TestResult {
    let client = client().await;

    client
        .user()
        .create("Brendan".to_string(), vec![])
        .select(user::select! {
            posts(vec![]).skip(3) {
                id
                author
                desc
            }
            favourite_posts(vec![]).take(2)
        })
        .exec()
        .await?;

    cleanup(client).await
}
