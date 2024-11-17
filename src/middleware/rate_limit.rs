use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
    body::{BoxBody, MessageBody},
};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::{
    task::{Context, Poll},
    sync::{Arc, Mutex},
    collections::HashMap,
    time::{Instant, Duration},
};

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

    fn is_rate_limited(&self, client_ip: &str) -> bool {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();
        
        let timestamps = requests.entry(client_ip.to_string()).or_insert_with(Vec::new);
        timestamps.retain(|&time| now.duration_since(time) < self.window_duration);
        
        if timestamps.len() >= self.max_requests {
            return true;
        }
        
        timestamps.push(now);
        false
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
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

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let client_ip = req
            .connection_info()
            .peer_addr()
            .unwrap_or("unknown")
            .to_string();

        if self.rate_limiter.is_rate_limited(&client_ip) {
            let response = HttpResponse::TooManyRequests()
                .body("Rate limit exceeded. Please try again later.");
            return Box::pin(async move {
                Ok(ServiceResponse::new(
                    req.into_parts().0,
                    response.map_into_boxed_body(),
                ))
            });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_boxed_body())
        })
    }
}
