use crate::models::File;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct FileRepository {
    storage: Mutex<HashMap<String, File>>,
}

impl FileRepository {
    pub fn new() -> Self {
        FileRepository {
            storage: Mutex::new(HashMap::new()),
        }
    }

    pub fn save(&self, file: File) {
        let mut storage = self.storage.lock().unwrap();
        storage.insert(file.id.clone(), file);
    }

    pub fn get(&self, file_id: &str) -> Option<File> {
        let storage = self.storage.lock().unwrap();
        storage.get(file_id).cloned()
    }
}
