use actix_web::{Error, HttpRequest, HttpResponse};
use actix_service::Service;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use futures::future::{ok, Ready};

pub struct Logger;

impl<S> actix_service::Transform<S> for Logger
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = LoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggerMiddleware { service })
    }
}

pub struct LoggerMiddleware<S> {
    service: S,
}

impl<S> Service for LoggerMiddleware<S>
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
        log::info!("Incoming request: {:?}", req);
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            log::info!("Response: {:?}", res);
            Ok(res)
        })
    }
}
