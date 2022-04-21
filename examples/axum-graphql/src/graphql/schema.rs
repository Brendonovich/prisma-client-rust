use async_graphql::{EmptySubscription, Schema};

use crate::{
    graphql::{mutation::Mutation, query::Query},
    db,
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the PrismaClient to the context
pub async fn build_schema() -> AppSchema {
    let db = db::new_client()
        .await
        .expect("Failed to create Prisma client");

    // For more information about schema data, see: https://async-graphql.github.io/async-graphql/en/context.html#schema-data
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish()
}
