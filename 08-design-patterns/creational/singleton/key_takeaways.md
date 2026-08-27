# Singleton Pattern - Key Takeaways

## Quick Reference

| Aspect | Details |
|--------|---------|
| **Purpose** | Ensure single instance with global access |
| **When to Use** | Expensive resources (logger, config) |
| **Key Benefit** | Thread-safe lazy initialization |
| **Rust Pattern** | once_cell::Lazy for immutable |

## Modern Rust Implementation

```rust
use once_cell::sync::Lazy;

pub static INSTANCE: Lazy<Type> = Lazy::new(|| {
    Type::new()
});
```

## Singleton Types

| Type | Use Case |
|------|----------|
| Immutable Static | Read-only config |
| Mutex-wrapped | Mutable shared state |
| Arc<Mutex<T>> | Thread-safe mutable |

## Real-World Examples

- Global logger instance
- Database connection pool
- Application configuration
- Metrics collector
- Service registry

## Key Points

✓ Use once_cell for lazy initialization
✓ Prefer immutable singletons
✓ Thread-safe by default with proper tools
✓ Avoid multiple interdependent singletons
✓ Consider if singleton really needed

## Avoid

✗ Mutable global state (test nightmare)
✗ Multiple conflicting singletons
✗ Singletons in library code
✗ Hidden dependencies via singletons
✗ Manual initialization complexity
