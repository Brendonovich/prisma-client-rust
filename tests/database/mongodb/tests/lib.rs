mod db;
mod utils;

use db::*;
use utils::*;

#[tokio::test]
async fn bruh() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create(
            "Title".to_string(),
            post::image::set(10, 10, "".to_string(), ImageFormat::Gif, vec![]),
            vec![post::image_2::set(width, height, url, format, params)],
        )
        .exec()
        .await?;

    client
        .post()
        .update(post::id::equals(post.id), vec![post::image_2::unset()])
        .exec()
        .await?;

    cleanup(client).await
}
