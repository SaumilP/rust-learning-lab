# Traits and Polymorphism

A trait describes shared behavior. Types opt into that behavior with `impl`,
allowing functions to work with capabilities instead of concrete types.

```rust
trait Summary {
    fn summary(&self) -> String;
}

fn print_summary(item: &impl Summary) {
    println!("{}", item.summary());
}
```

Use generics with trait bounds for static dispatch and `dyn Trait` when a
collection must contain different concrete types behind references or smart
pointers. Prefer static dispatch until runtime polymorphism is actually useful.

See `examples/summarizable.rs` for implementations, a generic function, and a
trait-object collection.

Prerequisites: ownership and borrowing. Next: generics.
