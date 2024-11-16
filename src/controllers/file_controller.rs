use actix_web::{web, HttpResponse, Responder, post, get};
use crate::models::{ApiResponse, File};
use crate::services::FileService;
use actix_multipart::Multipart;
use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;
use std::io::Write;
use uuid::Uuid;

#[post("/upload")]
async fn upload_file(mut payload: Multipart, file_service: web::Data<FileService>) -> impl Responder {
    // Simulate file upload logic
    let mut file_id = String::new();
    let mut file_name = String::new();
    let mut file_size: u64 = 0;

    // Iterate through the multipart payload
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        file_name = field.name().to_string();
        
        // For simplicity, we simulate saving the file by counting bytes
        while let Some(chunk) = field.next().await {
            let chunk = chunk.unwrap();
            file_size += chunk.len() as u64;
        }
    }

    // Simulate creating a file object and storing it in the file service
    file_id = Uuid::new_v4().to_string();
    let file = File::new(file_id.clone(), file_name, file_size);

    // Add to file service (this is an in-memory service in this case)
    file_service.add_file(file.clone());

    // Construct the API response
    let response = ApiResponse::<File>::new(
        0,
        "success",
        "File uploaded successfully",
        Some(file),
    );

    HttpResponse::Created().json(response)  // Return the response as JSON
}

#[get("/files/{id}")]
async fn get_file(file_id: web::Path<String>, file_service: web::Data<FileService>) -> impl Responder {
    let file_id = file_id.into_inner();

    // Retrieve file from the service
    if let Some(file) = file_service.get_file(&file_id) {
        // Construct the API response
        let response = ApiResponse::<File>::new(
            0,
            "success",
            "File retrieved successfully",
            Some(file),
        );

        HttpResponse::Ok().json(response)  // Return file data as JSON
    } else {
        // If file not found, return error response
        let response = ApiResponse::<File>::new(
            1,
            "error",
            "File not found",
            None,
        );

        HttpResponse::NotFound().json(response)  // Return 404 with error message
    }
}