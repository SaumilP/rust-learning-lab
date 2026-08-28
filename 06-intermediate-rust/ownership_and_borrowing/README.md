# Ownership and Borrowing

Ownership lets Rust manage memory without a garbage collector. Every value has
one owner, a value is dropped when its owner leaves scope, and assignment or a
function call may move ownership.

Borrow a value with `&T` when a function only needs to read it. Borrow with
`&mut T` when the function must update it. At a given time Rust allows either
many shared references or one mutable reference, which prevents data races and
iterator invalidation.

```rust
fn length(text: &str) -> usize {
    text.len()
}

let name = String::from("Ferris");
assert_eq!(length(&name), 6);
println!("{name}"); // borrowing did not move the String
```

Start with `examples/borrowing_values.rs`, then experiment by removing `&` at
the call site to see how a move changes what can be used afterward.

Prerequisites: module 01 data types and functions. Next: traits and generics.
