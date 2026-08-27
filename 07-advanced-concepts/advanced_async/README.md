# Section 15: Advanced Async Patterns

## Overview

Master advanced asynchronous programming patterns in Rust using Tokio, async-std, and other async runtimes. Learn to build high-performance concurrent systems with proper error handling, cancellation, and resource management.

## Why Advanced Async Matters

✅ **Scalability** - Handle millions of concurrent connections <br />
✅ **Efficiency** - Better resource utilization than threads <br />
✅ **Responsiveness** - Non-blocking I/O keeps systems responsive <br />
✅ **Composability** - Combine async operations elegantly <br />
✅ **Performance** - Avoid context switching overhead

## What You'll Learn

1. **Tokio Runtime** - Work stealing, executor internals
2. **Async Traits** - async fn in traits (AFIT)
3. **Streams** - Asynchronous iterators
4. **Channels** - mpsc, broadcast, watch, oneshot
5. **Select/Join** - Concurrent future composition
6. **Actors** - Message-passing concurrency
7. **Cancellation** - Graceful shutdown patterns
8. **Backpressure** - Flow control strategies
9. **Testing** - Async test patterns
10. **Performance** - Profiling and optimization

## Section Contents

### 01-tokio-runtime/
Deep dive into Tokio runtime internals

### 02-async-streams/
Stream processing and transformation

### 03-channel-patterns/
Different channel types and use cases

### 04-actor-system/
Building actor-based systems

### 05-cancellation/
Graceful cancellation and timeouts

### 06-backpressure/
Flow control and rate limiting

## Prerequisites

- Strong Rust fundamentals
- Understanding of async/await basics
- Familiarity with Tokio
- Concurrent programming concepts

## Key Concepts

### Tokio Runtime Architecture

```rust
use tokio::runtime::Runtime;

// Multi-threaded runtime (default)
let rt = Runtime::new()?;

// Single-threaded runtime
let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()?;

// Custom worker threads
let rt = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .thread_name("my-async-worker")
    .enable_all()
    .build()?;
```

### Async Traits (AFIT - Async Functions in Traits)

```rust
// Requires Rust 1.75+
trait AsyncService {
    async fn process(&self, data: String) -> Result<String, Error>;
}

struct MyService;

impl AsyncService for MyService {
    async fn process(&self, data: String) -> Result<String, Error> {
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok(data.to_uppercase())
    }
}

// Using async trait
async fn use_service<T: AsyncService>(service: &T) {
    let result = service.process("hello".to_string()).await;
}
```

### Streams - Async Iterators

```rust
use tokio_stream::{self as stream, StreamExt};
use futures::stream::Stream;

// Create stream from iterator
let mut stream = stream::iter(vec![1, 2, 3, 4, 5]);

// Transform stream
let doubled = stream.map(|x| x * 2);

// Filter stream
let evens = stream.filter(|x| x % 2 == 0);

// Collect stream
let collected: Vec<_> = stream.collect().await;

// Custom stream
async fn number_stream() -> impl Stream<Item = i32> {
    stream::iter(0..10)
        .then(|n| async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            n * n
        })
}
```

### Channel Patterns

```rust
use tokio::sync::{mpsc, broadcast, watch, oneshot};

// MPSC - Multiple producers, single consumer
let (tx, mut rx) = mpsc::channel(100);

tokio::spawn(async move {
    tx.send(42).await.unwrap();
});

if let Some(value) = rx.recv().await {
    println!("Got: {}", value);
}

// Broadcast - Multiple producers, multiple consumers
let (tx, mut rx1) = broadcast::channel(16);
let mut rx2 = tx.subscribe();

tx.send(10).unwrap();
assert_eq!(rx1.recv().await.unwrap(), 10);
assert_eq!(rx2.recv().await.unwrap(), 10);

// Watch - Single producer, multiple consumers (latest value)
let (tx, mut rx) = watch::channel("initial");

tokio::spawn(async move {
    tx.send("updated").unwrap();
});

rx.changed().await.unwrap();
println!("Value: {}", *rx.borrow());

// Oneshot - Single-use channel
let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    tx.send(123).unwrap();
});

let result = rx.await.unwrap();
```

### Select - Race Multiple Futures

