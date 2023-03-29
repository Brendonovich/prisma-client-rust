use crate::{db::*, utils::*};

user::partial_unchecked!(UserPartialType {
    name
    email
});

#[tokio::test]
async fn scalars() -> TestResult {
    let client = client().await;

    let user = client
        .user()
        .create("Brendan".to_string(), vec![])
        .exec()
        .await?;

    let updates = UserPartialType {
        name: None,
        email: Some(Some("brendonovich@outlook.com".to_string())),
    };

    let updated_user = client
        .user()
        .update_unchecked(user::id::equals(user.id), updates.to_params())
        .exec()
        .await?;

    assert_eq!(
        updated_user.email,
        Some("brendonovich@outlook.com".to_string())
    );

    cleanup(client).await
}

#[tokio::test]
async fn serde() -> TestResult {
    let client = client().await;

    let json = serde_json::json!({
        "email": "brendonovich@outlook.com",
    });

    let updates = UserPartialType {
        name: None,
        email: Some(Some("brendonovich@outlook.com".to_string())),
    };

    let deserialized: UserPartialType = serde_json::from_value(json).unwrap();

    assert_eq!(&deserialized.name, &updates.name);
    assert_eq!(&deserialized.email, &updates.email);

    let json = serde_json::json!({
        "name": "Brendan",
        "email": null
    });

    let updates = UserPartialType {
        name: Some("Brendan".to_string()),
        email: Some(None),
    };

    let deserialized: UserPartialType = serde_json::from_value(json).unwrap();

    assert_eq!(&deserialized.name, &updates.name);
    assert_eq!(&deserialized.email, &updates.email);

    let json = serde_json::json!({});

    let updates = UserPartialType {
        name: None,
        email: None,
    };

    let deserialized: UserPartialType = serde_json::from_value(json).unwrap();

    assert_eq!(&deserialized.name, &updates.name);
    assert_eq!(&deserialized.email, &updates.email);

    cleanup(client).await
}
