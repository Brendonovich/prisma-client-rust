use actix_web::{App, HttpServer, get, post, HttpResponse, Responder, web};
use serde::Deserialize;

mod prisma;
use prisma::PrismaClient;
use prisma::{user, post};

extern crate dotenv;

use dotenv::dotenv;

#[get("/users")]
async fn get_users(client: web::Data<PrismaClient>) -> impl Responder {
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
async fn create_user(client: web::Data<PrismaClient>, body: web::Json<CreateUserRequest>) -> impl Responder {
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
async fn get_posts(client: web::Data<PrismaClient>) -> impl Responder {
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
async fn create_post(client: web::Data<PrismaClient>, body: web::Json<CreatePostRequest>) -> impl Responder {
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
    let client = web::Data::new(prisma::new_client().await.unwrap());

    HttpServer::new(move || {
            App::new()
                .app_data(client.clone())
                .service(get_users)
                .service(create_user)
                .service(get_posts)
                .service(create_post)
        })
        .bind(("127.0.0.1", 3001))?
        .run()
        .await
}
