use db::new_client;

mod db;
mod db_custom_generator;

#[tokio::main]
async fn main() {
    let client = new_client().await.unwrap();
}
