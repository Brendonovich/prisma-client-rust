use crate::{db::*, utils::*};

#[tokio::test]
async fn query() -> TestResult {
    let client = client().await;

    let user_id = "sldfksldf".to_string();

    assert!(client
        .user()
        .find_unique(user::id::equals(user_id.to_string()))
        .exec()
        .await?
        .is_none());

    let user = client
        .user()
        .upsert(
            user::id::equals(user_id.to_string()),
            user::create(
                "Brendan".to_string(),
                vec![user::id::set(user_id.to_string())],
            ),
            vec![user::name::set("Brendan".to_string())],
        )
        .exec()
        .await?;

    assert_eq!(user.id, user_id);
    assert_eq!(user.name, "Brendan");

    let user = client
        .user()
        .upsert(
            user::id::equals(user_id.to_string()),
            user::create(
                "Oscar".to_string(),
                vec![user::id::set(user_id.to_string())],
            ),
            vec![user::name::set("Oscar".to_string())],
        )
        .exec()
        .await?;

    assert_eq!(user.id, user_id);
    assert_eq!(user.name, "Oscar");

    assert_eq!(client.user().find_many(vec![]).exec().await?.len(), 1);

    cleanup(client).await
}
