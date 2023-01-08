use crate::db::*;
use rspc::internal::specta;

#[test]
fn ts_export() {
    let ts = specta::ts_export::<user::Data>().unwrap();

    assert_eq!(ts, "export interface User { id: string, name: string, email: string | null, createdAt: string, underscored_: number | null }");

    user::include!(user_include { posts });

    let ts = specta::ts_export::<user_include::Data>().unwrap();

    assert_eq!(ts, "export interface UserInclude { id: string, name: string, email: string | null, createdAt: string, underscored_: number | null, posts: Array<Post> }");

    user::select!(user_select { id posts });

    let ts = specta::ts_export::<user_select::Data>().unwrap();

    assert_eq!(
        ts,
        "export interface UserSelect { id: string, posts: Array<Post> }"
    );
}
