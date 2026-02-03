use std::sync::Arc;

use axum::{routing::get,Router};

use crate::database::mongodb::MongoRepo;


pub fn static_routes()->Router<Arc<MongoRepo>>{
    Router::new().route("/hello-world",get(|| async {"Portfolio backend:hello world"}))
}