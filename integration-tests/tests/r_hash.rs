use crate::db::*;
use crate::utils::*;

#[tokio::test]
async fn field() -> TestResult {
    let client = client().await;

    let val = client.r_hash().create(vec![]).exec().await?;

    assert_eq!(val.r#impl, 4);

    client
        .r_hash()
        .delete(r_hash::r#impl::equals(4))
        .exec()
        .await?;

    cleanup(client).await
}
