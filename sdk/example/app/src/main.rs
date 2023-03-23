mod db;
mod db_custom_generator;

use db::*;

#[tokio::main]
async fn main() {
    let client = PrismaClient::_builder().build().await.unwrap();
}
