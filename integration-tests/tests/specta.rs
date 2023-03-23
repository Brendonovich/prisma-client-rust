use crate::db::*;
use specta;

#[test]
fn ts_export() {
    let ts = specta::ts::export::<user::Data>(&Default::default()).unwrap();

    assert_eq!(ts, "export type User = { id: string; name: string; email: string | null; createdAt: string; underscored_: number | null }");

    user::include!(user_include { posts });

    let ts = specta::ts::export::<user_include::Data>(&Default::default()).unwrap();

    assert_eq!(ts, "export type UserInclude = { id: string; name: string; email: string | null; createdAt: string; underscored_: number | null; posts: Post[] }");

    user::select!(user_select { id posts });

    let ts = specta::ts::export::<user_select::Data>(&Default::default()).unwrap();

    assert_eq!(ts, "export type UserSelect = { id: string; posts: Post[] }");
}
