use prisma_client_rust::{
    bigdecimal::BigDecimal, prisma_errors::query_engine::RecordRequiredButNotFound,
    queries::QueryError,
};

use crate::{db::*, utils::*};
use std::str::FromStr;

#[tokio::test]
async fn decimal() -> TestResult {
    let client = client().await;

    let dec = BigDecimal::from_str("1.1").unwrap();

    let record = client
        .types()
        .create(vec![types::decimal::set(Some(dec.clone()))])
        .exec()
        .await?;

    assert_eq!(record.decimal, Some(dec));

    cleanup(client).await
}
