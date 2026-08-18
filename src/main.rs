use actix_web::{App, HttpServer, web};
use tracing_subscriber::{fmt, EnvFilter};

mod config;
mod error;
mod handlers;
mod models;
mod repositories;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(env_filter).init();

    // Load configuration
    let cfg = config::Config::from_env().expect("Failed to load configuration");

    // Create DB pool
    let db_pool = sqlx::PgPool::connect(&cfg.database_url)
        .await
        .expect("Failed to create DB pool");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(handlers::init_routes)
    })
    .bind((cfg.host.as_str(), cfg.port))?
    .run()
    .await
}