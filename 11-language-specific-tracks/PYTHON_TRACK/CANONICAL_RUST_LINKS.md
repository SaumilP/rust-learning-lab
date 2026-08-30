# Canonical Rust Lessons for Python Developers

This track starts from familiar Python ideas, but these links are the canonical Rust instruction and runnable examples. A mapping is a learning aid, not a claim that the languages have the same runtime or semantics.

| Python starting point | Rust concept to learn | Important difference | Canonical lesson |
|---|---|---|---|
| Reference counting and garbage collection | Ownership, moves, and borrowing | Rust does not trace object reachability; each value has an owner and borrowing has explicit rules. | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| Mutable objects and aliases | Shared and exclusive references | Python permits aliases that can mutate an object; Rust permits either shared reads or one mutable borrow for a scope. | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| `None` | `Option<T>` | `None` is a singleton value; `Option<T>` makes absence part of the type a caller must handle. | [Result and recoverable errors](../../02-standard-library/error_handling_basics/README.md) |
| Exceptions | `Result<T, E>` and `?` | Python exceptions unwind control flow; ordinary Rust failure is returned in the type and propagated explicitly. | [Result and recoverable errors](../../02-standard-library/error_handling_basics/README.md) |
| Duck typing and protocols | Traits and generics | A Rust trait is declared and implemented explicitly; a generic bound is checked before the program runs. | [Traits and polymorphism](../../06-intermediate-rust/traits_and_polymorphism/README.md) |
| List comprehensions and generators | Iterator adapters | Rust iterators can borrow or consume their input, so the chosen iterator method affects later ownership. | [Iterator patterns](../../02-standard-library/iterator_patterns/README.md) |
| Lists and dictionaries | `Vec<T>` and `HashMap<K, V>` | Rust collections have uniform element types and expose ownership at their APIs. | [Collections](../../02-standard-library/collections/README.md) |
| `with` blocks and finalizers | Scope-based cleanup and `Drop` | Rust cleanup is tied to scope, not a garbage-collection or finalizer schedule. | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| `asyncio` tasks | Futures, runtimes, and async boundaries | Creating a Rust future does not start it; it must be awaited or driven by an executor. | [Advanced async overview](../../07-advanced-concepts/advanced_async/README.md) |
| Virtual environments and `pip` | Cargo packages and dependency resolution | Cargo has different manifest, lockfile, and build conventions; do not treat it as a Python package installer. | [Tooling and quality](../../03-tooling-and-quality/README.md) |

Start with ownership even when the immediate goal is faster code. It is the change that explains why the rest of the Rust API surface looks different.
