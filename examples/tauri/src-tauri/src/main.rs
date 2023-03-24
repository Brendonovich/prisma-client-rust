// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use db::*;
use serde::Deserialize;
use specta::{collect_types, Type};
use std::sync::Arc;
use tauri::State;
use tauri_specta::ts;

type DbState<'a> = State<'a, Arc<PrismaClient>>;

#[tauri::command]
#[specta::specta]
async fn get_posts(db: DbState<'_>) -> Result<Vec<post::Data>, ()> {
    db.post().find_many(vec![]).exec().await.map_err(|_| ())
}

#[derive(Deserialize, Type)]
struct CreatePostData {
    title: String,
    content: String,
}

#[tauri::command]
#[specta::specta]
async fn create_post(db: DbState<'_>, data: CreatePostData) -> Result<post::Data, ()> {
    db.post()
        .create(data.title, data.content, vec![])
        .exec()
        .await
        .map_err(|_| ())
}

#[tokio::main]
async fn main() {
    let db = PrismaClient::_builder().build().await.unwrap();

    #[cfg(debug_assertions)]
    ts::export(collect_types![get_posts, create_post], "../src/bindings.ts").unwrap();

    #[cfg(debug_assertions)]
    db._db_push().await.unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_posts, create_post,])
        .manage(Arc::new(db))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
