# Prototype: Key Takeaways

- Derive or implement `Clone` to define how a prototype is copied.
- Customizing a clone must not unexpectedly change the original.
- `Rc` and `Arc` share data instead of making deep copies.
- A constructor is clearer when baseline construction is already small.
