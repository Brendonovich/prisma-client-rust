use async_graphql::{Context, Object, Result};

use crate::{
    graphql::types::User,
    db::{user, PrismaClient},
};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .user()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect())
    }

    async fn get_user(&self, ctx: &Context<'_>, id: String) -> Result<Option<User>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .user()
            .find_unique(user::id::equals(id))
            .exec()
            .await?
            .map(|u| u.into()))
    }
}
