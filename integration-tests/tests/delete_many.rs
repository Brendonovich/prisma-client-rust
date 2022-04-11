use crate::{db::Post, utils::*};

#[tokio::test]
async fn delete_many() -> TestResult {
    let client = client().await;

    let posts = vec![
        client
            .post()
            .create(
                Post::title().set("Foo post".to_string()),
                Post::published().set(false),
                vec![],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                Post::title().set("Bar post".to_string()),
                Post::published().set(false),
                vec![],
            )
            .exec()
            .await?,
    ];

    let count = client.post().find_many(vec![]).delete().exec().await?;
    assert!(count >= 1);

    for post in posts {
        let found = client
            .post()
            .find_unique(Post::id().equals(post.id.clone()))
            .exec()
            .await?;
        assert!(found.is_none());
    }

    cleanup(client).await
}
