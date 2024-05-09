// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[allow(warnings, unused)]
mod db;

use db::*;
use serde::Deserialize;
use specta::Type;
use std::sync::Arc;
use tauri::State;
use tauri_specta::{collect_commands, ts};

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

    let invoke_handler = {
        let builder = ts::builder().commands(collect_commands![get_posts, create_post]);

        #[cfg(debug_assertions)]
        let builder = builder.path("../src/bindings.ts");

        builder.build().unwrap()
    };

    #[cfg(debug_assertions)]
    db._db_push().await.unwrap();

    tauri::Builder::default()
        .invoke_handler(invoke_handler)
        .manage(Arc::new(db))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
