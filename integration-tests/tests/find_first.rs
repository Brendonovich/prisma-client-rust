use prisma_client_rust::{and, not, or};

use crate::{db::*, utils::*};

#[tokio::test]
async fn test() -> TestResult {
    let client = client().await;

    let posts = vec![
        client
            .post()
            .create(
                "Test post 1".to_string(),
                false,
                vec![post::views::set(100)],
            )
            .exec()
            .await?,
        client
            .post()
            .create("Test post 2".to_string(), false, vec![])
            .exec()
            .await?,
        client
            .post()
            .create("Test post 3".to_string(), false, vec![])
            .exec()
            .await?,
        client
            .post()
            .create("Test post 4".to_string(), true, vec![post::views::set(500)])
            .exec()
            .await?,
        client
            .post()
            .create("Test post 5".to_string(), false, vec![])
            .exec()
            .await?,
        client
            .post()
            .create("Test post 6".to_string(), true, vec![])
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
            user::id::equals(
                client
                    .user()
                    .create("Brendan".to_string(), vec![])
                    .exec()
                    .await?
                    .id,
            ),
            "My very cool bio.".to_string(),
            "Australia".to_string(),
            vec![],
        )
        .exec()
        .await?;

    client
        .profile()
        .create(
            user::id::equals(
                client
                    .user()
                    .create("Oscar".to_string(), vec![])
                    .exec()
                    .await?
                    .id,
            ),
            "Hello world, this is my bio.".to_string(),
            "Australia".to_string(),
            vec![],
        )
        .exec()
        .await?;

    client
        .user()
        .create("Jamie".to_string(), vec![])
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
        .create("Brendan".to_string(), vec![])
        .exec()
        .await?;

    client
        .post()
        .create(
            "My first post".to_string(),
            true,
            vec![post::author::connect(user::id::equals(user.id.clone()))],
        )
        .exec()
        .await?;

    client
        .post()
        .create(
            "My second post".to_string(),
            false,
            vec![post::author::connect(user::id::equals(user.id.clone()))],
        )
        .exec()
        .await?;

    let user = client
        .user()
        .create("Oscar".to_string(), vec![])
        .exec()
        .await?;

    client
        .post()
        .create(
            "Hello, world!".to_string(),
            true,
            vec![post::author::connect(user::id::equals(user.id.clone()))],
        )
        .exec()
        .await?;

    client
        .post()
        .create(
            "My test post".to_string(),
            false,
            vec![post::author::connect(user::id::equals(user.id.clone()))],
        )
        .exec()
        .await?;

    client
        .user()
        .create("Jamie".to_string(), vec![])
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
        .order_by(user::name::order(SortOrder::Asc))
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "Brendan");

    let user = client
        .user()
        .find_first(vec![user::posts::some(vec![post::title::contains(
            "post".to_string(),
        )])])
        .order_by(user::name::order(SortOrder::Desc))
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
        .create("Brendan house".to_string(), vec![])
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
        .create("40 brendan".to_string(), vec![])
        .exec()
        .await?;
    let user = client
        .user()
        .find_first(vec![or![
            user::name::starts_with("40".to_string()),
            user::name::contains("40".to_string()),
            user::name::contains("house".to_string())
        ]])
        .order_by(user::created_at::order(SortOrder::Asc))
        .skip(1)
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "40 brendan");

    cleanup(client).await
}
