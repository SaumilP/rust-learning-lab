# Setup and Basics

Welcome to the Rust Learning Lab! This section covers the absolute fundamentals you need to start writing Rust code.

## What You'll Learn

1. **Installation** - Setting up Rust on your system
2. **Hello World** - Your first Rust program
3. **Variables & Mutability** - How Rust handles data
4. **Data Types** - Primitive types and type inference
5. **Control Flow** - if/else, loops, and match expressions

## Prerequisites

- Basic programming knowledge in any language
- A computer with internet access
- Terminal/command line familiarity

## Getting Started

Follow the sections in order:

1. Start with `rust_installation/` to set up your environment
2. Write your first program in `hello_world/`
3. Learn about variables in `variables_and_mutability/`
4. Explore types in `data_types/`
5. Master control flow in `control_flow/`
6. Practice with [exercises/](exercises/)

## Quick Start

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Create a new project
cargo new my_project
cd my_project

# Run your project
cargo run

# Check your code (fast)
cargo check

# Build release version
cargo build --release
```

## Learning Path

**Time estimate**: 2-4 hours for complete beginners, 1-2 hours if you know another language

Each subdirectory contains:
- `README.md` - Concept explanation
- `examples/` - Working code samples
- `src/main.rs` - Runnable demonstrations
- Exercises with solutions

## Tips

- Type out the examples yourself (don't copy-paste)
- Run `cargo clippy` to learn idiomatic Rust
- Read compiler errors carefully - they're helpful!
- Use `cargo doc --open` to see documentation

Ready? Let's dive in!
