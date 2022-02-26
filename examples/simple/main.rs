use crate::prisma::{Category, Post, PrismaClient};

pub mod prisma;

#[tokio::main]
pub async fn main() {
    let mut client = PrismaClient::new();

    client.engine.connect().await;

    let ret = client
        .post()
        .find_many(vec![
            Post::category().is(vec![
                Category::id().contains("test".into()),
            ]),
        ])
        .exec()
        .await;

    println!("{:?}", ret);
}


