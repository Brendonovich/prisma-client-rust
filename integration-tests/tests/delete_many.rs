use crate::{db::*, utils::*};

#[tokio::test]
async fn delete_many() -> TestResult {
    let client = client().await;

    let posts = vec![
        client
            .post()
            .create(
                post::title::set("Foo post".to_string()),
                post::published::set(false),
                vec![],
            )
            .exec()
            .await?,
        client
            .post()
            .create(
                post::title::set("Bar post".to_string()),
                post::published::set(false),
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
            .find_unique(post::id::equals(post.id.clone()))
            .exec()
            .await?;
        assert!(found.is_none());
    }

    cleanup(client).await
}
