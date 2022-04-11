use prisma_client_rust::{or, Direction};

use crate::{
    db::{Post, Profile, User},
    utils::*,
};

#[tokio::test]
async fn find_many() -> TestResult {
    let client = client().await;

    let posts = vec![
        client
            .post()
            .create(
                Post::title().set("Test post 1".to_string()),
                Post::published().set(false),
                vec![],
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
    ];

    let found = client
        .post()
        .find_many(vec![Post::title().equals("Test post 1".to_string())])
        .exec()
        .await?;
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].id, posts[0].id);

    let posts = client
        .post()
        .find_many(vec![or![
            Post::title().equals("Test post 1".to_string()),
            Post::title().equals("Test post 2".to_string()),
        ]])
        .exec()
        .await?;
    assert_eq!(posts.len(), 2);

    let posts = client
        .post()
        .find_many(vec![Post::title().contains("Test post".to_string())])
        .exec()
        .await?;
    assert_eq!(posts.len(), 2);

    let posts = client
        .post()
        .find_many(vec![Post::title().starts_with("Test post".to_string())])
        .exec()
        .await?;
    assert_eq!(posts.len(), 2);

    let posts = client
        .post()
        .find_many(vec![
            Post::title().not_in_vec(vec!["Test post 1".to_string()])
        ])
        .exec()
        .await?;
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].title, "Test post 2");

    let posts = client
        .post()
        .find_many(vec![Post::title().equals("Test post 2".to_string())])
        .exec()
        .await?;
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].title, "Test post 2");

    let posts = client
        .post()
        .find_many(vec![])
        .order_by(Post::title().order(Direction::Desc))
        .exec()
        .await?;
    assert_eq!(posts.len(), 2);
    assert_eq!(posts[0].title, "Test post 2");
    assert_eq!(posts[1].title, "Test post 1");

    let posts = client
        .post()
        .find_many(vec![])
        .order_by(Post::title().order(Direction::Asc))
        .exec()
        .await?;
    assert_eq!(posts.len(), 2);
    assert_eq!(posts[0].title, "Test post 1");
    assert_eq!(posts[1].title, "Test post 2");

    cleanup(client).await
}

#[tokio::test]
async fn cursor() -> TestResult {
    let client = client().await;

    let posts = vec![
        client
            .post()
            .create(
                Post::title().set("Foo 1".to_string()),
                Post::published().set(false),
                vec![],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                Post::title().set("Foo 2".to_string()),
                Post::published().set(false),
                vec![],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                Post::title().set("Foo 3".to_string()),
                Post::published().set(false),
                vec![],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                Post::title().set("Foo 4".to_string()),
                Post::published().set(false),
                vec![],
            )
            .exec()
            .await?,
    ];

    let found = client
        .post()
        .find_many(vec![])
        .cursor(Post::id().cursor(posts[1].id.clone()))
        .exec()
        .await?;
    assert_eq!(found.len(), 3);
    assert_eq!(found[0].title, "Foo 2".to_string());
    assert_eq!(found[1].title, "Foo 3".to_string());
    assert_eq!(found[2].title, "Foo 4".to_string());

    let found = client
        .post()
        .find_many(vec![])
        .cursor(Post::id().cursor(posts[3].id.clone()))
        .exec()
        .await?;
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].title, "Foo 4".to_string());

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

    let users = client
        .user()
        .find_many(vec![
            User::profile().is(vec![Profile::bio().contains("cool".to_string())])
        ])
        .exec()
        .await?;
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Brendan");
    assert!(users[0].profile().is_err());

    let users = client
        .user()
        .find_many(vec![
            User::profile().is(vec![Profile::bio().contains("bio".to_string())])
        ])
        .exec()
        .await?;
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "Brendan");
    assert_eq!(users[1].name, "Oscar");

    let users = client
        .user()
        .find_many(vec![
            User::profile().is_not(vec![Profile::bio().contains("bio".to_string())])
        ])
        .exec()
        .await?;
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Jamie");

    cleanup(client).await
}

#[tokio::test]
async fn filtering_one_to_many_relation() -> TestResult {
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

    let users = client
        .user()
        .find_many(vec![
            User::posts().every(vec![Post::title().contains("post".to_string())])
        ])
        .exec()
        .await?;
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "Brendan");
    assert_eq!(users[1].name, "Jamie");

    let users = client
        .user()
        .find_many(vec![
            User::posts().some(vec![Post::title().contains("Post".to_string())])
        ])
        .exec()
        .await?;
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "Brendan");
    assert_eq!(users[1].name, "Oscar");

    let users = client
        .user()
        .find_many(vec![
            User::posts().none(vec![Post::title().contains("Post".to_string())])
        ])
        .exec()
        .await?;
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Jamie");

    let users = client
        .user()
        .find_many(vec![
            User::posts().some(vec![Post::title().equals("foo".to_string())])
        ])
        .exec()
        .await?;
    assert_eq!(users.len(), 0);

    cleanup(client).await
}

#[tokio::test]
async fn ordering() -> TestResult {
    let client = client().await;

    client
        .post()
        .create(
            Post::title().set("Test post 1".to_string()),
            Post::published().set(false),
            vec![],
        )
        .exec()
        .await?;
    client
        .post()
        .create(
            Post::title().set("Test post 2".to_string()),
            Post::published().set(false),
            vec![],
        )
        .exec()
        .await?;
    client
        .post()
        .create(
            Post::title().set("Test post 3".to_string()),
            Post::published().set(true),
            vec![],
        )
        .exec()
        .await?;

    let found = client
        .post()
        .find_many(vec![Post::title().contains("Test".to_string())])
        .order_by(Post::published().order(Direction::Asc))
        .exec()
        .await?;
    assert_eq!(found.len(), 3);
    assert_eq!(found[0].published, false);
    assert_eq!(found[1].published, false);
    assert_eq!(found[2].published, true);
    
    let found = client
        .post()
        .find_many(vec![Post::title().contains("Test".to_string())])
        .order_by(Post::published().order(Direction::Desc))
        .exec()
        .await?;
    assert_eq!(found.len(), 3);
    assert_eq!(found[0].published, true);
    assert_eq!(found[1].published, false);
    assert_eq!(found[2].published, false);
    
    cleanup(client).await
}
