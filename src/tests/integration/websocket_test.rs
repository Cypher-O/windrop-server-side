use actix_web::{test, web, App};
use actix_web::http::StatusCode;
use std::sync::Arc;

use crate::services::discovery_service::DiscoveryService;
use crate::controllers::websocket_controller::websocket_route;

#[actix_rt::test]
async fn test_websocket_connection() {
    let discovery_service = Arc::new(DiscoveryService::new());
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Arc::clone(&discovery_service)))
            .route("/ws", web::get().to(websocket_route))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/ws?name=test-device")
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::SWITCHING_PROTOCOLS);
}