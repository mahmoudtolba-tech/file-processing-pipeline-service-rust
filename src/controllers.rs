use super::models;
use super::services;
use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct ProcessFileRequest {
    file_path: String,
}

pub async fn process_file(
    req: web::Json<ProcessFileRequest>,
) -> impl Responder {
    let result = services::process_file(req.file_path.clone()).await;
    match result {
        Ok(_) => HttpResponse::Ok().json("File processed successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to process file"),
    }
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("Service is up and running")
}