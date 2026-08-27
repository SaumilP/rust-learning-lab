# Observer Pattern - Key Takeaways

## Quick Reference

| Aspect | Details |
|--------|---------|
| **Purpose** | Notify multiple objects of state changes |
| **When to Use** | Event systems, MVC pattern |
| **Key Benefit** | Loose coupling between objects |
| **Rust Pattern** | Rc<dyn Observer> with RefCell |

## Implementation Pattern

```rust
pub trait Observer {
    fn update(&self, subject_state: &str);
}

pub struct Subject {
    state: String,
    observers: RefCell<Vec<Rc<dyn Observer>>>,
}

impl Subject {
    pub fn notify_observers(&self) {
        let observers = self.observers.borrow();
        for observer in observers.iter() {
            observer.update(&self.state);
        }
    }
}
```

## Observer Implementations

| Type | Usage |
|------|-------|
| Simple Observer | Trait object collection |
| Event Emitter | String-based event dispatch |
| Typed Observer | Generic event types |
| Subscription | RAII subscription cleanup |

## Real-World Examples

- UI state changes notify views
- Model changes notify observers
- Price changes notify traders
- Event system for decoupled communication

## Key Points

✓ Subject manages observer collection
✓ Observers notified on state change
✓ Rc<RefCell<>> for shared mutable state
✓ Add/remove observers dynamically
✓ Loose coupling between subject and observers

## Avoid

✗ Memory leaks from circular references
✗ Observer modifying subject state
✗ Relying on observer notification order
✗ Infinite notification loops
