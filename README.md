[![CI](https://github.com/SaumilP/rust-learning-lab/actions/workflows/ci.yml/badge.svg)](https://github.com/SaumilP/rust-learning-lab/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/Rust-stable-b7410e)](rust-toolchain.toml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

# Rust Learning Lab

Rust Learning Lab is a collection of short explanations, runnable examples,
exercises, and small projects for learning Rust by writing code. It starts with
the language basics, spends time on ownership and borrowing, and then moves into
tooling, application design, concurrency, patterns, and project work.

The repository is intended for people learning Rust for the first time and for
developers coming from Java, Python, Go, C++, or a similar language. Examples
favour clarity over compactness so that the reason behind each Rust feature is
visible in the code.

## What is included

- Focused examples that can be compiled and changed independently
- Exercises built around common mistakes and compiler feedback
- Concept notes, key takeaways, and suggested experiments
- Cargo projects for command-line applications and larger examples
- Make targets for compiling standalone examples and generating rustdoc output
- Transition guides for developers arriving from other languages

The material is under active development. The foundations and intermediate
modules are usable now; some advanced project folders are still planned or are
being expanded. The [roadmap](ROADMAP.md) records that distinction.

## Prerequisites

Install the stable Rust toolchain with `rustup`. You will need `rustc`, `cargo`,
`rustfmt`, `clippy`, Git, and Make. The included [toolchain file](rust-toolchain.toml)
selects the required Rust channel and components.

Verify the installation:

```bash
rustc --version
cargo --version
```

## Getting started

```bash
git clone https://github.com/SaumilP/rust-learning-lab.git
cd rust-learning-lab
cd 00-setup-and-basics/hello_world
cargo run
```

If you already know the basics, use the [learning path](LEARNING_PATH.md) to
choose a suitable starting point.

## Repository map

| Module | Subject | Current role |
|---|---|---|
| [`00-setup-and-basics`](00-setup-and-basics/) | Installation, variables, data types, and control flow | Starting point |
| [`01-core-fundamentals`](01-core-fundamentals/) | Ownership, borrowing, lifetimes, functions, and errors | Core foundation |
| [`02-standard-library`](02-standard-library/) | Collections, strings, iterators, traits, and smart pointers | Core library skills |
| [`03-tooling-and-quality`](03-tooling-and-quality/) | Cargo, testing, documentation, debugging, and code quality | Development workflow |
| [`04-simple-programs`](04-simple-programs/) | Small command-line programs and file processing | Applied practice |
| [`05-cli-and-console-games`](05-cli-and-console-games/) | Console I/O, game loops, state, and randomness | Applied practice |
| [`06-intermediate-rust`](06-intermediate-rust/) | Deeper type-system work and optional review examples | Intermediate study |
| [`07-advanced-concepts`](07-advanced-concepts/) | Networking, web APIs, WebAssembly, procedural macros, and performance | Advanced reference |
| [`08-design-patterns`](08-design-patterns/) | Creational, structural, behavioural, and Rust-specific patterns | Pattern study |
| [`09-mini-projects`](09-mini-projects/) | Multi-file programs that combine several concepts | Project practice |
| [`10-real-world-rust`](10-real-world-rust/) | Profiling, unsafe Rust, FFI, deployment, and organization | Production topics |
| [`11-language-specific-tracks`](11-language-specific-tracks/) | Rust guidance for Java, Python, Go, and C++ developers | Transition guides |
| [`challenges`](challenges/) | Progressive practice and interview-style problems | Additional exercises |

## Working with examples

Many concept modules contain standalone `.rs` files instead of Cargo packages.
Run their Makefile from the module directory:

```bash
cd 01-core-fundamentals
make list
make run EXAMPLE=control_flow/examples/loops.rs
make check
make clean
```

`make check` verifies formatting, compiles the examples with warnings denied,
runs embedded tests, and generates rustdoc pages under `build/docs/`. Generated
files are temporary; `make clean` removes the complete `build/` directory.

For a Cargo workspace or package, use Cargo from the directory containing its
`Cargo.toml`:

```bash
cd 04-simple-programs
cargo run -p calculator
cargo test --workspace
```

Some exercises are deliberately incomplete and should not compile until you
repair them. Read the exercise instructions before treating a compiler error as
a repository defect.

## A practical study routine

1. Read the topic's `README.md` and `key_takeaways.md`.
2. Run the example without changing it.
3. Predict the result of a small change, then test that prediction.
4. Complete the related exercise using the compiler messages as feedback.
5. Apply the topic in a program from modules 04, 05, or 09.

## Project status and quality

The [content status page](CONTENT_STATUS.md) explains which areas are ready to use, which have known gaps, and which are still outlines or placeholders.

Continuous integration checks the tracked Cargo packages and the standalone
examples used by the main learning modules. Local checks should be run before a
pull request; the exact commands are documented in
[CONTRIBUTING.md](CONTRIBUTING.md).

Planned additions and known content gaps are listed in [ROADMAP.md](ROADMAP.md).
For a compact sequence through the current material, see
[LEARNING_PATH.md](LEARNING_PATH.md).

## Contributing

Corrections, clearer explanations, new exercises, tests, and small focused
examples are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) before
opening a pull request. Community expectations are described in the
[Code of Conduct](CODE_OF_CONDUCT.md), and security reports should follow
[SECURITY.md](SECURITY.md).

## License

This project is available under the [MIT License](LICENSE).
