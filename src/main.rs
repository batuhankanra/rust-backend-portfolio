mod routes;
mod app;
mod  middleware;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    app::run().await
}