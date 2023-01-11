use std::time::Duration;

use prisma_client_rust::QueryError;

use crate::db::*;
use crate::utils::*;

// Closure

#[tokio::test]
async fn ok() -> TestResult {
    let client = client().await;

    let (user, post) = client
        ._transaction()
        .run(|client| async move {
            let user = client
                .user()
                .create("brendan".to_string(), vec![])
                .exec()
                .await?;

            client
                .post()
                .create(
                    "test".to_string(),
                    true,
                    vec![post::author::connect(user::id::equals(user.id.clone()))],
                )
                .exec()
                .await
                .map(|post| (user, post))
        })
        .await?;

    assert_eq!(&user.name, "brendan");
    assert_eq!(&post.author_id.unwrap(), &user.id);

    cleanup(client).await
}

#[tokio::test]
async fn err() -> TestResult {
    let client = client().await;

    let result = client
        ._transaction()
        .run(|client| async move {
            let user = client
                .user()
                .create("brendan".to_string(), vec![])
                .exec()
                .await?;

            client
                .post()
                .create(
                    "test".to_string(),
                    true,
                    vec![post::author::connect(user::id::equals("".to_string()))],
                )
                .exec()
                .await
                .map(|post| (user, post))
        })
        .await;

    assert!(result.is_err());
    assert!(client.user().find_many(vec![]).exec().await?.is_empty());
    assert!(client.post().find_many(vec![]).exec().await?.is_empty());

    cleanup(client).await
}

#[tokio::test]
async fn custom_error() -> TestResult {
    let client = client().await;

    #[derive(Debug, thiserror::Error)]
    enum TxError {
        #[error("prisma error")]
        Prisma(#[from] QueryError),
        #[error("post not found")]
        PostNotFound,
    }

    let result = client
        ._transaction()
        .run(|client| async move {
            client
                .post()
                .create("test".to_string(), false, vec![])
                .exec()
                .await?;

            client
                .post()
                .find_unique(post::id::equals("".to_string()))
                .exec()
                .await?
                .ok_or(TxError::PostNotFound)
        })
        .await;

    assert!(result.is_err());
    assert!(client.post().find_many(vec![]).exec().await?.is_empty());

    cleanup(client).await
}

#[tokio::test]
async fn timeout() -> TestResult {
    let client = client().await;

    let result = client
        ._transaction()
        .with_timeout(2000)
        .run(|client| async move {
            for _ in 0..3 {
                tokio::time::sleep(Duration::from_secs(1)).await;
                client.user().find_many(vec![]).exec().await?;
            }

            client.user().find_many(vec![]).exec().await
        })
        .await;

    assert!(result.is_err());

    let result = client
        ._transaction()
        .with_timeout(8000)
        .run(|client| async move {
            for _ in 0..3 {
                tokio::time::sleep(Duration::from_secs(1)).await;
                client.user().find_many(vec![]).exec().await?;
            }

            client.user().find_many(vec![]).exec().await
        })
        .await;

    assert!(result.is_ok());

    cleanup(client).await
}

#[tokio::test]
async fn batch() -> TestResult {
    let client = client().await;

    let users = client
        ._transaction()
        .run(|client| async move {
            client
                ._batch(vec![
                    client.user().create("brendan".to_string(), vec![]),
                    client.user().create("brendan".to_string(), vec![]),
                ])
                .await
        })
        .await?;

    assert_eq!(&users[0].name, "brendan");

    cleanup(client).await
}

// Imperative

#[tokio::test]
async fn commit() -> TestResult {
    let client = client().await;

    // separate function for nice error handling
    async fn run(client: &PrismaClient) -> prisma_client_rust::Result<(user::Data, post::Data)> {
        let user = client
            .user()
            .create("brendan".to_string(), vec![])
            .exec()
            .await?;

        let post = client
            .post()
            .create(
                "test".to_string(),
                true,
                vec![post::author::connect(user::id::equals(user.id.clone()))],
            )
            .exec()
            .await?;

        Ok((user, post))
    }

    // block to avoid shadowing `client`
    let (user, post) = {
        let (tx, client) = client._transaction().begin().await?;

        match run(&client).await {
            Ok(v) => tx.commit(client).await.map(|_| v),
            Err(e) => {
                tx.rollback(client).await?;
                Err(e)
            }
        }?
    };

    assert_eq!(&user.name, "brendan");
    assert_eq!(&post.author_id.unwrap(), &user.id);

    cleanup(client).await
}

#[tokio::test]
async fn rollback() -> TestResult {
    let client = client().await;

    // separate function for nice error handling
    async fn run(client: &PrismaClient) -> prisma_client_rust::Result<(user::Data, post::Data)> {
        let user = client
            .user()
            .create("brendan".to_string(), vec![])
            .exec()
            .await?;

        let post = client
            .post()
            .create(
                "test".to_string(),
                true,
                vec![post::author::connect(user::id::equals("".to_string()))],
            )
            .exec()
            .await?;

        Ok((user, post))
    }

    // block to avoid shadowing `client`
    let result = {
        let (tx, client) = client._transaction().begin().await?;

        match run(&client).await {
            Ok(v) => tx.commit(client).await.map(|_| v),
            Err(e) => {
                tx.rollback(client).await?;
                Err(e)
            }
        }
    };

    assert!(result.is_err());
    assert!(client.user().find_many(vec![]).exec().await?.is_empty());
    assert!(client.post().find_many(vec![]).exec().await?.is_empty());

    cleanup(client).await
}
