# Factory Pattern - Key Takeaways

## Quick Reference

| Aspect | Details |
|--------|---------|
| **Purpose** | Create objects without specifying exact classes |
| **When to Use** | Multiple related object types |
| **Key Benefit** | Decouples creation from usage |
| **Rust Pattern** | Associated functions or trait methods |

## Core Implementations

### Simple Factory Function
```rust
pub fn create_shape(shape_type: &str) -> Option<Box<dyn Shape>> {
    match shape_type {
        "circle" => Some(Box::new(Circle::new())),
        "square" => Some(Box::new(Square::new())),
        _ => None,
    }
}
```

### Associated Functions
```rust
impl Database {
    pub fn dev() -> Self { /* development config */ }
    pub fn prod() -> Self { /* production config */ }
}
```

## Factory Patterns

| Pattern | Usage |
|---------|-------|
| Function Factory | String-based type selection |
| Associated Functions | Named constructors (::dev, ::prod) |
| Enum-based Factory | Type-safe factory dispatch |
| Config Factory | Configuration-driven creation |
| Builder + Factory | Complex object creation |

## When to Use Factory

✓ Multiple concrete types implement same trait
✓ Creation logic is complex
✓ Object type determined at runtime
✓ Want to hide implementation details
✓ Planning to add new types frequently

## Real-World Examples

- Database connection pools (PostgreSQL, MySQL, SQLite)
- HTTP clients (standard, cached, resilient)
- Logging backends (console, file, database)
- Plugin systems with dynamic loading

## Key Points

✓ Trait objects for runtime polymorphism
✓ match on string or enum for dispatch
✓ Hide concrete types from clients
✓ Centralized creation logic
✓ Combine with builder for complex setup

## Avoid

✗ Overcomplicating simple object creation
✗ Factory that doesn't hide anything
✗ Unnecessary factory for single implementation
✗ Complex factory logic in production code
