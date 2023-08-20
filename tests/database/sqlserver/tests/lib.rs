use prisma_client_rust::raw;

use crate::db::{test_table, PrismaClient};

#[allow(warnings)]
mod db;

#[tokio::test]
async fn issue_378() -> Result<(), Box<dyn std::error::Error>> {
    let client = PrismaClient::_builder().build().await?;

    client.test_table().create(vec![]).exec().await?;
    let _: Vec<test_table::Data> = client
        ._query_raw(raw!("SELECT id FROM TestTable"))
        .exec()
        .await?;

    Ok(())
}
