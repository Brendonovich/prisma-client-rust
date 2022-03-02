use crate::db::{PrismaClient, User};

pub mod db;

#[tokio::main]
pub async fn main() {
    let client = PrismaClient::new().await;

    let user = client
        .user()
        .create_one(
            User::username().set("user0".to_string()),
            User::display_name().set("User 0".to_string()),
            // Optional arguments can be added in a vector as the last parameter
            vec![],
        )
        .exec()
        .await;

    let post = client
        .post()
        .create_one(
            Post::id().set("0".to_string()),
            Post::content().set("Some post content".to_string()),
            Post::user().link(User::username().equals(user.username.to_string())),
            vec![],
        )
        .exec()
        .await;

    println!("User: {:?}", user);
    println!("Post: {:?}", post);
}
