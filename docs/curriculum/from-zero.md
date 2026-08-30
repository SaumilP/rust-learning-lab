# From Zero: the canonical Rust path

This is the recommended route for someone starting Rust without assuming prior systems-programming experience. Work through a stage in order, run the examples, and use the checkpoint before moving on. Modules marked Review contain useful material and automated checks, but they are not yet represented as Stable website content.

| Stage | Learn | Start here | Checkpoint |
| --- | --- | --- | --- |
| 1. Setup and syntax | Toolchain, bindings, types, control flow | [Module 00](../../00-setup-and-basics/) | Run one Cargo example and explain the difference between a binding and a value. |
| 2. Ownership | Moves, borrowing, slices, lifetimes | [Module 01](../../01-core-fundamentals/) | Explain why a `String` move differs from an integer assignment and repair a borrowing error. |
| 3. Standard library | Collections, strings, iterators, errors | [Module 02](../../02-standard-library/) | Choose `Option` or `Result` for a realistic boundary and justify it. |
| 4. Everyday workflow | Cargo, tests, formatting, Clippy, documentation | [Module 03](../../03-tooling-and-quality/) | Add a test, run `cargo fmt`, and resolve a useful lint. |
| 5. Small programs | Input, state, files, command-line programs | [Module 04](../../04-simple-programs/) | Finish a program without copying its solution first. |
| 6. Applied practice | Games, randomness, state transitions | [Module 05](../../05-cli-and-console-games/) | Trace a state change and add an edge-case test. |
| 7. Type-system depth | Enums, traits, generics, lifetimes | [Module 06](../../06-intermediate-rust/) | Design a small trait-based API and explain its ownership choices. |

## How to study a topic

Read the topic overview, run the smallest example, predict one change, and let the compiler confirm or challenge that prediction. Then attempt the exercise or a small variation before reading solution notes. If a compiler error is unfamiliar, use the [compiler-error lab](../../labs/compiler-errors/) after first trying to explain the failure yourself.

## What comes later

Modules 07 through 10 are valuable reference and project material, but they are not a required first pass through Rust. Return to them after the seven stages above, choosing material that supports the program you want to build. Use the language-transition tracks alongside this path, not instead of it.

Use the [core concept prerequisites](prerequisites.md) when you need to understand why a later topic depends on an earlier one.
