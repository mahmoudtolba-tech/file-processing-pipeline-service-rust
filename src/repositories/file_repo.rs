use crate::models::file::FileRecord;
use async_trait::async_trait;
use sqlx::PgPool;
use crate::error::ServiceError;

/// Trait defining repository operations for `FileRecord`.
#[async_trait]
pub trait FileRepository: Send + Sync {
    async fn create(&self, record: &FileRecord) -> Result<FileRecord, ServiceError>;
    async fn get(&self, id: i32) -> Result<Option<FileRecord>, ServiceError>;
    async fn list(&self) -> Result<Vec<FileRecord>, ServiceError>;
    async fn mark_processed(&self, id: i32) -> Result<(), ServiceError>;
}

/// Concrete implementation using SQLx and PostgreSQL.
pub struct SqlxFileRepository {
    pool: PgPool,
}

impl SqlxFileRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl FileRepository for SqlxFileRepository {
    async fn create(&self, record: &FileRecord) -> Result<FileRecord, ServiceError> {
        let rec = sqlx::query_as!(
            FileRecord,
            r#"
            INSERT INTO files (filename, location, created_at, processed)
            VALUES ($1, $2, $3, $4)
            RETURNING id, filename, location, created_at, processed
            "#,
            record.filename,
            record.location,
            record.created_at,
            record.processed
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(rec)
    }

    async fn get(&self, id: i32) -> Result<Option<FileRecord>, ServiceError> {
        let rec = sqlx::query_as!(
            FileRecord,
            r#"
            SELECT id, filename, location, created_at, processed
            FROM files
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(rec)
    }

    async fn list(&self) -> Result<Vec<FileRecord>, ServiceError> {
        let rows = sqlx::query_as!(
            FileRecord,
            r#"
            SELECT id, filename, location, created_at, processed
            FROM files
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn mark_processed(&self, id: i32) -> Result<(), ServiceError> {
        sqlx::query!(
            r#"
            UPDATE files SET processed = true WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}