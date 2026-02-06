use std::sync::Arc;
use axum::{Router,routing::{ post}};

use crate::{database::mongodb::MongoRepo, handlers::auth_handlers};



pub fn auth_route()->Router<Arc<MongoRepo>>{
    Router::new()
        .route("/register", post(auth_handlers::register))
        .route("/login", post(auth_handlers::login_handlers))
}