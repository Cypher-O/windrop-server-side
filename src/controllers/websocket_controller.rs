use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::Deserialize;
use std::sync::Arc;

use crate::services::discovery_service::DiscoveryService;
use crate::websocket::connection::FileTransferWs;

#[derive(Debug, Deserialize)]
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
        Arc::clone(&discovery_service),
    );
    ws::start(ws, &req, stream)
}
