# Mini-Project 1: Todo CLI Application

## Overview

A command-line todo list application that demonstrates core Rust concepts every Java developer needs to understand. This project focuses on ownership, structs, error handling, and file I/O.

## What you'll learn

1. **Ownership and borrowing** - Understanding move semantics vs Java's reference semantics
2. **Structs and methods** - Rust's alternative to classes
3. **Error handling** - Using `Result<T, E>` instead of exceptions
4. **File I/O** - Reading and writing JSON data
5. **Pattern matching** - Handling different cases elegantly
6. **Collections** - Working with `Vec<T>` (similar to ArrayList)

## Project structure

```
01-todo-cli/
├── Cargo.toml          # Dependencies (like pom.xml or build.gradle)
├── src/
│   ├── main.rs         # CLI entry point
│   ├── lib.rs          # Core library code
│   ├── todo.rs         # Todo struct and methods
│   └── storage.rs      # File persistence
└── tests/
    └── integration_test.rs
```

## Features

- ✅ Add new todo items
- ✅ List all todos
- ✅ Mark todos as complete
- ✅ Delete todos
- ✅ Persist to JSON file
- ✅ Filter by status (pending/completed)

## Running the project

```bash
# Build
cargo build

# Run
cargo run -- add "Buy groceries"
cargo run -- list
cargo run -- complete 0
cargo run -- delete 1

# Run tests
cargo test

# Check without building
cargo check

# Lint
cargo clippy
```

## Java vs Rust: Key differences in this project

### 1. Ownership vs References

**Java**:
```java
public class TodoApp {
    private List<Todo> todos = new ArrayList<>();

    public void addTodo(Todo todo) {
        todos.add(todo);  // Multiple references to same object
    }

    public Todo getTodo(int index) {
        return todos.get(index);  // Returns reference
    }
}
```

**Rust**:
```rust
pub struct TodoApp {
    todos: Vec<Todo>,
}

impl TodoApp {
    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);  // Moves ownership into Vec
    }

    pub fn get_todo(&self, index: usize) -> Option<&Todo> {
        self.todos.get(index)  // Returns borrowed reference
    }
}
```

**Key insight**: In Rust, you can't have multiple mutable references. The `&mut self` in `add_todo` means "I'm the only one who can modify this right now."

### 2. Error Handling

**Java**:
```java
public void saveTodos() throws IOException {
    FileWriter writer = new FileWriter("todos.json");
    // Write data
    writer.close();
}

// Caller
try {
    app.saveTodos();
} catch (IOException e) {
    System.err.println("Failed: " + e.getMessage());
}
```

**Rust**:
```rust
pub fn save_todos(&self) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(&self.todos)?;
    fs::write("todos.json", json)?;
    Ok(())
}

// Caller
match app.save_todos() {
    Ok(_) => println!("Saved!"),
    Err(e) => eprintln!("Failed: {}", e),
}
```

**Key insight**: Errors are values in Rust. The `?` operator is like Java's "let exception propagate," but explicit.

### 3. No Null - Option Type

**Java**:
```java
public Todo findTodo(int id) {
    // Might return null
    for (Todo todo : todos) {
        if (todo.getId() == id) {
            return todo;
        }
    }
    return null;  // Danger: NPE waiting to happen
}

// Usage
Todo todo = app.findTodo(5);
if (todo != null) {  // Easy to forget this check!
    System.out.println(todo.getTitle());
}
```

**Rust**:
```rust
pub fn find_todo(&self, id: u32) -> Option<&Todo> {
    self.todos.iter().find(|todo| todo.id == id)
}

// Usage
match app.find_todo(5) {
    Some(todo) => println!("{}", todo.title),
    None => println!("Not found"),
}

// Or with if let
if let Some(todo) = app.find_todo(5) {
    println!("{}", todo.title);
}
```

**Key insight**: The type system forces you to handle the "not found" case. No `NullPointerException` possible.

### 4. Mutability

**Java**:
```java
// Everything is mutable by default
Todo todo = new Todo("Buy milk");
todo.setTitle("Buy almond milk");  // No problem
```

**Rust**:
```rust
// Immutable by default
let todo = Todo::new("Buy milk");
// todo.title = "Buy almond milk";  // ERROR: cannot mutate

// Must be explicit about mutability
let mut todo = Todo::new("Buy milk");
todo.title = String::from("Buy almond milk");  // OK
```

**Key insight**: Rust defaults to safety. You opt into mutability when you need it.

## Common mistakes (and fixes)

### Mistake 1: Trying to modify through immutable reference

```rust
fn mark_complete(todo: &Todo) {
    todo.completed = true;  // ❌ ERROR: cannot mutate through &
}

// Fix:
fn mark_complete(todo: &mut Todo) {
    todo.completed = true;  // ✅ OK
}
```

### Mistake 2: Moving out of Vec

```rust
let todos = vec![todo1, todo2];
let first = todos[0];  // ❌ ERROR: cannot move out of Vec

// Fix 1: Borrow
let first = &todos[0];  // ✅ Borrow reference

// Fix 2: Clone
let first = todos[0].clone();  // ✅ If Todo implements Clone
```

### Mistake 3: String vs &str confusion

```rust
struct Todo {
    title: &str,  // ❌ ERROR: missing lifetime specifier
}

// Fix: Use String for owned data
struct Todo {
    title: String,  // ✅ Owns the string
}
```

## Challenges to try

Once you've built the basic version, try these extensions:

1. **Add priorities** - High, Medium, Low (hint: use an enum)
2. **Add due dates** - Use `chrono` crate for date handling
3. **Add tags** - Vec<String> of tags
4. **Search functionality** - Find todos by text
5. **Undo/Redo** - Store command history
6. **Color output** - Use `colored` crate

## Next steps

After completing this project:

1. Review your code with `cargo clippy`
2. Compare with the provided solution
3. Try the challenges above
4. Move on to Mini-Project 2: Multithreaded Web Server

## Resources

- [Rust Book Chapter 4: Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust Book Chapter 9: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [serde documentation](https://serde.rs/)
- [Java to Rust Cheat Sheet](../../CHEAT_SHEET.md)

---

**Estimated time**: 4-6 hours for first implementation

**Difficulty**: ★★☆☆☆ (Beginner)
