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

    // Null set
    client
        .post()
        .update(
            post::id::equals(post.id.clone()),
            vec![post::image_2::set(None)],
        )
        .exec()
        .await?;

    // Unset
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

#[tokio::test]
async fn upsert() -> TestResult {
    let client = client().await;

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
            vec![post::image_2::upsert(
                image::create(
                    10,
                    10,
                    "yet://another.link".to_string(),
                    ImageFormat::Jpeg,
                    vec![],
                ),
                vec![image::url::set("woah://another.link".to_string())],
            )],
        )
        .exec()
        .await?;

    assert_eq!(&updated.image_2.unwrap().url, "yet://another.link");

    cleanup(client).await
}

#[tokio::test]
async fn push() -> TestResult {
    let client = client().await;

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
            vec![post::images::push(vec![image::create(
                10,
                10,
                "yet://another.link".to_string(),
                ImageFormat::Jpeg,
                vec![],
            )])],
        )
        .exec()
        .await?;

    assert_eq!(&updated.images[0].url, "yet://another.link");

    cleanup(client).await
}

#[tokio::test]
async fn update_many() -> TestResult {
    let client = client().await;

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
            vec![post::images::set(vec![image::create(
                20,
                20,
                "another://link.com".to_string(),
                ImageFormat::Gif,
                vec![],
            )])],
        )
        .exec()
        .await?;

    assert_eq!(post.images[0].format, ImageFormat::Gif);

    let updated = client
        .post()
        .update(
            post::id::equals(post.id),
            vec![post::images::update_many(
                vec![image::format::equals(ImageFormat::Gif)],
                vec![image::format::set(ImageFormat::Png)],
            )],
        )
        .exec()
        .await?;

    assert_eq!(updated.images[0].format, ImageFormat::Png);

    cleanup(client).await
}

#[tokio::test]
async fn delete_many() -> TestResult {
    let client = client().await;

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
            vec![post::images::set(vec![image::create(
                20,
                20,
                "another://link.com".to_string(),
                ImageFormat::Gif,
                vec![],
            )])],
        )
        .exec()
        .await?;

    assert_eq!(post.images[0].format, ImageFormat::Gif);

    let updated = client
        .post()
        .update(
            post::id::equals(post.id),
            vec![post::images::delete_many(vec![image::format::equals(
                ImageFormat::Gif,
            )])],
        )
        .exec()
        .await?;

    assert!(updated.images.is_empty());

    cleanup(client).await
}
