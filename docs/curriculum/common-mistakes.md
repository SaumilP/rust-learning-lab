# Common Rust mistakes and how to recover

Rust gives unusually direct feedback when a program's assumptions do not line up with its ownership, type, or error-handling rules. Treat that feedback as a clue about the model, not as an obstacle to work around. This reference names mistakes that commonly slow down new learners and points to the lesson that helps correct each one.

| Mistake | What to do instead | Canonical lesson |
| --- | --- | --- |
| Expecting every assignment to make an independent copy | Check whether the value implements `Copy`; move an owned value only when the new binding should take responsibility for it, or borrow it when both places only need access. | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| Adding `.clone()` before understanding the move error | First decide who should own the value and how long each use needs it. Clone only when two independent owned values are genuinely required. | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| Trying to keep a mutable reference while another reference is still in use | Keep the read-only or mutable access in the smallest useful scope, then make the next borrow after the earlier one has ended. | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| Treating `let` bindings as mutable by default | Mark a binding `mut` only when the binding's value must change; use shadowing when a transformed value deserves a new binding. | [Variables and mutability](../../01-core-fundamentals/variables_and_mutability/README.md) |
| Reaching for `unwrap()` in code that can receive ordinary bad input | Model absence with `Option` and recoverable failure with `Result`; use `?`, matching, or a deliberate fallback to make the chosen behaviour clear. | [Error handling basics](../../02-standard-library/error_handling_basics/README.md) |
| Using indexing when an input can be out of range | Prefer methods such as `get` when missing data is an expected possibility, then handle the returned `Option`. | [Collections](../../02-standard-library/collections/README.md) |
| Writing loops that collect temporary values when an iterator expresses the work directly | Start with `iter`, `iter_mut`, or `into_iter`, then use adapters such as `map`, `filter`, and `collect` when they make the transformation clearer. | [Iterator patterns](../../02-standard-library/iterator_patterns/README.md) |
| Assuming an `if` block or `match` arm can quietly return a value of a different type | Read the type mismatch from the inside out, make every branch agree on an output type, and introduce an enum when the alternatives are meaningfully different. | [Control flow](../../01-core-fundamentals/control_flow/README.md) |
| Editing until an error disappears without reading the diagnostics | Read the primary error, its highlighted span, and any suggested fix. Reduce the example if needed, then make one intentional change and compile again. | [Debugging](../../03-tooling-and-quality/debugging/README.md) |
| Leaving warnings and formatting until the end of a change | Run `cargo fmt`, `cargo clippy`, and `cargo check` in the normal edit loop so small problems stay small. | [Code quality tools](../../03-tooling-and-quality/code_quality_tools/README.md) |

## A useful recovery loop

When a Rust error feels opaque, pause before changing the code. State what you expected to happen, identify which value or type the compiler is discussing, and compare that expectation with the relevant rule. Try the smallest change that tests your explanation. The [compiler-error labs](../../labs/compiler-errors/) are a good place to practise this loop without risking a larger program.

## When to ask for help

Bring a reduced example, the full diagnostic, and a short description of the behaviour you wanted. That gives another learner or reviewer enough context to explain the rule rather than merely suggesting a patch. If you are still early in the curriculum, return to the matching stage in the [From Zero path](from-zero.md) before adding more abstractions.
