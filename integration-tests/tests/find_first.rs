use prisma_client_rust::{and, not, or, Direction};

use crate::{
    db::{Post, Profile, User},
    utils::*,
};

#[tokio::test]
async fn find_first() -> TestResult {
    let client = client().await;

    let posts = vec![
        client
            .post()
            .create(
                Post::title().set("Test post 1".to_string()),
                Post::published().set(false),
                vec![Post::views().set(100)],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                Post::title().set("Test post 2".to_string()),
                Post::published().set(false),
                vec![],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                Post::title().set("Test post 3".to_string()),
                Post::published().set(false),
                vec![],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                Post::title().set("Test post 4".to_string()),
                Post::published().set(true),
                vec![Post::views().set(500)],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                Post::title().set("Test post 5".to_string()),
                Post::published().set(false),
                vec![],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                Post::title().set("Test post 6".to_string()),
                Post::published().set(true),
                vec![],
            )
            .exec()
            .await?,
    ];

    let post = client
        .post()
        .find_first(vec![Post::published().equals(true)])
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.id, posts[3].id);
    assert_eq!(post.title, "Test post 4");
    assert!(post.published);

    let post = client
        .post()
        .find_first(vec![Post::title().contains("not found".to_string())])
        .exec()
        .await?;
    assert!(post.is_none());

    let post = client
        .post()
        .find_first(vec![Post::published().equals(true)])
        .skip(1)
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.id, posts[5].id);
    assert_eq!(post.title, "Test post 6");
    assert!(post.published);

    let post = client
        .post()
        .find_first(vec![not!(Post::published().equals(true))])
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.title, "Test post 1");

    let post = client
        .post()
        .find_first(vec![
            Post::title().contains("Test".to_string()),
            and!(Post::published().equals(true)),
        ])
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.title, "Test post 4");
    
    let post = client
        .post()
        .find_first(vec![and! {
            Post::published().equals(true),
            Post::title().contains("Test".to_string()),
        }])
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.title, "Test post 4");
    
    let post = client
        .post()
        .find_first(vec![or! {
            Post::views().gt(100),
            Post::published().equals(false)
        }])
        .exec()
        .await?
        .unwrap();
    assert_eq!(post.title, "Test post 1");
    
    let post = client
        .post()
        .find_first(vec![or! {
            Post::views().gt(100),
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
            Profile::user().link(
                User::id().equals(
                    client
                        .user()
                        .create(User::name().set("Brendan".to_string()), vec![])
                        .exec()
                        .await?
                        .id,
                ),
            ),
            Profile::bio().set("My very cool bio.".to_string()),
            Profile::country().set("Australia".to_string()),
            vec![],
        )
        .exec()
        .await?;

    client
        .profile()
        .create(
            Profile::user().link(
                User::id().equals(
                    client
                        .user()
                        .create(User::name().set("Oscar".to_string()), vec![])
                        .exec()
                        .await?
                        .id,
                ),
            ),
            Profile::bio().set("Hello world, this is my bio.".to_string()),
            Profile::country().set("Australia".to_string()),
            vec![],
        )
        .exec()
        .await?;

    client
        .user()
        .create(User::name().set("Jamie".to_string()), vec![])
        .exec()
        .await?;

    let user = client
        .user()
        .find_first(vec![
            User::profile().is(vec![Profile::bio().contains("cool".to_string())])
        ])
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "Brendan");
    assert!(user.profile().is_err());

    let user = client
        .user()
        .find_first(vec![
            User::profile().is_not(vec![Profile::bio().contains("bio".to_string())])
        ])
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
        .create(User::name().set("Brendan".to_string()), vec![])
        .exec()
        .await?;

    client
        .post()
        .create(
            Post::title().set("My first post".to_string()),
            Post::published().set(true),
            vec![Post::author_id().set(Some(user.id.clone()))],
        )
        .exec()
        .await?;

    client
        .post()
        .create(
            Post::title().set("My second post".to_string()),
            Post::published().set(false),
            vec![Post::author().link(User::id().equals(user.id.clone()))],
        )
        .exec()
        .await?;

    let user = client
        .user()
        .create(User::name().set("Oscar".to_string()), vec![])
        .exec()
        .await?;

    client
        .post()
        .create(
            Post::title().set("Hello, world!".to_string()),
            Post::published().set(true),
            vec![Post::author_id().set(Some(user.id.clone()))],
        )
        .exec()
        .await?;

    client
        .post()
        .create(
            Post::title().set("My test post".to_string()),
            Post::published().set(false),
            vec![Post::author().link(User::id().equals(user.id.clone()))],
        )
        .exec()
        .await?;

    client
        .user()
        .create(User::name().set("Jamie".to_string()), vec![])
        .exec()
        .await?;

    let user = client
        .user()
        .find_first(vec![
            User::posts().every(vec![Post::title().contains("post".to_string())])
        ])
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "Brendan");

    let user = client
        .user()
        .find_first(vec![
            User::posts().none(vec![Post::title().contains("Post".to_string())])
        ])
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "Jamie");

    let user = client
        .user()
        .find_first(vec![
            User::posts().some(vec![Post::title().equals("foo".to_string())])
        ])
        .exec()
        .await?;
    assert!(user.is_none());

    // Ordering

    let user = client
        .user()
        .find_first(vec![
            User::posts().some(vec![Post::title().contains("Post".to_string())])
        ])
        .order_by(User::name().order(Direction::Asc))
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "Brendan");

    let user = client
        .user()
        .find_first(vec![
            User::posts().some(vec![Post::title().contains("Post".to_string())])
        ])
        .order_by(User::name().order(Direction::Desc))
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
        .create(User::name().set("Brendan house".to_string()), vec![])
        .exec()
        .await?;
    let user = client
        .user()
        .find_first(vec![or![
            User::name().starts_with("40".to_string()),
            User::name().contains("40".to_string()),
            User::name().contains("house".to_string())
        ]])
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "Brendan house");

    client
        .user()
        .create(User::name().set("40 brendan".to_string()), vec![])
        .exec()
        .await?;
    let user = client
        .user()
        .find_first(vec![or![
            User::name().starts_with("40".to_string()),
            User::name().contains("40".to_string()),
            User::name().contains("house".to_string())
        ]])
        .order_by(User::created_at().order(Direction::Asc))
        .skip(1)
        .exec()
        .await?
        .unwrap();
    assert_eq!(user.name, "40 brendan");

    cleanup(client).await
}
