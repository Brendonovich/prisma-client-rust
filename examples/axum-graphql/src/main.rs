use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use graphql::schema::{build_schema, AppSchema};

#[allow(warnings, unused)]
pub mod db;
pub mod graphql;

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

// Note: This template uses Axum, but the bulk of the setup is for async_graphql. You should be able
// to easily swap out Axum for your preferred framework (e.g. Rocket, actix, etc).

#[tokio::main]
async fn main() {
    let schema = build_schema().await;

    let app = Router::new()
        // I prefer to prefix my graphql endpoint with /api, but use whatever you like.
        // just make sure it matches the path in graphql_playground()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .layer(Extension(schema));

    // macos Monterey i hate u so much for causing me so much headache to figure out
    // port 5000 is now taken??
    println!("Playground: http://localhost:5001/api/graphql");

    axum::Server::bind(&"0.0.0.0:5001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
