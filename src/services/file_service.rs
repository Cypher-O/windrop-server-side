use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::models::file::File;
use crate::repositories::FileRepository;
use crate::models::File;


pub struct FileService {
    files: Arc<Mutex<HashMap<String, File>>>,
}

impl FileService {
    pub fn new() -> Self {
        FileService {
            files: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_file(&self, file: File) {
        let mut files = self.files.lock().unwrap();
        files.insert(file.id.clone(), file);
    }

    pub fn get_file(&self, file_id: &str) -> Option<File> {
        let files = self.files.lock().unwrap();
        files.get(file_id).cloned()
    }
}
