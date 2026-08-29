# Canonical Rust Lessons for Java Developers

This track explains the Java-to-Rust mental-model shift. Use the links below for the canonical Rust instruction, runnable examples, and exercises; this guide does not replace those lessons.

| Java starting point | Rust concept to learn | Canonical lesson |
|---|---|---|
| Garbage collection and object references | Ownership, moves, and borrowing | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| Interfaces and inheritance | Traits, composition, and static or dynamic dispatch | [Traits and polymorphism](../../06-intermediate-rust/traits_and_polymorphism/README.md) |
| `null` and `Optional<T>` | `Option<T>` and explicit absence | [Result and recoverable errors](../../02-standard-library/error_handling_basics/README.md) |
| Checked and unchecked exceptions | `Result<T, E>`, `match`, and `?` | [Result and recoverable errors](../../02-standard-library/error_handling_basics/README.md) |
| Collections and streams | Collection ownership and iterators | [Collections](../../02-standard-library/collections/README.md) and [iterator patterns](../../02-standard-library/iterator_patterns/README.md) |
| `synchronized`, executors, and thread pools | Ownership-aware concurrency | [Concurrency introduction](../../10-real-world-rust/concurrency_intro/README.md) |
| `CompletableFuture` and virtual-thread mental models | Futures, tasks, and async boundaries | [Advanced async overview](../../07-advanced-concepts/advanced_async/README.md) |
| Maven or Gradle | Cargo commands, project layout, and quality tooling | [Tooling and quality](../../03-tooling-and-quality/README.md) |

Treat these as comparisons, not one-to-one translations. Rust’s ownership model and type system often change the shape of the program, not just its syntax.
