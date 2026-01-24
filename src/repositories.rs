use crate::models::File;
use sqlx::PgPool;

pub struct FileRepository {
    pool: PgPool,
}

impl FileRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_file(&self, file: File) -> Result<File, sqlx::Error> {
        sqlx::query("INSERT INTO files (name, content) VALUES ($1, $2) RETURNING *")
            .bind(file.name)
            .bind(file.content)
            .fetch_one(&self.pool)
            .await
    }
}

#### controllers.rs