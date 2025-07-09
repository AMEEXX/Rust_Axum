use axum::extract::Path;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::get_service;
use axum::{Router, response::Html};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
mod error;
#[tokio::main]
async fn main() {
    // Build our application with a route
    let route_all = Router::new()
        .merge(router_hello())
        .fallback_service(route_static()); // Only works if router_api is a `Router`, not a function call

    // Set the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->>> Listening on {addr}\n");

    // Run the server using axum's serve function
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, route_all).await.unwrap();
}
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
// for handling ?name=arpita  query is destructing parameter like a machine extracts the name and puts in the struct

async fn hello2(Path(new): Path<String>) -> impl IntoResponse {
    Html(format!("Hello <strong> {new}</strong"))
}
fn route_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
fn router_hello() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/user", get(user_handler))
        .route("/", get(defualt_router))
        .route("/hello2/:new", get(hello2))
}

async fn hello_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}!! </strong>"))
}
async fn user_handler() -> impl IntoResponse {
    Html("user are yoiu arpita ")
}
async fn defualt_router() -> impl IntoResponse {
    Html("how are you arpita")
}
