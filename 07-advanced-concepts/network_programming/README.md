# Section 16: Network Programming

## Overview

Master network programming in Rust, from low-level sockets to high-level protocols. Build TCP/UDP servers, HTTP clients, WebSocket applications, and implement custom protocols with Tokio's async networking stack.

## Why Rust for Network Programming?

✅ **Performance** - Zero-cost async I/O, minimal overhead <br />
✅ **Safety** - No buffer overflows or use-after-free <br />
✅ **Concurrency** - Handle millions of connections <br />
✅ **Memory Efficient** - Predictable memory usage <br />
✅ **Protocol Implementation** - Strong typing for protocols

## What You'll Learn

1. **TCP/UDP Sockets** - Low-level socket programming
2. **HTTP Clients** - reqwest, hyper
3. **WebSocket** - Real-time bidirectional communication
4. **gRPC** - High-performance RPC framework
5. **Protocol Design** - Custom binary protocols
6. **Load Balancing** - Proxy patterns
7. **TLS/SSL** - Secure communication
8. **DNS** - Name resolution
9. **Network Testing** - Mocking and testing
10. **Performance** - Optimization techniques

## Section Contents

### 01-tcp-echo-server/
Classic echo server with TCP

### 02-udp-chat/
UDP-based chat application

### 03-http-client/
Making HTTP requests with reqwest

### 04-websocket-server/
Real-time WebSocket communication

### 05-grpc-service/
gRPC server and client

### 06-custom-protocol/
Implementing a custom binary protocol

## Prerequisites

- Strong Rust fundamentals
- Understanding of async/await
- Basic networking concepts (TCP/IP)
- Familiarity with Tokio

## Key Concepts

### TCP Server with Tokio

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on port 8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => break, // Connection closed
                    Ok(n) => {
                        // Echo back
                        if socket.write_all(&buf[..n]).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    }
}
```

### TCP Client

```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn connect() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Send data
    stream.write_all(b"Hello, server!").await?;

    // Read response
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;

    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
    Ok(())
}
```

### UDP Socket

```rust
use tokio::net::UdpSocket;

async fn udp_server() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    let mut buf = vec![0; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("Received {} bytes from {}", len, addr);

        // Send response
        socket.send_to(&buf[..len], addr).await?;
    }
}
```

### HTTP Client with reqwest

```rust
use reqwest;

async fn fetch_data() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://api.github.com/repos/rust-lang/rust")
        .await?
        .text()
        .await?;

    Ok(response)
}

// POST request
async fn post_data() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .post("https://httpbin.org/post")
        .json(&serde_json::json!({
            "name": "Rust",
            "version": "1.75"
        }))
        .send()
        .await?;

    println!("Status: {}", response.status());
    Ok(())
}
```

### WebSocket Server

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures::{StreamExt, SinkExt};

async fn handle_connection(stream: TcpStream) {
    let ws_stream = accept_async(stream).await.unwrap();
    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) if msg.is_text() || msg.is_binary() => {
                write.send(msg).await.unwrap();
            }
            Ok(msg) if msg.is_close() => break,
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}
```

### gRPC Service

```rust
use tonic::{transport::Server, Request, Response, Status};

// Define service
pub mod hello {
    tonic::include_proto!("hello");
}

use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloRequest, HelloReply};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = HelloReply {
            message: format!("Hello, {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
```

### Custom Binary Protocol

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bytes::{Buf, BufMut, BytesMut};

// Protocol: [version(1)][command(1)][length(2)][payload(n)]

#[derive(Debug)]
struct Message {
    version: u8,
    command: u8,
    payload: Vec<u8>,
}

impl Message {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.put_u8(self.version);
        buf.put_u8(self.command);
        buf.put_u16(self.payload.len() as u16);
        buf.extend_from_slice(&self.payload);
        buf
    }

    async fn decode(stream: &mut TcpStream) -> std::io::Result<Self> {
        let version = stream.read_u8().await?;
        let command = stream.read_u8().await?;
        let length = stream.read_u16().await?;

        let mut payload = vec![0; length as usize];
        stream.read_exact(&mut payload).await?;

        Ok(Message {
            version,
            command,
            payload,
        })
    }
}
```

### Connection Pooling

```rust
use deadpool::managed::{Manager, Pool};

struct ConnectionManager;

impl Manager for ConnectionManager {
    type Type = TcpStream;
    type Error = std::io::Error;

    async fn create(&self) -> Result<TcpStream, Self::Error> {
        TcpStream::connect("127.0.0.1:8080").await
    }

    async fn recycle(&self, conn: &mut TcpStream) -> Result<(), Self::Error> {
        // Verify connection is still alive
        Ok(())
    }
}

async fn use_pool() {
    let mgr = ConnectionManager;
    let pool = Pool::builder(mgr).max_size(16).build().unwrap();

    let conn = pool.get().await.unwrap();
    // Use connection...
}
```

### Load Balancer

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::net::TcpStream;

struct LoadBalancer {
    backends: Vec<String>,
    next: AtomicUsize,
}

impl LoadBalancer {
    fn new(backends: Vec<String>) -> Self {
        LoadBalancer {
            backends,
            next: AtomicUsize::new(0),
        }
    }

    async fn get_backend(&self) -> std::io::Result<TcpStream> {
        let idx = self.next.fetch_add(1, Ordering::Relaxed) % self.backends.len();
        TcpStream::connect(&self.backends[idx]).await
    }
}
```

### TLS/SSL with rustls

