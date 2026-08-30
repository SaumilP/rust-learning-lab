# Language-Specific Tracks (Comparative Learning)

## Overview

Module 11 helps developers transitioning from other languages learn Rust idiomatically. Each track maps concepts from familiar languages to Rust equivalents.

Start with the [cross-language transition index](CROSS_LANGUAGE_COMPARISON.md) to select a track and find the canonical Rust lessons behind each comparison.

## Available Tracks

### 1. Java Developer → Rust
**Target**: Java professionals with OOP background

**Key Mappings**:
- Classes → Structs + Traits
- Interfaces → Traits
- Inheritance → Trait composition
- Null (NPE) → Option<T> and Result<T, E>
- Try-catch → Result and ? operator
- Checked exceptions → Result types
- Garbage collection → Ownership system
- Threads → Message passing + Arc<Mutex<T>>

**Learning Path**:
1. Ownership vs GC paradigm shift
2. Traits as interface replacement
3. Pattern matching vs switch statements
4. Error handling without exceptions
5. Concurrency without garbage pauses

**Example Equivalence**:
```java
// Java
public class User {
    private String name;
    
    public User(String name) {
        this.name = name;
    }
}
```

```rust
// Rust
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
```

---

### 2. Python Developer → Rust
**Target**: Python developers wanting performance

**Key Mappings**:
- Duck typing → Trait system
- Dynamic types → Static typing + generics
- Decorators → Procedural macros
- List comprehensions → Iterator adapters
- Context managers (with) → RAII pattern
- Modules → Modules and crates
- Exception handling → Result types
- Global state → Singletons with once_cell

**Learning Path**:
1. Static typing benefits
2. Iterator patterns vs list comprehensions
3. Compile-time safety advantages
4. Move semantics and lifetime parameters
5. Macro system fundamentals

**Example Equivalence**:
```python
# Python
numbers = [x*2 for x in range(10) if x % 2 == 0]
```

```rust
// Rust
let numbers: Vec<_> = (0..10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .collect();
```

---

### 3. Go Developer → Rust
**Target**: Go developers seeking stronger type safety

**Key Mappings**:
- Goroutines → Threads + async/await
- Channels → mpsc::channel + std::sync
- Interface{} → Trait objects
- Defer → RAII and Drop trait
- Init functions → Associated functions
- Error as value → Result<T, E>
- Packages → Modules and crates
- No null → Option<T> type

**Learning Path**:
1. Trait system vs interface{}
2. Lifetime parameters
3. Borrow checker paradigm
4. Compile-time guarantees
5. Memory safety without GC

**Key Differences**:
- Go: Simplicity over abstraction
- Rust: Safety and performance over convenience
- Go: Fast compilation
- Rust: Fast execution

---

### 4. C++ Developer → Rust
**Target**: C++ developers learning memory-safe alternatives

**Key Mappings**:
- Pointers → References
- Manual memory management → Ownership
- Smart pointers (unique_ptr, shared_ptr) → Move semantics, Rc, Arc
- Templates → Generics
- Operator overloading → Trait implementations
- RAII → Drop trait
- Undefined behavior → Compile-time errors
- Segfaults → Borrow checker prevention

**Learning Path**:
1. Fearless concurrency
2. Compile-time borrowing checking
3. Trait system for polymorphism
4. Zero-cost abstractions
5. No undefined behavior

**Familiar Concepts**:
- Zero-cost abstractions
- Systems programming
- Performance critical sections
- Custom memory management
- Template metaprogramming → Macros

---

## Track Structure

Each track includes:

### Fundamentals Comparison
- Type system differences
- Memory management paradigms
- Syntax and idioms
- Error handling approaches
- Testing strategies

### Feature Mapping Tables

| Java | Rust | Notes |
|------|------|-------|
| class | struct | Rust adds traits |
| interface | trait | Rust has default implementations |
| null | Option<T> | Prevents null pointer errors |
| throws | Result<T, E> | Checked at compile time |

### Common Gotchas

- Python: Static typing is stricter
- Go: Rust's learning curve steeper
- C++: Borrow checker constraints
- Java: No exceptions, ownership required

### Hands-On Exercises

1. Translate existing code to Rust
2. Rewrite familiar patterns idiomatically
3. Benchmark performance improvements
4. Identify safety advantages
5. Practice new syntax

## Progression

### Week 1: Fundamentals
- Syntax and basic concepts
- Type system understanding
- Ownership introduction

### Week 2: Advanced Concepts
- Traits and generics
- Error handling
- Ownership mastery

### Week 3: Idioms
- Rust-idiomatic code
- Performance patterns
- Advanced features

### Week 4: Projects
- Rewrite familiar project in Rust
- Optimize for performance
- Explore ecosystem

## Estimated Learning Time

| Track | Time | Difficulty |
|-------|------|-----------|
| Java | 30-40 hours | Medium |
| Python | 25-35 hours | Medium |
| Go | 20-30 hours | Medium-Hard |
| C++ | 15-25 hours | Hard |

## Real-World Examples

Each track includes:
- Sample projects in both languages
- Performance comparisons
- Idiomatic vs non-idiomatic code
- Common pitfalls and solutions

## Post-Track Recommendations

1. Contribute to open-source projects
2. Study language-specific libraries
3. Build project in new language
4. Join language community
5. Mentor others transitioning

## Resources

- Official Rust Book
- Language-specific documentation
- Comparison articles and blog posts
- Example repositories
- Community forums
