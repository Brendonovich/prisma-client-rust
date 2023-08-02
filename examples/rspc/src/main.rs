use std::sync::Arc;

use rspc::{ExportConfig, Rspc};

#[allow(warnings, unused)]
mod db;

type Ctx = Arc<db::PrismaClient>;

const R: Rspc<Ctx> = Rspc::new();

db::user::include!(named_user_include { posts });

#[tokio::main]
async fn main() {
    let client = db::new_client().await.unwrap();

    #[cfg(debug_assertions)]
    client._db_push().await.unwrap();

    // A router doesn't do anything on its own, you need to use it with
    // an integration to make it do anything: https://rspc.dev/

    let router = R
        .router()
        .procedure(
            "users",
            R.query(|db, _: ()| async move {
                db.user().find_many(vec![]).exec().await.map_err(Into::into)
            }),
        )
        .procedure(
            "userNames",
            R.query(|db, _: ()| async move {
                db.user()
                    .find_many(vec![])
                    .select(db::user::select!({ display_name }))
                    .exec()
                    .await
                    .map_err(Into::into)
            }),
        )
        .procedure(
            "usersWithPosts",
            R.query(|db, _: ()| async move {
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
            }),
        )
        .procedure(
            "namedUserType",
            R.query(|db, _: ()| async move {
                db.user()
                    .find_many(vec![])
                    .include(named_user_include::include())
                    .exec()
                    .await
                    .map_err(Into::into)
            }),
        )
        .procedure(
            "posts",
            R.query(|db, _: ()| async move {
                db.post().find_many(vec![]).exec().await.map_err(Into::into)
            }),
        )
        .build()
        .unwrap();

    router.export_ts(ExportConfig::new("./bindings.ts")).ok();
}
