use prisma_client_rust::{
    bigdecimal::BigDecimal, prisma_errors::query_engine::RecordRequiredButNotFound,
    queries::QueryError,
};

use crate::{db::*, utils::*};
use std::str::FromStr;

async fn create_user(client: &PrismaClient) -> Result<String, QueryError> {
    client
        .user()
        .create("Brendan".to_string(), vec![])
        .exec()
        .await
        .map(|user| user.id)
}

#[tokio::test]
async fn query() -> TestResult {
    let client = client().await;

    let user_id = create_user(&client).await?;

    let post = client
        .post()
        .create(
            "Hi from Create!".to_string(),
            true,
            vec![
                post::desc::set(Some(
                    "Prisma is a database toolkit that makes databases easy.".to_string(),
                )),
                post::author::connect(user::id::equals(user_id.clone())),
            ],
        )
        .exec()
        .await?;
    assert!(post.author().is_err());
    assert_eq!(post.title, "Hi from Create!");

    let updated = client
        .post()
        .update(
            post::id::equals(post.id.clone()),
            vec![
                post::title::set("Hi from Update!".to_string()),
                post::published::set(false),
            ],
        )
        .exec()
        .await?;
    assert_eq!(updated.title, "Hi from Update!");
    assert_ne!(updated.updated_at, post.updated_at);
    assert_eq!(updated.created_at, post.created_at);

    let updated = client
        .post()
        .update(
            post::id::equals(post.id.clone()),
            vec![
                post::published::set(false),
                post::desc::set(Some("Updated desc.".to_string())),
            ],
        )
        .with(post::author::fetch())
        .exec()
        .await?;
    assert!(!updated.published);
    assert_eq!(updated.desc, Some("Updated desc.".to_string()));
    assert_eq!(updated.author.unwrap().unwrap().name, "Brendan");

    cleanup(client).await
}

// TODO: update with nested create & delete/disconnect

#[tokio::test]
async fn disconnect() -> TestResult {
    let client = client().await;

    let user_id = create_user(&client).await?;

    let user = client
        .user()
        .find_unique(user::id::equals(user_id.clone()))
        .with(user::posts::fetch(vec![]))
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.posts.unwrap().len(), 0);

    let post = client
        .post()
        .create("My post".to_string(), true, vec![])
        .exec()
        .await?;

    let post_2 = client
        .post()
        .create("Another post".to_string(), true, vec![])
        .exec()
        .await?;

    let updated = client
        .user()
        .update(
            user::id::equals(user_id.clone()),
            vec![user::posts::connect(vec![post::id::equals(
                post.id.clone(),
            )])],
        )
        .with(user::posts::fetch(vec![]))
        .exec()
        .await?;
    assert_eq!(updated.posts.unwrap().len(), 1);

    let updated = client
        .user()
        .update(
            user::id::equals(user_id.clone()),
            vec![
                user::posts::disconnect(vec![post::id::equals(post.id.clone())]),
                user::posts::connect(vec![post::id::equals(post_2.id.clone())]),
            ],
        )
        .with(user::posts::fetch(vec![]))
        .exec()
        .await?;
    assert_eq!(updated.posts().unwrap().len(), 1);
    assert_eq!(updated.posts().unwrap()[0].id, post_2.id);

    cleanup(client).await
}

#[tokio::test]
async fn unchecked() -> TestResult {
    let client = client().await;

    let user_id = create_user(&client).await?;

    let user = client
        .user()
        .find_unique(user::id::equals(user_id.clone()))
        .with(user::posts::fetch(vec![]))
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.posts.unwrap().len(), 0);

    let post = client
        .post()
        .create("My post".to_string(), true, vec![])
        .exec()
        .await?;

    let updated = client
        .post()
        .update_unchecked(
            post::id::equals(post.id.clone()),
            vec![post::author_id::set(Some(user.id.clone()))],
        )
        .with(post::author::fetch())
        .exec()
        .await?;
    assert!(updated.author().unwrap().is_some());

    cleanup(client).await
}

#[tokio::test]
async fn atomic() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create("My post".to_string(), false, vec![])
        .exec()
        .await?;
    assert_eq!(post.title, "My post");
    assert_eq!(post.views, 0);

    let updated = client
        .post()
        .update(
            post::id::equals(post.id.clone()),
            vec![post::views::increment(1)],
        )
        .exec()
        .await?;
    assert_eq!(updated.views, 1);

    cleanup(client).await
}

#[tokio::test]
async fn record_not_found() -> TestResult {
    let client = client().await;

    let error = client
        .post()
        .update(
            post::id::equals("wow".to_string()),
            vec![post::title::set("My post".to_string())],
        )
        .exec()
        .await
        .unwrap_err();

    assert!(error.is_prisma_error::<RecordRequiredButNotFound>());

    cleanup(client).await
}

