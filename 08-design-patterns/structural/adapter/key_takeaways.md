# Adapter Pattern - Key Takeaways

## Quick Reference

| Aspect | Details |
|--------|---------|
| **Purpose** | Make incompatible interfaces work together |
| **When to Use** | Integrating legacy code or third-party libs |
| **Key Benefit** | Non-intrusive interface conversion |
| **Rust Pattern** | Composition with trait implementation |

## Implementation Strategy

```rust
pub struct Adapter<T> {
    inner: T,
}

impl<T: OldTrait> NewTrait for Adapter<T> {
    fn new_method(&self) -> String {
        self.inner.old_method().to_uppercase()
    }
}
```

## Adapter Patterns

| Pattern | Usage |
|---------|-------|
| Wrapper Adapter | Composition wrapping |
| Extension Trait | Add methods to existing types |
| Type Adapter | Convert between types |
| Protocol Adapter | Bridge protocol differences |

## Real-World Use Cases

- Making JSON work as database rows
- Adapting file system to virtual storage
- Converting between API versions
- Bridging different crate interfaces

## Key Points

✓ Non-intrusive to original types
✓ Composition over inheritance
✓ Use trait objects for dynamic adaptation
✓ One-way or two-way conversion
✓ Transparent to clients

## Avoid

✗ Complex multi-step adaptations
✗ Adapting when code could change instead
✗ Leaky abstractions exposing wrapped type
