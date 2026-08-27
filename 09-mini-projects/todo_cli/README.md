# Todo CLI Application

## Overview

A command-line todo list manager with persistence. Demonstrates practical Rust for everyday utilities.

## Concepts Learned

- **File I/O**: Read/write JSON data structures
- **Error Handling**: Result types for operations
- **Collections**: Vec for todo storage
- **Enums**: Status states (Pending, Completed)
- **Structs**: Todo and TodoList data models
- **Traits**: Serialization/Display implementations
- **CLI Parsing**: Command and argument handling

## Features

1. **Add Todo**: Create new task with optional priority
2. **List Todos**: Display all or filtered todos
3. **Mark Complete**: Update todo status
4. **Delete Todo**: Remove completed tasks
5. **Save/Load**: Persist todos to JSON file

## Data Structures

```rust
#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: u32,
    title: String,
    description: String,
    status: Status,
    priority: Priority,
    created_at: String,
}

pub enum Status {
    Pending,
    Completed,
}

pub enum Priority {
    Low,
    Medium,
    High,
}
```

## Commands

```bash
# Add a todo
cargo run -- add "Task title" --priority high

# List all todos
cargo run -- list

# List pending only
cargo run -- list --pending

# Complete a todo
cargo run -- complete 1

# Delete a todo
cargo run -- delete 1

# Search todos
cargo run -- search "keyword"
```

## Implementation Details

### Module Organization
```
src/
├── main.rs          # CLI entry point
├── models.rs        # Data structures
├── storage.rs       # File I/O
├── operations.rs    # Business logic
└── cli.rs           # Command parsing
```

### Key Functions

```rust
pub fn add_todo(list: &mut TodoList, title: &str) -> Result<(), Error>
pub fn complete_todo(list: &mut TodoList, id: u32) -> Result<(), Error>
pub fn list_todos(list: &TodoList, filter: Option<Filter>) -> Vec<&Todo>
pub fn save_todos(list: &TodoList, path: &str) -> Result<(), Error>
pub fn load_todos(path: &str) -> Result<TodoList, Error>
```

### Error Handling

- File not found during load
- Invalid JSON format
- Duplicate todo IDs
- Invalid command arguments

## Testing

```bash
# Run unit tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run

# Performance testing
time cargo run -- list
```

## Extension Possibilities

1. **Database Backend**: Replace JSON with SQLite
2. **Due Dates**: Add datetime tracking
3. **Recurring Tasks**: Automatically create recurring items
4. **Web API**: RESTful HTTP interface
5. **Sync**: Cloud synchronization
6. **Filtering**: Complex query language

## Learning Outcomes

After completing this project:
- ✓ Practical file I/O with Rust
- ✓ Struct and enum design patterns
- ✓ Error handling in real applications
- ✓ Command-line interface design
- ✓ Data persistence strategies

## Common Patterns Used

### Result Pattern
```rust
pub fn operation() -> Result<(), Error> {
    // Try operation
    value?; // Propagate errors
    Ok(())
}
```

### Pattern Matching
```rust
match todo.status {
    Status::Pending => println!("[ ] {}", todo.title),
    Status::Completed => println!("[x] {}", todo.title),
}
```

### Ownership Transfer
```rust
pub fn add_todo(mut list: TodoList, todo: Todo) -> TodoList {
    list.todos.push(todo);
    list
}
```

## Estimated Development Time

- Beginner: 4-6 hours
- Intermediate: 2-3 hours
- Experienced Rust: 1-2 hours

## Related Modules

- Module 04: Basic data structures
- Module 06: Ownership and traits
- Module 07: Error handling patterns

