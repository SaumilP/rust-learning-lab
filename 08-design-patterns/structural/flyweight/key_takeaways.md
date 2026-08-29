# Flyweight: Key Takeaways

- Intrinsic data is shared; context-specific data stays on each object.
- Shared data should normally be immutable.
- `Rc` and `Arc` express shared ownership in Rust.
- Measure memory usage before adding interning or caching complexity.
