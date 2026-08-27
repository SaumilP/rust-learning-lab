# Actor System Pattern

Building concurrent systems using the actor model with Tokio.

## What This Demonstrates

- Message-passing concurrency
- Actor lifecycle management
- Supervisor patterns
- Actor handles and communication
- Graceful shutdown

## Actors Implemented

1. **Counter Actor** - Simple stateful actor
2. **Worker Pool** - Load-balanced task distribution
3. **Supervisor** - Monitors and restarts failed actors

## Running

```bash
cargo run
```

## Key Concepts

### Actor Benefits

- **Isolation** - Each actor has private state
- **Concurrency** - Actors run concurrently
- **Location Transparency** - Actors communicate via messages
- **Fault Tolerance** - Actors can be supervised and restarted

### Message Types

```rust
enum CounterMessage {
    Increment,
    Decrement,
    GetCount { respond_to: oneshot::Sender<i64> },
    Reset,
    Shutdown,
}
```

### Actor Pattern vs Shared State

**Shared State**:
- Locks required
- Potential deadlocks
- Complex error handling

**Actor Model**:
- No shared mutable state
- Message queues prevent race conditions
- Isolated failure domains

## Real-World Use Cases

- Web servers handling concurrent requests
- Game servers with entity systems
- Chat applications with user sessions
- Background job processing
