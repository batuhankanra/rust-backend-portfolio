use axum::{routing::get,Router};


pub fn static_routes()->Router{
    Router::new()
     .route("/", get(|| async {"portfolio backend"}))
     .route("/api", get(|| async {"OK"}))
}