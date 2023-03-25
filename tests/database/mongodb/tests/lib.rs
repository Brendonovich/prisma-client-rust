mod db;
mod utils;

use db::*;
use utils::*;

#[tokio::test]
async fn set_unset() -> TestResult {
    let client = client().await;

    // Required set
    let post = client
        .post()
        .create(
            "Title".to_string(),
            image::create(
                10,
                10,
                "some://link.com".to_string(),
                ImageFormat::Png,
                vec![],
            ),
            vec![],
        )
        .exec()
        .await?;

    // Optional set
    client
        .post()
        .update(
            post::id::equals(post.id.clone()),
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
        .update(
            post::id::equals(post.id.clone()),
            vec![post::image_2::set(None)],
        )
        .exec()
        .await?;

    client
        .post()
        .update(
            post::id::equals(post.id.clone()),
            vec![post::image_2::unset()],
        )
        .exec()
        .await?;

    cleanup(client).await
}

#[tokio::test]
async fn update() -> TestResult {
    let client = client().await;

    // Required set
    let post = client
        .post()
        .create(
            "Title".to_string(),
            image::create(
                10,
                10,
                "some://link.com".to_string(),
                ImageFormat::Png,
                vec![],
            ),
            vec![],
        )
        .exec()
        .await?;

    let updated = client
        .post()
        .update(
            post::id::equals(post.id),
            vec![
                post::image::update(vec![image::url::set("another://link.com".to_string())]),
                post::image_2::set(None),
            ],
        )
        .exec()
        .await?;

    assert_eq!(&updated.image.url, "another://link.com");

    cleanup(client).await
}
