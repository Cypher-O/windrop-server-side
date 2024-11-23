use std::collections::HashMap;
use std::sync::RwLock;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::device::DeviceInfo;

pub struct DiscoveryService {
    devices: RwLock<HashMap<String, DeviceInfo>>,
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
            last_seen: Utc::now(),
        };

        self.devices
            .write()
            .unwrap()
            .insert(device.id.clone(), device.clone());

        device
    }

    pub fn get_nearby_devices(&self) -> Vec<DeviceInfo> {
        let now = Utc::now();
        self.devices
            .read()
            .unwrap()
            .values()
            .filter(|device| {
                (now - device.last_seen).num_seconds() < 30
            })
            .cloned()
            .collect()
    }
}
