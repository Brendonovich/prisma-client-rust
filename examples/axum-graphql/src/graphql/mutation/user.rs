use async_graphql::{Context, InputObject, Object, Result};

use crate::{
    graphql::types::User,
    db::{user, PrismaClient},
};

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

        let created = db
            .user()
            .create(user::display_name::set(input.display_name), vec![])
            .exec()
            .await?;

        Ok(created.into())
    }
}
