use crate::{
    db::{Post, User},
    utils::*,
};

#[tokio::test]
async fn delete() -> TestResult {
    let client = client().await;
    
    let author = client
        .user()
        .create(
            User::name().set("Brendan".to_string()),
            vec![]
        )
        .exec()
        .await?;
        
    let post = client
        .post()
        .create(
            Post::title().set("Hi from Prisma!".to_string()),
            Post::published().set(false),
            vec![Post::author_id().set(Some(author.id.clone()))],
        )
        .with(Post::author().fetch())
        .exec()
        .await?;
    assert_eq!(post.title, "Hi from Prisma!");
    let author = post.author().unwrap().unwrap();
    assert_eq!(author.name, "Brendan");
    
    let deleted = client
        .post()
        .find_unique(Post::id().equals(post.id.clone()))
        .delete()
        .with(Post::author().fetch())
        .exec()
        .await?.unwrap();
    assert_eq!(deleted.title, "Hi from Prisma!");
    let author = deleted.author().unwrap().unwrap();
    assert_eq!(author.name, "Brendan");
    
    let found = client.post().find_unique(Post::id().equals(post.id.clone())).exec().await?;
    assert!(found.is_none());
    
    let user = client.user().find_unique(User::id().equals(author.id.clone())).exec().await?;
    assert_eq!(user.unwrap().name, "Brendan");
    
    cleanup(client).await
}

#[tokio::test]
async fn delete_record_not_found() -> TestResult {
    let client = client().await;
    
    let deleted = client
        .post()
        .find_unique(Post::id().equals("sdlfskdf".to_string()))
        .delete()
        .exec()
        .await?;
    assert!(deleted.is_none());
    
    cleanup(client).await
}