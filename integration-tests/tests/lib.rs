use utils::{cleanup, TestResult};

mod db;
mod utils;

// No need to migrate manually if db_push can do it for you!
// funny name to make it run first
#[tokio::test]
async fn aaaa_db_push() -> TestResult {
    let client = db::new_client().await.unwrap();

    client._migrate_deploy().await.unwrap();

    client
        .user()
        .create("Brendan".to_string(), vec![])
        .exec()
        .await?;

    cleanup(client).await
}

pub mod batch;
mod count;
mod create;
mod create_many;
mod delete;
mod delete_many;
mod find_first;
mod find_many;
mod find_unique;
mod include;
mod raw;
mod select;
mod update;
mod upsert;
mod with;
