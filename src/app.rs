use axum::Router;
use std::net::SocketAddr;
use crate::routes::project_routes;


pub async fn run() {
    let app=Router::new().nest("/api", project_routes::static_routes());
    let addr=SocketAddr::from(([127,0,0,1],3000));
    println!("sunucu ayağa kalktı: http://{} ",addr);
    let listener=tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener,app).await.unwrap();

}