use crate::{db::*, utils::*};

user::partial!(UserPartialType {
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
        .update(user::id::equals(user.id), updates.to_params())
        .exec()
        .await?;

    assert_eq!(
        updated_user.email,
        Some("brendonovich@outlook.com".to_string())
    );

    cleanup(client).await
}
