# Canonical Rust Lessons for C++ Developers

Rust and modern C++ share many goals, especially deterministic cleanup and systems-level performance. The comparisons here identify useful starting points; they are not source-to-source translations and should not be used to infer identical guarantees.

| C++ starting point | Rust concept to learn | Important difference | Canonical lesson |
|---|---|---|---|
| Object lifetime and RAII | Ownership, moves, and scope-based cleanup | Both use deterministic cleanup, but Rust makes aliasing and mutation rules part of the type-checked program. | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| Raw pointers and references | References and borrowing | Rust references are not nullable and must satisfy borrowing rules; raw pointers are a separate unsafe facility. | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| `std::unique_ptr` and move operations | Owned values and moves | An ordinary Rust value is already an owning value; `Box<T>` is not the default spelling for every owner. | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| `std::shared_ptr` | `Rc<T>` and `Arc<T>` | Shared ownership is explicit and cannot by itself provide mutable access; it is not a general replacement for ordinary ownership. | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| Templates and concepts | Generics and trait bounds | Rust monomorphizes many generic calls, but trait coherence, specialization, and overload rules differ from C++. | [Generics](../../06-intermediate-rust/generics/README.md) and [traits](../../06-intermediate-rust/traits_and_polymorphism/README.md) |
| Class inheritance and virtual functions | Structs, enums, traits, and trait objects | Traits describe capabilities, not a shared base-object layout or inherited fields. | [Traits and polymorphism](../../06-intermediate-rust/traits_and_polymorphism/README.md) |
| Exceptions and error codes | `Result<T, E>` | Expected failure is normally visible in the return type; panic is not an exception-handling mechanism. | [Result and recoverable errors](../../02-standard-library/error_handling_basics/README.md) |
| STL algorithms and ranges | Iterators and collection APIs | Iterator methods express whether values are borrowed, mutably borrowed, or consumed. | [Iterator patterns](../../02-standard-library/iterator_patterns/README.md) and [collections](../../02-standard-library/collections/README.md) |
| `std::thread`, locks, and atomics | `Send`, `Sync`, and synchronization | Rust still has locks and atomics, but invalid cross-thread ownership is rejected before execution. | [Concurrency introduction](../../10-real-world-rust/concurrency_intro/README.md) |
| CMake and package managers | Cargo packages and tooling | Cargo’s manifest, workspace, build-script, and lockfile model differs from CMake targets and external package managers. | [Tooling and quality](../../03-tooling-and-quality/README.md) |

The first useful goal is not to recreate a C++ architecture in Rust. Learn to let ownership, enums, and explicit error types choose a simpler shape.
