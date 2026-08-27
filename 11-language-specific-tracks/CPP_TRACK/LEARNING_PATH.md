# Learning Path for C/C++ Developers

This guide maps out a practical path for C/C++ developers learning Rust. You already understand memory management, performance, and low-level programming—Rust builds on that knowledge while adding safety guarantees.

## The mental model shift

The biggest challenge isn't learning new syntax—it's understanding how Rust's ownership system gives you C/C++ performance with memory safety guarantees you've never had before.

### From manual memory management to ownership

In C/C++, you manually track object lifetimes:

```cpp
// C++
void process() {
    Resource* r = new Resource();
    // ... use r ...
    delete r;  // Don't forget!
    // What if we returned early? Memory leak!
    // What if we delete twice? Crash!
    // What if we use after delete? Use-after-free!
}

// Modern C++
void process() {
    auto r = std::make_unique<Resource>();
    // Automatically cleaned up
    // But you can still get raw pointers and shoot yourself
}
```

Rust makes ownership explicit and enforced:

```rust
// Rust
fn process() {
    let r = Resource::new();
    // ... use r ...
    // Automatically dropped, no other option
    // Can't forget to free
    // Can't double-free
    // Can't use-after-free
}
```

This feels restrictive at first, but it's liberating: the compiler prevents entire classes of bugs.

### From NULL to Option

```c
// C - NULL can appear anywhere
int* ptr = get_value();
if (ptr != NULL) {
    *ptr = 42;
}
// Forgot to check? Segfault!
```

```cpp
// Modern C++ - optional helps
std::optional<int> maybe = get_value();
if (maybe.has_value()) {
    *maybe = 42;
}
// But you can still get raw pointers
```

```rust
// Rust - no null pointers exist
let maybe: Option<i32> = get_value();
match maybe {
    Some(value) => println!("{}", value),
    None => println!("No value"),
}
// Compiler forces you to handle both cases
```

### From exceptions/errno to Result

```cpp
// C++ - exceptions are invisible
void process_file(const std::string& path) {
    // Might throw, who knows?
    auto data = read_file(path);
}
```

```rust
// Rust - errors are in the type signature
fn process_file(path: &str) -> Result<(), std::io::Error> {
    let data = read_file(path)?;  // Explicit error propagation
    Ok(())
}
```

### From inheritance to composition

C++ loves inheritance hierarchies. Rust doesn't have inheritance at all.

```cpp
// C++
class Animal {
public:
    virtual void speak() = 0;
};

class Dog : public Animal {
public:
    void speak() override { std::cout << "Woof!\n"; }
};
```

```rust
// Rust - composition + traits
trait Animal {
    fn speak(&self);
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
```

## Your learning path

Here's the recommended order for C/C++ developers:

### Phase 1: Core Ownership (Week 1-2)

**Goal**: Understand ownership, borrowing, and lifetimes

1. **Basic syntax** (1-2 days)
   - Variables and types (similar to C++)
   - Functions and control flow
   - Basic types
   - If you know C++, this is easy

2. **Ownership system** (4-5 days)
   - The three ownership rules
   - Moving vs copying
   - References (&T and &mut T)
   - **This is completely different from C++**—take your time

3. **Lifetimes** (3-4 days)
   - Why lifetimes exist (preventing dangling pointers)
   - Explicit lifetime annotations
   - Lifetime elision
   - Think of it as compile-time borrow checking

**Milestone**: Rewrite a simple C/C++ utility in Rust

**Resources**:
- Rust Book chapters 1-10
- Rustlings ownership exercises
- FUNDAMENTALS_COMPARISON.md

### Phase 2: Rust idioms (Week 3-4)

**Goal**: Write idiomatic Rust, not C++ in Rust syntax

1. **Enums and pattern matching** (2-3 days)
   - Algebraic data types (not just C-style enums!)
   - Pattern matching with `match`
   - Option<T> and Result<T, E>
   - This is more powerful than C++ variants

2. **Traits** (3-4 days)
   - Defining and implementing traits
   - Trait bounds on generics
   - Common traits (Clone, Copy, Debug, Display)
   - Similar to C++ concepts, but clearer

