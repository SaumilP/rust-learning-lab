# Variables and mutability

Rust bindings are immutable unless they are declared with `mut`. The example also demonstrates constants, shadowing, scope, type inference, and destructuring.

Run it from this directory with `cargo run`. Uncomment one intentionally invalid assignment at a time if you want to read the compiler's explanation, then restore it before continuing.

## What to notice

- `let value = ...` creates an immutable binding by default.
- `let mut value = ...` permits reassignment, but it does not permit changing the binding's type.
- Shadowing creates a new binding, so the later binding may have a different type.
- A binding is dropped when its scope ends.

## Practice

Use the [variables exercise](../exercises/01-variables/README.md) after running the example.

## Next step

Continue to [Data types](../data_types/README.md) to choose types for the values a binding holds.
