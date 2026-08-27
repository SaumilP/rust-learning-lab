# WebSocket Server

Real-time bidirectional communication server using WebSocket protocol.

## Features

- WebSocket handshake handling
- Bidirectional message streaming
- Text and binary message support
- Connection lifecycle management
- Broadcasting to multiple clients

## Running

Server:
```bash
cargo run
```

Client (in browser console or use websocat):
```bash
# Install websocat
cargo install websocat

# Connect
websocat ws://127.0.0.1:8080
```

Or use JavaScript in browser:
```javascript
const ws = new WebSocket('ws://127.0.0.1:8080');
ws.onmessage = (event) => console.log('Received:', event.data);
ws.send('Hello, server!');
```

## What You'll Learn

- WebSocket protocol basics
- tokio-tungstenite usage
- Stream splitting for bidirectional I/O
- Message framing
- Connection state management
