use prisma_client_rust::{raw, PrismaValue};

use crate::{db::*, utils::*};

#[tokio::test]
async fn query_raw_model() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create("My post title!".to_string(), false, vec![])
        .exec()
        .await?;

    let result: Vec<post::Data> = client
        ._query_raw(raw!(
            "SELECT * FROM Post WHERE id = {}",
            PrismaValue::String(post.id.clone())
        ))
        .exec()
        .await?;

    assert_eq!(result.len(), 1);
    assert_eq!(&result[0].id, &post.id);
    assert_eq!(result[0].published, false);

    cleanup(client).await
}

#[tokio::test]
async fn query_raw_no_result() -> TestResult {
    let client = client().await;

    let result: Vec<post::Data> = client
        ._query_raw(raw!(
            "SELECT * FROM Post WHERE id = {}",
            PrismaValue::String("sdldsd".to_string())
        ))
        .exec()
        .await?;
    assert_eq!(result.len(), 0);

    cleanup(client).await
}

#[tokio::test]
async fn execute_raw() -> TestResult {
    let client = client().await;

    let post = client
        .post()
        .create("My post title!".to_string(), false, vec![])
        .exec()
        .await?;

    let count = client
        ._execute_raw(raw!(
            "UPDATE Post SET title = {} WHERE id = {}",
            PrismaValue::String("My edited title".to_string()),
            PrismaValue::String(post.id.clone())
        ))
        .exec()
        .await?;
    assert_eq!(count, 1);

    let found = client
        .post()
        .find_unique(post::id::equals(post.id.clone()))
        .exec()
        .await?;
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(&found.id, &post.id);
    assert_eq!(&found.title, "My edited title");

    cleanup(client).await
}

#[tokio::test]
async fn execute_raw_no_result() -> TestResult {
    let client = client().await;

    let count = client
        ._execute_raw(raw!(
            "UPDATE Post SET title = {} WHERE id = {}",
            PrismaValue::String("updated title".to_string()),
            PrismaValue::String("sdldsd".to_string())
        ))
        .exec()
        .await?;
    assert_eq!(count, 0);

    cleanup(client).await
}

// query_first?
