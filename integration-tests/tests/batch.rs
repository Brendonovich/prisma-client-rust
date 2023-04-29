#![allow(unused_must_use)]
use prisma_client_rust::prisma_errors::query_engine::UniqueKeyViolation;

use crate::db::*;
use crate::utils::*;

#[tokio::test]
async fn tuple() -> TestResult {
    let client = client().await;

    let (brendan, oscar) = client
        ._batch((
            client.user().create("Brendan".to_string(), vec![]),
            client.user().create("Oscar".to_string(), vec![]),
        ))
        .await?;

    assert_eq!(&brendan.name, "Brendan");
    assert_eq!(&oscar.name, "Oscar");

    cleanup(client).await
}

#[tokio::test]
async fn vec() -> TestResult {
    let client = client().await;

    let users = client
        ._batch(vec![
            client.user().create("Brendan".to_string(), vec![]),
            client.user().create("Oscar".to_string(), vec![]),
        ])
        .await?;

    assert_eq!(&users[0].name, "Brendan");
    assert_eq!(&users[1].name, "Oscar");

    cleanup(client).await
}

#[tokio::test]
async fn tuple_in_vec() -> TestResult {
    let client = client().await;

    let users = client
        ._batch(vec![
            (
                client.user().create("Brendan".to_string(), vec![]),
                client.user().create("Brendan".to_string(), vec![]),
            ),
            (
                client.user().create("Oscar".to_string(), vec![]),
                client.user().create("Oscar".to_string(), vec![]),
            ),
        ])
        .await?;

    assert_eq!(users.len(), 2);
    assert_eq!(&users[0].1.name, "Brendan");
    assert_eq!(&users[1].1.name, "Oscar");

    cleanup(client).await
}

#[tokio::test]
async fn vec_in_tuple() -> TestResult {
    let client = client().await;

    let (brendan, oscar) = client
        ._batch((
            vec![
                client.user().create("Brendan".to_string(), vec![]),
                client.user().create("Brendan".to_string(), vec![]),
            ],
            vec![
                client.user().create("Oscar".to_string(), vec![]),
                client.user().create("Oscar".to_string(), vec![]),
            ],
        ))
        .await?;

    assert_eq!(&brendan[1].name, "Brendan");
    assert_eq!(&oscar[1].name, "Oscar");

    cleanup(client).await
}

#[tokio::test]
async fn error() -> TestResult {
    let client = client().await;

    let error = client
        ._batch((
            client.user().create(
                "Brendan".to_string(),
                vec![user::id::set("abc".to_string())],
            ),
            client.user().create(
                "Brendan 2".to_string(),
                vec![user::id::set("abc".to_string())],
            ),
        ))
        .await
        .unwrap_err();

    assert!(error.is_prisma_error::<UniqueKeyViolation>());

    cleanup(client).await
}

#[tokio::test]
async fn mixing_models() -> TestResult {
    let client = client().await;

    let (user, profile) = client
        ._batch((
            client.user().create(
                "Brendan".to_string(),
                vec![user::id::set("abc".to_string())],
            ),
            client.profile().create(
                user::id::equals("abc".to_string()),
                "Brendan's profile".to_string(),
                "Australia".to_string(),
                vec![],
            ),
        ))
        .await?;

    assert_eq!(&user.name, "Brendan");
    assert_eq!(&profile.bio, "Brendan's profile");

    assert_eq!(client.user().count(vec![]).exec().await?, 1);
    assert_eq!(client.profile().count(vec![]).exec().await?, 1);

    cleanup(client).await
}

#[tokio::test]
async fn mixing_actions() -> TestResult {
    let client = client().await;

    client
        ._batch((
            client.user().create("Brendan".to_string(), vec![]),
            client
                .user()
                .delete_many(vec![user::name::equals("Brendan".to_string())]),
        ))
        .await?;

    assert_eq!(client.user().count(vec![]).exec().await?, 0);

    cleanup(client).await
}

#[tokio::test]
async fn large_query() -> TestResult {
    let client = client().await;

    client
        ._batch(
            vec![(); 1000]
                .into_iter()
                .map(|_| client.user().create("Brendan".to_string(), vec![])),
        )
        .await?;

    assert_eq!(client.user().count(vec![]).exec().await?, 1000);

    cleanup(client).await
}

