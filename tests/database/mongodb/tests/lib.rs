mod db;
mod utils;

use db::*;
use utils::*;

#[tokio::test]
async fn bruh() -> TestResult {
    let client = client().await;

    client
        .post()
        .create(
            "Title".to_string(),
            post::image::set(10, 10, "".to_string(), ImageFormat::Gif, vec![]),
            vec![],
        )
        .exec()
        .await?;

    client
        .post()
        .update(post::id::equals(0), vec![post::image_2::unset()])
        .exec()
        .await?;

    cleanup(client).await
}
