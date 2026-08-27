# Strategy Pattern - Key Takeaways

## Quick Reference

| Aspect | Details |
|--------|---------|
| **Purpose** | Encapsulate interchangeable algorithms |
| **When to Use** | Multiple implementations of same task |
| **Key Benefit** | Eliminates conditional logic |
| **Rust Pattern** | Box<dyn Strategy> trait objects |

## Implementation Template

```rust
pub trait Strategy {
    fn execute(&self) -> Result<String, Error>;
}

pub struct Context {
    strategy: Box<dyn Strategy>,
}

impl Context {
    pub fn set_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategy = strategy;
    }
}
```

## Strategy Selection Patterns

| Method | Best For |
|--------|----------|
| String match | Configuration-driven |
| Enum | Type-safe selection |
| Factory function | Complex creation logic |
| Builder pattern | Complex configuration |

## Real-World Examples

- Payment methods (credit card, PayPal, crypto)
- Sorting algorithms (quick, merge, bubble)
- Compression formats (ZIP, GZIP, BZIP2)
- Validation rules (email, phone, URL)

## Key Points

✓ Strategy encapsulates single algorithm
✓ Runtime strategy switching
✓ Each strategy independent and testable
✓ Box<dyn Trait> for polymorphism
✓ Eliminates if/else chains

## Avoid

✗ Strategies sharing mutable state
✗ Complex strategy selection logic
✗ Creating strategy per operation
✗ Strategies with side effects
