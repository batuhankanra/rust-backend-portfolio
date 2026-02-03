mod routes;
mod app;
mod  middleware;
mod config;
mod database;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    app::run().await
}