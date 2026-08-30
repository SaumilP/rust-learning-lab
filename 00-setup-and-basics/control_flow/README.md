# Control flow

This example covers `if`, `loop`, `while`, `for`, `match`, `if let`, and `while let`. Rust control-flow constructs are expressions where it is useful for them to produce a value, and pattern matching must account for every possible case.

Run it from this directory with `cargo run`. Change a branch condition, modify a range, or add a `match` arm to see how the compiler enforces the structure of the program.

## What to notice

- An `if` expression can produce a value when both branches have compatible types.
- `loop` can return a value through `break`.
- `for` works with iterators and is usually clearer than indexing a collection manually.
- `match` is exhaustive; a wildcard arm is one way to handle remaining cases.

## Practice

Use the [control-flow exercise](../exercises/03-control-flow/README.md) after running the example.

## Next step

Move to the [From-Zero curriculum](../../docs/curriculum/from-zero.md) for the recommended sequence after these foundations.
