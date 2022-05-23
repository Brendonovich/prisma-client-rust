use prisma_client_rust::queries::Error;

use crate::{db::*, utils::*};

async fn create_user(client: &PrismaClient) -> Result<String, Error> {
    client
        .user()
        .create(user::name::set("Brendan".to_string()), vec![])
        .exec()
        .await
        .map(|user| user.id)
}

#[tokio::test]
async fn update() -> TestResult {
    let client = client().await;

    let user_id = create_user(&client).await?;

    let post = client
        .post()
        .create(
            post::title::set("Hi from Create!".to_string()),
            post::published::set(true),
            vec![
                post::desc::set(Some(
                    "Prisma is a database toolkit that makes databases easy.".to_string(),
                )),
                post::author_id::set(Some(user_id.clone())),
            ],
        )
        .exec()
        .await?;
    assert!(post.author().is_err());
    assert_eq!(post.title, "Hi from Create!");

    let updated = client
        .post()
        .find_unique(post::id::equals(post.id.clone()))
        .update(vec![
            post::title::set("Hi from Update!".to_string()),
            post::published::set(false),
        ])
        .exec()
        .await?
        .unwrap();
    assert_eq!(updated.title, "Hi from Update!");
    assert_ne!(updated.updated_at, post.updated_at);
    assert_eq!(updated.created_at, post.created_at);

    let updated = client
        .post()
        .find_unique(post::id::equals(post.id.clone()))
        .update(vec![
            post::published::set(false),
            post::desc::set(Some("Updated desc.".to_string())),
        ])
        .with(post::author::fetch())
        .exec()
        .await?
        .unwrap();
    assert!(!updated.published);
    assert_eq!(updated.desc, Some("Updated desc.".to_string()));
    assert_eq!(updated.author.unwrap().unwrap().name, "Brendan");

    cleanup(client).await
}

// TODO: update with nested create & delete/unlink

#[tokio::test]
async fn update_and_unlink() -> TestResult {
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
        .create(
            post::title::set("My post".to_string()),
            post::published::set(true),
            vec![],
        )
        .exec()
        .await?;

    let updated = client
        .user()
        .find_unique(user::id::equals(user_id.clone()))
        .update(vec![user::posts::link(vec![post::id::equals(
            post.id.clone(),
        )])])
        .with(user::posts::fetch(vec![]))
        .exec()
        .await?
        .unwrap();
    assert_eq!(updated.posts.unwrap().len(), 1);

    let updated = client
        .user()
        .find_unique(user::id::equals(user_id.clone()))
        .update(vec![user::posts::unlink(vec![post::id::equals(
            post.id.clone(),
        )])])
        .with(user::posts::fetch(vec![]))
        .exec()
        .await?
        .unwrap();
    assert_eq!(updated.posts.unwrap().len(), 0);

    cleanup(client).await
}

#[tokio::test]
async fn atomic_update() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create(
            post::title::set("My post".to_string()),
            post::published::set(false),
            vec![],
        )
        .exec()
        .await?;
    assert_eq!(post.title, "My post");
    assert_eq!(post.views, 0);

    let updated = client
        .post()
        .find_unique(post::id::equals(post.id.clone()))
        .update(vec![post::views::increment(1)])
        .exec()
        .await?
        .unwrap();
    assert_eq!(updated.views, 1);

    cleanup(client).await
}

#[tokio::test]
async fn update_record_not_found() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .find_unique(post::id::equals("wow".to_string()))
        .update(vec![post::title::set("My post".to_string())])
        .exec()
        .await?;
    assert!(post.is_none());

    cleanup(client).await
}

#[tokio::test]
async fn setting_field_to_null() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create(
            post::title::set("post".to_string()),
            post::published::set(false),
            vec![post::desc::set(Some("My description".to_string()))],
        )
        .exec()
        .await?;
    assert_eq!(post.desc, Some("My description".to_string()));

    let updated = client
        .post()
        .find_unique(post::id::equals(post.id.clone()))
        .update(vec![post::desc::set(None)])
        .exec()
        .await?
        .unwrap();
    assert_eq!(updated.id, post.id);
    assert!(updated.desc.is_none());

    cleanup(client).await
}

#[tokio::test]
async fn update_id_field() -> TestResult {
    let client = client().await;

    let user = client
        .user()
        .create(user::name::set("Brendan".to_string()), vec![])
        .exec()
        .await?;

    let updated = client
        .user()
        .find_unique(user::id::equals(user.id.clone()))
        .update(vec![user::id::set("new_id".to_string())])
        .exec()
        .await?
        .unwrap();
    assert_eq!(updated.id, "new_id");

    cleanup(client).await
}

#[tokio::test]
async fn update_id_field_atomic() -> TestResult {
    let client = client().await;

    let record = client.types().create(vec![]).exec().await?;
    let updated = client
        .types()
        .find_unique(types::id::equals(record.id))
        .update(vec![types::id::increment(500)])
        .exec()
        .await?
        .unwrap();
    assert_eq!(updated.id, record.id + 500);

    cleanup(client).await
}

#[tokio::test]
async fn update_unique_field() -> TestResult {
    let client = client().await;

    let user = client
        .user()
        .create(
            user::name::set("Brendan".to_string()),
            vec![user::email::set(Some(
                "brendonovich@outlook.com".to_string(),
            ))],
        )
        .exec()
        .await?;
    let email = user.email.unwrap();

    let updated = client
        .user()
        .find_unique(user::email::equals(email))
        .update(vec![user::email::set(Some("foo@gmail.com".to_string()))])
        .exec()
        .await?
        .unwrap();
    assert_eq!(updated.id, user.id);
    assert_eq!(updated.email, Some("foo@gmail.com".to_string()));

    cleanup(client).await
}

#[tokio::test]
async fn update_many() -> TestResult {
    let client = client().await;

    let posts = vec![
        client
            .post()
            .create(
                post::title::set("Test post 1".to_string()),
                post::published::set(false),
                vec![],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                post::title::set("Test post 2".to_string()),
                post::published::set(false),
                vec![],
            )
            .exec()
            .await?,
    ];

    let count = client
        .post()
        .find_many(vec![post::published::equals(false)])
        .update(vec![post::published::set(true)])
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
        .find_many(vec![post::published::equals(false)])
        .update(vec![post::published::set(true)])
        .exec()
        .await?;
    assert_eq!(count, 0);

    let count = client
        .post()
        .find_many(vec![post::id::equals(posts[0].id.clone())])
        .update(vec![post::published::set(false)])
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
        .find_many(vec![post::published::equals(false)])
        .update(vec![post::views::set(10)])
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
        .find_many(vec![post::id::equals(posts[0].id.clone())])
        .update(vec![post::id::set("sdlfkjs".to_string())])
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
    //     .find_many(vec![])
    //     .update(vec![post::author().link(user::id().equals(user_id.clone()))])
    //     .exec()
    //     .await?;
    // assert_eq!(posts, 5);

    cleanup(client).await
}

#[tokio::test]
async fn setting_many_to_null() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create(
            post::title::set("Foo".to_string()),
            post::published::set(true),
            vec![post::desc::set(Some("Description".to_string()))],
        )
        .exec()
        .await?;
    assert_eq!(post.desc, Some("Description".to_string()));

    let count = client
        .post()
        .find_many(vec![])
        .update(vec![post::desc::set(None)])
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
