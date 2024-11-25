const WebSocket = require('ws');

const ws = new WebSocket('ws://localhost:8080/api/ws?name=TestDevice');

ws.on('open', function open() {
    console.log('Connected to server');
    
    // Send device discovery request
    ws.send(JSON.stringify({
        type: "DeviceDiscovery",
        timestamp: new Date().toISOString()
    }));
});

ws.on('message', function incoming(data) {
    console.log('Received:', JSON.parse(data));
});

ws.on('close', function close() {
    console.log('Disconnected from server');
});

ws.on('error', function error(err) {
    console.error('WebSocket error:', err);
});

// Keep the connection alive
setInterval(() => {
    if (ws.readyState === WebSocket.OPEN) {
        ws.ping();
    }
}, 5000);