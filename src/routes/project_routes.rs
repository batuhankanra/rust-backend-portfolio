use std::sync::Arc;

use axum::{Router, routing::{delete, get, post, put}};

use crate::{database::mongodb::MongoRepo, handlers::project_handlers};


pub fn static_routes()->Router<Arc<MongoRepo>>{
    Router::new()
        .route("/hello-world",get(|| async {"Portfolio backend:hello world"}))
        .route("/list", get(project_handlers::list_projects))
        .route("/add", post(project_handlers::add_projects))
        .route("/{id}", get(project_handlers::get_project))
        .route("/{id}", put(project_handlers::update_project))    
        .route("/{id}", delete(project_handlers::delete_project))
}