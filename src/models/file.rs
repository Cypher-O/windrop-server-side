use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub filename: String,
    pub size: u64,
    #[serde(skip_serializing)]
    pub file_path: PathBuf,
}

impl File {
    pub fn new(filename: String, size: u64, file_path: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            filename,
            size,
            file_path,
        }
    }
}
