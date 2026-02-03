use axum::{Router, middleware};
use std::net::SocketAddr;
use std::sync::Arc;

use crate::database::mongodb::MongoRepo;
use crate::middleware::logger_middleware;
use crate::routes::project_routes;
use crate::config::env::Config;

pub async fn run() {
    let config = Config::init();
    
    // Veritabanını başlat ve State olarak hazırla
    let db = MongoRepo::init().await;
    let app_state = Arc::new(db);

    // Uygulama rotaları ve katmanları
    let app = Router::new()
        .nest("/api", project_routes::static_routes())
        .layer(middleware::from_fn(logger_middleware)) // Renkli log middleware'imiz
        .with_state(app_state); // Veritabanını tüm handler'lara enjekte et

    let addr_str = format!("127.0.0.1:{}", config.server_port);
    let addr: SocketAddr = addr_str.parse().expect("Geçersiz adres!");

    println!("\x1b[1;37;44m 🚀 SERVER RUNNING \x1b[0m \x1b[34m http://{}\x1b[0m", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}