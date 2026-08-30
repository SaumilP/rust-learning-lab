[![CI](https://github.com/SaumilP/rust-learning-lab/actions/workflows/ci.yml/badge.svg)](https://github.com/SaumilP/rust-learning-lab/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/Rust-stable-b7410e)](rust-toolchain.toml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

# Rust Learning Lab

Rust Learning Lab is a practical path through Rust: read a focused explanation, run a small program, change it, and use the compiler's feedback to deepen the idea. It supports both first-time Rust learners and experienced developers moving from Java, Python, Go, or C++.

The repository is actively growing. Most foundations, applications, intermediate material, and design-pattern examples are ready for review; some advanced projects and production topics are still Draft or Planned. [Content status](CONTENT_STATUS.md) explains the evidence behind each area, and the [roadmap](ROADMAP.md) records the remaining work.

## Choose your starting point

### New to Rust

Start with [`00-setup-and-basics`](00-setup-and-basics/), then work through [`01-core-fundamentals`](01-core-fundamentals/), [`02-standard-library`](02-standard-library/), and [`03-tooling-and-quality`](03-tooling-and-quality/). This route establishes the language, ownership model, standard library, testing, and everyday development tools.

### Ready to build programs

Move through [`04-simple-programs`](04-simple-programs/) and [`05-cli-and-console-games`](05-cli-and-console-games/) to combine I/O, algorithms, state, and user interaction. Continue to [`06-intermediate-rust`](06-intermediate-rust/) for ownership and borrowing, traits, generics, enums, and lifetimes.

### Moving from another language

Use [`11-language-specific-tracks`](11-language-specific-tracks/) alongside the core path. The Java, Python, Go, and C++ guides connect familiar language features to Rust's ownership, error-handling, and type-system model without duplicating the core lessons.

## Learning journeys

| Journey | Modules | What you will practice |
|---|---|---|
| Foundations | [00](00-setup-and-basics/)–[03](03-tooling-and-quality/) | Syntax, ownership, standard types, testing, documentation, and tooling |
| Applied Rust | [04](04-simple-programs/)–[05](05-cli-and-console-games/) | Command-line programs, file handling, algorithms, state, and games |
| Type-system depth | [06](06-intermediate-rust/) | Traits, generics, enums, pattern matching, lifetimes, and optional supplementary review |
| Advanced reference | [07](07-advanced-concepts/)–[08](08-design-patterns/) | Networking, APIs, WebAssembly, macros, performance, and design patterns |
| Projects and production | [09](09-mini-projects/)–[10](10-real-world-rust/) | Multi-file programs, organization, deployment, profiling, and systems topics |

The [learning path](LEARNING_PATH.md) gives the same sequence in a compact format. Module 06 also includes [supplementary review material](06-intermediate-rust/supplementary/README.md); it is useful after the primary sequence and remains Draft until it receives its own validation coverage.

For the detailed beginner route, including stage checkpoints, use [From Zero: the canonical Rust path](docs/curriculum/from-zero.md).

## Get running

Install the stable Rust toolchain with `rustup`, including `rustfmt` and `clippy`, plus Git and Make. The included [toolchain file](rust-toolchain.toml) selects the repository's Rust channel and components.

```bash
git clone https://github.com/SaumilP/rust-learning-lab.git
cd rust-learning-lab
cd 00-setup-and-basics/hello_world
cargo run
```

Many concept modules use standalone Rust files. Run their Makefile from the module directory:

```bash
cd 01-core-fundamentals
make list
make run EXAMPLE=control_flow/examples/loops.rs
make check
make clean
```

`make check` formats, compiles, tests, and documents the module's checked examples. Generated `build/` and `target/` directories are disposable. Some exercises intentionally begin with broken code; read the problem statement before treating those compiler errors as repository defects.

For a Cargo package or workspace, run Cargo from the directory containing its `Cargo.toml`:

```bash
cd 04-simple-programs
cargo run -p calculator
cargo test --workspace
```

## Study routine

1. Read the topic `README.md` and `key_takeaways.md`.
2. Run the example before changing it.
3. Predict a small change, then test that prediction.
4. Repair the related exercise using compiler feedback.
5. Apply the idea in a program or project before moving on.

## Contribute

Corrections, clearer explanations, focused examples, exercises, and tests are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md) for the learning-content standards and validation commands, [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for community expectations, and [SECURITY.md](SECURITY.md) for security reporting.

## License

This project is available under the [MIT License](LICENSE).
