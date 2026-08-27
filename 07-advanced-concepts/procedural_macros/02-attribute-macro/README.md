# Attribute Macro Examples

Attribute macros for function modification and cross-cutting concerns.

## Macros Included

1. **log_entry_exit** - Logs when function is entered and exited
2. **time_execution** - Measures and prints function execution time
3. **retry** - Automatic retry logic with configurable attempts and delay
4. **deprecated_fn** - Marks functions as deprecated with custom messages

## Project Structure

```
02-attribute-macro/
├── my-attribute/   # The proc-macro crate
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── example/        # Usage examples
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Running

```bash
cd example
cargo run
```

## What You'll Learn

- Creating attribute macros with `#[proc_macro_attribute]`
- Parsing function items with `syn`
- Handling attribute arguments
- Wrapping function bodies
- Combining multiple attributes
- Error handling patterns

## Usage Examples

### Simple Attribute

```rust
#[log_entry_exit]
fn my_function() {
    // Function body
}
```

### Attribute with Arguments

```rust
#[retry(times = 3, delay_ms = 100)]
fn network_call() -> Result<(), Error> {
    // Function body
}
```

### Multiple Attributes

```rust
#[log_entry_exit]
#[time_execution]
fn combined() {
    // Both macros apply
}
```
