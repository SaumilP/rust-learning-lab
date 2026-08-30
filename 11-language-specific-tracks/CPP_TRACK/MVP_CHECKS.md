# C++-to-Rust Migration Checks

These prompts distinguish concepts that look familiar from guarantees that are actually different. Use the linked canonical lessons for implementation practice.

| Focus | C++ mental model | Rust mental model | Common mistake to avoid | Challenge |
|---|---|---|---|---|
| Ownership | RAII is a convention implemented through destructors and resource-owning types. | Ownership is the default relationship for values and determines moves and cleanup. | Wrapping every value in `Box<T>` or a reference-counted pointer. | Explain why a plain `String` owns its allocation and when `Box<T>` is genuinely useful. |
| References | References and pointers have broad expressive power, including aliases that may be mutated. | `&T` allows shared reading; `&mut T` grants exclusive access for a bounded lifetime. | Treating a Rust reference as a nullable pointer or bypassing a borrow error with a clone. | Show why two mutable borrows of the same vector overlap incorrectly, then shorten one borrow without cloning. |
| Polymorphism | Inheritance and virtual methods often combine state reuse and dynamic dispatch. | Traits, generic bounds, trait objects, and enums solve separate problems. | Building a base-class hierarchy with trait objects before considering an enum or generic function. | Choose an enum, generic trait bound, or trait object for a fixed set of runtime-selected commands and justify it. |
| Errors | Exceptions may leave ordinary function signatures unchanged. | `Result<T, E>` advertises recoverable failure at the call boundary. | Using `panic!` for malformed user input. | Give a parser signature with a useful error type and describe what `?` returns to its caller. |
| Iteration | Algorithms operate over iterators, ranges, and callable objects. | Iterators also encode whether the source is borrowed or consumed. | Translating a range algorithm without deciding who owns the input afterwards. | For `Vec<String>`, select the iterator form that retains the vector and the one that transfers its strings to a new collection. |
| Concurrency | A data race can compile and become undefined behavior. | Types must satisfy `Send` and `Sync` before data crosses or is shared between threads. | Assuming the compiler prevents deadlocks or makes mutex design unnecessary. | Contrast a channel-based worker handoff with an `Arc<Mutex<T>>` design and name the trade-off each makes. |

If a solution relies on an unsafe escape hatch, state the safety contract first. Unsafe code changes who proves the invariant; it does not suspend the invariant.
