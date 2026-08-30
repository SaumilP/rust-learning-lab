# Cross-Language Rust Transition Index

Choose a track for the language that has most shaped your instincts, then use the canonical Rust lessons for the underlying instruction. This index compares mental models; a cell is not a promise of equivalent syntax, runtime behavior, or library APIs.

| Area | Java | Python | Go | C++ | Canonical Rust lesson |
|---|---|---|---|---|---|
| Memory management | Garbage-collected object reachability | Reference counting and garbage collection | Garbage collection | RAII, pointers, and smart pointers | [Ownership](../01-core-fundamentals/ownership/README.md) |
| Aliasing and mutation | References plus API or lock discipline | Mutable object aliases are common | Pointers and shared state are conventional | References and pointers have broad expressive power | [Ownership](../01-core-fundamentals/ownership/README.md) |
| Absent values | `null` and `Optional<T>` | `None` | `nil` | Null pointers or optional-like libraries | [Result and recoverable errors](../02-standard-library/error_handling_basics/README.md) |
| Recoverable errors | Checked and unchecked exceptions | Exceptions | Explicit `error` values | Exceptions, error codes, or expected-like types | [Result and recoverable errors](../02-standard-library/error_handling_basics/README.md) |
| Polymorphism | Classes, interfaces, and inheritance | Duck typing and protocols | Structural interfaces | Inheritance, templates, and virtual functions | [Traits and polymorphism](../06-intermediate-rust/traits_and_polymorphism/README.md) |
| Collection pipelines | Streams | Comprehensions and generators | `range` loops | STL algorithms and ranges | [Iterator patterns](../02-standard-library/iterator_patterns/README.md) |
| Concurrent work | Executors, locks, and futures | Threads, processes, and `asyncio` | Goroutines and channels | Threads, locks, and atomics | [Concurrency introduction](../10-real-world-rust/concurrency_intro/README.md) and [advanced async](../07-advanced-concepts/advanced_async/README.md) |
| Build and dependencies | Maven or Gradle | `pip` and virtual environments | Go modules | CMake and package managers | [Tooling and quality](../03-tooling-and-quality/README.md) |

## Pick the right starting track

| Your strongest background | Track | First comparison | Migration checks |
|---|---|---|---|
| Java or JVM frameworks | [Java track](JAVA_TRACK/README.md) | [Canonical links](JAVA_TRACK/CANONICAL_RUST_LINKS.md) | [Java migration checks](JAVA_TRACK/MVP_CHECKS.md) |
| Python applications, scripts, or data work | [Python track](PYTHON_TRACK/README.md) | [Canonical links](PYTHON_TRACK/CANONICAL_RUST_LINKS.md) | [Python migration checks](PYTHON_TRACK/MVP_CHECKS.md) |
| Go services and concurrent programs | [Go track](GO_TRACK/README.md) | [Canonical links](GO_TRACK/CANONICAL_RUST_LINKS.md) | [Go migration checks](GO_TRACK/MVP_CHECKS.md) |
| C or C++ systems programming | [C++ track](CPP_TRACK/README.md) | [Canonical links](CPP_TRACK/CANONICAL_RUST_LINKS.md) | [C++ migration checks](CPP_TRACK/MVP_CHECKS.md) |

## The common shift

Every track reaches the same central Rust questions: who owns this value, who may borrow it now, what failure is expected, and which execution boundary is responsible for this work? Begin there even if a familiar library or framework appears to offer a direct translation.
