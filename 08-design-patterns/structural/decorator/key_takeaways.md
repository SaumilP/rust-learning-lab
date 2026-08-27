# Decorator Pattern - Key Takeaways

## Quick Reference

| Aspect | Details |
|--------|---------|
| **Purpose** | Add behavior to objects dynamically |
| **When to Use** | Multiple optional features/combinations |
| **Key Benefit** | Compose features without subclasses |
| **Rust Pattern** | Trait object wrapping with trait impl |

## Implementation Template

```rust
pub struct Decorator<T: Trait> {
    inner: T,
}

impl<T: Trait> Trait for Decorator<T> {
    fn method(&self) -> String {
        format!("Decorated({})", self.inner.method())
    }
}
```

## Decorator Types

| Type | Use Case |
|------|----------|
| Generic Decorator | Type-safe wrapping |
| Trait Object | Runtime polymorphism |
| Stateful | Decorator with internal state |
| Functional | Closure-based decoration |

## Real-World Examples

- Logging decorator wrapping HTTP handlers
- Compression decorator for file I/O
- Caching decorator for database queries
- Authentication decorator for API routes

## Key Points

✓ Stack decorators for feature combinations
✓ Each decorator adds single concern
✓ Transparent to outer clients
✓ Order matters in decorator chain
✓ Decorator itself implements trait

## Avoid

✗ Deep decorator nesting (hard to debug)
✗ Decorators modifying shared state
✗ Circular decorator dependencies
✗ Decorator doing too much

