use crate::models::file::File;
use crate::repositories::file_repository::FileRepository;
use std::sync::Arc;

pub struct FileService {
    repository: Arc<FileRepository>,
}

impl FileService {
    pub fn new() -> Self {
        FileService {
            repository: Arc::new(FileRepository::new()),
        }
    }

    pub fn add_file(&self, file: File) {
        self.repository.save(file);
    }

    pub fn get_file(&self, file_id: &str) -> Option<File> {
        self.repository.get(file_id)
    }
}
