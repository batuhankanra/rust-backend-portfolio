use axum::Router;
use std::sync::Arc;
use crate::{database::mongodb::MongoRepo, routes::auth_routes::auth_route};



pub mod project_routes;
pub mod auth_routes;


pub fn create_router(app_state:Arc<MongoRepo>)->Router{
    Router::new()
        .nest("/project", project_routes::project_route())
        .nest("/auth", auth_route())
        .with_state(app_state)
}
