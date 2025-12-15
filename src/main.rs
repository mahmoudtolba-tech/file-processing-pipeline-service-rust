use std::env;
use dotenv::dotenv;

mod router;
mod models;
mod repositories;
mod services;
mod controllers;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://user:password@localhost/database".to_string());
    
    log::info!("Server started on port {}", port);
    router::start_router(port.parse().unwrap(), db_url).await;
}