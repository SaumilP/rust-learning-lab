# Language-Specific Learning Tracks

**Rust Learning Lab - Polyglot Edition**

Welcome! This directory contains dedicated learning tracks for developers transitioning to Rust from Java, Python, or Go. Each track provides language-specific comparisons, mental model translations, and hands-on projects tailored to your background.

---

## 🎯 Choose Your Track

### [Java Developer Track](./JAVA_TRACK/) ✅ **COMPLETE**

**For**: Spring Boot, Jakarta EE, Android developers
**Status**: 100% Complete - Ready to use!

You'll learn:
- Ownership vs Garbage Collection mental models
- Traits vs Interfaces (and why traits are more powerful)
- Error handling with `Result<T, E>` instead of exceptions
- Fearless concurrency without `synchronized` blocks
- Zero-cost abstractions vs runtime polymorphism

**Mini-Projects**:
1. ✅ Todo CLI - Ownership + basic structures (COMPLETE with code)
2. Multithreaded Web Server - Concurrency patterns (README complete)
3. Async Database Client - Async/await ecosystem (README complete)

[**Start Learning →**](./JAVA_TRACK/LEARNING_PATH.md)

---

### [Python Developer Track](./PYTHON_TRACK/) ⚠️ **IN PROGRESS**

**For**: Django/Flask developers, data scientists, automation engineers
**Status**: 20% Complete - Foundation ready

You'll learn:
- Static typing that actually helps (vs optional type hints)
- Breaking free from the GIL for true parallelism
- Compile-time guarantees instead of runtime surprises
- Iterator chains similar to comprehensions
- Memory efficiency without garbage collection

**Mini-Projects** (Planned):
1. Data Processing Pipeline - Types + iterators (pandas-like)
2. REST API Server - Web frameworks (Flask/FastAPI comparison)
3. Async File Processor - Concurrent operations (asyncio comparison)

[**Start Learning →**](./PYTHON_TRACK/README.md)

**Next Steps**: FUNDAMENTALS_COMPARISON.md and mini-projects need implementation

---

### [Go Developer Track](./GO_TRACK/) ⚠️ **IN PROGRESS**

**For**: Microservices developers, systems programmers, DevOps engineers
**Status**: 20% Complete - Foundation ready

You'll learn:
- Ownership vs GC - deterministic cleanup
- Explicit vs implicit interfaces (trait system)
- Async/await vs goroutines (different, not harder)
- Error handling: `Result<T, E>` vs `if err != nil`
- RAII (Drop trait) vs defer

**Mini-Projects** (Planned):
1. CLI Tool - Traits + error handling (Cobra comparison)
2. Worker Pool - Channels + concurrency (goroutine patterns)
3. Service Discovery - Async networking (gRPC/REST client)

[**Start Learning →**](./GO_TRACK/README.md)

**Next Steps**: FUNDAMENTALS_COMPARISON.md and mini-projects need implementation

---

## 📊 What Each Track Includes

Every language track provides:

### 📚 Learning Materials

| Document | Purpose | Java | Python | Go |
|----------|---------|------|--------|-----|
| **README.md** | Track overview & motivation | ✅ | ✅ | ✅ |
| **LEARNING_PATH.md** | Progressive curriculum | ✅ | 📋 | 📋 |
| **FUNDAMENTALS_COMPARISON.md** | Side-by-side code examples | ✅ | 📋 | 📋 |
| **DESIGN_PATTERNS_GUIDE.md** | Pattern translations | ✅ | 📋 | 📋 |
| **ANTI_PATTERNS.md** | Common mistakes | ✅ | 📋 | 📋 |
| **CHEAT_SHEET.md** | Quick reference | ✅ | 📋 | 📋 |
| **GOTCHAS.md** | Language-specific surprises | ✅ | 📋 | 📋 |
| **TIPS_AND_TRICKS.md** | Productivity guide | ✅ | 📋 | 📋 |

**Legend**: ✅ Complete | 📋 Planned

### 🛠️ Mini-Projects

Each track includes 3 projects (Beginner → Intermediate → Advanced):

**Difficulty Levels**:
- ★★☆☆☆ Beginner - Core concepts, ~300 LOC
- ★★★☆☆ Intermediate - Concurrency, ~450 LOC
- ★★★★☆ Advanced - Async + advanced patterns, ~550 LOC

