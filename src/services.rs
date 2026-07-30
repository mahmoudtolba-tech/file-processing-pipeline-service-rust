use crate::errors::ServiceError;
use crate::models::FileRecord;
use crate::repositories::FileRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub struct FileService<'a> {
    pool: &'a PgPool,
}

impl<'a> FileService<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn upload(&self, filename: &str, size_bytes: i64) -> Result<FileRecord, ServiceError> {
        if filename.trim().is_empty() {
            return Err(ServiceError::Validation("Filename cannot be empty".into()));
        }
        if size_bytes <= 0 {
            return Err(ServiceError::Validation("File size must be positive".into()));
        }
        FileRepository::create(self.pool, filename, size_bytes).await
    }

    pub async fn get(&self, id: Uuid) -> Result<FileRecord, ServiceError> {
        FileRepository::get(self.pool, id).await
    }

    pub async fn list(&self) -> Result<Vec<FileRecord>, ServiceError> {
        FileRepository::list(self.pool).await
    }

    pub async fn process(&self, id: Uuid) -> Result<(), ServiceError> {
        // Placeholder for real processing logic
        // For demo, just mark as processed
        FileRepository::mark_processed(self.pool, id).await
    }
}