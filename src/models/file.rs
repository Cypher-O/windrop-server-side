use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct File {
    pub id: String,
    pub name: String,
    pub size: u64,  // Size in bytes
    pub data: Vec<u8>,  // File data
}

impl File {
    pub fn new(name: String, size: u64, data: Vec<u8>) -> Self {
        File {
            id: Uuid::new_v4().to_string(),
            name,
            size,
            data,
        }
    }
}
