use std::sync::Arc;

use rspc::{Config, Router};

#[allow(warnings, unused)]
mod db;

type Ctx = Arc<db::PrismaClient>;

db::user::include!(named_user_include { posts });

#[tokio::main]
async fn main() {
    let client = db::new_client().await.unwrap();

    #[cfg(debug_assertions)]
    client._db_push().await.unwrap();

    // A router doesn't do anything on its own, you need to use it with
    // an integration to make it do anything: https://rspc.dev/

    Router::<Ctx>::new()
        .config(Config::new().export_ts_bindings("./bindings.ts"))
        .query("users", |t| t(|db, _: ()| async move {
            db.user().find_many(vec![]).exec().await.map_err(Into::into)
        }))
        .query("userNames", |t| t(|db, _: ()| async move {
            db.user()
                .find_many(vec![])
                .select(db::user::select!({ display_name }))
                .exec()
                .await
                .map_err(Into::into)
        }))
        .query("usersWithPosts", |t| t(|db, _: ()| async move {
            db.user()
                .find_many(vec![])
                .include(db::user::include!({
                    posts(vec![]).skip(1): select {
                        id
                        content
                        user: select {
                            id
                            display_name
                        }
                    }
                }))
                .exec()
                .await
                .map_err(Into::into)
        }))
        .query("namedUserType", |t| t(|db, _: ()| async move {
            db.user()
                .find_many(vec![])
                .include(named_user_include::include())
                .exec()
                .await
                .map_err(Into::into)
        }))
        .query("posts", |t| t(|db, _: ()| async move {
            db.post().find_many(vec![]).exec().await.map_err(Into::into)
        }))
        .build();
}
