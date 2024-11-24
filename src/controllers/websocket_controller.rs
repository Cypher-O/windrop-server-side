use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::Deserialize;
use crate::{services::discovery_service::DiscoveryService, websocket::connection::FileTransferWs};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct DeviceName {
    name: String,
}

pub async fn websocket_route(
    req: HttpRequest,
    stream: web::Payload,
    device_name: web::Query<DeviceName>,
    discovery_service: web::Data<Arc<DiscoveryService>>,
) -> Result<HttpResponse, Error> {
    let ws = FileTransferWs::new(
        device_name.name.clone(),
        discovery_service.into_inner(),
    );
    ws::start(ws, &req, stream)
}