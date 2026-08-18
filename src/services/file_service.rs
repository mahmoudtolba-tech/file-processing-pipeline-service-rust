use crate::models::file::FileRecord;
use crate::repositories::file_repo::{FileRepository, SqlxFileRepository};
use crate::error::ServiceError;
use chrono::Utc;
use validator::Validate;

/// Business logic for file processing.
pub struct FileService<R: FileRepository> {
    repo: R,
}

impl FileService<SqlxFileRepository> {
    /// Create service with a concrete SQLx repository.
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            repo: SqlxFileRepository::new(pool),
        }
    }
}

impl<R: FileRepository> FileService<R> {
    /// Register a new file in the system.
    pub async fn register_file(
        &self,
        filename: String,
        location: String,
    ) -> Result<FileRecord, ServiceError> {
        let new_file = FileRecord {
            id: 0,
            filename,
            location,
            created_at: Utc::now(),
            processed: false,
        };
        new_file.validate()?;
        self.repo.create(&new_file).await
    }

    /// Retrieve a file by id.
    pub async fn get_file(&self, id: i32) -> Result<FileRecord, ServiceError> {
        self.repo
            .get(id)
            .await?
            .ok_or(ServiceError::NotFound)
    }

    /// List all files.
    pub async fn list_files(&self) -> Result<Vec<FileRecord>, ServiceError> {
        self.repo.list().await
    }

    /// Mark a file as processed.
    pub async fn process_file(&self, id: i32) -> Result<(), ServiceError> {
        self.repo.mark_processed(id).await
    }
}