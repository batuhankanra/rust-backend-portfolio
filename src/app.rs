use axum::http::{Method};
use axum::{Router, middleware};
use tower_http::cors::{CorsLayer};
use std::net::SocketAddr;
use std::sync::Arc;

use crate::database::mongodb::MongoRepo;
use crate::middleware::logger::logger_middleware;
use crate::routes::{create_router};
use crate::config::env::Config;

pub async fn run() {
    let config = Config::init();
    


    let db = MongoRepo::init().await;
    let app_state = Arc::new(db);

    let cors =CorsLayer::new()
        .allow_methods([Method::GET,Method::POST,Method::PUT,Method::DELETE])
        .allow_origin("http://localhost:3000".parse::<axum::http::HeaderValue>().unwrap())
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .nest("/api", create_router(Arc::clone(&app_state)))
        .layer(middleware::from_fn(logger_middleware)) 
        .layer(cors); 

    let addr_str = format!("127.0.0.1:{}", config.server_port);
    let addr: SocketAddr = addr_str.parse().expect("Geçersiz adres!");

    println!("\x1b[1;37;44m 🚀 SERVER RUNNING \x1b[0m \x1b[34m http://{}\x1b[0m", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}