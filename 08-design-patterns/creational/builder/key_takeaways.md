# Builder Pattern - Key Takeaways

## Quick Reference

| Aspect | Details |
|--------|---------|
| **Purpose** | Construct complex objects step by step |
| **When to Use** | Objects with many optional/required fields |
| **Key Benefit** | Readable, safe object construction |
| **Rust Pattern** | Method chaining with `mut self` |

## Essential Code Pattern

```rust
pub struct Builder {
    field1: String,
    field2: u32,
}

impl Builder {
    pub fn field1(mut self, val: String) -> Self {
        self.field1 = val;
        self
    }
    
    pub fn build(self) -> ComplexObject {
        ComplexObject { /* ... */ }
    }
}
```

## Common Use Cases

1. **Configuration objects** - Database config, HTTP client config
2. **Complex domain objects** - Order, User with many optional fields
3. **API builders** - Request builders with flexible options
4. **Immutable objects** - Build then freeze

## Patterns to Remember

| Pattern | Use Case |
|---------|----------|
| Fluent Builder | Chainable method calls |
| Typed Builder | Compile-time required fields |
| Validation Builder | Validate in build() method |
| Builder with Defaults | Pre-configured starting state |

## Implementation Tips

- Return `Self` from setter methods for chaining
- Use `Box::new()` when needed for trait objects
- Consider validation in `build()` not setters
- Make builder fields public for easy access
- Use builder for 3+ configuration options

## Common Mistakes

- **Missing required fields** - Use type state pattern to enforce at compile time
- **Over-use** - Don't use for simple objects (1-2 fields)
- **Validation timing** - Validate in build() not individual setters
- **Immutability loss** - Builder creates immutable object, don't expose internals

## Real-World Examples

| Library | Pattern |
|---------|---------|
| reqwest | HTTP client builder |
| sqlx | Query builder |
| tokio | Runtime builder |

## Key Points

✓ Fluent interface with method chaining
✓ Chainable via returning Self
✓ Separates construction from representation
✓ Optional and required fields easy to manage
✓ Great for reducing parameter lists
