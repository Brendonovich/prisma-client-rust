pub mod post;
pub mod user;

pub use post::PostQuery;
pub use user::UserQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(PostQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(PostQuery, UserQuery);
