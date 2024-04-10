use axum::{routing::get, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod api;

#[derive(OpenApi)]
#[openapi(paths(api::user), components(schemas(api::User)))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .merge(SwaggerUi::new("/").url("/api-docs/openapi.json", ApiDoc::openapi()))
        //.route("/item", get(item()))
        .route("/user/:id", get(api::user))
        .route("/story/:id", get(api::story))
        .route("/comment/:id", get(api::comment))
        .route("/topstories", get(api::topstories))
        .route("/cache_top", get(api::cache));
    //.route("/newstories", get(newstories()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
