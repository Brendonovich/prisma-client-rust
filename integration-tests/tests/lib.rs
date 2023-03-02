use utils::{cleanup, TestResult};

mod db;
mod transaction;
mod utils;

#[tokio::test]
async fn aaaa_run_migrations() -> TestResult {
    let client = db::new_client().await.unwrap();

    client._db_push().await.unwrap();

    client
        .user()
        .create("Brendan".to_string(), vec![])
        .exec()
        .await?;

    cleanup(client).await
}

mod batch;
// mod callbacks;
mod count;
mod create;
mod create_many;
mod delete;
mod delete_many;
mod find_first;
mod find_many;
mod find_unique;
mod include;
mod mock;
mod raw;
mod select;
mod specta;
mod update;
mod upsert;
mod with;
