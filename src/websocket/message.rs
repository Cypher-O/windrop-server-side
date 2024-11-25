use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::models::device::DeviceInfo;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FileTransferMessage {
    DeviceDiscovery {
        timestamp: DateTime<Utc>,
    },
    DeviceList {
        devices: Vec<DeviceInfo>,
        timestamp: DateTime<Utc>,
    },
    FileTransferInit {
        transfer_id: String,
        filename: String,
        file_size: u64,
        sender_id: String,
        receiver_id: String,
    },
    FileChunk {
        transfer_id: String,
        chunk_index: usize,
        total_chunks: usize,
        data: String,
        receiver_id: String,
    },
    FileTransferComplete {
        transfer_id: String,
        receiver_id: String,
    },
    TransferRequest {
        file_id: String,
        filename: String,
        size: u64,
        timestamp: DateTime<Utc>,
    },
    TransferAccept {
        file_id: String,
        timestamp: DateTime<Utc>,
    },
    TransferReject {
        file_id: String,
        timestamp: DateTime<Utc>,
    },
    TransferProgress {
        file_id: String,
        bytes_transferred: u64,
        total_bytes: u64,
        timestamp: DateTime<Utc>,
    },
    Error {
        message: String,
        timestamp: DateTime<Utc>,
    },
}