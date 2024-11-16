use actix_service::Service;
use actix_web::{Error, HttpRequest, HttpResponse};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use std::time::{Instant, Duration};
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use futures::future::{ok, Ready};

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_duration: Duration) -> Self {
        RateLimiter {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_duration,
        }
    }
}

impl<S> actix_service::Transform<S> for RateLimiter
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = RateLimiterMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RateLimiterMiddleware {
            service,
            rate_limiter: self.clone(),
        })
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    rate_limiter: RateLimiter,
}

impl<S> Service for RateLimiterMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let client_ip = req.peer_addr().map(|addr| addr.ip().to_string()).unwrap_or_default();
        let now = Instant::now();
        
        let mut requests = self.rate_limiter.requests.lock().unwrap();
        let timestamps = requests.entry(client_ip.clone()).or_insert_with(Vec::new);

        // Cleanup old timestamps
        timestamps.retain(|&time| now.duration_since(time) < self.rate_limiter.window_duration);
        
        if timestamps.len() >= self.rate_limiter.max_requests {
            return Box::pin(async {
                Ok(req.error_response(HttpResponse::TooManyRequests().finish()))
            });
        }

        timestamps.push(now);
        self.service.call(req)
    }
}
