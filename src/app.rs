use axum::{Router,middleware};
use std::net::SocketAddr;
use crate::middleware::logger_middleware;

pub async fn run() {
    let app=Router::new()
        .nest("/api", crate::routes::project_routes::static_routes())
        .layer(middleware::from_fn(logger_middleware))
    ;
    let addr=SocketAddr::from(([127,0,0,1],3000));
    println!("server is running: http://{}",addr);
    let listener=tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener,app).await.unwrap();
}