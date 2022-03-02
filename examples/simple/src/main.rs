use crate::db::{Post, PrismaClient, User};

pub mod db;

#[tokio::main]
pub async fn main() {
    let client = PrismaClient::new().await;

    let user = client
        .user()
        .create_one(
            User::id().set("user0".to_string()),
            User::display_name().set("User 0".to_string()),
            vec![],
        )
        .exec()
        .await;

    let post = client
        .post()
        .create_one(
            Post::id().set("post0".to_string()),
            Post::content().set("Some post content".to_string()),
            Post::user().link(User::id().equals(user.id.to_string())),
            vec![],
        )
        .exec()
        .await;

    println!("User: {:?}", user);
    println!("Post: {:?}", post);

    let post_with_user = client
        .post()
        .find_unique(Post::id().equals("0".to_string()))
        .with(vec![Post::user().fetch()])
        .exec()
        .await;

    println!("Post user: {:?}", post_with_user.user().unwrap());

    let user_with_posts = client
        .user()
        .find_unique(User::username().equals("user0".to_string()))
        .with(vec![User::posts().fetch(vec![])])
        .exec()
        .await;

    println!("User posts: {:?}", user_with_posts.posts().unwrap());

    let deleted_posts = client.post().find_many(vec![]).delete().exec().await;
    println!("Deleted {} posts", deleted_posts);

    let deleted_users_count = client.user().find_many(vec![]).delete().exec().await;
    println!("Deleted {} users", deleted_users_count);
}