3. **Error handling** (2-3 days)
   - Result<T, E> patterns
   - The `?` operator
   - Custom error types
   - No exceptions—errors are values

**Milestone**: Build the Memory Allocator mini-project

**Resources**:
- Rust Book chapters 6, 9, 10
- DESIGN_PATTERNS_GUIDE.md
- CHEAT_SHEET.md

### Phase 3: Systems programming (Month 2)

**Goal**: Build real systems code in Rust

1. **Collections and iterators** (3-4 days)
   - Vec<T>, HashMap<K, V>, etc.
   - Iterator trait and combinators
   - Zero-cost abstractions
   - Compare performance with C++ STL

2. **Smart pointers** (3-4 days)
   - Box<T> (like unique_ptr)
   - Rc<T> (like shared_ptr, single-threaded)
   - Arc<T> (like shared_ptr, thread-safe)
   - RefCell<T> (interior mutability)

3. **Unsafe Rust** (4-5 days)
   - When and why to use unsafe
   - Raw pointers
   - FFI with C/C++
   - Writing safe abstractions around unsafe code

**Milestone**: Build the Concurrent Data Structure mini-project

**Resources**:
- Rust Book chapters 15, 19
- Unsafe Rust chapter
- FUNDAMENTALS_COMPARISON.md (FFI section)

### Phase 4: Concurrent systems (Month 3)

**Goal**: Write fearless concurrent code

1. **Concurrency basics** (3-4 days)
   - std::thread
   - Send and Sync traits
   - Why data races are impossible
   - Compare with C++ threading

2. **Shared state** (3-4 days)
   - Mutex<T> and RwLock<T>
   - Arc<Mutex<T>> pattern
   - Atomic types
   - Lock-free programming

3. **Async/await** (5-6 days)
   - async fn and Future trait
   - Tokio runtime
   - When to use async vs threads
   - Compare with C++20 coroutines

**Milestone**: Build the Systems Tool mini-project

**Resources**:
- Rust Book chapter 16
- Tokio documentation
- DESIGN_PATTERNS_GUIDE.md (concurrency patterns)

### Phase 5: Advanced topics (Month 4+)

**Goal**: Master advanced Rust

1. **Macros** (4-5 days)
   - Declarative macros
   - Procedural macros
   - Code generation at compile time

2. **Advanced traits** (4-5 days)
   - Associated types
   - Higher-ranked trait bounds
   - Blanket implementations

3. **Performance optimization** (ongoing)
   - Profiling with perf/valgrind
   - SIMD
   - Inline assembly
   - Matching C++ performance

**Milestone**: Contribute to open source Rust projects

**Resources**:
- Rust Book advanced chapters
- Rustonomicon (unsafe Rust guide)
- Performance book

## Common stumbling blocks for C/C++ developers

Based on experience helping hundreds of C++ developers learn Rust:

### Week 1: "Why won't the compiler let me...?"

The borrow checker will reject code that C++ would accept. This is frustrating but temporary.

**What helps**:
- The compiler is preventing real bugs (use-after-free, data races)
- Read error messages carefully—they suggest fixes
- When stuck, clone first, optimize later

### Week 2: "Lifetimes are complicated"

Lifetimes feel foreign because C++ doesn't have them explicitly.

**What helps**:
- They're just compile-time borrow checking
- Most code doesn't need explicit lifetimes (elision)
- Think: "How long is this reference valid?"

### Week 3: "Why can't I just use raw pointers?"

Coming from C++, you might reach for unsafe code too quickly.

**What helps**:
- Try safe Rust first—it's more powerful than you think
- unsafe should be rare (< 1% of code)
- Most C++ use cases have safe Rust equivalents

### Month 2: "The borrow checker is being pedantic"

You'll write code that's "obviously safe" but the compiler rejects it.

**What helps**:
- The compiler prevents bugs you didn't see
- Redesign your data structures to fit ownership
- Use Arc/Rc only when truly needed

## Progress checkpoints

How do you know you're making progress?

**You're getting somewhere when**:
- Borrow checker errors make sense
- You design with ownership in mind
- You write code that compiles on first try
- You stop reaching for unsafe

