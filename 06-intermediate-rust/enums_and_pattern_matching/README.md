# Enums and Pattern Matching

An enum defines a value that can be exactly one of several variants. Each
variant may carry different data, making enums a precise way to represent
states and events.

```rust
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

`match` must handle every possible variant. This exhaustiveness turns forgotten
cases into compiler errors. Use `if let` or `let else` when only one pattern is
interesting.

See `examples/order_state.rs` for an enum-driven state transition.

Prerequisites: structs and functions. Next: lifetimes.
