use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, DateTime};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct FileRecord {
    pub id: Uuid,
    pub filename: String,
    pub size_bytes: i64,
    pub uploaded_at: DateTime<Utc>,
    pub processed: bool,
}