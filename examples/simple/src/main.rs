use crate::db::{Comment, Post, PrismaClient};

pub mod db;

#[tokio::main]
pub async fn main() {
    let mut client = PrismaClient::new();

    client.engine.connect().await;

    // let post = client
    //     .post()
    //     .create_one(Post::id().set("0".into()), vec![])
    //     .exec()
    //     .await;

    let post = client
        .post()
        .find_unique(Post::id().equals("0".into()))
        .exec()
        .await;

    println!("{:?}", post);

    // let comment = client
    //     .comment()
    //     .create_one(
    //         Comment::id().set("0".into()),
    //         Comment::post().link(Post::id().equals(post.id.into())),
    //         vec![],
    //     )
    //     .exec()
    //     .await;

    // println!("{:?}", comment);

    let comment = client
        .comment()
        .find_many(vec![Comment::post().is(vec![Post::id().equals(post.id.clone())])])
        .exec()
        .await;

    println!("{:?}", comment);

    // client.comment().find_many(vec![]).delete().exec().await;
    // client.post().find_many(vec![]).delete().exec().await;
}
