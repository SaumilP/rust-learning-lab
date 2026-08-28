// Simple HTTP Server in Rust
// Demonstrates TCP networking, threading, and HTTP protocol basics

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Simple HTTP Server ===\n");

    // Thread pool for handling connections
    let thread_pool = Arc::new(ThreadPool::new(4));

    // Statistics tracking
    let stats = Arc::new(Mutex::new(ServerStats::new()));

    // Bind to localhost:7878
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");

    println!("Server running on http://127.0.0.1:7878");
    println!("Routes:");
    println!("  GET  /           - Home page");
    println!("  GET  /hello      - Hello message");
    println!("  GET  /sleep      - Slow endpoint (5s)");
    println!("  GET  /stats      - Server statistics");
    println!("  POST /echo       - Echo request body");
    println!("\nPress Ctrl+C to stop\n");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let pool = Arc::clone(&thread_pool);
                let stats = Arc::clone(&stats);

                pool.execute(move || {
                    handle_connection(stream, stats);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, stats: Arc<Mutex<ServerStats>>) {
    let mut buffer = [0; 1024];

    // Read request
    if let Ok(size) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..size]);

        // Parse request line
        let request_line = request.lines().next().unwrap_or("");

        println!(
            "[{}] {}",
            std::thread::current().name().unwrap_or("unknown"),
            request_line
        );

        // Increment request counter
        stats.lock().unwrap().increment_requests();

        // Route handling
        let response = match request_line {
            line if line.starts_with("GET / ") => handle_root(),
            line if line.starts_with("GET /hello") => handle_hello(),
            line if line.starts_with("GET /sleep") => handle_sleep(),
            line if line.starts_with("GET /stats") => {
                let stats = stats.lock().unwrap();
                handle_stats(&stats)
            }
            line if line.starts_with("POST /echo") => handle_echo(&request),
            _ => handle_404(),
        };

        // Send response
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write response: {}", e);
        }

        if let Err(e) = stream.flush() {
            eprintln!("Failed to flush stream: {}", e);
        }
    }
}

// Route handlers

fn handle_root() -> String {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Rust HTTP Server</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
            background: #f5f5f5;
        }
        h1 { color: #333; }
        .route {
            background: white;
            padding: 15px;
            margin: 10px 0;
            border-radius: 5px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        code {
            background: #e0e0e0;
            padding: 2px 6px;
            border-radius: 3px;
        }
    </style>
</head>
<body>
    <h1>🦀 Rust HTTP Server</h1>
    <p>A simple HTTP server written in Rust!</p>

    <h2>Available Routes:</h2>
    <div class="route">
        <strong>GET /</strong> - This page
    </div>
    <div class="route">
        <strong>GET /hello</strong> - Hello message
    </div>
    <div class="route">
        <strong>GET /sleep</strong> - Slow endpoint (demonstrates threading)
    </div>
    <div class="route">
        <strong>GET /stats</strong> - Server statistics
    </div>
    <div class="route">
        <strong>POST /echo</strong> - Echo the request body
    </div>

    <h2>Try it:</h2>
    <code>curl http://localhost:7878/hello</code><br>
    <code>curl http://localhost:7878/stats</code><br>
    <code>curl -X POST -d "Hello Rust!" http://localhost:7878/echo</code>
</body>
</html>
    "#;

    http_response(200, "OK", "text/html", html)
}

fn handle_hello() -> String {
    let json = r#"{"message": "Hello from Rust!", "status": "success"}"#;
    http_response(200, "OK", "application/json", json)
}

fn handle_sleep() -> String {
    // Simulate slow operation
    thread::sleep(Duration::from_secs(5));

    let json = r#"{"message": "Woke up after 5 seconds!", "status": "success"}"#;
    http_response(200, "OK", "application/json", json)
}

fn handle_stats(stats: &ServerStats) -> String {
    let json = format!(
        r#"{{"total_requests": {}, "uptime_seconds": {}}}"#,
        stats.total_requests,
        stats.start_time.elapsed().as_secs()
    );
    http_response(200, "OK", "application/json", &json)
}

fn handle_echo(request: &str) -> String {
    // Find body (after \r\n\r\n)
    let body = request
        .split("\r\n\r\n")
        .nth(1)
        .unwrap_or("")
        .trim_end_matches('\0');

    let json = format!(r#"{{"echo": "{}"}}"#, body);
    http_response(200, "OK", "application/json", &json)
}

fn handle_404() -> String {
    let html = r#"
<!DOCTYPE html>
<html>
<head><title>404 Not Found</title></head>
<body>
    <h1>404 - Not Found</h1>
    <p>The requested resource was not found.</p>
    <a href="/">Go home</a>
</body>
</html>
    "#;

    http_response(404, "Not Found", "text/html", html)
}

// HTTP response builder
fn http_response(status_code: u16, status_text: &str, content_type: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        status_text,
        content_type,
        body.len(),
        body
    )
}

// Thread Pool Implementation

struct ThreadPool {
    _workers: Vec<Worker>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { _workers: workers }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Simplified: just spawn a new thread
        // In production, use a proper thread pool
        thread::spawn(f);
    }
}

struct Worker {
    _id: usize,
}

impl Worker {
    fn new(id: usize) -> Self {
        Worker { _id: id }
    }
}

// Server Statistics

struct ServerStats {
    total_requests: u64,
    start_time: std::time::Instant,
}

impl ServerStats {
    fn new() -> Self {
        ServerStats {
            total_requests: 0,
            start_time: std::time::Instant::now(),
        }
    }

    fn increment_requests(&mut self) {
        self.total_requests += 1;
    }
}

/*
HTTP Server Summary:
====================

FEATURES:
- Multi-threaded request handling
- Multiple routes (GET, POST)
- JSON responses
- HTML pages
- Request statistics
- Proper HTTP protocol

CONCEPTS DEMONSTRATED:
- TCP networking (TcpListener, TcpStream)
- Threading (thread::spawn)
- Mutex for shared state
- Arc for shared ownership
- String parsing
- HTTP protocol basics

ARCHITECTURE:
1. Main thread accepts connections
2. Thread pool spawns worker threads
3. Each connection handled concurrently
4. Shared statistics with Mutex

IMPROVEMENTS FOR PRODUCTION:
- Use a real HTTP library (hyper, actix-web)
- Proper thread pool (rayon, tokio)
- Better error handling
- Logging
- Configuration
- HTTPS support
- Request parsing library
- Routing framework

RUN THIS:
    cargo run

TEST WITH:
    curl http://localhost:7878/
    curl http://localhost:7878/hello
    curl http://localhost:7878/stats
    curl -X POST -d "test data" http://localhost:7878/echo

    # Test threading:
    curl http://localhost:7878/sleep &
    curl http://localhost:7878/hello

BROWSER:
    Open http://localhost:7878 in your browser

KEY LEARNINGS:
- TCP connection handling
- HTTP protocol structure
- Concurrent request processing
- Thread-safe shared state
- Basic routing

COMPARISON TO OTHER LANGUAGES:
Node.js:  http.createServer(), single-threaded
Python:   Flask/Django, WSGI servers
Go:       net/http, goroutines
Rust:     Manual but safe, zero-cost threads

NEXT STEPS:
- Add middleware support
- Implement WebSocket
- Add static file serving
- Build a REST API
- Add authentication
*/