#[tokio::test]
async fn delete() -> TestResult {
    let client = client().await;

    let user = client
        .user()
        .create("Brendan".to_string(), vec![])
        .exec()
        .await?;

    assert!(client
        .user()
        .find_first(vec![user::id::equals(user.id.clone())])
        .exec()
        .await?
        .is_some());

    client
        ._batch(client.user().delete(user::id::equals(user.id.clone())))
        .await?;

    assert!(client
        .user()
        .find_first(vec![user::id::equals(user.id.clone())])
        .exec()
        .await?
        .is_none());

    cleanup(client).await
}

#[tokio::test]
async fn update() -> TestResult {
    let client = client().await;

    let user = client
        .user()
        .create("Brendan".to_string(), vec![])
        .exec()
        .await?;

    assert!(client
        .user()
        .find_first(vec![user::id::equals(user.id.clone())])
        .exec()
        .await?
        .is_some());

    client
        ._batch(client.user().update(
            user::id::equals(user.id.clone()),
            vec![user::name::set("Oscar".to_string())],
        ))
        .await?;

    let new = client
        .user()
        .find_unique(user::id::equals(user.id.clone()))
        .exec()
        .await?
        .unwrap();

    assert_eq!(&new.id, &user.id);
    assert_eq!(&new.name, "Oscar");

    cleanup(client).await
}

#[tokio::test]
async fn upsert() -> TestResult {
    let client = client().await;

    let user_id = "abc123";

    assert!(client
        .user()
        .find_unique(user::id::equals(user_id.to_string()))
        .exec()
        .await?
        .is_none());

    client
        ._batch(client.user().upsert(
            user::id::equals(user_id.to_string()),
            user::create(
                "Brendan".to_string(),
                vec![user::id::set(user_id.to_string())],
            ),
            vec![user::name::set("Oscar".to_string())],
        ))
        .await?;

    let user = client
        .user()
        .find_unique(user::id::equals(user_id.to_string()))
        .exec()
        .await?
        .unwrap();

    assert_eq!(&user.id, user_id);
    assert_eq!(&user.name, "Brendan");

    client
        ._batch(client.user().upsert(
            user::id::equals(user_id.to_string()),
            user::create(
                "Brendan".to_string(),
                vec![user::id::set(user_id.to_string())],
            ),
            vec![user::name::set("Oscar".to_string())],
        ))
        .await?;

    let user = client
        .user()
        .find_unique(user::id::equals(user_id.to_string()))
        .exec()
        .await?
        .unwrap();

    assert_eq!(&user.id, user_id);
    assert_eq!(&user.name, "Oscar");
    assert_eq!(client.user().count(vec![]).exec().await?, 1);

    cleanup(client).await
}

#[tokio::test]
async fn update_many() -> TestResult {
    let client = client().await;

    client
        .user()
        .create("Brendan".to_string(), vec![])
        .exec()
        .await?;
    client
        .user()
        .create("Brendan 2".to_string(), vec![])
        .exec()
        .await?;

    client
        ._batch(client.user().update_many(
            vec![user::name::starts_with("Brendan".to_string())],
            vec![user::name::set("Brendan".to_string())],
        ))
        .await?;

    let users = client.user().find_many(vec![]).exec().await?;

    assert_eq!(users.len(), 2);
    assert_eq!(&users[0].name, "Brendan");
    assert_eq!(&users[1].name, "Brendan");

    cleanup(client).await
}

#[tokio::test]
async fn async_trait() -> TestResult {
    #[async_trait::async_trait]
    trait TestTrait {
        async fn test(&self, client: &PrismaClient) -> prisma_client_rust::Result<()>;
    }

    struct Test;

    #[async_trait::async_trait]
    impl TestTrait for Test {
        async fn test(&self, client: &PrismaClient) -> prisma_client_rust::Result<()> {
            let (brendan, oscar) = client
                ._batch((
                    vec![client.user().create("Brendan".to_string(), vec![])],
                    vec![client.user().create("Oscar".to_string(), vec![])],
                ))
                .await?;

            assert_eq!(&brendan[0].name, "Brendan");
            assert_eq!(&oscar[0].name, "Oscar");

            Ok(())
        }
    }

    let client = client().await;

    Test.test(&client).await?;

    cleanup(client).await
}

#[tokio::test]
async fn allows_empty() -> TestResult {
    let client = client().await;

    client._batch(vec![] as Vec<user::CreateQuery>).await?;
    client
        ._batch(vec![vec![]] as Vec<Vec<user::CreateQuery>>)
        .await?;

    cleanup(client).await
}
