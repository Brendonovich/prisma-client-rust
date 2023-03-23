use crate::{db::*, utils::*};

#[tokio::test]
async fn query() -> TestResult {
    let client = client().await;

    // client.unsupported();

    cleanup(client).await
}
