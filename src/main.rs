mod controllers;
mod models;
mod services;
mod repositories;
mod middleware;
mod storage;
mod websocket;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger as ActixLogger;
use actix_cors::Cors;
use controllers::websocket_controller;
use services::file_service::FileService;
use controllers::file_controller::{upload_file, get_file};
use crate::middleware::{logger::RequestLogger, rate_limit::RateLimiter};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    // Initialize storage path
    let storage_path = std::env::current_dir()?.join("file_storage");
    
    // Create FileService instance with proper error handling
    let file_service = match FileService::new(storage_path) {
        Ok(service) => {
            log::info!("FileService initialized successfully");
            web::Data::new(service)
        },
        Err(e) => {
            log::error!("Failed to initialize FileService: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to initialize FileService"
            ));
        }
    };
    
    let rate_limiter = RateLimiter::new(100, Duration::from_secs(60));

    log::info!("Starting HTTP server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(ActixLogger::default())
            .wrap(RequestLogger)
            .wrap(rate_limiter.clone())
            .wrap(Cors::permissive())
            .app_data(file_service.clone())
            .app_data(web::JsonConfig::default().limit(usize::MAX))
            .app_data(web::PayloadConfig::default().limit(usize::MAX))
            // .timeout(std::time::Duration::from_secs(300))
            .service(
                web::scope("/api")
                    .route("/upload", web::post().to(upload_file))
                    .route("/files/{id}", web::get().to(get_file))
                    .route("/ws", web::get().to(websocket_controller::websocket_route))
            )
    })
    .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
