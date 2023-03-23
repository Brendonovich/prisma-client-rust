use axum::{extract::Extension, Router};
use std::sync::Arc;

use crate::db::*;

pub mod db;
pub mod routes;

#[tokio::main]
async fn main() {
    let prisma_client = Arc::new(PrismaClient::_builder().build().await.unwrap());

    #[cfg(debug_assertions)]
    prisma_client._db_push().await.unwrap();

    let app = Router::new()
        .nest("/api", routes::create_route())
        .layer(Extension(prisma_client));

    println!("Example Prisma x Axum running on http://localhost:5000/api");

    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
