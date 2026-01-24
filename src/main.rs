use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use dotenv::dotenv;

mod controllers;
mod models;
mod services;
mod repositories;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(controllers::file_controller::file_pipeline)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#### models.rs