#[tokio::test]
async fn set_none() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create(
            "post".to_string(),
            false,
            vec![post::desc::set(Some("My description".to_string()))],
        )
        .exec()
        .await?;
    assert_eq!(post.desc, Some("My description".to_string()));

    let updated = client
        .post()
        .update(
            post::id::equals(post.id.clone()),
            vec![post::desc::set(None)],
        )
        .exec()
        .await?;
    assert_eq!(updated.id, post.id);
    assert!(updated.desc.is_none());

    cleanup(client).await
}

#[tokio::test]
async fn id_field() -> TestResult {
    let client = client().await;

    let user = client
        .user()
        .create("Brendan".to_string(), vec![])
        .exec()
        .await?;

    let updated = client
        .user()
        .update(
            user::id::equals(user.id.clone()),
            vec![user::id::set("new_id".to_string())],
        )
        .exec()
        .await?;
    assert_eq!(updated.id, "new_id");

    cleanup(client).await
}

#[tokio::test]
async fn id_field_atomic() -> TestResult {
    let client = client().await;

    let record = client.types().create(vec![]).exec().await?;
    let updated = client
        .types()
        .update(
            types::id_string(record.id, "".to_string()),
            vec![types::id::increment(500)],
        )
        .exec()
        .await?;
    assert_eq!(updated.id, record.id + 500);

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
    let email = user.email.unwrap();

    let updated = client
        .user()
        .update(
            user::email::equals(email),
            vec![user::email::set(Some("foo@gmail.com".to_string()))],
        )
        .exec()
        .await?;
    assert_eq!(updated.id, user.id);
    assert_eq!(updated.email, Some("foo@gmail.com".to_string()));

    cleanup(client).await
}

#[tokio::test]
async fn many() -> TestResult {
    let client = client().await;

    let posts = vec![
        client
            .post()
            .create("Test post 1".to_string(), false, vec![])
            .exec()
            .await?,
        client
            .post()
            .create("Test post 2".to_string(), false, vec![])
            .exec()
            .await?,
    ];

    let count = client
        .post()
        .update_many(
            vec![post::published::equals(false)],
            vec![post::published::set(true)],
        )
        .exec()
        .await?;
    assert_eq!(count, 2);

    let post = client
        .post()
        .find_unique(post::id::equals(posts[0].id.clone()))
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.published, true);

    let count = client
        .post()
        .update_many(
            vec![post::published::equals(false)],
            vec![post::published::set(true)],
        )
        .exec()
        .await?;
    assert_eq!(count, 0);

    let count = client
        .post()
        .update_many(
            vec![post::id::equals(posts[0].id.clone())],
            vec![post::published::set(false)],
        )
        .exec()
        .await?;
    assert_eq!(count, 1);

    let post = client
        .post()
        .find_unique(post::id::equals(posts[0].id.clone()))
        .exec()
        .await?
        .unwrap();
    assert!(!post.published);

    let count = client
        .post()
        .update_many(
            vec![post::published::equals(false)],
            vec![post::views::set(10)],
        )
        .exec()
        .await?;
    assert_eq!(count, 1);

    let post = client
        .post()
        .find_unique(post::id::equals(posts[0].id.clone()))
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.views, 10);

    let count = client
        .post()
        .update_many(
            vec![post::id::equals(posts[0].id.clone())],
            vec![post::id::set("sdlfkjs".to_string())],
        )
        .exec()
        .await?;
    assert_eq!(count, 1);

    let post = client
        .post()
        .find_unique(post::id::equals("sdlfkjs".to_string()))
        .exec()
        .await?;
    assert!(post.is_some());

    let post = client
        .post()
        .find_unique(post::id::equals(posts[0].id.clone()))
        .exec()
        .await?;
    assert!(post.is_none());

    // THIS SHOULDN'T BE ALLOWED
    // let posts = client
    //     .post()
    //     .update_many(vec![], vec![post::author().connect(user::id().equals(user_id.clone()))])
    //     .exec()
    //     .await?;
    // assert_eq!(posts, 5);

    cleanup(client).await
}

#[tokio::test]
async fn set_many_none() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create(
            "Foo".to_string(),
            true,
            vec![post::desc::set(Some("Description".to_string()))],
        )
        .exec()
        .await?;
    assert_eq!(post.desc, Some("Description".to_string()));

    let count = client
        .post()
        .update_many(vec![], vec![post::desc::set(None)])
        .exec()
        .await?;
    assert_eq!(count, 1);

    let found = client
        .post()
        .find_unique(post::id::equals(post.id.clone()))
        .exec()
        .await?
        .unwrap();
    assert_eq!(found.id, post.id);
    assert_eq!(found.title, "Foo");
    assert_eq!(found.desc, None);

    cleanup(client).await
}
