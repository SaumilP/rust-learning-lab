# Module 03: Tooling & Code Quality

Welcome to Tooling & Code Quality! This module covers testing, documentation, and tools that help you write better Rust code.

## Learning Objectives

By completing this module, you will understand:
- How to write unit tests and integration tests
- Documentation best practices and doc tests
- Code quality tools (clippy, rustfmt)
- Debugging techniques and strategies
- Building professional-quality Rust projects

## Module Structure

### 1. Testing
**Concepts**:
- Unit tests with `#[test]`
- Test organization and modules
- Assertion macros (`assert!`, `assert_eq!`, etc.)
- Test fixtures and setup
- Running specific tests
- Integration tests

**Time**: 1.5-2 hours
**Prerequisite**: Module 01 (Functions)

**Key Files**:
- `testing/README.md` - Detailed explanation
- `testing/examples/` - Working code examples
- `testing/key_takeaways.md` - Quick reference

---

### 2. Documentation
**Concepts**:
- Doc comments (`///`, `//!`)
- Markdown in documentation
- Doc tests (running examples)
- Examples in documentation
- Generating HTML docs with `cargo doc`
- Best practices for documentation

**Time**: 1-1.5 hours
**Prerequisite**: Module 01 (Functions, Comments)

**Key Files**:
- `documentation/README.md` - Detailed explanation
- `documentation/examples/` - Working code examples
- `documentation/key_takeaways.md` - Quick reference

---

### 3. Code Quality Tools
**Concepts**:
- Clippy (linting) - catching common mistakes
- Rustfmt (formatting) - consistent style
- Cargo check - fast compilation checking
- Cargo build variations (debug vs release)
- Working with warnings and errors

**Time**: 1-1.5 hours
**Prerequisite**: Basic Rust knowledge

**Key Files**:
- `code_quality_tools/README.md` - Detailed explanation
- `code_quality_tools/examples/` - Working code examples
- `code_quality_tools/key_takeaways.md` - Quick reference

---

### 4. Debugging
**Concepts**:
- `println!` debugging
- `dbg!` macro for quick debugging
- Basic debugging workflow
- Identifying bottlenecks
- Reading error messages
- Common debugging strategies

**Time**: 1-1.5 hours
**Prerequisite**: Module 01, 02

**Key Files**:
- `debugging/README.md` - Detailed explanation
- `debugging/examples/` - Working code examples
- `debugging/key_takeaways.md` - Quick reference

---

## Exercises

Practice writing tests, documentation, and using tools:

### Exercise 1: Write Failing Tests
**Concepts Tested**: Unit Testing, Test Organization
**Difficulty**: Easy

Write tests first, then fix the code to pass them (TDD approach).

### Exercise 2: Add Documentation
**Concepts Tested**: Doc Comments, Examples, Doc Tests
**Difficulty**: Easy-Medium

Document existing code and verify with doc tests.

### Exercise 3: Fix Clippy Warnings
**Concepts Tested**: Code Quality, Best Practices
**Difficulty**: Easy-Medium

Identify and fix clippy warnings in provided code.

### Exercise 4: Debug Output
**Concepts Tested**: Debugging, `println!`, `dbg!`
**Difficulty**: Medium

Use debugging techniques to identify bugs in code.

---

## Learning Path

### Recommended Order
1. Start with **Testing** (verify your code works)
2. Learn **Documentation** (help others understand)
3. Study **Code Quality Tools** (write professional code)
4. Master **Debugging** (fix problems efficiently)
5. Complete exercises in order

### Time Estimate
- Concepts: 5-7 hours
- Exercises: 2-3 hours
- Total: 7-10 hours for complete mastery

### Progression Tips
- Write tests as you develop (TDD)
- Document public APIs thoroughly
- Run clippy regularly while developing
- Use debugging tools instead of guessing

---

## Prerequisites

Before starting this module:
- Complete Module 01 and 02
- Understand Rust syntax and functions
- Be comfortable with basic Rust concepts
- Have Rust installed and Cargo working

### Quick Verification
```bash
# Test that your setup is ready
cargo new test_project
cd test_project
cargo test
cargo doc --open
```

---

## Related Concepts

### Foundation
- **Module 01**: Functions and control flow
- **Module 02**: Collections and error handling

### Application
- **Module 04+**: All programs need testing and documentation
- **Challenges**: Tests verify your solutions work

### Advanced
- **Continuous Integration**: Testing in CI/CD pipelines
- **Performance**: Profiling and benchmarking
- **Security**: Tools for security auditing

---

## Common Patterns

### Writing Tests
```rust
#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4);
}
```

### Documentation
```rust
/// Adds two numbers together.
///
/// # Examples
/// ```
/// assert_eq!(add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Using Clippy
```bash
cargo clippy
cargo clippy -- -W clippy::all
```

### Debugging
```rust
let x = 5;
dbg!(x);  // Prints value and location
println!("x = {}", x);
```

---

## Tool Cheat Sheet

### Testing
| Command | Purpose |
|---------|---------|
| `cargo test` | Run all tests |
| `cargo test test_name` | Run specific test |
| `cargo test -- --nocapture` | Show println! output |
| `#[should_panic]` | Test that panics |
| `#[ignore]` | Skip test |

### Documentation
| Task | How |
|------|-----|
| Generate docs | `cargo doc --open` |
| Doc comment | `/// comment` |
| Module doc | `//! comment` |
| Doc test | Code in `///` runs as test |

### Quality Tools
| Tool | Command | Purpose |
|------|---------|---------|
| Clippy | `cargo clippy` | Linting |
| Rustfmt | `cargo fmt` | Formatting |
| Check | `cargo check` | Fast validation |
| Build | `cargo build` | Compile |

---

## Resources

### Official Documentation
- [The Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rustdoc Guide](https://doc.rust-lang.org/rustdoc/)
- [Clippy Lint List](https://rust-lang.github.io/rust-clippy/)

### Tools
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Rust Formatter](https://github.com/rust-lang/rustfmt)
- [Debugging Guide](https://docs.rust-embedded.org/book/appendix-01-installation.html)

### Learning
- [Test-Driven Development in Rust](https://blog.logrocket.com/unit-testing-in-rust/)
- [Documentation Best Practices](https://rust-lang.github.io/api-guidelines/documentation.html)

---

## Module Status

- Complete — Structure established
- In progress — Concept documentation in progress
- In progress — Code examples needed
- In progress — Exercises pending

## Next Steps

1. Read through each concept README
2. Run and modify all examples
3. Complete exercises
4. Move to Module 04 or continue with other modules

---

**Last Updated**: 2026-08-27
**Estimated Completion**: Phase 3-4
**Questions?** Refer to the Rust Book or official documentation.
