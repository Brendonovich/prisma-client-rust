use prisma_client_rust::or;

use crate::{db::*, utils::*};

#[tokio::test]
async fn query() -> TestResult {
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

    let found = client
        .post()
        .find_many(vec![post::title::equals("Test post 1".to_string())])
        .exec()
        .await?;
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].id, posts[0].id);

    let posts = client
        .post()
        .find_many(vec![or![
            post::title::equals("Test post 1".to_string()),
            post::title::equals("Test post 2".to_string()),
        ]])
        .exec()
        .await?;
    assert_eq!(posts.len(), 2);

    let posts = client
        .post()
        .find_many(vec![post::title::contains("Test post".to_string())])
        .exec()
        .await?;
    assert_eq!(posts.len(), 2);

    let posts = client
        .post()
        .find_many(vec![post::title::starts_with("Test post".to_string())])
        .exec()
        .await?;
    assert_eq!(posts.len(), 2);

    let posts = client
        .post()
        .find_many(vec![post::title::not_in_vec(vec![
            "Test post 1".to_string()
        ])])
        .exec()
        .await?;
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].title, "Test post 2");

    let posts = client
        .post()
        .find_many(vec![post::title::equals("Test post 2".to_string())])
        .exec()
        .await?;
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].title, "Test post 2");

    let posts = client
        .post()
        .find_many(vec![])
        .order_by(post::title::order(SortOrder::Desc))
        .exec()
        .await?;
    assert_eq!(posts.len(), 2);
    assert_eq!(posts[0].title, "Test post 2");
    assert_eq!(posts[1].title, "Test post 1");

    let posts = client
        .post()
        .find_many(vec![])
        .order_by(post::title::order(SortOrder::Asc))
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
            .create("Foo 1".to_string(), false, vec![])
            .exec()
            .await?,
        client
            .post()
            .create("Foo 2".to_string(), false, vec![])
            .exec()
            .await?,
        client
            .post()
            .create("Foo 3".to_string(), false, vec![])
            .exec()
            .await?,
        client
            .post()
            .create("Foo 4".to_string(), false, vec![])
            .exec()
            .await?,
    ];

    let found = client
        .post()
        .find_many(vec![])
        .cursor(post::id::equals(posts[1].id.clone()))
        .exec()
        .await?;
    assert_eq!(found.len(), 3);
    assert_eq!(found[0].title, "Foo 2".to_string());
    assert_eq!(found[1].title, "Foo 3".to_string());
    assert_eq!(found[2].title, "Foo 4".to_string());

    let found = client
        .post()
        .find_many(vec![])
        .cursor(post::id::equals(posts[3].id.clone()))
        .exec()
        .await?;
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].title, "Foo 4".to_string());

    cleanup(client).await
}

