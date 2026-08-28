# Generics

Generics let one definition work with multiple types while retaining compile-
time type checking. Add trait bounds when the implementation needs particular
operations.

```rust
fn largest<T: Ord>(values: &[T]) -> Option<&T> {
    values.iter().max()
}
```

Generic structs and enums are common throughout Rust: `Vec<T>`, `Option<T>`
and `Result<T, E>` are familiar examples. Rust normally monomorphizes generic
code, producing specialized machine code without runtime type checks.

See `examples/generic_pair.rs` for generic functions, structs, and methods.

Prerequisites: traits. Next: enums and pattern matching.
