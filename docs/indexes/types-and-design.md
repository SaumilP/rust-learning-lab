# Types and program design concept index

Rust's type-system features are most useful when they solve a concrete design problem. Use this index to locate the primary lesson for the question you are trying to answer, rather than treating traits, generics, enums, and lifetimes as interchangeable tools.

| Design question or concept | Start here | Useful next step |
| --- | --- | --- |
| How do I describe a value that can be one of several meaningful cases? | [Enums and pattern matching](../../06-intermediate-rust/enums_and_pattern_matching/README.md) | [Error handling basics](../../02-standard-library/error_handling_basics/README.md) |
| How do I make a decision for every possible case? | [Control flow](../../01-core-fundamentals/control_flow/README.md) | [Enums and pattern matching](../../06-intermediate-rust/enums_and_pattern_matching/README.md) |
| How do I share behavior without a class hierarchy? | [Traits and polymorphism](../../06-intermediate-rust/traits_and_polymorphism/README.md) | [Common traits](../../02-standard-library/common_traits/README.md) |
| How do I write one function or type that works with several types? | [Generics](../../06-intermediate-rust/generics/README.md) | [Traits and polymorphism](../../06-intermediate-rust/traits_and_polymorphism/README.md) |
| Which trait bounds belong in a public function signature? | [Traits and polymorphism](../../06-intermediate-rust/traits_and_polymorphism/README.md) | [Generics](../../06-intermediate-rust/generics/README.md) |
| Why can a reference not outlive the value it points to? | [Lifetimes](../../06-intermediate-rust/lifetimes/README.md) | [Ownership and borrowing](../../06-intermediate-rust/ownership_and_borrowing/README.md) |
| Should this function own its input or borrow it? | [Ownership and borrowing](../../06-intermediate-rust/ownership_and_borrowing/README.md) | [Ownership](../../01-core-fundamentals/ownership/README.md) |
| How should a type report an expected absence or failure? | [Error handling basics](../../02-standard-library/error_handling_basics/README.md) | [Enums and pattern matching](../../06-intermediate-rust/enums_and_pattern_matching/README.md) |
| How do I make values easy to inspect, print, clone, or parse? | [Common traits](../../02-standard-library/common_traits/README.md) | [Traits and polymorphism](../../06-intermediate-rust/traits_and_polymorphism/README.md) |

## Keep the distinctions sharp

Enums model a known set of alternatives, while traits describe shared behavior. Generics let an API work with a family of types, while trait bounds state the capabilities that family must provide. Lifetimes do not extend a value's life; they describe relationships the compiler must be able to prove. Beginning with the lesson that matches the design question makes those distinctions easier to retain.
