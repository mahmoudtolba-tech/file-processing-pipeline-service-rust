use actix_web::{ResponseError, HttpResponse};
use thiserror::Error;
use std::fmt::{self, Display};

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("Not found")]
    NotFound,

    #[error("Internal server error")]
    Internal,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::Validation(msg) => HttpResponse::BadRequest().body(msg.clone()),
            ServiceError::NotFound => HttpResponse::NotFound().finish(),
            ServiceError::Db(_) => HttpResponse::InternalServerError().finish(),
            ServiceError::Internal => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
}