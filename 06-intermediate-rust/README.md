# Module 06: Intermediate Rust

This module connects Rust's basic syntax to the type-system tools used in real
applications. Work through the concepts in order:

1. [Ownership and borrowing](ownership_and_borrowing/README.md)
2. [Traits and polymorphism](traits_and_polymorphism/README.md)
3. [Generics](generics/README.md)
4. [Enums and pattern matching](enums_and_pattern_matching/README.md)
5. [Lifetimes](lifetimes/README.md)

Each concept contains a short explanation, a runnable example, and a compact
`key_takeaways.md`. The `exercises/` directory contains deliberately broken
programs; read the problem first, attempt a repair, and consult hints only when
needed.

## Commands

```bash
make list
make build
make run EXAMPLE=ownership_and_borrowing/examples/borrowing_values.rs
make run-all
make test
make docs
make check
make clean
```

Generated binaries and documentation live under `build/`. They are disposable
and can always be recreated with `make build` or `make docs`.

Prerequisites: modules 01–03. Continue with module 07 after completing all four
exercises.
