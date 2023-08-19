use crate::{db::*, utils::*};

#[tokio::test]
async fn _count() -> TestResult {
    let client = client().await;

    for index in 1..=3 {
        let user = client
            .user()
            .create(format!("User {index}"), vec![])
            .exec()
            .await?;

        client
            .post()
            .create_many(
                (1..=index)
                    .map(|post_index| {
                        post::create_unchecked(
                            format!("User {index} Post {post_index}"),
                            false,
                            vec![post::author_id::set(Some(user.id.clone()))],
                        )
                    })
                    .collect(),
            )
            .exec()
            .await?;
    }

    let users_post_count_asc = client
        .user()
        .find_many(vec![])
        .order_by(user::posts::order(vec![post::_count::order(
            SortOrder::Asc,
        )]))
        .exec()
        .await?;
    assert_eq!(users_post_count_asc[0].name.as_str(), "User 1");

    let users_post_count_desc = client
        .user()
        .find_many(vec![])
        .order_by(user::posts::order(vec![post::_count::order(
            SortOrder::Desc,
        )]))
        .exec()
        .await?;
    assert_eq!(users_post_count_desc[0].name.as_str(), "User 3");

    cleanup(client).await
}

#[tokio::test]
async fn relation() -> TestResult {
    let client = client().await;

    for index in 1..=3 {
        let user = client
            .user()
            .create(format!("User {index}"), vec![])
            .exec()
            .await?;

        client
            .post()
            .create(
                format!("Some Post {index}"),
                false,
                vec![post::author::connect(user::id::equals(user.id))],
            )
            .exec()
            .await?;
    }

    let posts_users_asc = client
        .post()
        .find_many(vec![])
        .order_by(post::author::order(vec![user::name::order(SortOrder::Asc)]))
        .exec()
        .await?;
    assert_eq!(posts_users_asc[0].title.as_str(), "Some Post 1");

    let posts_users_desc = client
        .post()
        .find_many(vec![])
        .order_by(post::author::order(vec![user::name::order(
            SortOrder::Desc,
        )]))
        .exec()
        .await?;
    assert_eq!(posts_users_desc[0].title.as_str(), "Some Post 3");

    cleanup(client).await
}
