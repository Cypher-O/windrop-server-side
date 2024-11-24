use std::collections::HashMap;
use std::sync::RwLock;
use chrono::Utc;
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

    pub fn register_device(&self, id: String, name: String) {
        let device = DeviceInfo {
            id: id.clone(),
            name,
            last_seen: Utc::now(),
        };

        self.devices
            .write()
            .unwrap()
            .insert(id, device);
    }

    pub fn update_device_timestamp(&self, id: &str) {
        if let Some(device) = self.devices.write().unwrap().get_mut(id) {
            device.last_seen = Utc::now();
        }
    }

    pub fn remove_device(&self, id: &str) {
        self.devices.write().unwrap().remove(id);
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