use chrono::Utc;
use crate::models::device::DeviceInfo;

#[test]
fn test_device_info_creation() {
    let device = DeviceInfo {
        id: "test-id".to_string(),
        name: "test-device".to_string(),
        last_seen: Utc::now(),
    };
    
    assert_eq!(device.id, "test-id");
    assert_eq!(device.name, "test-device");
}
