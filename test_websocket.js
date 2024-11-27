const WebSocket = require('ws');
const readline = require('readline');

const ws = new WebSocket('ws://localhost:8080/api/ws?name=TestDevice');

ws.on('open', function open() {
    console.log('Connected to server');
    
    // Interactive WebSocket testing
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout
    });

    rl.on('line', (input) => {
        try {
            const message = JSON.parse(input);
            ws.send(JSON.stringify(message));
        } catch (e) {
            console.error('Invalid JSON');
        }
    });

    ws.send(JSON.stringify({
        type: "DeviceDiscovery",
        timestamp: new Date().toISOString()
    }));
});