// From Spacedrive
#[tokio::test]
async fn cursor_order() -> TestResult {
    let client = client().await;

    let user = client
        .user()
        .create("Brendan".to_string(), vec![])
        .exec()
        .await?;

    client
        .file_path()
        .create_many(
            (0..1000)
                .into_iter()
                .map(|id| {
                    file_path::create_unchecked(
                        id,
                        format!("File Path {id}"),
                        user.id.clone(),
                        vec![],
                    )
                })
                .collect(),
        )
        .exec()
        .await?;

    let file_paths = client
        .file_path()
        .find_many(vec![file_path::user_id::equals(user.id.clone())])
        .cursor(file_path::user_id_local_id(user.id.clone(), 100))
        .order_by(file_path::local_id::order(SortOrder::Asc))
        .take(200)
        .skip(1)
        .exec()
        .await?;

    assert_eq!(file_paths[0].local_id, 101);

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

    let users = client
        .user()
        .find_many(vec![user::profile::is(vec![profile::bio::contains(
            "cool".to_string(),
        )])])
        .exec()
        .await?;
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Brendan");
    assert!(users[0].profile().is_err());

    let users = client
        .user()
        .find_many(vec![user::profile::is(vec![profile::bio::contains(
            "bio".to_string(),
        )])])
        .exec()
        .await?;
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "Brendan");
    assert_eq!(users[1].name, "Oscar");

    let users = client
        .user()
        .find_many(user::filter! { profile: { is_not: { bio: { contains: "bio".to_string() } } } })
        .exec()
        .await?;
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Jamie");

    let users = client
        .user()
        .find_many(vec![user::profile::is_null()])
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

    let users = client
        .user()
        .find_many(vec![user::posts::every(vec![post::title::contains(
            "post".to_string(),
        )])])
        .order_by(user::name::order(SortOrder::Asc))
        .exec()
        .await?;
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "Brendan");
    assert_eq!(users[1].name, "Jamie");

    let users = client
        .user()
        .find_many(vec![user::posts::some(vec![post::title::contains(
            "post".to_string(),
        )])])
        .order_by(user::name::order(SortOrder::Asc))
        .exec()
        .await?;
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "Brendan");
    assert_eq!(users[1].name, "Oscar");

    let users = client
        .user()
        .find_many(vec![user::posts::none(vec![post::title::contains(
            "post".to_string(),
        )])])
        .exec()
        .await?;
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Jamie");

    let users = client
        .user()
        .find_many(vec![user::posts::some(vec![post::title::equals(
            "foo".to_string(),
        )])])
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
        .create("Test post 1".to_string(), false, vec![])
        .exec()
        .await?;
    client
        .post()
        .create("Test post 2".to_string(), false, vec![])
        .exec()
        .await?;
    client
        .post()
        .create("Test post 3".to_string(), true, vec![])
        .exec()
        .await?;

    let found = client
        .post()
        .find_many(vec![post::title::contains("Test".to_string())])
        .order_by(post::published::order(SortOrder::Asc))
        .order_by(post::id::order(SortOrder::Asc))
        .exec()
        .await?;
    assert_eq!(found.len(), 3);
    assert_eq!(found[0].published, false);
    assert_eq!(found[1].published, false);
    assert_eq!(found[2].published, true);

    let found = client
        .post()
        .find_many(vec![post::title::contains("Test".to_string())])
        .order_by(post::published::order(SortOrder::Desc))
        .exec()
        .await?;
    assert_eq!(found.len(), 3);
    assert_eq!(found[0].published, true);
    assert_eq!(found[1].published, false);
    assert_eq!(found[2].published, false);

    cleanup(client).await
}

#[tokio::test]
async fn select() -> TestResult {
    let client = client().await;

    client
        .user()
        .create("Brendan".to_string(), vec![])
        .exec()
        .await?;

    client
        .user()
        .create("Oscar".to_string(), vec![])
        .exec()
        .await?;

    let users = client
        .user()
        .find_many(vec![])
        .select(user::select!({
            id
            name
            profile: select {
                id
            }
            posts(vec![]).take(5): select {
                id
                title
                desc
                categories(vec![]).take(5): select {
                    id
                    name
                }
            }
        }))
        .exec()
        .await?;

    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "Brendan".to_string());
    assert_eq!(users[1].name, "Oscar".to_string());
    assert!(users[0].profile.is_none());
    assert!(users[1].profile.is_none());
    assert_eq!(users[0].posts.len(), 0);

    cleanup(client).await
}

#[tokio::test]
async fn filter() -> TestResult {
    let client = client().await;

    let (brendan, _, _) = client
        ._batch((
            client.user().create("Brendan".to_string(), vec![]),
            client.user().create("Oscar".to_string(), vec![]),
            client.user().create("Jamie".to_string(), vec![]),
        ))
        .await?;

    client
        .post()
        .create(
            "Test".to_string(),
            true,
            vec![post::author::connect(user::id::equals(brendan.id))],
        )
        .exec()
        .await?;

    let users = client
        .user()
        .find_many(user::filter! {
            name: { equals: "Brendan".to_string() }
        })
        .include(user::include!({ posts }))
        .exec()
        .await?;

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].posts.len(), 1);

    cleanup(client).await
}