```rust
use tokio::select;

async fn race_futures() {
    let fut1 = async { /* ... */ };
    let fut2 = async { /* ... */ };
    let fut3 = async { /* ... */ };

    select! {
        result = fut1 => {
            println!("fut1 completed first: {:?}", result);
        }
        result = fut2 => {
            println!("fut2 completed first: {:?}", result);
        }
        result = fut3 => {
            println!("fut3 completed first: {:?}", result);
        }
    }
}

// With timeout
use tokio::time::{timeout, Duration};

select! {
    result = some_async_operation() => {
        println!("Completed: {:?}", result);
    }
    _ = tokio::time::sleep(Duration::from_secs(5)) => {
        println!("Timeout!");
    }
}
```

### Join - Run Multiple Futures Concurrently

```rust
use tokio::join;

async fn fetch_user(id: u64) -> User { /* ... */ }
async fn fetch_posts(user_id: u64) -> Vec<Post> { /* ... */ }
async fn fetch_comments(user_id: u64) -> Vec<Comment> { /* ... */ }

async fn load_user_data(id: u64) {
    // All run concurrently, wait for all to complete
    let (user, posts, comments) = join!(
        fetch_user(id),
        fetch_posts(id),
        fetch_comments(id)
    );

    // Use results...
}

// With error handling
use futures::future::try_join;

let result = try_join!(
    fallible_operation_1(),
    fallible_operation_2(),
    fallible_operation_3()
).await?;
```

### Actor Pattern

```rust
use tokio::sync::mpsc;

enum Message {
    GetCount { respond_to: oneshot::Sender<u64> },
    Increment,
}

struct Counter {
    count: u64,
    receiver: mpsc::Receiver<Message>,
}

impl Counter {
    fn new(receiver: mpsc::Receiver<Message>) -> Self {
        Counter {
            count: 0,
            receiver,
        }
    }

    async fn run(mut self) {
        while let Some(msg) = self.receiver.recv().await {
            match msg {
                Message::GetCount { respond_to } => {
                    let _ = respond_to.send(self.count);
                }
                Message::Increment => {
                    self.count += 1;
                }
            }
        }
    }
}

// Actor handle
#[derive(Clone)]
struct CounterHandle {
    sender: mpsc::Sender<Message>,
}

impl CounterHandle {
    async fn get_count(&self) -> u64 {
        let (tx, rx) = oneshot::channel();
        self.sender.send(Message::GetCount { respond_to: tx }).await.unwrap();
        rx.await.unwrap()
    }

    async fn increment(&self) {
        self.sender.send(Message::Increment).await.unwrap();
    }
}

// Usage
let (tx, rx) = mpsc::channel(32);
let counter = Counter::new(rx);
tokio::spawn(counter.run());

let handle = CounterHandle { sender: tx };
handle.increment().await;
let count = handle.get_count().await;
```

### Cancellation and Timeout

```rust
use tokio::time::{timeout, Duration};
use tokio_util::sync::CancellationToken;

// Timeout pattern
async fn with_timeout() -> Result<Data, Error> {
    timeout(Duration::from_secs(5), fetch_data())
        .await
        .map_err(|_| Error::Timeout)?
}

// Cancellation token
async fn cancellable_task(token: CancellationToken) {
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                println!("Task cancelled, cleaning up...");
                break;
            }
            result = do_work() => {
                handle_result(result);
            }
        }
    }
}

// Graceful shutdown
async fn server_with_shutdown(shutdown: CancellationToken) {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                println!("Shutting down...");
                break;
            }
            Ok((socket, _)) = listener.accept() => {
                tokio::spawn(handle_connection(socket));
            }
        }
    }
}
```

### Backpressure and Flow Control

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

// Limit concurrent operations
async fn rate_limited_requests(urls: Vec<String>) {
    let semaphore = Arc::new(Semaphore::new(10)); // Max 10 concurrent

    let tasks: Vec<_> = urls.into_iter()
        .map(|url| {
            let permit = semaphore.clone();
            tokio::spawn(async move {
                let _permit = permit.acquire().await.unwrap();
                fetch_url(&url).await
            })
        })
        .collect();

    futures::future::join_all(tasks).await;
}

// Bounded channel for backpressure
let (tx, mut rx) = mpsc::channel(100);

// Producer blocks when buffer is full
tx.send(data).await?; // Applies backpressure
```

### Async Drop and Resource Cleanup

```rust
use tokio::task::JoinHandle;

