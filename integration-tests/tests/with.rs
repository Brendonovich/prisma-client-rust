use prisma_client_rust::{or, query::Error};

use crate::{db::*, utils::*};

async fn setup(client: &PrismaClient) -> Result<String, Error> {
    let user = client
        .user()
        .create(user::name::set("Brendan".to_string()), vec![])
        .exec()
        .await?;

    let posts = vec![
        client
            .post()
            .create(
                post::title::set("post 1".to_string()),
                post::published::set(false),
                vec![post::author::link(user::id::equals(user.id.clone()))],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                post::title::set("post 2".to_string()),
                post::published::set(true),
                vec![post::author::link(user::id::equals(user.id.clone()))],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                post::title::set("post 3".to_string()),
                post::published::set(true),
                vec![post::author::link(user::id::equals(user.id.clone()))],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                post::title::set("post 4".to_string()),
                post::published::set(false),
                vec![post::author::link(user::id::equals(user.id.clone()))],
            )
            .exec()
            .await?,
    ];

    client
        .category()
        .create(
            category::name::set("My category".to_string()),
            vec![category::posts::link(vec![
                post::id::equals(posts[0].id.clone()),
                post::id::equals(posts[1].id.clone()),
            ])],
        )
        .exec()
        .await?;

    Ok(user.id)
}

#[tokio::test]
async fn find_unique_with() -> TestResult {
    let client = client().await;

    let user_id = setup(&client).await?;

    let user = client
        .user()
        .find_unique(user::id::equals(user_id.clone()))
        .with(user::posts::fetch(vec![]))
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "Brendan");
    let posts = user.posts().unwrap();
    assert_eq!(posts.len(), 4);

    for (i, post) in posts.iter().enumerate().collect::<Vec<_>>() {
        assert!(post.author().is_err());
        assert_eq!(post.author_id, Some(user.id.clone()));
        assert_eq!(post.title, format!("post {}", i + 1));
    }

    cleanup(client).await
}

#[tokio::test]
async fn find_unique_with_optional() -> TestResult {
    let client = client().await;

    let user_id = setup(&client).await?;

    let user = client
        .user()
        .find_unique(user::id::equals(user_id.clone()))
        .with(user::profile::fetch())
        .exec()
        .await?
        .unwrap();

    let profile = user.profile();
    
    dbg!(&user);
        
    assert!(profile.is_ok());
    assert!(profile.unwrap().is_none());

    client
        .profile()
        .create(
            profile::user::link(user::id::equals(user.id.clone())),
            profile::bio::set("Bio".to_string()),
            profile::country::set("Country".to_string()),
            vec![],
        )
        .exec()
        .await?;

    let user = client
        .user()
        .find_unique(user::id::equals(user_id.clone()))
        .with(user::profile::fetch())
        .exec()
        .await?
        .unwrap();

    let profile = user.profile();

    assert!(profile.is_ok());
    assert!(profile.unwrap().is_some());

    cleanup(client).await
}

#[tokio::test]
async fn find_unique_with_take() -> TestResult {
    let client = client().await;

    let user_id = setup(&client).await?;

    let user = client
        .user()
        .find_unique(user::id::equals(user_id.clone()))
        .with(user::posts::fetch(vec![]).take(1))
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.posts().unwrap().len(), 1);

    cleanup(client).await
}

#[tokio::test]
async fn find_unique_with_where() -> TestResult {
    let client = client().await;

    let user_id = setup(&client).await?;

    let posts = client.post().find_many(vec![]).exec().await?;

    let user = client
        .user()
        .find_unique(user::id::equals(user_id.clone()))
        .with(user::posts::fetch(vec![post::created_at::equals(
            posts[0].created_at,
        )]))
        .exec()
        .await?
        .unwrap();
    let user_posts = user.posts().unwrap();

    assert_eq!(user_posts.len(), 1);
    assert_eq!(user_posts[0].id, posts[0].id);

    cleanup(client).await
}

#[tokio::test]
async fn find_unique_with_pagination() -> TestResult {
    let client = client().await;

    let user_id = setup(&client).await?;

    let posts = client.post().find_many(vec![]).exec().await?;

    let user = client
        .user()
        .find_unique(user::id::equals(user_id.clone()))
        .with(
            user::posts::fetch(vec![])
                .cursor(post::id::cursor(posts[0].id.clone()))
                .take(1)
                .skip(1),
        )
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.posts().unwrap().len(), 1);
    assert_eq!(user.posts().unwrap()[0].id, posts[1].id);

    let user = client
        .user()
        .find_unique(user::id::equals(user_id.clone()))
        .with(
            user::posts::fetch(vec![])
                .cursor(post::id::cursor(posts[1].id.clone()))
                .take(-1)
                .skip(1),
        )
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.posts().unwrap().len(), 1);
    assert_eq!(user.posts().unwrap()[0].id, posts[0].id);

    cleanup(client).await
}

#[tokio::test]
async fn find_unique_with_nested_where_or() -> TestResult {
    let client = client().await;

    let user_id = setup(&client).await?;

    let posts = client.post().find_many(vec![]).exec().await?;

    let user = client
        .user()
        .find_unique(user::id::equals(user_id.clone()))
        .with(user::posts::fetch(vec![or![
            post::published::equals(true),
            post::id::equals(posts[0].id.clone()),
        ]]))
        .exec()
        .await?
        .unwrap();
    assert_eq!(posts[0].published, false);

    let user_posts = user.posts().unwrap();
    assert_eq!(user_posts.len(), 3);

    assert_eq!(user_posts[0].id, posts[0].id);
    assert_eq!(user_posts[1].id, posts[1].id);
    assert_eq!(user_posts[2].id, posts[2].id);

    assert!(!user_posts[0].published);
    assert!(user_posts[1].published);
    assert!(user_posts[2].published);

    cleanup(client).await
}

#[tokio::test]
async fn find_unique_with_nested_with() -> TestResult {
    let client = client().await;

    let user_id = setup(&client).await?;

    let user = client
        .user()
        .find_unique(user::id::equals(user_id.clone()))
        .with(
            user::posts::fetch(vec![])
                .with(post::categories::fetch(vec![]).with(category::posts::fetch(vec![]))),
        )
        .exec()
        .await?
        .unwrap();
    assert!(user.profile().is_err());

    for post in user.posts().unwrap() {
        for category in post.categories().unwrap() {
            assert!(category.posts().is_ok())
        }
    }

    cleanup(client).await
}

// TODO: Nested create

// #[tokio::test]
// async fn create_with() -> TestResult {
//     let client = client().await;

//     let post = client
//         .post()
//         .create(
//             post::title().set("post 4".to_string()),
//             post::published().set(false),
//             vec![post::author().create(user::name().set("Brendan".to_string()))],
//         )
//         .with(post::author().fetch())
//         .exec()
//         .await?;
//     assert_eq!(post.author().unwrap(), Some("Brendan".to_string()));

//     cleanup(client).await
// }
