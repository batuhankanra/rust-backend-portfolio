use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::{database::mongodb::MongoRepo, handlers::project_handlers};


pub fn static_routes()->Router<Arc<MongoRepo>>{
    Router::new()
        .route("/hello-world",get(|| async {"Portfolio backend:hello world"}))
        .route("/list", get(project_handlers::list_projects))
        .route("/add", post(project_handlers::add_projects))
    
}