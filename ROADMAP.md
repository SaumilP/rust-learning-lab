# 🗺️ Rust Learning Lab - Roadmap

This roadmap outlines a **recommended learning path** through the repostiory. <br />
You can follow it sequentially or jump to topics you need the most.

The goal is not just to *learn Rust syntax*, but to:

- Build correct mental model
- Write idiomatic Rust
- Gain confidence for real-world Rust projects

---

## 🟢 Beginner Level

> Focus: Rust basics, ownership, and confidence with the compiler

### Setup & Basics

- [ ] Install Rust & toolchain
- [ ] `cargo` basics
- [ ] Hello World
- [ ] Variables & mutability
- [ ] Data types
- [ ] Control flow

📁 `00-setup-and-basics/`

---

### Core fundamentals

- [ ] Ownership rules
- [ ] Move vs Copy semantics
- [ ] Borrowing & references
- [ ] Lifetimes (introduction)
- [ ] Structs & enums
- [ ] Pattern matching
- [ ] Basic error handling (`Result`, `Option`)

📁 `01-core-fundamentals/`

---

### Standard Library Essentials

- [ ] `Vec`, `HashMap`
- [ ] `String` vs `&str`
- [ ] Iterators
- [ ] Traits (basics)
- [ ] Generics
- [ ] Smart Pointers (`Box`, `Rc`, `RefCell`)

📁 `02-standard-library/`

---

## Intermediate Level

> Focus: writing maintainable, testable, idiomatic Rust

### Tooling & Code Quality

- [ ] Cargo workspaces
- [ ] Formatting with `rustfmt`
- [ ] Linting with `clippy`
- [ ] Logging
- [ ] Writing unit tests
- [ ] Integration tests
- [ ] Documentation comments

📁 `03-tooling-and-quality/`

---

### Simple Programs

- [ ] CLI calculator
- [ ] Word Counter
- [ ] File search tool
- [ ] Todo CLI app

📁 `04-simple-programs/`

---

### CLI & Console Games

- [ ] Tic-Tac-Toe
- [ ] Hangman
- [ ] Snake
- [ ] Minesweeper
- [ ] ASCII Roguelike

Focus areas:

- State management
- Enums & pattern matching
- Modular code design
- Input handling

📁 `05-cli-and-console-games/`

---

### Intermediate Rust Concepts

- [ ] Modules & crate structure
- [ ] Error handling best practices
- [ ] Custom error types
- [ ] Concurrency (threads, mutexes)
- [ ] Channels
- [ ] Async/await
- [ ] Introduction to macros

📁 `06-intermediate-rust/`

---

### Design Patterns

- [ ] Creational patterns
- [ ] Structural patterns
- [ ] Behavioral patterns
- [ ] Rust-idioatic patterns:
  - Newtype
  - Typestate
  - RAII
  - Interior mutability

📁 `07-design-patterns/`

---

## 🔵 Advanced / Real-world (Optional)

> Focus: performance, safety boundaries, and production readiness

### Mini Projects

- [ ] HTTP Server
- [ ] Chat Application
- [ ] Log parser
- [ ] Key-value store
- [ ] Markdown Parser

📁 `08-mini-projects/`

---

### Real-World Rust

- [ ] Performance optimization
- [ ] Benchmarking
- [ ] Profiling
- [ ] Unsafe Rust
- [ ] FFI (C interoperability)

📁 `09-real-world-rust/`

---

## 🧠 Challenges

- [ ] Beginner challenges
- [ ] Intermediate challenges
- [ ] Interview-style problems

📁 `challenges/`

---

## ✅ Completion Goal

By completing this roadmap, you should be able to:
- Confidently read & write Rust code
- Design small-to-medium Rust projects
- Debug ownwership & lifetime projects
- Understand async & concurrency tradeoffs
- Write idiomatic, maintainable Rust

---

Happy hacking 🦀