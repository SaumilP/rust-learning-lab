# Pattern Audit Notes

This is an idiomaticity audit of the runnable examples, not a claim that every pattern is the preferred Rust design. The programs are useful demonstrations of pattern participants; production code should select the smallest representation that makes the domain clear.

| Area | Audit result | Practical guidance |
|---|---|---|
| Creational patterns | Examples correctly show construction choices, but factories and builders should not be automatic ceremony. | Prefer constructors, enums, and configuration values until runtime product selection or complex named construction is real. |
| Structural patterns | The examples demonstrate wrappers and delegation accurately. | Make wrappers earn their place with a policy boundary, compatibility boundary, or independent variation; do not wrap a direct call merely to match a catalog name. |
| Behavioural patterns | The examples correctly demonstrate dispatch and coordination. | Enums, iterator adapters, functions, and channels are often more direct Rust forms of the same behaviour. |
| Dynamic dispatch | Several examples use `Box<dyn Trait>` to make participants visible. | This is appropriate for runtime heterogeneity; prefer generics or enums when the set of implementations is known. |
| Shared state | Observer and proxy examples illustrate interior mutability and synchronization. | Treat `RefCell` as single-threaded runtime borrow checking and treat locks as part of the design, not as incidental plumbing. |
| Singleton documentation | The README includes older `static mut` and `unsafe` teaching snippets alongside modern lazy initialization. | Do not copy those snippets into application code. Prefer `std::sync::OnceLock` or `LazyLock`; explicit construction remains the default for dependencies. |
| Error handling | Most focused runnable examples use explicit results where transitions may fail. | Public APIs should return domain-relevant `Result` values and avoid panic-driven routine control flow. |

## Review conclusion

The module is suitable as a catalog of runnable demonstrations, provided readers use the selection guidance. The most important correction to a class-oriented interpretation is that a named GoF pattern is optional in Rust: an enum, a function, an iterator, a generic type, or explicit composition is often the final design.

Read [WHEN_NOT_TO_USE.md](WHEN_NOT_TO_USE.md), [RUST_NATIVE_PATTERNS.md](RUST_NATIVE_PATTERNS.md), and [TRANSFER_GUIDE.md](TRANSFER_GUIDE.md) alongside the individual examples.
