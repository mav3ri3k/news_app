#![cfg_attr(doc_cfg, feature(doc_cfg))]
//! Implementation of Rest API for a sudo news app
//! Written in rust -> To start you need cargo toolchain install
//! Start in dev mode using command `cargo run`
//! It will start server at `localhost:3000'
//! Refer to `localhost:3000/docs' for API documentation

use axum::{routing::get, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// Backend handler and schema definitions
pub mod api;

#[derive(OpenApi)]
#[openapi(paths(api::user,
                api::story,
                api::comment,
                api::topstories,
                api::cache,
                api::search_story,
                ), components(schemas(api::User, api::Story, api::Comment, api::TopStories, api::SearchResults)))]
/// Helper to use OpenApi
struct ApiDoc;

#[tokio::main]
/// Entry point for the server and routes
async fn main() {
    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/user/:id", get(api::user))
        .route("/story/:id", get(api::story))
        .route("/comment/:id", get(api::comment))
        .route("/topstories", get(api::topstories))
        .route("/cache_top", get(api::cache))
        .route("/search/story/:word", get(api::search_story));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
