# State Pattern - Key Takeaways

## Quick Reference

| Aspect | Details |
|--------|---------|
| **Purpose** | Alter behavior based on internal state |
| **When to Use** | State machines, workflow systems |
| **Key Benefit** | Encapsulates state-specific behavior |
| **Rust Pattern** | Box<dyn State> with owned Box pattern |

## Implementation Pattern

```rust
pub trait State {
    fn handle(&self, context: &mut Context);
}

pub struct Context {
    state: Box<dyn State>,
}

impl Context {
    pub fn request(&mut self) {
        self.state.handle(self);
    }
    
    pub fn transition(&mut self, new_state: Box<dyn State>) {
        self.state = new_state;
    }
}
```

## State Patterns

| Type | Usage |
|------|-------|
| Simple State | Basic behavior switching |
| State with Entry/Exit | Lifecycle callbacks |
| Typed State | Compile-time safety |
| Hierarchical States | Nested state machines |

## State Machine Examples

- Order workflow (pending → confirmed → shipped → delivered)
- TCP connection (listening → established → closing → closed)
- Traffic light (red → green → yellow → red)

## Key Points

✓ State encapsulates specific behaviors
✓ Invalid transitions throw errors or panic
✓ Transitions create new state objects
✓ Eliminates large if/else on state
✓ Clear state diagram representation

## Avoid

✗ Too many states (use composition)
✗ Complex state transition logic
✗ States sharing mutable context
✗ Infinite state loops (design error)
