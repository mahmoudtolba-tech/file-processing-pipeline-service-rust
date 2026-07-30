use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod repositories;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    // Load .env
    dotenvy::dotenv().ok();

    // Read configuration
    let cfg = config::Config::from_env().expect("Failed to load configuration");

    // Create DB pool
    let pool = PgPoolOptions::new()
        .max_connections(cfg.db.max_connections)
        .connect(&cfg.db.url)
        .await
        .expect("Failed to create DB pool");

    // Run migrations
    db::run_migrations(&pool).await.expect("Migrations failed");

    // Build server
    info!("Starting server at http://{}", cfg.server.bind_addr);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(handlers::router)
    })
    .bind(cfg.server.bind_addr)?
    .run()
    .await
}