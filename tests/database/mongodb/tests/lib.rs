mod db;
mod utils;

use db::*;
use utils::*;

#[tokio::test]
async fn set_unset() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create(
            "Title".to_string(),
            image::create(10, 10, "".to_string(), ImageFormat::Gif, vec![]),
            vec![],
        )
        .exec()
        .await?;

    client
        .post()
        .update(
            post::id::equals(post.id),
            vec![post::image_2::set(Some(image::create(
                10,
                10,
                "".to_string(),
                ImageFormat::Gif,
                vec![],
            )))],
        )
        .exec()
        .await?;

    client
        .post()
        .update(post::id::equals(post.id), vec![post::image_2::set(None)])
        .exec()
        .await?;

    client
        .post()
        .update(post::id::equals(post.id), vec![post::image_2::unset()])
        .exec()
        .await?;

    cleanup(client).await
}
