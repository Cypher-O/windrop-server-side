use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::Deserialize;

use crate::websocket::connection::FileTransferWs;

pub async fn websocket_route(
    req: HttpRequest,
    stream: web::Payload,
    device_name: web::Query<DeviceName>,
) -> Result<HttpResponse, Error> {
    let ws = FileTransferWs::new(device_name.name.clone());
    ws::start(ws, &req, stream)
}

#[derive(Deserialize)]
pub struct DeviceName {
    name: String,
}