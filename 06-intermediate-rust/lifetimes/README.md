# Lifetimes

A lifetime describes how long a reference remains valid. Most lifetimes are
inferred. Add an annotation when a function accepts multiple references and
returns a reference whose origin would otherwise be ambiguous.

```rust
fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}
```

The annotation does not extend either input's lifetime. It tells the compiler
that the returned borrow cannot outlive the shorter input borrow. Structs that
store references also need lifetime parameters.

See `examples/longest_word.rs` for a small borrowed-data example.

Prerequisites: ownership and borrowing. Next: module 07 advanced concepts.
