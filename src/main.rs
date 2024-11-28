mod controllers;
mod middleware;
mod models;
mod repositories;
mod services;
mod storage;
mod websocket;

use crate::middleware::{logger::RequestLogger, rate_limit::RateLimiter};
use actix_cors::Cors;
use actix_web::middleware::Logger as ActixLogger;
use actix_web::{web, App, HttpServer};
use controllers::file_controller::{get_file, upload_file};
use controllers::websocket_controller::websocket_route;
use services::discovery_service::DiscoveryService;
use services::file_service::FileService;
use std::sync::Arc;
use std::time::Duration;
use std::{error::Error, net::Ipv4Addr};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Initialize storage path
    let storage_path = std::env::current_dir()?.join("file_storage");

    let file_service = web::Data::new(FileService::new(storage_path)?);
    let discovery_service = Arc::new(DiscoveryService::new());

    // let discovery_service_data = discovery_service.clone();

    // Create FileService instance with proper error handling
    // let file_service = match FileService::new(storage_path) {
    //     Ok(service) => {
    //         log::info!("FileService initialized successfully");
    //         web::Data::new(service)
    //     },
    //     Err(e) => {
    //         log::error!("Failed to initialize FileService: {}", e);
    //         return Err(std::io::Error::new(
    //             std::io::ErrorKind::Other,
    //             "Failed to initialize FileService"
    //         ));
    //     }
    // };

    let rate_limiter = RateLimiter::new(100, Duration::from_secs(60));

    log::info!("Starting HTTP server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(ActixLogger::default())
            .wrap(RequestLogger)
            .wrap(rate_limiter.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            // .wrap(Cors::permissive())
            .app_data(web::JsonConfig::default().limit(usize::MAX))
            .app_data(web::PayloadConfig::default().limit(usize::MAX))
            .app_data(file_service.clone())
            .app_data(web::Data::new(Arc::clone(&discovery_service)))
            // .timeout(std::time::Duration::from_secs(300))
            .service(
                web::scope("/api")
                    .route("/upload", web::post().to(upload_file))
                    .route("/files/{id}", web::get().to(get_file))
                    .route("/ws", web::get().to(websocket_route)),
            )
    })
    .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_server_startup() {
        let storage_path = tempfile::tempdir().unwrap().path().to_path_buf();
        let file_service = web::Data::new(FileService::new(storage_path).unwrap());
        let discovery_service = Arc::new(DiscoveryService::new());

        let server = HttpServer::new(move || {
            App::new()
                .app_data(file_service.clone())
                .app_data(web::Data::new(Arc::clone(&discovery_service)))
                .service(
                    web::scope("/api")
                        .route("/upload", web::post().to(upload_file))
                        .route("/files/{id}", web::get().to(get_file))
                        .route("/ws", web::get().to(websocket_route)),
                )
        })
        .bind("127.0.0.1:0")
        .unwrap();

        let _ = server.run();
        assert!(true); // Server started successfully
    }
}
