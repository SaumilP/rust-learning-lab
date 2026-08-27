# Mini-Project 2: Multithreaded Web Server

## Overview

Build a simple HTTP server that handles multiple concurrent requests using thread pools and message passing. This project demonstrates Rust's fearless concurrency and how it compares to Java's `ExecutorService` pattern.

## What you'll learn

1. **Thread pools** - Building a worker pool (like Java's `ExecutorService`)
2. **Channels** - Message passing between threads (`mpsc`)
3. **Arc and Mutex** - Shared state across threads (thread-safe reference counting)
4. **TCP networking** - Low-level socket programming
5. **Trait objects** - Dynamic dispatch for request handlers
6. **Error handling in concurrent contexts** - Propagating errors across threads

## Features

- ✅ TCP server listening on configurable port
- ✅ Thread pool with configurable worker count
- ✅ Request routing based on HTTP method and path
- ✅ Static file serving
- ✅ Graceful shutdown
- ✅ Request/response logging
- ✅ Thread-safe connection counting

## Java vs Rust: Concurrency Comparison

### Thread Pool Implementation

**Java (ExecutorService)**:
```java
ExecutorService executor = Executors.newFixedThreadPool(4);

for (int i = 0; i < 100; i++) {
    executor.submit(() -> {
        // Handle request
        processRequest();
    });
}

executor.shutdown();
executor.awaitTermination(1, TimeUnit.MINUTES);
```

**Rust (Custom Thread Pool)**:
```rust
let pool = ThreadPool::new(4);

for _ in 0..100 {
    pool.execute(|| {
        // Handle request
        process_request();
    });
}

// Pool automatically shuts down when dropped
```

### Shared State

**Java (synchronized)**:
```java
public class ConnectionCounter {
    private int count = 0;

    public synchronized void increment() {
        count++;
    }

    public synchronized int getCount() {
        return count;
    }
}
```

**Rust (Arc<Mutex<T>>)**:
```rust
pub struct ConnectionCounter {
    count: Arc<Mutex<u32>>,
}

impl ConnectionCounter {
    pub fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }

    pub fn get_count(&self) -> u32 {
        *self.count.lock().unwrap()
    }
}
```

**Key difference**: Rust's type system enforces thread safety at compile time. You can't forget to synchronize.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      TCP Listener                       │
│                    (main thread)                        │
└────────────────┬────────────────────────────────────────┘
                 │
                 │ Accepts connections
                 ▼
┌─────────────────────────────────────────────────────────┐
│                      Thread Pool                        │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐     │
│  │Worker 1 │  │Worker 2 │  │Worker 3 │  │Worker 4 │     │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘     │
└─────────────────────────────────────────────────────────┘
                 │
                 │ mpsc::channel
                 ▼
┌─────────────────────────────────────────────────────────┐
│                   Request Handlers                      │
│     (Process HTTP requests and generate responses)      │
└─────────────────────────────────────────────────────────┘
```

## Project Structure

```
02-multithreaded-server/
├── Cargo.toml
├── src/
│   ├── main.rs           # Server entry point
│   ├── lib.rs            # Public API
│   ├── thread_pool.rs    # Thread pool implementation
│   ├── http.rs           # HTTP request/response parsing
│   ├── router.rs         # Request routing
│   └── handlers.rs       # Request handlers
├── static/
│   └── index.html        # Static files to serve
└── tests/
    └── integration_test.rs
```

## Running the Project

```bash
# Build and run
cargo run

# In another terminal, test with curl:
curl http://localhost:7878
curl http://localhost:7878/hello
curl http://localhost:7878/static/index.html

# Load test (requires Apache Bench)
ab -n 1000 -c 10 http://localhost:7878/

# Run tests
cargo test
```

## Key Concepts

### 1. Thread Safety Without `synchronized`

In Java, you manually add `synchronized` keywords. In Rust, the compiler enforces it:

```rust
// This won't compile if you try to share mutable state unsafely
let counter = Arc::new(Mutex::new(0));

thread::spawn(move || {
    let mut count = counter.lock().unwrap();
    *count += 1;
    // Mutex guard automatically released here
});
```

### 2. Message Passing with Channels

```rust
use std::sync::mpsc;

let (sender, receiver) = mpsc::channel();

thread::spawn(move || {
    sender.send("Hello from thread").unwrap();
});

let message = receiver.recv().unwrap();
println!("{}", message);
```

### 3. Graceful Shutdown

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Drop sender to signal workers to shut down
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

## Challenges

1. **Add request timeout** - Cancel requests that take too long
2. **Implement keep-alive** - Reuse connections
3. **Add middleware** - Logging, authentication, compression
4. **WebSocket support** - Upgrade HTTP connections
5. **HTTPS support** - Use `rustls` for TLS

## Performance Comparison

Expected results (approximate):

| Implementation | Requests/sec | Memory (MB) |
|----------------|--------------|-------------|
| Java (Tomcat)  | ~8,000      | ~150        |
| Rust (this)    | ~15,000     | ~5          |

**Why Rust is faster**: No GC pauses, zero-cost abstractions, stack allocation

## Common Pitfalls

### Deadlocks

```rust
// ❌ Can deadlock
let data = Arc::new(Mutex::new(0));
let d1 = data.lock().unwrap();
let d2 = data.lock().unwrap();  // Deadlock!
```

### Sharing Non-Send Types

```rust
// ❌ Won't compile - Rc is not thread-safe
let rc = Rc::new(5);
thread::spawn(move || {
    println!("{}", rc);  // ERROR: Rc<T> is not Send
});

// ✅ Use Arc instead
let arc = Arc::new(5);
thread::spawn(move || {
    println!("{}", arc);  // OK: Arc<T> is Send
});
```

## Resources

- [Rust Book Chapter 16: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust Book Chapter 20: Final Project - Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [Thread Pool Pattern](https://doc.rust-lang.org/book/ch20-02-multithreaded.html)

---

**Estimated time**: 8-12 hours

**Difficulty**: ★★★☆☆ (Intermediate)

**Prerequisites**: Complete Mini-Project 1 (Todo CLI)
