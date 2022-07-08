use actix_web::{App, HttpServer, get, post, HttpResponse, Responder, web};
use serde::Deserialize;

mod prisma;
use prisma::{user, post};

extern crate dotenv;

use dotenv::dotenv;

#[get("/users")]
async fn get_users() -> impl Responder {
    let client = prisma::new_client().await.unwrap();

    let users = client
        .user()
        .find_many(vec![])
        .exec()
        .await
        .unwrap();

    HttpResponse::Ok().json(users)
}

#[derive(Deserialize)]
struct CreateUserRequest {
    display_name: String,
}

#[post("/user")]
async fn create_user(body: web::Json<CreateUserRequest>) -> impl Responder {
    let client = prisma::new_client().await.unwrap();

    let user = client
        .user()
        .create(
            user::display_name::set(body.display_name.to_string()),
            vec![]
        )
        .exec()
        .await
        .unwrap();

    HttpResponse::Ok().json(user)
}

#[get("/posts")]
async fn get_posts() -> impl Responder {
    let client = prisma::new_client().await.unwrap();

    let posts = client
        .post()
        .find_many(vec![])
        .exec()
        .await
        .unwrap();

    HttpResponse::Ok().json(posts)
}

#[derive(Deserialize)]
struct CreatePostRequest {
    content: String,
    user_id: String,
}

#[post("/post")]
async fn create_post(body: web::Json<CreatePostRequest>) -> impl Responder {
    let client = prisma::new_client().await.unwrap();

    let post = client
        .post()
        .create(
            post::content::set(body.content.to_string()),
            post::user::link(
                user::id::equals(body.user_id.to_string())
            ),
            vec![]
        )
        .exec()
        .await
        .unwrap();

    HttpResponse::Ok().json(post)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
            App::new()
                .service(get_users)
                .service(create_user)
                .service(get_posts)
                .service(create_post)
        })
        .bind(("127.0.0.1", 3001))?
        .run()
        .await
}
