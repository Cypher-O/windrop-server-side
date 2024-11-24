use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use chrono::Utc;
use std::sync::Arc;
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::services::discovery_service::DiscoveryService;
use super::message::FileTransferMessage;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
const DISCOVERY_INTERVAL: Duration = Duration::from_secs(10);

pub struct FileTransferWs {
    id: String,
    device_name: String,
    hb: Instant,
    discovery_service: Arc<DiscoveryService>,
}

impl FileTransferWs {
    pub fn new(device_name: String, discovery_service: Arc<DiscoveryService>) -> Self {
        let id = Uuid::new_v4().to_string();
        discovery_service.register_device(id.clone(), device_name.clone());
        
        Self {
            id,
            device_name,
            hb: Instant::now(),
            discovery_service,
        }
    }

    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                log::info!("Client timeout, disconnecting: {}", act.id);
                act.discovery_service.remove_device(&act.id);
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

    fn start_discovery(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(DISCOVERY_INTERVAL, |act, ctx| {
            act.discovery_service.update_device_timestamp(&act.id);
            let devices = act.discovery_service.get_nearby_devices();
            let message = FileTransferMessage::DeviceList {
                devices,
                timestamp: Utc::now(),
            };
            
            if let Ok(json) = serde_json::to_string(&message) {
                ctx.text(json);
            }
        });
    }
}

impl Actor for FileTransferWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("WebSocket connection started for device: {}", self.device_name);
        self.heartbeat(ctx);
        self.start_discovery(ctx);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        log::info!("WebSocket connection stopped for device: {}", self.device_name);
        self.discovery_service.remove_device(&self.id);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for FileTransferWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                if let Ok(message) = serde_json::from_str::<FileTransferMessage>(&text) {
                    match message {
                        FileTransferMessage::DeviceDiscovery { .. } => {
                            let devices = self.discovery_service.get_nearby_devices();
                            let response = FileTransferMessage::DeviceList {
                                devices,
                                timestamp: Utc::now(),
                            };
                            if let Ok(json) = serde_json::to_string(&response) {
                                ctx.text(json);
                            }
                        }
                        _ => {
                            ctx.text(text);
                        }
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                self.discovery_service.remove_device(&self.id);
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
