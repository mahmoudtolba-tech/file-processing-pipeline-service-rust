use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger::init;
use std::env;

mod controllers;
mod models;
mod repositories;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    init();
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/process_file").route(web::post().to(controllers::process_file)))
            .service(web::resource("/health_check").route(web::get().to(controllers::health_check)))
    })
    .bind((host, port.parse().unwrap()))?
    .run()
    .await
}