**You're proficient when**:
- You understand when to use Box/Rc/Arc
- Lifetimes feel natural
- You contribute to Rust projects
- You catch bugs the compiler missed in C++

**You're experienced when**:
- You design APIs around ownership
- You know when unsafe is justified
- You teach others Rust
- You prefer Rust to C++ for new projects

## Tips for staying motivated

Learning Rust is harder than learning most languages—but easier than mastering C++.

1. **Leverage your C++ knowledge**
   - You understand memory already
   - You know what undefined behavior is
   - You appreciate zero-cost abstractions

2. **Embrace the differences**
   - Don't fight the borrow checker
   - Redesign instead of working around
   - Trust the compiler

3. **Compare performance**
   - Benchmark Rust vs your C++ code
   - See that it's just as fast
   - Appreciate the safety guarantees

4. **Join the community**
   - r/rust on Reddit
   - Rust Discord
   - This Week in Rust newsletter

5. **Build real projects**
   - Port a C++ tool to Rust
   - Write new code in Rust
   - See the benefits firsthand

## What to focus on each week

**Week 1**: Ownership
- Spend time on this—it's foundational
- Do all the Rustlings ownership exercises
- Don't rush

**Week 2**: Borrowing and lifetimes
- Understand references deeply
- Practice with different lifetime scenarios
- It clicks eventually

**Week 3-4**: Idiomatic Rust
- Pattern matching everywhere
- Use iterators instead of loops
- Embrace Result and Option

**Week 5-6**: Collections and smart pointers
- When to use Box vs Rc vs Arc
- Iterator combinators
- Compare with C++ STL

**Week 7-8**: Concurrency
- Understand Send and Sync
- See data races prevented at compile time
- This is magical after C++ threading

**Month 3+**: Advanced features
- Unsafe Rust when needed
- Macros for code generation
- Performance optimization

## Comparing your experience levels

If you're comfortable in C/C++, here's how concepts map:

| C/C++ Experience | Rust Equivalent | Learning Time |
|------------------|-----------------|---------------|
| Pointers | References & ownership | 2 weeks |
| Manual memory mgmt | Ownership system | 2 weeks |
| RAII | Drop trait | 2 days |
| unique_ptr | Box<T> | 1 day |
| shared_ptr | Rc<T>/Arc<T> | 2 days |
| Templates | Generics | 1 week |
| Virtual functions | Trait objects | 3 days |
| Exceptions | Result<T, E> | 1 week |
| Multithreading | std::thread + ownership | 1 week |
| const | & vs &mut | 3 days |

## Next steps

Ready to dive in? Here's your immediate action plan:

1. **Read FUNDAMENTALS_COMPARISON.md** - See C++ patterns in Rust
2. **Install Rust** - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. **Do Rustlings** - `cargo install rustlings; rustlings`
4. **Start Mini-Project 1** - Memory Allocator (MINI_PROJECTS/01-memory-allocator)
5. **Keep CHEAT_SHEET.md handy** - Quick C++ → Rust reference

## Managing expectations

**After 1 week**: Fighting the borrow checker constantly. Normal. Keep going.

**After 2 weeks**: Ownership clicks intellectually. Still making mistakes.

**After 1 month**: Writing working Rust code. Still learning idioms.

**After 2 months**: Comfortable with Rust. Appreciate the safety.

**After 3 months**: Productive in Rust. Miss the borrow checker in C++.

**After 6 months**: Prefer Rust for new projects. Understand the tradeoffs.

## The honest truth

**C++ is more mature**. It has more libraries, more documentation, more jobs.

**But Rust is better for new projects** if you want:
- Memory safety without GC
- Fearless concurrency
- Fearless refactoring
- Modern tooling
- Less time debugging segfaults

The learning curve is steep, but you've climbed steeper (you learned C++!).

## Remember

C++ taught you to write fast, low-level code.
Rust will teach you to write *safe* fast, low-level code.

And once you experience impossible-to-compile data races and use-after-free bugs caught at compile time, you won't want to go back.

Welcome to Rust! 🦀

---

*Pro tip: When the borrow checker rejects your code, ask yourself: "Could this cause a use-after-free or data race in C++?" The answer is usually yes.*
