# Command Pattern

## Overview

The Command pattern encapsulates a request as an object, thereby letting you parameterize clients with different requests, queue requests, and support undoable operations. It decouples sender from receiver.

## Problem It Solves

- Parameterizing objects with operations
- Queuing, logging, and undoing operations
- Supporting macro commands (command composition)
- Decoupling sender and receiver
- Creating transactional systems

## Implementation in Rust

### Basic Command Pattern

```rust
// Command trait
pub trait Command {
    fn execute(&mut self);
    fn undo(&mut self);
}

// Receiver - the object that performs work
pub struct Light {
    is_on: bool,
}

impl Light {
    pub fn new() -> Self {
        Self { is_on: false }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
        println!("Light is on");
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
        println!("Light is off");
    }
}

// Concrete commands
pub struct TurnOnCommand {
    light: Light,
}

impl TurnOnCommand {
    pub fn new(light: Light) -> Self {
        Self { light }
    }
}

impl Command for TurnOnCommand {
    fn execute(&mut self) {
        self.light.turn_on();
    }

    fn undo(&mut self) {
        self.light.turn_off();
    }
}

pub struct TurnOffCommand {
    light: Light,
}

impl TurnOffCommand {
    pub fn new(light: Light) -> Self {
        Self { light }
    }
}

impl Command for TurnOffCommand {
    fn execute(&mut self) {
        self.light.turn_off();
    }

    fn undo(&mut self) {
        self.light.turn_on();
    }
}

// Invoker - executes commands
pub struct RemoteControl {
    commands: Vec<Box<dyn Command>>,
}

impl RemoteControl {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    pub fn execute_commands(&mut self) {
        for command in &mut self.commands {
            command.execute();
        }
    }
}
```

## Benefits

1. **Decoupling** - Decouples sender from receiver
2. **Queueing** - Commands can be queued and executed later
3. **Undo/Redo** - Can be extended to support undo/redo operations
4. **Macro Commands** - Composite commands from simpler ones
5. **Logging** - Can log all executed commands

## Common Patterns

### Pattern 1: Simple Command
```rust
pub trait Command {
    fn execute(&self);
}

pub struct SimpleCommand {
    receiver: Receiver,
}
```

### Pattern 2: Undoable Command
```rust
pub trait UndoableCommand {
    fn execute(&mut self);
    fn undo(&mut self);
}
```

### Pattern 3: Command with State
```rust
pub struct StatefulCommand {
    state: CommandState,
}

pub enum CommandState {
    Pending,
    Executing,
    Completed,
}
```

### Pattern 4: Async Command
```rust
pub trait AsyncCommand {
    async fn execute(&self);
}
```

### Pattern 5: Macro Command
```rust
pub struct MacroCommand {
    commands: Vec<Box<dyn Command>>,
}

impl Command for MacroCommand {
    fn execute(&self) {
        for command in &self.commands {
            command.execute();
        }
    }
}
```

## Real-World Examples

### Text Editor Commands
```rust
pub trait EditorCommand {
    fn execute(&mut self, editor: &mut Editor);
    fn undo(&mut self, editor: &mut Editor);
}

pub struct CopyCommand {
    text: String,
}

impl EditorCommand for CopyCommand {
    fn execute(&mut self, editor: &mut Editor) {
        self.text = editor.get_selected_text();
    }

    fn undo(&mut self, editor: &mut Editor) {
        // Undo copy
    }
}
```

### Transaction Command
```rust
pub struct TransactionCommand {
    account: Account,
    amount: f64,
}

impl Command for TransactionCommand {
    fn execute(&mut self) {
        self.account.debit(self.amount);
    }

    fn undo(&mut self) {
        self.account.credit(self.amount);
    }
}
```

### Batch Processing
```rust
pub struct BatchProcessor {
    queue: Vec<Box<dyn Command>>,
}

impl BatchProcessor {
    pub fn queue_command(&mut self, command: Box<dyn Command>) {
        self.queue.push(command);
    }

    pub fn process_batch(&mut self) {
        while let Some(mut command) = self.queue.pop() {
            command.execute();
        }
    }
}
```

## Anti-Patterns to Avoid

- **Too Many Command Types** - Command explosion
- **No Clear Receiver** - Command doesn't know who to invoke
- **Undo/Redo Complexity** - Can become difficult to manage state
- **Command Coupling** - Commands too tightly coupled to receivers

## Key Takeaways

- ✓ Command encapsulates requests as objects
- ✓ Decouples sender from receiver
- ✓ Enables queuing, logging, and undo operations
- ✓ Use trait objects for polymorphism
- ✓ Maintain clear separation between command and receiver
- ✓ Consider command history for undo/redo
- ✓ Can be combined with macro commands for complex operations

