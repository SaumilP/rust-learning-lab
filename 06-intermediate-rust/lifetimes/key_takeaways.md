# Lifetimes: Key Takeaways

- Lifetimes describe relationships between references, not elapsed time.
- Rust infers lifetimes in most functions through elision rules.
- Returned references must be tied to valid input references.
- A struct storing a reference needs a lifetime parameter.
- `'static` means a reference can live for the entire program, not that it must.
