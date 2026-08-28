# Async Chat Server

A TCP chat server demonstrating Tokio tasks, broadcast channels, shared state,
and line-oriented network I/O.

```bash
cargo run -p chat_app
nc 127.0.0.1 8080
```

After connecting, enter a name, send messages, use `/users` to list connected
clients, or `/quit` to disconnect. This is a learning server intended for local
use; it does not provide authentication, encryption, or production hardening.
