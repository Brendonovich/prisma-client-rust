use async_graphql::{Context, InputObject, Object, Result};

use crate::{
    db::{user, PrismaClient},
    graphql::types::Post,
};

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreatePostInput {
    pub content: String,
    // this really would be grabbed from session or something but just for demo
    pub user_id: String,
}

#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    pub async fn create_post(&self, ctx: &Context<'_>, input: CreatePostInput) -> Result<Post> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let created = db
            .post()
            .create(input.content, user::id::equals(input.user_id), vec![])
            .exec()
            .await?;

        Ok(created.into())
    }
}
