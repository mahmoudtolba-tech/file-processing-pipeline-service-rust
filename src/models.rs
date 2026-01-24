use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct File {
    id: i32,
    name: String,
    content: String,
}

#### services.rs