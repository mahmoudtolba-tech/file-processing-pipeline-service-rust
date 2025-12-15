use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileModel {
    pub id: i32,
    pub name: String,
    pub size: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInput {
    pub name: String,
    pub size: i32,
}