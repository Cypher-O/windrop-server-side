use actix_web::{web, HttpResponse, Error, Result};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use crate::services::file_service::FileService;
use crate::models::response::ApiResponse;
use actix_web::http::header::{ContentDisposition, DispositionType, DispositionParam};
use tokio::io::BufReader;
use tokio_util::io::ReaderStream;

pub async fn upload_file(
    mut payload: Multipart, 
    file_service: web::Data<FileService>
) -> Result<HttpResponse, Error> {
    while let Some(item) = payload.next().await {
        let field = item?;
        
        match file_service.save_file(field).await {
            Ok(file) => {
                let response = ApiResponse::new(
                    0,
                    "success",
                    "File uploaded successfully",
                    Some(file),
                );
                return Ok(HttpResponse::Created().json(response));
            }
            Err(e) => {
                return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::new(
                    1,
                    "error",
                    &format!("Failed to save file: {}", e),
                    None,
                )));
            }
        }
    }
    
    Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::new(
        1,
        "error",
        "No file provided",
        None,
    )))
}

pub async fn get_file(
    file_id: web::Path<String>,
    file_service: web::Data<FileService>,
) -> Result<HttpResponse, Error> {
    match file_service.read_file(&file_id).await {
        Ok((file_info, file_handle)) => {
            // Get file size
            let metadata = file_handle.metadata().await?;
            let file_size = metadata.len();

            // Create buffered reader with a reasonable buffer size
            let reader = BufReader::with_capacity(8192, file_handle);
            
            // Create stream with chunks
            let stream = ReaderStream::new(reader);
            
            // Determine content type
            let content_type = mime_guess::from_path(&file_info.filename)
                .first_or_octet_stream();

            // Build response with proper headers
            Ok(HttpResponse::Ok()
                .insert_header(("Content-Type", content_type.as_ref()))
                .insert_header(("Content-Length", file_size.to_string()))
                .insert_header(ContentDisposition {
                    disposition: DispositionType::Attachment,
                    parameters: vec![DispositionParam::Filename(file_info.filename)],
                })
                .insert_header(("Accept-Ranges", "bytes"))
                .streaming(stream))
        }
        Err(e) => {
            log::error!("File retrieval error: {}", e);
            Ok(HttpResponse::NotFound().json(ApiResponse::<()>::new(
                1,
                "error",
                &format!("File error: {}", e),
                None,
            )))
        }
    }
}
