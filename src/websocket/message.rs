use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::models::device::DeviceInfo;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FileTransferMessage {
    // ... previous message types ...
    DeviceDiscovery {
        timestamp: DateTime<Utc>,
    },
    DeviceList {
        devices: Vec<DeviceInfo>,
        timestamp: DateTime<Utc>,
    },
}
