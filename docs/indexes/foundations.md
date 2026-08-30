# Foundations concept index

Use this index after the setup module to find the first lesson for a core language idea. The sequence generally follows the [canonical path](../curriculum/from-zero.md), but you can return here when a compiler message or an exercise points to a concept you want to refresh.

| Concept | Start here | Useful next step |
| --- | --- | --- |
| Installing Rust, Cargo, and the command line | [Setup and basics](../../00-setup-and-basics/README.md) | [Tooling and quality](../../03-tooling-and-quality/README.md) |
| Variables, `mut`, constants, and shadowing | [Variables and mutability](../../01-core-fundamentals/variables_and_mutability/README.md) | [Data types](../../01-core-fundamentals/data_types/README.md) |
| Scalars, tuples, arrays, and type inference | [Data types](../../01-core-fundamentals/data_types/README.md) | [Functions](../../01-core-fundamentals/functions/README.md) |
| Functions, parameters, return values, and expressions | [Functions](../../01-core-fundamentals/functions/README.md) | [Control flow](../../01-core-fundamentals/control_flow/README.md) |
| `if`, `match`, loops, and exhaustive decisions | [Control flow](../../01-core-fundamentals/control_flow/README.md) | [Enums and pattern matching](../../06-intermediate-rust/enums_and_pattern_matching/README.md) |
| Ownership, moves, and when values are dropped | [Ownership](../../01-core-fundamentals/ownership/README.md) | [Ownership and borrowing](../../06-intermediate-rust/ownership_and_borrowing/README.md) |
| Shared and mutable references | [Ownership and borrowing](../../06-intermediate-rust/ownership_and_borrowing/README.md) | [Lifetimes](../../06-intermediate-rust/lifetimes/README.md) |
| `String`, `&str`, parsing, and text operations | [String operations](../../02-standard-library/string_operations/README.md) | [Collections](../../02-standard-library/collections/README.md) |
| `Vec`, maps, sets, and choosing a collection | [Collections](../../02-standard-library/collections/README.md) | [Iterator patterns](../../02-standard-library/iterator_patterns/README.md) |
| `iter`, `map`, `filter`, `fold`, and `collect` | [Iterator patterns](../../02-standard-library/iterator_patterns/README.md) | [Common traits](../../02-standard-library/common_traits/README.md) |
| `Copy`, `Clone`, `Debug`, `Display`, and conversions | [Common traits](../../02-standard-library/common_traits/README.md) | [Traits and polymorphism](../../06-intermediate-rust/traits_and_polymorphism/README.md) |
| Absence with `Option` and recoverable failure with `Result` | [Error handling basics](../../02-standard-library/error_handling_basics/README.md) | [Advanced error handling](../../10-real-world-rust/error_handling_advanced/README.md) |

## A practical order for a stuck learner

If a program fails because a value was moved or borrowed, revisit ownership before looking for a library workaround. If a `match` feels unwieldy, review enums and pattern matching. If iterator code is difficult to read, first make the equivalent loop work, then return to iterator patterns and change one operation at a time.
