use crate::utils::*;

#[tokio::test]
async fn issue_282() -> TestResult {
    let client = client().await;

    client
        .child()
        .create("This string is required!".to_string(), vec![])
        .exec()
        .await;

    cleanup(client).await
}
