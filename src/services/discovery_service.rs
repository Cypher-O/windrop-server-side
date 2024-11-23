use std::time::{Duration, Instant};
use std::{collections::HashMap};
use std::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct DiscoveryService {
    devices: RwLock<HashMap<String, DeviceInfo>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub last_seen: Instant,
}

impl DiscoveryService {
    pub fn new() -> Self {
        Self {
            devices: RwLock::new(HashMap::new()),
        }
    }

    pub fn register_device(&self, name: String) -> DeviceInfo {
        let device = DeviceInfo {
            id: Uuid::new_v4().to_string(),
            name,
            last_seen: Instant::now(),
        };

        self.devices
            .write()
            .unwrap()
            .insert(device.id.clone(), device.clone());

        device
    }

    pub fn get_nearby_devices(&self) -> Vec<DeviceInfo> {
        self.devices
            .read()
            .unwrap()
            .values()
            .filter(|device| device.last_seen.elapsed() < Duration::from_secs(30))
            .cloned()
            .collect()
    }
}
