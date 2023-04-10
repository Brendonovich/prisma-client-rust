#[allow(warnings, unused)]
mod db;
mod db_custom_generator;

use db::*;

#[tokio::main]
async fn main() {
    PrismaClient::_builder().build().await.unwrap();
}
