use crate::db::{Post, User};

pub mod db;

#[tokio::main]
pub async fn main() {
    let client = db::new_client().await.unwrap();

    let user = client
        .user()
        .create(
            User::id().set("user0".to_string()),
            User::display_name().set("User 0".to_string()),
            vec![],
        )
        .exec()
        .await
        .unwrap();

    let post = client
        .post()
        .create(
            Post::id().set("post0".to_string()),
            Post::content().set("Some post content".to_string()),
            Post::user().link(User::id().equals(user.id.to_string())),
            vec![],
        )
        .exec()
        .await
        .unwrap();

    println!("User: {:?}", user);
    println!("Post: {:?}", post);

    let post_with_user = client
        .post()
        .find_unique(Post::id().equals("post0".to_string()))
        .with(Post::user().fetch())
        .exec()
        .await
        .unwrap()
        .unwrap();

    println!("Post user: {:?}", post_with_user.user().unwrap());

    let user_with_posts = client
        .user()
        .find_unique(User::id().equals("user0".to_string()))
        .with(User::posts().fetch(vec![]))
        .exec()
        .await
        .unwrap()
        .unwrap();

    println!("User posts: {:?}", user_with_posts.posts().unwrap());

    let deleted_posts = client
        .post()
        .find_many(vec![])
        .delete()
        .exec()
        .await
        .unwrap();
    println!("Deleted {} posts", deleted_posts);

    let deleted_users_count = client
        .user()
        .find_many(vec![])
        .delete()
        .exec()
        .await
        .unwrap();
    println!("Deleted {} users", deleted_users_count);
}
