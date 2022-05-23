use prisma_client_rust::{and, not, or, Direction};

use crate::{
    db::{post, profile, user},
    utils::*,
};

#[tokio::test]
async fn find_first() -> TestResult {
    let client = client().await;

    let posts = vec![
        client
            .post()
            .create(
                post::title::set("Test post 1".to_string()),
                post::published::set(false),
                vec![post::views::set(100)],
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
        client
            .post()
            .create(
                post::title::set("Test post 3".to_string()),
                post::published::set(false),
                vec![],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                post::title::set("Test post 4".to_string()),
                post::published::set(true),
                vec![post::views::set(500)],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                post::title::set("Test post 5".to_string()),
                post::published::set(false),
                vec![],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                post::title::set("Test post 6".to_string()),
                post::published::set(true),
                vec![],
            )
            .exec()
            .await?,
    ];

    let post = client
        .post()
        .find_first(vec![post::published::equals(true)])
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.id, posts[3].id);
    assert_eq!(post.title, "Test post 4");
    assert!(post.published);

    let post = client
        .post()
        .find_first(vec![post::title::contains("not found".to_string())])
        .exec()
        .await?;
    assert!(post.is_none());

    let post = client
        .post()
        .find_first(vec![post::published::equals(true)])
        .skip(1)
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.id, posts[5].id);
    assert_eq!(post.title, "Test post 6");
    assert!(post.published);

    let post = client
        .post()
        .find_first(vec![not!(post::published::equals(true))])
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.title, "Test post 1");

    let post = client
        .post()
        .find_first(vec![
            post::title::contains("Test".to_string()),
            and!(post::published::equals(true)),
        ])
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.title, "Test post 4");

    let post = client
        .post()
        .find_first(vec![and! {
            post::published::equals(true),
            post::title::contains("Test".to_string()),
        }])
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.title, "Test post 4");

    let post = client
        .post()
        .find_first(vec![or! {
            post::views::gt(100),
            post::published::equals(false)
        }])
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.title, "Test post 1");

    let post = client
        .post()
        .find_first(vec![or! {
            post::views::gt(100),
        }])
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.title, "Test post 4");

    cleanup(client).await
}

#[tokio::test]
async fn filtering_one_to_one_relation() -> TestResult {
    let client = client().await;

    client
        .profile()
        .create(
            profile::user::link(user::id::equals(
                client
                    .user()
                    .create(user::name::set("Brendan".to_string()), vec![])
                    .exec()
                    .await?
                    .id,
            )),
            profile::bio::set("My very cool bio.".to_string()),
            profile::country::set("Australia".to_string()),
            vec![],
        )
        .exec()
        .await?;

    client
        .profile()
        .create(
            profile::user::link(user::id::equals(
                client
                    .user()
                    .create(user::name::set("Oscar".to_string()), vec![])
                    .exec()
                    .await?
                    .id,
            )),
            profile::bio::set("Hello world, this is my bio.".to_string()),
            profile::country::set("Australia".to_string()),
            vec![],
        )
        .exec()
        .await?;

    client
        .user()
        .create(user::name::set("Jamie".to_string()), vec![])
        .exec()
        .await?;

    let user = client
        .user()
        .find_first(vec![user::profile::is(vec![profile::bio::contains(
            "cool".to_string(),
        )])])
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "Brendan");
    assert!(user.profile().is_err());

    let user = client
        .user()
        .find_first(vec![user::profile::is_not(vec![profile::bio::contains(
            "bio".to_string(),
        )])])
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "Jamie");
    assert!(user.profile().is_err());

    cleanup(client).await
}

#[tokio::test]
async fn filtering_and_ordering_one_to_many_relation() -> TestResult {
    let client = client().await;

    let user = client
        .user()
        .create(user::name::set("Brendan".to_string()), vec![])
        .exec()
        .await?;

    client
        .post()
        .create(
            post::title::set("My first post".to_string()),
            post::published::set(true),
            vec![post::author_id::set(Some(user.id.clone()))],
        )
        .exec()
        .await?;

    client
        .post()
        .create(
            post::title::set("My second post".to_string()),
            post::published::set(false),
            vec![post::author::link(user::id::equals(user.id.clone()))],
        )
        .exec()
        .await?;

    let user = client
        .user()
        .create(user::name::set("Oscar".to_string()), vec![])
        .exec()
        .await?;

    client
        .post()
        .create(
            post::title::set("Hello, world!".to_string()),
            post::published::set(true),
            vec![post::author_id::set(Some(user.id.clone()))],
        )
        .exec()
        .await?;

    client
        .post()
        .create(
            post::title::set("My test post".to_string()),
            post::published::set(false),
            vec![post::author::link(user::id::equals(user.id.clone()))],
        )
        .exec()
        .await?;

    client
        .user()
        .create(user::name::set("Jamie".to_string()), vec![])
        .exec()
        .await?;

    let user = client
        .user()
        .find_first(vec![user::posts::every(vec![post::title::contains(
            "post".to_string(),
        )])])
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "Brendan");

    let user = client
        .user()
        .find_first(vec![user::posts::none(vec![post::title::contains(
            "post".to_string(),
        )])])
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "Jamie");

    let user = client
        .user()
        .find_first(vec![user::posts::some(vec![post::title::equals(
            "foo".to_string(),
        )])])
        .exec()
        .await?;
    assert!(user.is_none());

    // Ordering

    let user = client
        .user()
        .find_first(vec![user::posts::some(vec![post::title::contains(
            "post".to_string(),
        )])])
        .order_by(user::name::order(Direction::Asc))
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "Brendan");

    let user = client
        .user()
        .find_first(vec![user::posts::some(vec![post::title::contains(
            "post".to_string(),
        )])])
        .order_by(user::name::order(Direction::Desc))
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "Oscar");

    cleanup(client).await
}

#[tokio::test]
async fn list_wrapper_query_transformation() -> TestResult {
    let client = client().await;

    client
        .user()
        .create(user::name::set("Brendan house".to_string()), vec![])
        .exec()
        .await?;
    let user = client
        .user()
        .find_first(vec![or![
            user::name::starts_with("40".to_string()),
            user::name::contains("40".to_string()),
            user::name::contains("house".to_string())
        ]])
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "Brendan house");

    client
        .user()
        .create(user::name::set("40 brendan".to_string()), vec![])
        .exec()
        .await?;
    let user = client
        .user()
        .find_first(vec![or![
            user::name::starts_with("40".to_string()),
            user::name::contains("40".to_string()),
            user::name::contains("house".to_string())
        ]])
        .order_by(user::created_at::order(Direction::Asc))
        .skip(1)
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "40 brendan");

    cleanup(client).await
}
