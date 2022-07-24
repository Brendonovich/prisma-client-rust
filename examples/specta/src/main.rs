use std::future::Future;

use prisma_client_rust::specta;

mod db;

fn ts_export_fn<Ret, Out>(_: impl Fn() -> Ret)
where
    Ret: Future<Output = Out>,
    Out: specta::Type,
{
    println!("{:?}", specta::ts_definition::<Out>());
}

fn main() {
    ts_export_fn(|| async {
        let db = db::new_client().await.unwrap();

        db.user()
            .find_many(vec![])
            .exec()
            .await
            .unwrap()
    });
    ts_export_fn(|| async {
        let db = db::new_client().await.unwrap();

        db.user()
            .find_many(vec![])
            .select(db::user::select!(id))
            .exec()
            .await
            .unwrap()
    });
}
