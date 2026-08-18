use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Representation of a file record stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Validate)]
pub struct FileRecord {
    #[serde(skip_deserializing)]
    pub id: i32,

    #[validate(length(min = 1, max = 255))]
    pub filename: String,

    #[validate(url)]
    pub location: String,

    pub created_at: DateTime<Utc>,
    pub processed: bool,
}