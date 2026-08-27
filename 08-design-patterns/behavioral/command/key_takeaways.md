# Command Pattern - Key Takeaways

## Quick Reference

| Aspect | Details |
|--------|---------|
| **Purpose** | Encapsulate requests as objects |
| **When to Use** | Undo/redo, command queuing, scheduling |
| **Key Benefit** | Decouples sender from receiver |
| **Rust Pattern** | Trait objects queued in vectors |

## Implementation Pattern

```rust
pub trait Command {
    fn execute(&mut self);
    fn undo(&mut self);
}

pub struct Invoker {
    commands: Vec<Box<dyn Command>>,
}

impl Invoker {
    pub fn execute_all(&mut self) {
        for cmd in &mut self.commands {
            cmd.execute();
        }
    }
}
```

## Command Types

| Type | Use Case |
|------|----------|
| Simple Command | Single action execution |
| Undoable Command | Undo/redo support |
| Macro Command | Composite commands |
| Async Command | Asynchronous execution |

## Real-World Examples

- Text editor undo/redo
- Transaction processing
- Job scheduling and queueing
- Macro recording and playback

## Key Points

✓ Command encapsulates request
✓ Invoker executes without knowing details
✓ Receiver performs actual work
✓ Queue commands for batch processing
✓ Undo support via reversible commands

## Avoid

✗ Commands with side effects
✗ Circular command dependencies
✗ Complex undo logic
✗ Commands modifying shared state
