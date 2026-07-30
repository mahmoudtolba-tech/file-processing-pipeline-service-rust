use actix_web::{web, HttpResponse, Responder};
use crate::services::FileService;
use crate::errors::ServiceError;
use uuid::Uuid;
use serde::Deserialize;

#[derive(Deserialize)]
struct UploadPayload {
    filename: String,
    size_bytes: i64,
}

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/files", web::post().to(upload_file))
            .route("/files", web::get().to(list_files))
            .route("/files/{id}", web::get().to(get_file))
            .route("/files/{id}/process", web::post().to(process_file)),
    );
}

async fn upload_file(
    pool: web::Data<sqlx::PgPool>,
    payload: web::Json<UploadPayload>,
) -> Result<impl Responder, ServiceError> {
    let service = FileService::new(&pool);
    let record = service.upload(&payload.filename, payload.size_bytes).await?;
    Ok(HttpResponse::Created().json(record))
}

async fn get_file(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ServiceError> {
    let service = FileService::new(&pool);
    let record = service.get(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(record))
}

async fn list_files(pool: web::Data<sqlx::PgPool>) -> Result<impl Responder, ServiceError> {
    let service = FileService::new(&pool);
    let records = service.list().await?;
    Ok(HttpResponse::Ok().json(records))
}

async fn process_file(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ServiceError> {
    let service = FileService::new(&pool);
    service.process(path.into_inner()).await?;
    Ok(HttpResponse::Ok().body("File processed"))
}