# Threaded HTTP Server

A dependency-free HTTP/1 learning server built with `TcpListener`, a small
thread pool, and shared request statistics.

```bash
cargo run -p http_server
curl http://127.0.0.1:7878/hello
curl http://127.0.0.1:7878/stats
```

The parser intentionally covers a small subset of HTTP. Use a maintained HTTP
library for production services.
