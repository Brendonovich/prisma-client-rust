use prisma_client_rust::error_is_type;
use prisma_client_rust::prisma_errors;
use prisma_client_rust::Error;

use crate::db::*;
use crate::utils::*;

#[tokio::test]
async fn test_batch() -> TestResult {
    let client = client().await;

    let (brendan, oscar) = client
        ._batch((
            client.user().create("Brendan".to_string(), vec![]),
            client.user().create("Oscar".to_string(), vec![]),
        ))
        .await?;

    assert!(brendan.is_ok());
    assert_eq!(&brendan?.name, "Brendan");

    assert!(oscar.is_ok());
    assert_eq!(&oscar?.name, "Oscar");

    cleanup(client).await
}

#[tokio::test]
async fn test_batch_error() -> TestResult {
    let client = client().await;

    let res = client
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
        .await;

    let err = match res {
        Err(Error::Execute(e)) => e,
        _ => unreachable!(),
    };

    assert!(error_is_type::<
        prisma_errors::query_engine::UniqueKeyViolation,
    >(&err));

    cleanup(client).await
}

#[tokio::test]
async fn test_mixing_models() -> TestResult {
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

    assert!(user.is_ok());
    assert_eq!(&user?.name, "Brendan");
    assert!(profile.is_ok());
    assert_eq!(&profile?.bio, "Brendan's profile");

    assert_eq!(client.user().count().exec().await?, 1);
    assert_eq!(client.profile().count().exec().await?, 1);

    cleanup(client).await
}

#[tokio::test]
async fn test_mixing_actions() -> TestResult {
    let client = client().await;

    let (user, profile) = client
        ._batch((
            client.user().create("Brendan".to_string(), vec![]),
            client
                .user()
                .delete_many(vec![user::name::equals("Brendan".to_string())]),
        ))
        .await?;

    assert_eq!(client.user().count().exec().await?, 0);

    cleanup(client).await
}

#[tokio::test]
async fn test_large_query() -> TestResult {
    let client = client().await;

    client
        ._batch(
            vec![(); 1000]
                .into_iter()
                .map(|_| client.user().create("Brendan".to_string(), vec![])),
        )
        .await?;

    assert_eq!(client.user().count().exec().await?, 1000);

    cleanup(client).await
}

#[tokio::test]
async fn test_delete() -> TestResult {
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
async fn test_update() -> TestResult {
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
async fn test_upsert() -> TestResult {
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
    assert_eq!(client.user().count().exec().await?, 1);

    cleanup(client).await
}

#[tokio::test]
async fn test_update_many() -> TestResult {
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