All mini-projects include:
- Complete working code (or detailed skeleton)
- Comprehensive README with language comparisons
- Tests and examples
- Architecture diagrams
- Challenge extensions

---

## 🎓 How to Use These Tracks

### Recommended Learning Path

1. **Choose your track** based on your primary language
2. **Read the track README** to understand the journey
3. **Follow LEARNING_PATH.md** for progressive learning
4. **Study FUNDAMENTALS_COMPARISON.md** - this is crucial!
5. **Build mini-projects** in order (they build on each other)
6. **Reference other guides** as needed (cheat sheet, gotchas, etc.)
7. **Browse other tracks** for additional perspectives

### Time Investment

| Track Progress | Estimated Time |
|----------------|----------------|
| **Core Documentation** | 8-12 hours reading |
| **Mini-Project 1** | 4-6 hours |
| **Mini-Project 2** | 8-12 hours |
| **Mini-Project 3** | 12-16 hours |
| **Total per Track** | 32-46 hours |

**Timeline**: Most developers become productive in Rust within 2-3 months of part-time study.

---

## 🔍 Quick Comparison: Your Language vs Rust

### Java → Rust

```java
// Java: GC manages memory
List<String> names = new ArrayList<>();
names.add("Alice");
String first = names.get(0);
```

```rust
// Rust: Ownership manages memory
let mut names = Vec::new();
names.push(String::from("Alice"));
let first = &names[0];  // Borrow, don't move
```

**Key Difference**: Rust's compiler enforces ownership rules Java's GC handles at runtime.

### Python → Rust

```python
# Python: Dynamic typing
def process(items):
    return [x * 2 for x in items if x > 0]
```

```rust
// Rust: Static typing with inference
fn process(items: &[i32]) -> Vec<i32> {
    items.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .collect()
}
```

**Key Difference**: Rust catches type errors at compile time, Python at runtime.

### Go → Rust

```go
// Go: Implicit interfaces
type Reader interface {
    Read(p []byte) (n int, err error)
}
// Any type with Read() is a Reader
```

```rust
// Rust: Explicit trait implementation
trait Reader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}
impl Reader for MyType { /* ... */ }
```

**Key Difference**: Go's interfaces are flexible; Rust's traits are explicit and more powerful.

---

## 📈 Repository Structure

```
10-language-specific-tracks/
├── README.md (you are here)
├── IMPLEMENTATION_STATUS.md (detailed progress tracking)
│
├── JAVA_TRACK/ ✅ COMPLETE
│   ├── README.md
│   ├── LEARNING_PATH.md
│   ├── FUNDAMENTALS_COMPARISON.md
│   ├── DESIGN_PATTERNS_GUIDE.md
│   ├── ANTI_PATTERNS.md
│   ├── CHEAT_SHEET.md
│   ├── GOTCHAS.md
│   ├── TIPS_AND_TRICKS.md
│   └── MINI_PROJECTS/
│       ├── 01-todo-cli/ (complete with code)
│       ├── 02-multithreaded-server/ (README complete)
│       └── 03-async-database-client/ (README complete)
│
├── PYTHON_TRACK/ ⚠️ 20% COMPLETE
│   ├── README.md ✅
│   └── MINI_PROJECTS/ (planned)
│
├── GO_TRACK/ ⚠️ 20% COMPLETE
│   ├── README.md ✅
│   └── MINI_PROJECTS/ (planned)
│
├── diagrams/
│   └── ARCHITECTURE_DIAGRAMS.md (C4 + PlantUML examples)
│
└── CROSS_LANGUAGE/ (planned)
    └── comparison-project/ (same app in all 4 languages)
```

---

## 🎨 Visual Learning: Architecture Diagrams

We provide C4 model diagrams and PlantUML visualizations for:

- System context (how tracks relate)
- Mini-project architectures
- Ownership and borrowing models
- Concurrency patterns
- Deployment comparisons

[**View Diagrams →**](./diagrams/ARCHITECTURE_DIAGRAMS.md)

---

## 🔗 Integration with Main Repository

These tracks complement the main `rust-learning-lab` structure:

