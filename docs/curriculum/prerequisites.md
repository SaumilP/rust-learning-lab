# Core concept prerequisites

This map gives the recommended dependency order for the canonical From-Zero route. It is a learning aid, not a claim that every listed topic already has migrated metadata or Stable status. The topic metadata validator remains the source of truth for migrated prerequisite IDs.

```text
installation
  └── variables-and-mutability
        ├── data-types
        ├── control-flow
        └── functions
              └── ownership
                    ├── borrowing
                    │     └── lifetimes
                    ├── strings-and-collections
                    └── error-handling
                          ├── option
                          └── result
                                ├── enums-and-pattern-matching
                                ├── traits
                                │     └── generics
                                └── iterator-patterns
                                      └── testing-and-tooling
```

## How to use the map

Follow an edge when a topic’s examples assume the earlier idea. For example, start ownership before borrowing, and understand `Option` and `Result` before trying to design an error-handling API. You can revisit a branch without restarting the whole route; the graph is about dependencies, not a measure of difficulty.

## Canonical lessons

| Concept | Canonical lesson |
| --- | --- |
| Variables and mutability | [Module 00 variables](../../00-setup-and-basics/variables_and_mutability/) and [Module 01 topic](../../01-core-fundamentals/variables_and_mutability/) |
| Data types and control flow | [Data types](../../01-core-fundamentals/data_types/) and [control flow](../../01-core-fundamentals/control_flow/) |
| Functions and ownership | [Functions](../../01-core-fundamentals/functions/) and [ownership](../../01-core-fundamentals/ownership/) |
| Borrowing and lifetimes | [Borrowing](../../01-core-fundamentals/borrowing/) and [lifetimes](../../01-core-fundamentals/lifetimes/) |
| Errors and standard-library patterns | [Error handling](../../01-core-fundamentals/error_handling/), [Option and Result](../../02-standard-library/error_handling_basics/), and [iterators](../../02-standard-library/iterator_patterns/) |
| Traits and generics | [Traits](../../06-intermediate-rust/traits_and_polymorphism/) and [generics](../../06-intermediate-rust/generics/) |
| Testing and tooling | [Module 03](../../03-tooling-and-quality/) |

As more topics migrate to metadata, their stable IDs and validated prerequisite edges should be added to this map. Do not infer a topic is Stable merely because it appears in the graph.
