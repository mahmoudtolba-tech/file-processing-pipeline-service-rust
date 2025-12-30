use super::schema;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct File {
    pub id: i32,
    pub file_path: String,
}