```rust
use tokio_rustls::{TlsAcceptor, rustls};
use std::sync::Arc;

async fn tls_server() -> std::io::Result<()> {
    let certs = load_certs("cert.pem")?;
    let key = load_private_key("key.pem")?;

    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .unwrap();

    let acceptor = TlsAcceptor::from(Arc::new(config));
    let listener = TcpListener::bind("0.0.0.0:443").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            let tls_stream = acceptor.accept(stream).await.unwrap();
            // Handle TLS connection
        });
    }
}
```

### DNS Resolution

```rust
use trust_dns_resolver::TokioAsyncResolver;

async fn resolve_domain() -> Result<(), Box<dyn std::error::Error>> {
    let resolver = TokioAsyncResolver::tokio_from_system_conf()?;

    let response = resolver.lookup_ip("www.example.com").await?;

    for ip in response.iter() {
        println!("IP: {}", ip);
    }

    Ok(())
}
```

## Advanced Patterns

### Framed Protocol

```rust
use tokio_util::codec::{Framed, LengthDelimitedCodec};

async fn framed_connection(socket: TcpStream) {
    let framed = Framed::new(socket, LengthDelimitedCodec::new());
    let (mut sink, mut stream) = framed.split();

    while let Some(frame) = stream.next().await {
        let data = frame.unwrap();
        // Process frame
        sink.send(data).await.unwrap();
    }
}
```

### Multiplexing

```rust
use tokio::io::{split, AsyncReadExt, AsyncWriteExt};

async fn multiplex(socket: TcpStream) {
    let (mut reader, mut writer) = split(socket);

    let read_task = tokio::spawn(async move {
        let mut buf = vec![0; 1024];
        while let Ok(n) = reader.read(&mut buf).await {
            if n == 0 { break; }
            // Process read data
        }
    });

    let write_task = tokio::spawn(async move {
        loop {
            // Write data
            writer.write_all(b"data").await.unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    tokio::try_join!(read_task, write_task).unwrap();
}
```

### Circuit Breaker

```rust
use std::sync::atomic::{AtomicU32, Ordering};

struct CircuitBreaker {
    failures: AtomicU32,
    threshold: u32,
}

impl CircuitBreaker {
    fn new(threshold: u32) -> Self {
        CircuitBreaker {
            failures: AtomicU32::new(0),
            threshold,
        }
    }

    fn is_open(&self) -> bool {
        self.failures.load(Ordering::Relaxed) >= self.threshold
    }

    fn record_success(&self) {
        self.failures.store(0, Ordering::Relaxed);
    }

    fn record_failure(&self) {
        self.failures.fetch_add(1, Ordering::Relaxed);
    }
}

async fn request_with_circuit_breaker(
    breaker: &CircuitBreaker,
    url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    if breaker.is_open() {
        return Err("Circuit breaker is open".into());
    }

    match reqwest::get(url).await {
        Ok(resp) => {
            breaker.record_success();
            Ok(resp.text().await?)
        }
        Err(e) => {
            breaker.record_failure();
            Err(e.into())
        }
    }
}
```

## Performance Optimization

### Zero-Copy I/O

```rust
use tokio::io::copy;

async fn zero_copy_proxy(client: TcpStream, server: TcpStream) -> std::io::Result<()> {
    let (client_read, client_write) = client.into_split();
    let (server_read, server_write) = server.into_split();

    tokio::try_join!(
        copy(&mut client_read, &mut server_write),
        copy(&mut server_read, &mut client_write)
    )?;

    Ok(())
}
```

### Buffer Pool

```rust
use bytes::{Bytes, BytesMut};
use std::sync::Arc;

struct BufferPool {
    pool: Arc<Vec<BytesMut>>,
}

impl BufferPool {
    fn new(count: usize, size: usize) -> Self {
        let pool = (0..count)
            .map(|_| BytesMut::with_capacity(size))
            .collect();

        BufferPool {
            pool: Arc::new(pool),
        }
    }

    fn acquire(&self) -> BytesMut {
        // Pool management logic
        BytesMut::with_capacity(4096)
    }
}
```

## Testing

```rust
#[tokio::test]
async fn test_tcp_echo() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = vec![0; 1024];
        let n = socket.read(&mut buf).await.unwrap();
        socket.write_all(&buf[..n]).await.unwrap();
    });

    let mut client = TcpStream::connect(addr).await.unwrap();
    client.write_all(b"hello").await.unwrap();

    let mut buf = vec![0; 1024];
    let n = client.read(&mut buf).await.unwrap();
    assert_eq!(&buf[..n], b"hello");
}
```

## Security Considerations

1. **Input Validation** - Validate all network input
2. **Rate Limiting** - Prevent DoS attacks
3. **TLS** - Encrypt sensitive data
4. **Authentication** - Verify client identity
5. **Timeout** - Prevent resource exhaustion
6. **Buffer Limits** - Prevent buffer overflow
7. **Connection Limits** - Limit concurrent connections

## Resources

- [Tokio Networking](https://tokio.rs/tokio/tutorial)
- [reqwest Documentation](https://docs.rs/reqwest/)
- [tonic (gRPC)](https://docs.rs/tonic/)
- [tokio-tungstenite (WebSocket)](https://docs.rs/tokio-tungstenite/)
- [Network Programming Book](https://rust-lang-nursery.github.io/rust-cookbook/net.html)

## Next Steps

After completing this section:
- Build a complete networked application
- Implement a custom protocol
- Create a load balancer
- Study P2P networking

---

**Estimated Time**: 18-24 hours
**Difficulty**: ★★★★☆ (Advanced)
**Prerequisites**: Strong Rust, async/await, networking basics
