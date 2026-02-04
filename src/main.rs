mod routes;
mod app;
mod  middleware;
mod config;
mod database;
mod models;
mod repositories;
mod handlers;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    app::run().await
}