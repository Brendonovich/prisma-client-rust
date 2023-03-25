use async_graphql::{Context, Object, Result};

use crate::{db::PrismaClient, graphql::types::Post};

#[derive(Default)]
pub struct PostQuery;

#[Object]
impl PostQuery {
    async fn get_posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .post()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}
