use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use crate::models::file::File;
use crate::models::response::ApiResponse;
use crate::services::file_service::FileService;
use bytes::BytesMut;

pub async fn upload_file(mut payload: Multipart, file_service: web::Data<FileService>) -> impl Responder {
    let mut buffer = BytesMut::new();
    let mut filename = String::new();
    
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        filename = field.content_disposition()
            .get_filename()
            .unwrap_or("unknown")
            .to_string();
        
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            buffer.extend_from_slice(&data);
        }
    }

    let file = File::new(
        filename,
        buffer.len() as u64,
        buffer.to_vec(),
    );

    file_service.add_file(file.clone());

    let response = ApiResponse::new(
        0,
        "success",
        "File uploaded successfully",
        Some(file),
    );

    HttpResponse::Created().json(response)
}

pub async fn get_file(
    file_id: web::Path<String>,
    file_service: web::Data<FileService>,
) -> impl Responder {
    if let Some(file) = file_service.get_file(&file_id) {
        let response = ApiResponse::new(
            0,
            "success",
            "File retrieved successfully",
            Some(file),
        );
        HttpResponse::Ok().json(response)
    } else {
        let response = ApiResponse::<File>::new(
            1,
            "error",
            "File not found",
            None,
        );
        HttpResponse::NotFound().json(response)
    }
}
