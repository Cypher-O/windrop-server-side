// main.rs
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use crate::controllers::{upload_file, get_file}; 
use crate::services::FileService; // Correct import for FileService
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let file_service = web::Data::new(FileService::new());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(file_service.clone())
            .route("/upload", web::post().to(upload_file))
            .route("/files/{id}", web::get().to(get_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
