use std::sync::Arc;

use axum::{Router, middleware, routing::{delete, get, post, put}};

use crate::{database::mongodb::MongoRepo, handlers::project_handlers, middleware::auth_middleware::auth_guard};


pub fn project_route()->Router<Arc<MongoRepo>>{

    let public_routes=Router::new()
        .route("/list", get(project_handlers::list_projects))
        .route("/{id}", get(project_handlers::get_project));

    let admin_routes=Router::new()
        .route("/add", post(project_handlers::add_projects))
        .route("/{id}", put(project_handlers::update_project))    
        .route("/{id}", delete(project_handlers::delete_project))
        .layer(middleware::from_fn(auth_guard));
    Router::new().merge(public_routes).merge(admin_routes)


}