use std::path::PathBuf;
use std::sync::RwLock;
use std::collections::HashMap;
use std::fs;
use std::io;
use crate::models::file::File;

pub struct FileStore {
    storage_path: PathBuf,
    files: RwLock<HashMap<String, File>>,
}

impl FileStore {
    pub fn new(storage_path: PathBuf) -> io::Result<Self> {
        if !storage_path.exists() {
            fs::create_dir_all(&storage_path)?;
        }
        
        if !storage_path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Storage path must be a directory"
            ));
        }
        
        Ok(Self {
            storage_path,
            files: RwLock::new(HashMap::new()),
        })
    }

    pub fn add_file(&self, file: File) -> io::Result<()> {
        let mut files = self.files.write().map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "Failed to acquire write lock")
        })?;
        files.insert(file.id.clone(), file);
        Ok(())
    }

    pub fn get_file(&self, id: &str) -> Option<File> {
        self.files.read().ok()?.get(id).cloned()
    }

    pub fn generate_storage_path(&self) -> PathBuf {
        self.storage_path.clone()
    }

    pub fn generate_file_path(&self, id: &str) -> PathBuf {
        self.generate_storage_path().join(id) 
    }
}
