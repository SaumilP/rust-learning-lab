# Python-to-Rust Migration Checks

Use these prompts after the comparison material. They test whether the mental-model change is clear; the linked canonical lessons provide the runnable instruction.

| Focus | Python mental model | Rust mental model | Common mistake to avoid | Challenge |
|---|---|---|---|---|
| Values and aliases | Several names can refer to one mutable object. | A non-`Copy` value has one owner; aliases are explicit borrows with rules. | Adding `clone()` to silence every ownership error. | Explain what happens when a `String` is assigned to a second binding and name two ways to retain access to the original data. |
| Absence | A function may return `None` and callers may forget to check it. | A potentially missing value uses `Option<T>`, which is handled with `match` or combinators. | Calling `unwrap()` for ordinary missing input. | Design a lookup signature for a possibly absent user and show how its caller handles both cases. |
| Errors | Exceptions can emerge from any call in the chain. | Recoverable errors appear as `Result<T, E>` in the signature. | Replacing `try`/`except` with `unwrap()`. | Convert a parsing function that raises `ValueError` into a Rust signature and explain what `?` does. |
| Iteration | A comprehension creates a collection and a generator is lazily consumed. | Iterator adapters are lazy until consumed, and may borrow or take ownership. | Choosing `into_iter()` when the collection is needed afterwards. | Build the iterator chain for the even doubled values in a `Vec<i32>` and state whether it consumes the vector. |
| Dynamic behavior | Any object with the right method may be accepted at runtime. | A trait bound states the required capability and is checked at compile time. | Reaching for `Box<dyn Trait>` before a generic bound or enum is appropriate. | Specify a trait bound for a formatter function and explain when dynamic dispatch would be justified. |
| Async work | Scheduling a coroutine or task starts work under the event loop. | A future is inert until awaited or scheduled by a runtime. | Blocking an async runtime thread with a slow synchronous call. | Explain why creating a future is not enough to run it and where blocking file work belongs. |

When an answer sounds like a syntax replacement, revisit the ownership or execution-model difference. That is normally where the Python-to-Rust translation changes shape.
