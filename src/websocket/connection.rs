use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Serialize, Deserialize)]
pub enum FileTransferMessage {
    TransferRequest {
        file_id: String,
        filename: String,
        size: u64,
    },
    TransferAccept {
        file_id: String,
    },
    TransferReject {
        file_id: String,
    },
    TransferProgress {
        file_id: String,
        bytes_transferred: u64,
        total_bytes: u64,
    },
    TransferComplete {
        file_id: String,
    },
    Error {
        message: String,
    },
}

pub struct FileTransferWs {
    id: String,
    hb: Instant,
    device_name: String,
}

impl FileTransferWs {
    pub fn new(device_name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            hb: Instant::now(),
            device_name,
        }
    }

    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for FileTransferWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
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
                    // Handle different message types
                    match message {
                        FileTransferMessage::TransferRequest { .. } => {
                            // Notify other connected clients about the transfer request
                            ctx.text(text);
                        }
                        _ => {
                            ctx.text(text);
                        }
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
