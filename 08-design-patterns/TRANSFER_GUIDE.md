# Moving Design Patterns from Java and C++ to Rust

Translate the design pressure, not the class diagram. Rust's ownership, enums, traits, and zero-cost generics change which patterns are useful and how their costs appear.

## Common Java transfers that go wrong

### Rebuilding inheritance trees with trait objects

Java interfaces often lead directly to `Box<dyn Trait>`. In Rust, start with an enum when the variants are closed, a generic when the implementation is chosen by the caller, and a trait object only for runtime heterogeneity or an extension boundary. Trait objects add object-safety limits, allocation or indirection in many designs, and less visible concrete behaviour.

### Treating `Arc<Mutex<T>>` as ordinary object state

`Arc<Mutex<T>>` is not a general substitute for Java references. It makes sharing and locking part of every access path. First decide who should own the data; then use a channel, immutable sharing with `Arc<T>`, or a lock only when concurrent mutation is truly required. Never keep a `MutexGuard` across an `.await`.

### Replacing exceptions with `unwrap`

`unwrap` is not Rust's normal error-handling mechanism. Propagate with `?`, model recoverable cases with `Result`, and reserve `expect` for a documented invariant. A panic is appropriate for a bug or unrecoverable program state, not for routine invalid input or I/O failure.

### Turning every service into a singleton

Global service locators hide construction order, dependencies, and test configuration. Build the application graph at startup and pass dependencies explicitly. Use `OnceLock` or `LazyLock` only for process-wide immutable or carefully synchronized state that is genuinely global.

## Common C++ transfers that go wrong

### Porting raw ownership protocols

Do not reproduce `new`, `delete`, nullable owning pointers, or manual reference counting. Prefer owned values and borrowing; use `Box<T>` for a value with a stable heap address or recursive size, `Rc<T>` for single-threaded shared ownership, and `Arc<T>` for thread-safe shared ownership. Shared ownership can create cycles, so use `Weak<T>` for back-references.

### Using inheritance for runtime variation

Rust has no implementation inheritance. Use composition for shared data and traits for shared behaviour. Default trait methods can share small pieces of behaviour, but a trait should describe a capability rather than become a base class with hidden state.

### Copying move semantics without checking the API contract

Moves in Rust are the default transfer of ownership, not an optimization hint. A function that accepts `T` takes ownership; `&T` borrows read-only access; `&mut T` borrows exclusive mutable access. Design these choices into public signatures so callers can predict whether a value remains usable.

## Translation checklist

1. What owns each resource and for how long?
2. Is the variant set open or closed?
3. Does the caller need runtime choice, or would a generic or enum be clearer?
4. How does a failure reach the caller?
5. Does the design need shared mutation, or can one owner coordinate changes?

See [RUST_NATIVE_PATTERNS.md](RUST_NATIVE_PATTERNS.md) for the Rust-first alternatives and [WHEN_NOT_TO_USE.md](WHEN_NOT_TO_USE.md) for counter-signals for every example in this module.
