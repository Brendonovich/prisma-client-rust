use crate::{db::*, utils::*};

#[tokio::test]
async fn test() -> TestResult {
    let client = client().await;

    let posts = vec![
        client
            .post()
            .create("Foo post".to_string(), false, vec![])
            .exec()
            .await?,
        client
            .post()
            .create("Bar post".to_string(), false, vec![])
            .exec()
            .await?,
    ];

    let count = client.post().delete_many(vec![]).exec().await?;
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
