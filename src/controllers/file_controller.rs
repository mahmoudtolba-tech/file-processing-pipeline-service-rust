use actix_web::{web, HttpResponse, Responder};
use crate::models::File;
use crate::services::FileService;
use crate::repositories::FileRepository;
use sqlx::PgPool;

pub async fn file_pipeline() -> impl Responder {
    let pool = PgPool::connect("postgres://user:password@localhost/database")
        .await
        .unwrap();

    let repository = FileRepository::new(pool);
    let service = FileService::new(repository);

    let file = File {
        id: 1,
        name: "example.txt".to_string(),
        content: "Hello, world!".to_string(),
    };

    match service.process_file(file).await {
        Ok(_) => HttpResponse::Ok().body("File processed successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Error processing file"),
    }
}

#### Dockerfile