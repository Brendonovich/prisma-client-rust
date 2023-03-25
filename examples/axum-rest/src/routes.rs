use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Extension, Router,
};
use prisma_client_rust::{
    prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation},
    QueryError,
};
use std::sync::Arc;

use serde::Deserialize;

use crate::db::*;

type Database = Extension<Arc<PrismaClient>>;
type AppResult<T> = Result<T, AppError>;
type AppJsonResult<T> = AppResult<Json<T>>;

// Define all your requests schema
#[derive(Deserialize)]
struct UserRequest {
    username: String,
    email: String,
}

#[derive(Deserialize)]
struct CommentRequest {
    user: i32,
    message: String,
}

/*

/api/user => GET, POST
/api/user/:username => PUT, DELETE
/api/comment => POST

*/
pub fn create_route() -> Router {
    Router::new()
        .route("/user", get(handle_user_get).post(handle_user_post))
        .route(
            "/user/:username",
            put(handle_user_put).delete(handle_user_delete),
        )
        .route("/comment", post(handle_comment_post))
}

async fn handle_user_get(db: Database) -> AppJsonResult<Vec<user::Data>> {
    let users = db
        .user()
        .find_many(vec![])
        .with(user::comments::fetch(vec![]))
        .exec()
        .await?;

    Ok(Json::from(users))
}

async fn handle_user_post(
    db: Database,
    Json(input): Json<UserRequest>,
) -> AppJsonResult<user::Data> {
    let data = db
        .user()
        .create(input.username, input.email, vec![])
        .exec()
        .await?;

    Ok(Json::from(data))
}

async fn handle_user_put(
    db: Database,
    Path(username): Path<String>,
    Json(input): Json<UserRequest>,
) -> AppJsonResult<user::Data> {
    let updated_user = db
        .user()
        .update(
            user::username::equals(username),
            vec![
                user::username::set(input.username),
                user::email::set(input.email),
            ],
        )
        .exec()
        .await?;

    Ok(Json::from(updated_user))
}

async fn handle_user_delete(db: Database, Path(username): Path<String>) -> AppResult<StatusCode> {
    db.user()
        .delete(user::username::equals(username))
        .exec()
        .await?;

    Ok(StatusCode::OK)
}

async fn handle_comment_post(
    db: Database,
    Json(req): Json<CommentRequest>,
) -> AppJsonResult<comments::Data> {
    let comment = db
        .comments()
        .create(req.message, user::id::equals(req.user), vec![])
        .exec()
        .await?;

    Ok(Json::from(comment))
}

enum AppError {
    PrismaError(QueryError),
    NotFound,
}

impl From<QueryError> for AppError {
    fn from(error: QueryError) -> Self {
        match error {
            e if e.is_prisma_error::<RecordNotFound>() => AppError::NotFound,
            e => AppError::PrismaError(e),
        }
    }
}

// This centralizes all different errors from our app in one place
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::PrismaError(error) if error.is_prisma_error::<UniqueKeyViolation>() => {
                StatusCode::CONFLICT
            }
            AppError::PrismaError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
        };

        status.into_response()
    }
}
