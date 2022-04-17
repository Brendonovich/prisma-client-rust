use crate::db::*;

pub mod db;

#[tokio::main]
pub async fn main() {
    let client = db::new_client().await.unwrap();

    let user = client
        .user()
        .create(
            user::id::set("user0".to_string()),
            user::display_name::set("User 0".to_string()),
            vec![],
        )
        .exec()
        .await
        .unwrap();

    let post = client
        .post()
        .create(
            post::id::set("post0".to_string()),
            post::content::set("Some post content".to_string()),
            post::user::link(user::id::equals(user.id.to_string())),
            vec![],
        )
        .exec()
        .await
        .unwrap();

    println!("User: {:?}", user);
    println!("Post: {:?}", post);

    let post_with_user = client
        .post()
        .find_unique(post::id::equals("post0".to_string()))
        .with(post::user::fetch())
        .exec()
        .await
        .unwrap()
        .unwrap();

    println!("Post user: {:?}", post_with_user.user().unwrap());

    let user_with_posts = client
        .user()
        .find_unique(user::id::equals("user0".to_string()))
        .with(user::posts::fetch(vec![]))
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
