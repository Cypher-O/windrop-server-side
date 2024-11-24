use std::sync::Arc;
use crate::services::discovery_service::DiscoveryService;

#[test]
fn test_discovery_service() {
    let service = DiscoveryService::new();
    
    // Test device registration
    service.register_device("test-id".to_string(), "test-device".to_string());
    
    let devices = service.get_nearby_devices();
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].name, "test-device");
    
    // Test device removal
    service.remove_device("test-id");
    let devices = service.get_nearby_devices();
    assert_eq!(devices.len(), 0);
}
