# Canonical Rust Lessons for Go Developers

Go and Rust are both practical systems languages, but they make different choices about memory management, interfaces, and concurrency. These mappings point to the canonical Rust lessons; they are not one-for-one implementation recipes.

| Go starting point | Rust concept to learn | Important difference | Canonical lesson |
|---|---|---|---|
| Garbage collection | Ownership, moves, and borrowing | Rust determines validity and cleanup from ownership rather than tracing reachable objects. | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| Pointers and address-taking | Shared and mutable references | A Rust mutable reference must be exclusive; passing a pointer in Go does not carry that rule. | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| `nil` pointers, maps, slices, and interfaces | `Option<T>` and explicit initialization | `Option<T>` represents expected absence; it is not merely a differently spelled `nil`. | [Result and recoverable errors](../../02-standard-library/error_handling_basics/README.md) |
| `error` return values | `Result<T, E>` and `?` | Both make failure a value, but Rust encodes success and failure together in an enum that must be handled. | [Result and recoverable errors](../../02-standard-library/error_handling_basics/README.md) |
| Implicit interfaces | Traits and generic bounds | Go types satisfy interfaces structurally; Rust trait implementations are explicit and bounds are checked statically. | [Traits and polymorphism](../../06-intermediate-rust/traits_and_polymorphism/README.md) |
| Slices, maps, and `range` | Collections and iterator adapters | Rust collection APIs expose borrowing and ownership, so iteration has a different effect on later use. | [Collections](../../02-standard-library/collections/README.md) and [iterator patterns](../../02-standard-library/iterator_patterns/README.md) |
| `defer` | Scope-based cleanup and `Drop` | Both make cleanup easier to write, but Rust resource cleanup follows ownership and scope rather than a deferred call list. | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| Goroutines and channels | Threads, channels, futures, and async runtimes | A Rust future is not a goroutine: it needs a runtime to poll it, and blocking must be designed deliberately. | [Concurrency introduction](../../10-real-world-rust/concurrency_intro/README.md) and [advanced async](../../07-advanced-concepts/advanced_async/README.md) |
| `go test`, modules, and tooling | Cargo packages, tests, and quality checks | Cargo has a different manifest and lockfile model, even where commands look familiar. | [Tooling and quality](../../03-tooling-and-quality/README.md) |

Learn ownership before treating `Arc<Mutex<T>>` as the equivalent of passing pointers between goroutines. The compiler’s restrictions are intended to change the design boundary, not decorate the same shared-state design.
