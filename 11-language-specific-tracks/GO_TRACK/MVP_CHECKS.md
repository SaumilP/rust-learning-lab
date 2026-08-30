# Go-to-Rust Migration Checks

Use these short prompts to test the conceptual changes behind the syntax. The canonical lessons linked from [CANONICAL_RUST_LINKS.md](CANONICAL_RUST_LINKS.md) provide the runnable practice.

| Focus | Go mental model | Rust mental model | Common mistake to avoid | Challenge |
|---|---|---|---|---|
| Object lifetime | The garbage collector keeps a reachable value alive. | A value is dropped when its owner goes out of scope unless ownership has moved elsewhere. | Keeping every value behind `Arc` because it feels safer than an owner. | Explain why assigning a `String` to another variable moves it and show when borrowing is the better function boundary. |
| Interface satisfaction | A type implements an interface by having matching methods. | A type explicitly implements a trait, and a function states its trait bounds. | Expecting a method with the right name to satisfy a Rust trait automatically. | Define a small trait for a dependency and state when a generic parameter is preferable to `dyn Trait`. |
| Absence and failure | `nil` and `error` are conventional return values. | `Option<T>` and `Result<T, E>` make the two cases distinct in the signature. | Using `unwrap()` where an error or absence comes from ordinary input. | Choose `Option` or `Result` for a missing configuration key and explain the caller’s handling path. |
| Iteration | `range` is a convenient default over collections. | `iter`, `iter_mut`, and `into_iter` describe different ownership actions. | Consuming a collection with `into_iter` before a later use. | For `Vec<String>`, choose the correct iterator to print strings and then reuse the vector. |
| Concurrency | A goroutine can be started cheaply and channels coordinate it. | Rust threads and tasks must satisfy ownership rules; async futures are driven by a runtime. | Calling a blocking operation inside an async task because goroutines do not impose the same boundary. | Describe when a channel is clearer than a mutex and why spawning an async future does not itself prove it will run. |
| Cleanup | `defer` records cleanup near resource acquisition. | Drop-based cleanup happens as ownership leaves scope. | Assuming a destructor can safely borrow values that have already been dropped. | Sketch a file-processing scope and explain when its resource cleanup occurs, including on an early `?` return. |

The intended answer is a design explanation, not a translation of Go syntax. If the result is only a keyword substitution, revisit the ownership or runtime difference.
