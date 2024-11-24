use actix_web::{test, web, App};
use actix_web::http::StatusCode;
use std::time::Duration;
use crate::middleware::rate_limit::RateLimiter;

#[actix_rt::test]
async fn test_rate_limiter() {
    let rate_limiter = RateLimiter::new(2, Duration::from_secs(1));
    
    let app = test::init_service(
        App::new()
            .wrap(rate_limiter)
            .route("/test", web::get().to(|| async { "ok" }))
    ).await;
    
    // First request should succeed
    let req = test::TestRequest::get().uri("/test").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    
    // Second request should succeed
    let req = test::TestRequest::get().uri("/test").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    
    // Third request should be rate limited
    let req = test::TestRequest::get().uri("/test").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::TOO_MANY_REQUESTS);
}