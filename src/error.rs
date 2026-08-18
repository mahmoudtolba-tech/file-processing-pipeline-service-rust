use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use validator::ValidationErrors;

/// Central error type for the service.
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),

    #[error("Not found")]
    NotFound,

    #[error("Internal server error")]
    InternalError,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::DbError(_) => {
                HttpResponse::InternalServerError().json("Database error")
            }
            ServiceError::ValidationError(e) => {
                HttpResponse::BadRequest().json(e)
            }
            ServiceError::NotFound => HttpResponse::NotFound().finish(),
            ServiceError::InternalError => {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}