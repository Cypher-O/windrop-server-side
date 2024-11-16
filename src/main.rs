mod controllers;
mod models;
mod services;
mod repositories;
mod middleware;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger as ActixLogger;
use actix_cors::Cors;
use services::file_service::FileService;
use controllers::file_controller::{upload_file, get_file};
use crate::middleware::{logger::RequestLogger, rate_limit::RateLimiter};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let file_service = web::Data::new(FileService::new());
    
    // Configure rate limiter: 100 requests per minute per IP
    let rate_limiter = RateLimiter::new(100, Duration::from_secs(60));

    HttpServer::new(move || {
        App::new()
            .wrap(ActixLogger::default())
            .wrap(RequestLogger)
            .wrap(rate_limiter.clone())
            .wrap(Cors::permissive())
            .app_data(file_service.clone())
            .service(
                web::scope("/api")
                    .route("/upload", web::post().to(upload_file))
                    .route("/files/{id}", web::get().to(get_file))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
