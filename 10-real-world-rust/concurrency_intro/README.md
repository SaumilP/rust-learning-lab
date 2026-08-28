# Concurrency Introduction

Rust prevents data races by enforcing ownership rules across threads. Move
owned data into a thread, use channels for message passing, and share state only
when the state genuinely needs shared ownership.

```rust
use std::sync::mpsc;
use std::thread;

let (sender, receiver) = mpsc::channel();
thread::spawn(move || sender.send(42).unwrap());
assert_eq!(receiver.recv().unwrap(), 42);
```

Use `Arc<T>` for shared ownership and `Mutex<T>` or `RwLock<T>` for protected
mutation. Keep lock scopes short and never hold a synchronous lock across an
`.await`. Choose async tasks for waiting-heavy workloads and threads or a
parallelism library for CPU-heavy work.
