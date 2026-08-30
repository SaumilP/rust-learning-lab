# Data types

This example surveys scalar types, compound types, strings, conversions, aliases, and checked arithmetic. It is an orientation to the types you will encounter in early Rust programs, not a substitute for consulting the standard-library documentation when choosing an API.

Run it from this directory with `cargo run`. Change a numeric literal, add a tuple element, or replace a checked addition with a wrapping addition to compare the behavior.

## What to notice

- Integer and floating-point literals can be inferred, but annotations make important boundaries explicit.
- Tuples can hold different types; arrays hold one type and have a fixed length.
- `String` owns growable text, while `&str` borrows a string slice.
- Checked arithmetic returns an `Option` instead of silently producing an out-of-range result.

## Practice

Use the [types exercise](../exercises/02-types/README.md) after running the example.

## Next step

Continue to [Control flow](../control_flow/README.md) to make decisions and repeat work with these values.