| Main Repo Section | Language Tracks Role |
|-------------------|----------------------|
| **00-setup-and-basics** | Tracks provide language-specific setup guides |
| **01-core-fundamentals** | Tracks translate to Java/Python/Go equivalents |
| **04-simple-programs** | Tracks' mini-projects extend these concepts |
| **07-design-patterns** | Tracks show language-specific pattern translations |
| **challenges/** | Tracks provide language-specific challenge hints |

**Use both together** for maximum learning effectiveness!

---

## 📊 Progress Tracking

### Overall Completion

```
Java Track:        ████████████████████ 100%
Python Track:      ████                  20%
Go Track:          ████                  20%
Cross-Language:    .                      0%
Diagrams:          ██████                30%
─────────────────────────────────────────────
Overall:           ████████              40%
```

[**Detailed Status →**](./IMPLEMENTATION_STATUS.md)

---

## 🤝 Contributing

Want to help complete the Python or Go tracks? Here's what's needed:

### High Priority

1. **Python Track**:
   - FUNDAMENTALS_COMPARISON.md (dynamic vs static typing, GIL, etc.)
   - Mini-projects implementation
   - Remaining documentation files

2. **Go Track**:
   - FUNDAMENTALS_COMPARISON.md (goroutines vs async, interfaces vs traits)
   - Mini-projects implementation
   - Remaining documentation files

3. **All Tracks**:
   - More C4/PlantUML diagrams
   - Cross-language comparison project

### Guidelines

- **Tone**: Write peer-to-peer, not AI-generated style
- **Code**: All examples must be idiomatic and compile
- **Comparisons**: Show real code from both languages
- **Honesty**: Acknowledge tradeoffs, don't oversell Rust
- **Testing**: Ensure all mini-projects have working tests

See [IMPLEMENTATION_STATUS.md](./IMPLEMENTATION_STATUS.md) for detailed roadmap.

---

## 📚 External Resources

### By Language Background

**Java Developers**:
- [Migrating from Java to Rust | corrode](https://corrode.dev/learn/migration-guides/java-to-rust/)
- [Rust vs Java for Backend Engineers 2026](https://rustify.rs/articles/rust-vs-java-2026)

**Python Developers**:
- [Migrating from Python to Rust | corrode](https://corrode.dev/learn/migration-guides/python-to-rust/)
- [Rust for Python Developers Guide](https://dasroot.net/posts/2026/04/rust-for-python-developers-migration-guide/)

**Go Developers**:
- [Rust vs Go: Which Language in 2026?](https://blog.jetbrains.com/rust/2025/06/12/rust-vs-go/)
- [Go vs Rust: Honest Backend Comparison](https://levelupgo.dev/blog/go-vs-rust-2026-honest-backend-comparison)

### Universal Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official documentation
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises
- [Rust Playground](https://play.rust-lang.org/) - Try code online

---

## 💡 Learning Tips

### For All Developers

1. **Don't fight the compiler** - It's catching real bugs
2. **Use `cargo clippy`** - It teaches idiomatic Rust
3. **Read error messages** - Rust errors are actually helpful
4. **Start small** - Build simple programs first
5. **Join the community** - Rust Discord/forums are friendly

### Language-Specific Advice

**From Java**: Embrace ownership instead of GC; it's liberating once you get it

**From Python**: Static typing helps more than Python's hints; trust the compiler

**From Go**: Lifetimes are like Go's escape analysis, but explicit

---

## 🎯 Success Metrics

You'll know you're making progress when:

- ✅ Compiler errors start making sense
- ✅ You fix borrow checker issues without googling
- ✅ You write ownership-correct code on first try
- ✅ You appreciate compile-time guarantees
- ✅ You contribute to open source Rust projects

---

## 📝 Feedback & Questions

- **Issues**: Report problems or suggest improvements via GitHub issues
- **Discussions**: Ask questions in the main repository discussions
- **Community**: Join [Rust Discord](https://discord.gg/rust-lang) for help

---

## 📄 License

This content is part of the rust-learning-lab repository and follows the same license.

---

**Ready to start?** Choose your track above and begin your Rust journey! 🚀

**Questions?** Check [IMPLEMENTATION_STATUS.md](./IMPLEMENTATION_STATUS.md) for detailed information about what's available and what's coming.

---

*Last Updated: 2026-08-26*
*Repository: rust-learning-lab*
*Track Version: 0.1.0*
