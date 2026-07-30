use crate::errors::ServiceError;
use crate::models::FileRecord;
use sqlx::PgPool;
use uuid::Uuid;

pub struct FileRepository;

impl FileRepository {
    pub async fn create(pool: &PgPool, filename: &str, size_bytes: i64) -> Result<FileRecord, ServiceError> {
        let rec = sqlx::query_as!(
            FileRecord,
            r#"
            INSERT INTO files (id, filename, size_bytes, uploaded_at, processed)
            VALUES ($1, $2, $3, $4, false)
            RETURNING id, filename, size_bytes, uploaded_at, processed
            "#,
            Uuid::new_v4(),
            filename,
            size_bytes,
            chrono::Utc::now()
        )
        .fetch_one(pool)
        .await?;
        Ok(rec)
    }

    pub async fn get(pool: &PgPool, id: Uuid) -> Result<FileRecord, ServiceError> {
        let rec = sqlx::query_as!(
            FileRecord,
            r#"
            SELECT id, filename, size_bytes, uploaded_at, processed
            FROM files
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?
        .ok_or(ServiceError::NotFound)?;
        Ok(rec)
    }

    pub async fn list(pool: &PgPool) -> Result<Vec<FileRecord>, ServiceError> {
        let rows = sqlx::query_as!(
            FileRecord,
            r#"
            SELECT id, filename, size_bytes, uploaded_at, processed
            FROM files
            ORDER BY uploaded_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    pub async fn mark_processed(pool: &PgPool, id: Uuid) -> Result<(), ServiceError> {
        let affected = sqlx::query!(
            r#"
            UPDATE files SET processed = true WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?
        .rows_affected();

        if affected == 0 {
            Err(ServiceError::NotFound)
        } else {
            Ok(())
        }
    }
}