use actix_web::{test, web, App};
use actix_web::http::StatusCode;
use bytes::Bytes;
use futures_util::stream::once;
use crate::services::file_service::FileService;
use crate::controllers::file_controller::{upload_file, get_file};

#[actix_rt::test]
async fn test_file_upload_and_download() {
    // Setup
    let temp_dir = tempfile::tempdir().unwrap();
    let file_service = web::Data::new(FileService::new(temp_dir.path().to_path_buf()).unwrap());
    
    let app = test::init_service(
        App::new()
            .app_data(file_service.clone())
            .service(
                web::scope("/api")
                    .route("/upload", web::post().to(upload_file))
                    .route("/files/{id}", web::get().to(get_file))
            )
    ).await;
    
    // Test file upload
    let file_content = Bytes::from_static(b"test file content");
    let payload = once(async move { Ok::<_, actix_web::Error>(file_content) });
    
    let req = test::TestRequest::post()
        .uri("/api/upload")
        .set_payload(payload)
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    
    // Extract file ID from response
    let body: serde_json::Value = test::read_body_json(resp).await;
    let file_id = body["data"]["id"].as_str().unwrap();
    
    // Test file download
    let req = test::TestRequest::get()
        .uri(&format!("/api/files/{}", file_id))
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    
    let downloaded_content = test::read_body(resp).await;
    assert_eq!(downloaded_content, Bytes::from_static(b"test file content"));
}
