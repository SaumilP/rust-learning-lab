# Channel Patterns

Comprehensive examples of Tokio's different channel types and their use cases.

## Channel Types Covered

1. **mpsc** - Multiple producers, single consumer
2. **broadcast** - Multiple producers, multiple consumers
3. **watch** - Single producer, multiple consumers (state updates)
4. **oneshot** - Single-use channel for request-response

## Use Cases

### MPSC
- Task queues
- Event aggregation
- Command patterns
- Worker pools

### Broadcast
- Event notifications
- Pub/sub systems
- Cache invalidation
- Real-time updates

### Watch
- Configuration updates
- State synchronization
- Signal propagation
- Shared state observation

### Oneshot
- Request-response pattern
- Future results
- Graceful shutdown signals
- One-time notifications

## Running

```bash
cargo run
```

## Key Differences

| Channel | Producers | Consumers | Behavior |
|---------|-----------|-----------|----------|
| mpsc | Many | One | Queue of messages |
| broadcast | Many | Many | All consumers get all messages |
| watch | One | Many | Only latest value |
| oneshot | One | One | Single message |
