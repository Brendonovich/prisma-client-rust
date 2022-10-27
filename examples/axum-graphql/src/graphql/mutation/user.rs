use async_graphql::{Context, InputObject, Object, Result};

use crate::{db::PrismaClient, graphql::types::User};

#[derive(InputObject)]
pub struct CreateUserInput {
    pub display_name: String,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let created = db.user().create(input.display_name, vec![]).exec().await?;

        Ok(created.into())
    }
}
