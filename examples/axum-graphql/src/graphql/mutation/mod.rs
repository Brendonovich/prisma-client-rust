pub mod post;
pub mod user;

pub use post::PostMutation;
pub use user::UserMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(PostMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(PostMutation, UserMutation);
