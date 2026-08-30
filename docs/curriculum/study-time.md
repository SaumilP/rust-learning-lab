# Study time for the From-Zero Rust path

These estimates describe focused learning time: reading, running examples, changing code, and completing the checkpoint for each stage. They are planning ranges rather than promises. Rust experience varies widely because the compiler asks learners to make ownership and type decisions that may be new even when the syntax is familiar.

The route below follows the [canonical From-Zero path](from-zero.md). It is deliberately a first pass: the aim is a working mental model and small-program confidence, not mastery of every linked module.

| Stage | Canonical module | Suggested focused time | What affects the estimate |
| --- | --- | --- | --- |
| 1. Setup and syntax | [Module 00: Setup and basics](../../00-setup-and-basics/) | 4–8 hours | Toolchain setup can be quick, while a first programming language often needs more time for expressions, types, and control flow. |
| 2. Ownership | [Module 01: Core fundamentals](../../01-core-fundamentals/) | 10–18 hours | Learners new to ownership, borrowing, slices, or lifetimes should expect to spend time making and repairing small compiler mistakes. |
| 3. Standard library | [Module 02: Standard library](../../02-standard-library/) | 8–14 hours | Time grows when collections, iterator adapters, string handling, and error design are all new. |
| 4. Everyday workflow | [Module 03: Tooling and quality](../../03-tooling-and-quality/) | 4–7 hours | The lower end suits someone comfortable with command-line development and tests; the upper end includes writing and debugging a few tests. |
| 5. Small programs | [Module 04: Simple programs](../../04-simple-programs/) | 8–16 hours | Building a program independently, including file or command-line behaviour, takes longer than following an example. |
| 6. Applied practice | [Module 05: CLI and console games](../../05-cli-and-console-games/) | 6–12 hours | Extra time is useful for tracing state changes, testing edge cases, and adding a small feature of your own. |
| 7. Type-system depth | [Module 06: Intermediate Rust](../../06-intermediate-rust/) | 12–20 hours | Traits, generics, and lifetimes repay deliberate practice; experienced developers may move faster through syntax but still need time for Rust's semantics. |

## Planning a first pass

Most learners should budget roughly 52–95 focused hours for the seven stages. At five focused hours each week, that is about 10–19 weeks; at ten focused hours each week, it is about 5–10 weeks. The ranges include practice and troubleshooting, but not long detours into a personal project.

Treat a stage checkpoint as the signal to move forward. If the checkpoint still feels difficult, repeat a small example or exercise rather than trying to cover more topics in the same session. Short, regular sessions tend to make compiler feedback easier to absorb than occasional marathon sessions.

## Adjusting the estimate

Add time if Rust is your first programming language, if you are learning command-line tooling at the same time, or if you want to write every exercise from scratch before seeing hints. You may need less time in the setup and workflow stages if you already use Cargo-like tooling, but do not skip ownership practice because you know another language.

The later modules are optional for the first route. When you are ready for a specific goal, use [advanced concepts](../../07-advanced-concepts/), [design patterns](../../08-design-patterns/), [interview challenges](../../challenges/interview/), or [real-world Rust](../../10-real-world-rust/) as targeted follow-up rather than extending this estimate into an open-ended total.
