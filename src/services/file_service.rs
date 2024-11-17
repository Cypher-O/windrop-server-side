use std::{fs, io};
use std::sync::{Arc, Mutex};
use tokio::fs::File as TokioFile;
use tempfile::NamedTempFile;
use futures_util::StreamExt;
use actix_multipart::Field;
use crate::repositories::file_repository::FileRepository;
use crate::storage::file_store::FileStore;
use crate::models::file::File;
use std::io::Write;

pub struct FileService {
    store: Arc<FileStore>,
    repository: Arc<Mutex<FileRepository>>, 
}

impl FileService {
    pub fn new(storage_path: std::path::PathBuf) -> io::Result<Self> {
        fs::create_dir_all(&storage_path)?;

        let store = FileStore::new(storage_path.clone())?;
        
        Ok(Self {
            store: Arc::new(store),
            repository: Arc::new(Mutex::new(FileRepository::new())),
        })
    }

    pub async fn save_file(&self, mut field: Field) -> io::Result<File> {
        // Get filename from field
        let filename = field
            .content_disposition()
            .get_filename()
            .map(|f| sanitize_filename::sanitize(f))
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "No filename provided"))?;

        // Create temporary file
        let temp_file = NamedTempFile::new()?;
        let mut writer = std::io::BufWriter::new(&temp_file);
        let mut size = 0u64;

        // Stream to temporary file
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            size += data.len() as u64;
            writer.write_all(&data)?;
        }

        // Ensure writer is flushed and dropped before moving temp_file
        writer.flush()?;
        drop(writer);

        // Generate final path
        let file_id = uuid::Uuid::new_v4().to_string();
        let final_path = self.store.generate_file_path(&file_id);

        // Persist file
        temp_file.persist(&final_path)?;

        // Create file record
        let file = File::new(filename, size, final_path);

        // Save to the in-memory repository for caching
        self.repository.lock().unwrap().save(file.clone());

        // Persist file metadata to the FileStore
        self.store.add_file(file.clone())?;

        Ok(file)
    }

    pub async fn read_file(&self, id: &str) -> io::Result<(File, TokioFile)> {
        // First try cache
        if let Some(file) = self.repository.lock().unwrap().get(id) {
            match TokioFile::open(&file.file_path).await {
                Ok(file_handle) => {
                    log::info!("File found and opened: {}", id);
                    return Ok((file, file_handle));
                }
                Err(e) => {
                    log::error!("Error opening file {}: {}", id, e);
                    return Err(e);
                }
            }
        }

        // If not in cache, check storage
        if let Some(file) = self.store.get_file(id) {
            match TokioFile::open(&file.file_path).await {
                Ok(file_handle) => {
                    log::info!("File found in storage: {}", id);
                    return Ok((file, file_handle));
                }
                Err(e) => {
                    log::error!("Error opening file from storage {}: {}", id, e);
                    return Err(e);
                }
            }
        }

        log::error!("File not found: {}", id);
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }
}
