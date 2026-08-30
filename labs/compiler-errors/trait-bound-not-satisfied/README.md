# Trait bound not satisfied

The generic function wants to display a value. Predict why `broken.rs` fails before compiling it.

Hint: a generic type parameter can stand for types with very different capabilities.

Formatting with `{value}` requires `Display`, but unconstrained `T` does not promise that trait. The compiler reports E0277. `fixed.rs` makes the required capability explicit with the `T: Display` bound.

An alternative is to format with `Debug` and require `T: Debug` when debug-style output is what the API means. Pick the bound that matches the operation, rather than adding bounds mechanically.
