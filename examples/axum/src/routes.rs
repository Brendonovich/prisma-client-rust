use axum::{
    extract::{Json, Path},
    routing::{get, put, post},
    response::{IntoResponse, Response},
    http::StatusCode,
    Extension,
    Router,
};
use prisma_client_rust::{
    prisma_errors::query_engine::UniqueKeyViolation,
    Error,
    error_is_type,
};

use serde::Deserialize;

use crate::db::{self, user, comments};

type Database = Extension<std::sync::Arc<db::PrismaClient>>;
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
/comment => POST

*/
pub fn create_route() -> Router {
    Router::new()
        .route(
            "/user",
            get(handle_user_get)
            .post(handle_user_post)
        )
        .route(
            "/user/:username",
            put(handle_user_put)
            .delete(handle_user_delete),
        )
        .route("/comment", post(handle_comment_post))
}

async fn handle_user_get(db: Database) -> AppResult<Json<Vec<user::Data>>> {
    let users = db.user()
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
    let data = db.user()
        .create(
            user::username::set(input.username),
            user::email::set(input.email),
            vec![],
        )
        .exec()
        .await?;

    Ok(Json::from(data))
}

async fn handle_user_put(
    db: Database,
    Path(username): Path<String>,
    Json(input): Json<UserRequest>,
) -> AppJsonResult<user::Data> {
    let result = db.user()
        .find_unique(user::username::equals(username))
        .update(vec![
            user::username::set(input.username),
            user::email::set(input.email),
        ])
        .exec()
        .await?;
    
    match result {
        Some(updated_user) => Ok(Json::from(updated_user)),
        _ => Err(AppError::NotFound)
    }
}

async fn handle_user_delete(
    db: Database,
    Path(username): Path<String>,
) -> AppResult<StatusCode> {
    db.user()
        .find_unique(user::username::equals(username))
        .delete()
        .exec()
        .await?;
    
    Ok(StatusCode::OK)
}

async fn handle_comment_post(
    db: Database,
    Json(req): Json<CommentRequest>,
) -> AppJsonResult<comments::Data> {
    let comment = db.comments()
        .create(
            comments::message::set(req.message),
            comments::author::link(user::UniqueWhereParam::IdEquals(req.user)),
            vec![]
        )
        .exec()
        .await?;
    
    Ok(Json::from(comment))
}

enum AppError {
    PrismaError(Error),
    NotFound,
}

impl From<Error> for AppError {
    fn from(inner: Error) -> Self {
        AppError::PrismaError(inner)
    }
}

// This centralizes all differents errors from our app in one place
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::PrismaError(Error::Execute(prisma_err)) => {
                if error_is_type::<UniqueKeyViolation>(&prisma_err) {
                    StatusCode::CONFLICT
                } else {
                    StatusCode::BAD_REQUEST
                }
            },
            AppError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        };

        status.into_response()
    }
}
