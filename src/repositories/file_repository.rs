use std::env;
use sqlx::pool::PgPool;
use sqlx::PgDatabase;
use models::FileModel;

pub struct FileRepository {
    pool: PgPool,
}

impl FileRepository {
    pub fn new(db_url: String) -> Self {
        let pool = PgPool::connect(&db_url).unwrap();
        FileRepository { pool }
    }

    pub async fn create_file(&self, file: FileInput) -> Result<FileModel, sqlx::Error> {
        sqlx::query_as!(
            models::FileModel,
            "INSERT INTO files (name, size) VALUES ($1, $2) RETURNING id, name, size",
            file.name,
            file.size,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_all_files(&self) -> Result<Vec<FileModel>, sqlx::Error> {
        sqlx::query_as!(models::FileModel, "SELECT id, name, size FROM files")
            .fetch_all(&self.pool)
            .await
    }
}