struct AsyncResource {
    handle: Option<JoinHandle<()>>,
}

impl AsyncResource {
    fn new() -> Self {
        let handle = tokio::spawn(async {
            // Background task
        });

        AsyncResource {
            handle: Some(handle),
        }
    }

    async fn close(mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
            let _ = handle.await;
        }
    }
}
```

### Testing Async Code

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_operation().await;
    assert_eq!(result, expected);
}

// Test with timeout
#[tokio::test]
async fn test_with_timeout() {
    tokio::time::timeout(
        Duration::from_secs(1),
        async_operation()
    ).await.unwrap();
}

// Mock time
#[tokio::test(start_paused = true)]
async fn test_with_mock_time() {
    let start = tokio::time::Instant::now();
    tokio::time::sleep(Duration::from_secs(10)).await;
    let elapsed = start.elapsed();
    assert!(elapsed < Duration::from_millis(100)); // Fast in tests
}
```

## Advanced Patterns

### Async Recursion

```rust
use async_recursion::async_recursion;

#[async_recursion]
async fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1).await
    }
}
```

### Async State Machines

```rust
enum State {
    Start,
    Fetching(JoinHandle<Data>),
    Processing(Data),
    Done(Result),
}

async fn state_machine() {
    let mut state = State::Start;

    loop {
        state = match state {
            State::Start => {
                let handle = tokio::spawn(fetch_data());
                State::Fetching(handle)
            }
            State::Fetching(handle) => {
                let data = handle.await.unwrap();
                State::Processing(data)
            }
            State::Processing(data) => {
                let result = process(data).await;
                State::Done(result)
            }
            State::Done(result) => {
                return result;
            }
        }
    }
}
```

### Fan-Out/Fan-In

```rust
// Fan-out: Distribute work to multiple workers
async fn fan_out<T>(items: Vec<T>) -> Vec<Result<Output>> {
    let futures: Vec<_> = items.into_iter()
        .map(|item| tokio::spawn(process(item)))
        .collect();

    futures::future::join_all(futures).await
}

// Fan-in: Collect results from multiple sources
async fn fan_in(sources: Vec<Source>) -> Vec<Item> {
    let mut streams: Vec<_> = sources.into_iter()
        .map(|s| s.stream())
        .collect();

    stream::select_all(streams)
        .collect()
        .await
}
```

## Performance Tips

1. **Avoid Blocking** - Never use blocking I/O in async code
2. **Task Granularity** - Don't spawn too many tiny tasks
3. **Minimize Allocations** - Reuse buffers when possible
4. **Channel Sizing** - Choose appropriate buffer sizes
5. **Worker Threads** - Match CPU count for CPU-bound work
6. **Lazy Initialization** - Use `tokio::task::spawn_blocking` for expensive setup
7. **Profile** - Use `tokio-console` for debugging

## Common Pitfalls

### Blocking in Async Context

```rust
// ❌ Wrong: Blocks async runtime
async fn bad() {
    std::thread::sleep(Duration::from_secs(1));
}

// ✅ Correct: Non-blocking
async fn good() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

### Forgetting to Await

```rust
// ❌ Wrong: Future not executed
async fn bad() {
    expensive_operation(); // Does nothing!
}

// ✅ Correct: Actually runs
async fn good() {
    expensive_operation().await;
}
```

### Holding Locks Across Await

```rust
// ❌ Wrong: Can cause deadlocks
async fn bad(mutex: &Mutex<Data>) {
    let guard = mutex.lock().unwrap();
    some_async_op().await; // Still holding lock!
    drop(guard);
}

// ✅ Correct: Release lock before await
async fn good(mutex: &Mutex<Data>) {
    let data = {
        let guard = mutex.lock().unwrap();
        guard.clone()
    };
    some_async_op().await;
}
```

## Resources

- [Tokio Documentation](https://tokio.rs/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [tokio-console](https://github.com/tokio-rs/console)
- [async-trait crate](https://docs.rs/async-trait/)
- [futures crate](https://docs.rs/futures/)

## Next Steps

After completing this section:
- Build a high-performance async server
- Implement custom async runtimes
- Explore embassy for embedded async
- Study zero-copy async patterns

---

**Estimated Time**: 20-28 hours
**Difficulty**: ★★★★★ (Expert)
**Prerequisites**: Strong async/await knowledge, concurrent programming
