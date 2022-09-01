use crate::{db::*, utils::*};

#[tokio::test]
async fn relations() -> TestResult {
    let client = client().await;

    client
        .user()
        .create("Brendan".to_string(), vec![])
        .include(user::include!({
            profile
            posts
            favourite_posts
        }))
        .exec()
        .await?;

    cleanup(client).await
}

#[tokio::test]
async fn relations_nested() -> TestResult {
    let client = client().await;

    client
        .user()
        .create("Brendan".to_string(), vec![])
        .include(user::include!({
            profile: include {
                user
            }
            posts: select {
                id
                author
                desc
            }
        }))
        .exec()
        .await?;

    cleanup(client).await
}

#[tokio::test]
async fn many_relation_args() -> TestResult {
    let client = client().await;

    client
        .user()
        .create("Brendan".to_string(), vec![])
        .include(user::include!({
            posts(vec![]).skip(3): include {
                author
            }
            favourite_posts(vec![]).take(2)
        }))
        .exec()
        .await?;

    cleanup(client).await
}

#[tokio::test]
async fn arguments() -> TestResult {
    let client = client().await;

    let skip = 3;
    let take = 2;
    let filters = vec![post::title::contains("prisma".to_string())];

    client
        .user()
        .create("Brendan".to_string(), vec![])
        .include(user::include!({
            posts(filters).skip(skip): include {
                author
            }
            favourite_posts(vec![]).take(take)
        }))
        .exec()
        .await?;

    cleanup(client).await
}

#[tokio::test]
async fn external_selection() -> TestResult {
    let client = client().await;

    user::include!(user_and_posts {
        posts(vec![]).skip(3): select {
            id
            author
            desc
        }
        favourite_posts(vec![]).take(2)
    });

    async fn returns_selection(client: &PrismaClient) -> user_and_posts::Data {
        client
            .user()
            .create("Brendan".to_string(), vec![])
            .include(user_and_posts::include())
            .exec()
            .await
            .unwrap()
    }

    returns_selection(&client).await;

    cleanup(client).await
}

#[tokio::test]
async fn external_selection_args() -> TestResult {
    let client = client().await;

    user::include!((skip: i64, take: i64, filters: Vec<post::WhereParam>) => user_and_posts {
        posts(filters).skip(skip): select {
            id
            author
            desc
        }
        favourite_posts(vec![]).take(take)
    });

    async fn returns_selection(client: &PrismaClient) -> user_and_posts::Data {
        let skip = 3;
        let take = 2;
        let filters = vec![post::title::contains("prisma".to_string())];

        client
            .user()
            .create("Brendan".to_string(), vec![])
            .include(user_and_posts::include(skip, take, filters))
            .exec()
            .await
            .unwrap()
    }

    returns_selection(&client).await;

    cleanup(client).await
}
