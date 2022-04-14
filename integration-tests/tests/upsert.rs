use crate::{db::User, utils::*};

#[tokio::test]
async fn upsert() -> TestResult {
    let client = client().await;

    let user_id = "sldfksldf".to_string();

    assert!(client
        .user()
        .find_unique(User::id().equals(user_id.to_string()))
        .exec()
        .await?
        .is_none());

    let user = client
        .user()
        .upsert(User::id().equals(user_id.to_string()))
        .create(
            User::name().set("Brendan".to_string()),
            vec![User::id().set(user_id.to_string())],
        )
        .update(vec![User::name().set("Brendan".to_string())])
        .exec()
        .await?;
    assert_eq!(user.id, user_id);
    assert_eq!(user.name, "Brendan");

    let user = client
        .user()
        .upsert(User::id().equals(user_id.to_string()))
        .create(
            User::name().set("Oscar".to_string()),
            vec![User::id().set(user_id.to_string())],
        )
        .update(vec![User::name().set("Oscar".to_string())])
        .exec()
        .await?;
    assert_eq!(user.id, user_id);
    assert_eq!(user.name, "Oscar");

    assert_eq!(client.user().find_many(vec![]).exec().await?.len(), 1);

    cleanup(client).await
}
