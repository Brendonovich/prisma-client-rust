use crate::db::{Comment, Post, PrismaClient, User};

pub mod db;

#[tokio::main]
pub async fn main() {
    let mut client = PrismaClient::new();

    client.engine.connect().await;
    client
        .post()
        .find_unique(Post::id().equals("0".to_string()))
        .delete()
        .exec()
        .await;

    client.user().find_many(vec![]).delete().exec().await;
}
