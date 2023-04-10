use std::sync::Arc;

use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;

#[allow(warnings, unused)]
pub mod db;

use db::{post, user};

// This is the struct that will hold the prisma client. This will be managed by
// Rocket, accessible in the routing functions using the type alias `Ctx` below.
// See https://rocket.rs/v0.5-rc/guide/state/
#[derive(Clone)]
pub struct Context {
    pub db: Arc<db::PrismaClient>,
}

// Type alias not required, just personal preference for this in particular so I
// don't have to write `rocket::State<Context>` every single time.
pub type Ctx = rocket::State<Context>;

/// Get all posts
#[get("/posts")]
async fn get_posts(ctx: &Ctx) -> Json<Vec<post::Data>> {
    // Note: you should add some error handling :)
    Json(ctx.db.post().find_many(vec![]).exec().await.unwrap())
}

/// Get all users. Simple demonstration for basic route params.
#[get("/users?<load_posts>")]
async fn get_users(ctx: &Ctx, load_posts: Option<bool>) -> Json<Vec<user::Data>> {
    let mut query = ctx.db.user().find_many(vec![]);

    if load_posts.unwrap_or(true) {
        query = query.with(user::posts::fetch(vec![]));
    }

    // Note: you should add some error handling :)
    Json(query.exec().await.unwrap())
}

#[launch]
async fn rocket() -> _ {
    let db = Arc::new(
        db::new_client()
            .await
            .expect("Failed to create Prisma client"),
    );

    #[cfg(debug_assert)]
    db._db_push(false).await.unwrap();

    rocket::build()
        .manage(Context { db })
        .mount("/api", routes![get_posts, get_users])
}
