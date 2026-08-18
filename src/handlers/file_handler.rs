use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::services::file_service::FileService;
use crate::error::ServiceError;
use serde::Deserialize;

/// Payload for creating a file record.
#[derive(Debug, Deserialize)]
pub struct CreateFilePayload {
    pub filename: String,
    pub location: String,
}

/// GET /files
#[get("/files")]
async fn list_files(
    data: web::Data<sqlx::PgPool>,
) -> Result<impl Responder, ServiceError> {
    let service = FileService::new(data.get_ref().clone());
    let files = service.list_files().await?;
    Ok(HttpResponse::Ok().json(files))
}

/// GET /files/{id}
#[get("/files/{id}")]
async fn get_file(
    data: web::Data<sqlx::PgPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, ServiceError> {
    let service = FileService::new(data.get_ref().clone());
    let file = service.get_file(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(file))
}

/// POST /files
#[post("/files")]
async fn create_file(
    data: web::Data<sqlx::PgPool>,
    payload: web::Json<CreateFilePayload>,
) -> Result<impl Responder, ServiceError> {
    let service = FileService::new(data.get_ref().clone());
    let file = service
        .register_file(payload.filename.clone(), payload.location.clone())
        .await?;
    Ok(HttpResponse::Created().json(file))
}

/// PUT /files/{id}/process
#[put("/files/{id}/process")]
async fn process_file(
    data: web::Data<sqlx::PgPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, ServiceError> {
    let service = FileService::new(data.get_ref().clone());
    service.process_file(path.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_files)
        .service(get_file)
        .service(create_file)
        .service(process_file);
}