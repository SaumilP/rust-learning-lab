# TCP Echo Server

A production-ready TCP echo server demonstrating async networking fundamentals with Tokio.

## Features

- Concurrent connection handling
- Graceful error handling
- Connection statistics
- Async I/O with zero-copy where possible

## Running

Terminal 1 (Server):
```bash
cargo run
```

Terminal 2 (Client):
```bash
telnet 127.0.0.1 8080
# or
nc 127.0.0.1 8080
```

## Architecture

- Main task accepts connections
- Each connection spawned as separate task
- Shared state using Arc for connection counting
- Graceful shutdown handling

## What You'll Learn

- TCP server basics with `TcpListener`
- Async task spawning
- Buffer management
- Connection lifecycle
- Error handling patterns
