use crate::prisma::{Post, PrismaClient};

pub mod prisma;

#[tokio::main]
pub async fn main() {
    let mut client = PrismaClient::new();

    client.engine.connect().await;

    let id = "0";

    client
        .post()
        .create_one(
            Post::id().set(id.into()),
            vec![Post::name().set("Testing".into())],
        )
        .exec()
        .await;

    let ret = client
        .post()
        .find_many(vec![Post::id().equals(id.into())])
        .exec()
        .await;

    println!("{:?}", ret);

    client
        .post()
        .find_unique(Post::id().equals(id.into()))
        .delete()
        .exec()
        .await;
}
