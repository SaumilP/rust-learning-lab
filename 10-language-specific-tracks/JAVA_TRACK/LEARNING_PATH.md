# Learning Path for Java Developers

This guide maps out a practical path for Java developers learning Rust. I've organized it around the mental shifts you'll need to make, not just syntax differences.

## The mental model shift

The biggest challenge isn't learning Rust syntax—it's unlearning some Java assumptions. Here are the core shifts:

### From garbage collection to ownership

In Java, you create objects and the GC figures out when to clean them up. You can have dozens of references to the same object, and everything just works™.

```java
// Java: References everywhere
String name = "Alice";
String anotherRef = name;  // Both point to same data
String yetAnother = name;  // No problem!
```

Rust forces you to think about *ownership*. At any moment, there's exactly one owner of each piece of data. When that owner goes out of scope, the data is cleaned up immediately.

This feels restrictive at first, but it's actually liberating: you never wonder "when will this memory be freed?" You know exactly when.

### From exceptions to Result types

Java has checked and unchecked exceptions. Sometimes you handle them, sometimes you don't, and sometimes you get surprised at runtime.

```java
// Java: Exceptions might be hiding
public void processFile(String path) {
    // This might throw, might not, who knows?
    String content = Files.readString(Path.of(path));
}
```

Rust makes errors explicit in the type system. If a function can fail, it returns `Result<T, E>`. You *have* to deal with it.

```rust
// Rust: Errors are in the type signature
fn process_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)  // Returns Result
}
```

### From inheritance to composition

Java loves inheritance hierarchies. You extend classes, override methods, and build deep inheritance trees.

Rust doesn't have inheritance. At all.

Instead, you compose behavior with traits (like interfaces) and use plain old data structures. It feels limiting until you realize it actually makes code easier to reason about.

### From runtime polymorphism to compile-time dispatch

In Java, when you call `shape.area()`, the JVM looks up which implementation to call at runtime. This is flexible but has a cost.

Rust prefers to figure out which function to call at compile time using generics. The result is zero-cost abstraction: no virtual method table lookups, no runtime overhead.

## Your learning path

Here's the order I recommend:

### Phase 1: Fundamentals (Week 1-2)

**Goal**: Get comfortable with basic syntax and ownership rules

1. **Basic syntax** (1-2 days)
   - Variables and mutability
   - Functions and control flow
   - Basic types (integers, floats, bools, chars)
   - If you know Java, this part is easy

2. **Ownership** (3-4 days)
   - The ownership rules
   - Moving vs borrowing
   - References (&T and &mut T)
   - This is the hard part—take your time

3. **Structs and enums** (2-3 days)
   - Creating data structures
   - Methods (impl blocks)
   - Pattern matching
   - Option and Result types

**Milestone**: Build a simple CLI tool (calculator or file reader)

**Resources**:
- Rust Book chapters 1-6
- Rustlings exercises on ownership
- FUNDAMENTALS_COMPARISON.md (in this track)

### Phase 2: Standard library and error handling (Week 3-4)

**Goal**: Write programs that do real work

1. **Collections** (2-3 days)
   - Vec<T> (like ArrayList)
   - HashMap<K, V> (like HashMap)
   - String vs &str (this is important!)

2. **Error handling** (2-3 days)
   - Result<T, E> in depth
   - The ? operator
   - Custom error types
   - When to panic vs return errors

3. **Traits** (3-4 days)
   - Defining and implementing traits
   - Trait bounds on generics
   - Common traits (Debug, Clone, Display)
   - This is your "interfaces but better"

**Milestone**: Build the Todo CLI project (see MINI_PROJECTS/01-todo-cli)

**Resources**:
- Rust Book chapters 7-10
- DESIGN_PATTERNS_GUIDE.md for trait patterns
- CHEAT_SHEET.md for quick lookups

### Phase 3: Concurrency and async (Month 2)

**Goal**: Write concurrent programs safely

1. **Threading basics** (3-4 days)
   - std::thread::spawn
   - Arc<T> and Mutex<T>
   - Channels (mpsc)
   - Why Rust prevents data races

2. **Async/await** (4-5 days)
   - Understanding Future trait
   - async fn and .await
   - Tokio runtime basics
   - When to use async vs threads

3. **Practical concurrency** (5-7 days)
   - Worker pools
   - Request handling
   - Graceful shutdown
   - Error handling in async code

**Milestone**: Build the Multithreaded Web Server (MINI_PROJECTS/02-multithreaded-server)

**Resources**:
- Rust Book chapter 16 (fearless concurrency)
- Tokio tutorial
- FUNDAMENTALS_COMPARISON.md concurrency section

### Phase 4: Advanced patterns (Month 3)

**Goal**: Write idiomatic Rust

1. **Lifetimes** (4-5 days)
   - Why lifetimes exist
   - Explicit lifetime annotations
   - Lifetime elision rules
   - 'static lifetime

2. **Advanced traits** (4-5 days)
   - Associated types
   - Default implementations
   - Trait objects (dyn Trait)
   - When to use generics vs trait objects

3. **Iterators** (3-4 days)
   - Iterator trait
   - Iterator combinators
   - Writing custom iterators
   - Performance characteristics

4. **Smart pointers** (3-4 days)
   - Box<T> (heap allocation)
   - Rc<T> and Arc<T> (reference counting)
   - RefCell<T> (interior mutability)
   - When to use each

**Milestone**: Build the Async Database Client (MINI_PROJECTS/03-async-database-client)

**Resources**:
- Rust Book chapters 15, 17, 19
- DESIGN_PATTERNS_GUIDE.md
- ANTI_PATTERNS.md (to avoid common mistakes)

## Common stumbling blocks for Java developers

Based on conversations with dozens of Java developers, here's where people get stuck:

### Week 1: "Why can't I just pass this variable?"

The borrow checker will reject code that seems perfectly fine to you. This is normal. Read the error messages—they're actually helpful.

**What helps**:
- Start with references (&T) for reading
- Use mutable references (&mut T) only when you need to modify
- Clone when you're stuck (you'll learn when to avoid this later)

### Week 2-3: "String vs &str is so confusing"

Yeah, it is. Here's the simple rule:
- Use `&str` when you're just reading/passing around text
- Use `String` when you need to own/modify text
- When in doubt, start with `&str`

### Month 2: "Why do I need lifetimes?"

You'll probably try to write something like this:

```rust
fn get_first<'a>(items: &'a Vec<String>) -> &'a str {
    &items[0]  // Returning a reference to borrowed data
}
```

Lifetimes just tell the compiler "this reference I'm returning lives as long as the input". Once you understand *why* this matters (preventing dangling pointers), it makes sense.

### Month 3: "When do I use Box vs Arc vs Rc?"

- `Box<T>`: Single owner, heap-allocated (use this most often)
- `Rc<T>`: Multiple owners, single-threaded (like Java references)
- `Arc<T>`: Multiple owners, thread-safe (like Java with synchronized)

## Progress checkpoints

How do you know you're making progress? Here are some signs:

**You're getting somewhere when**:
- Compiler errors start making sense
- You fix borrow checker errors without googling
- You read code and can predict what will compile
- You stop trying to write Java-style code

**You're proficient when**:
- You write ownership-correct code on the first try
- You know when to clone vs borrow without thinking
- You can read standard library source code
- You contribute to open source Rust projects

**You're experienced when**:
- You design APIs around ownership
- You use lifetimes fluently
- You know the performance implications of your choices
- The borrow checker feels like a helpful assistant

## Tips for staying motivated

Learning Rust is harder than learning most languages. Here's how to stick with it:

1. **Build things** - Reading is good, but building cements the knowledge

2. **Don't compare to Java** - Rust isn't "better Java". It solves different problems.

3. **Embrace the struggle** - If you're frustrated, you're learning. The borrow checker fight is a rite of passage.

4. **Join the community** - The Rust Discord is incredibly helpful. Ask questions!

5. **Celebrate small wins** - Your first program that compiles on the first try is a milestone

6. **Use the right tools**:
   - `cargo check` - Fast feedback without codegen
   - `cargo clippy` - Catches un-idiomatic code
   - `cargo fmt` - Formats code consistently
   - `rust-analyzer` - IDE support (works with VS Code, IntelliJ)

## What to focus on each week

Here's a realistic weekly breakdown:

**Week 1**: Ownership and borrowing
- Don't try to learn everything
- Focus on move semantics
- Practice with small examples

**Week 2**: Structs, enums, pattern matching
- Build a simple project
- Use Option and Result
- Get comfortable with match expressions

**Week 3-4**: Traits and error handling
- Implement traits for your types
- Write proper error handling
- Complete the Todo CLI project

**Week 5-6**: Collections and iterators
- Replace loops with iterator chains
- Learn when to collect vs iterate
- Use HashMap and Vec fluently

**Week 7-8**: Basic concurrency
- Threads and channels
- Shared state with Arc/Mutex
- Understand Send and Sync traits

**Week 9-12**: Async/await and advanced topics
- Tokio async runtime
- Async functions and futures
- Complete the Web Server project

## Next steps

Ready to dive in? Here's your immediate action plan:

1. **Read FUNDAMENTALS_COMPARISON.md** - Get the mental models right
2. **Do Rustlings** - Interactive exercises (install with `cargo install rustlings`)
3. **Start Mini-Project 01** - Todo CLI (MINI_PROJECTS/01-todo-cli)
4. **Keep CHEAT_SHEET.md handy** - Quick reference for Java → Rust

Remember: every expert Rust developer was once where you are now, staring at borrow checker errors and wondering if they'd ever get it. You will get it. It just takes time.

Good luck! 🦀

---

*Pro tip: When you get stuck, explaining the problem to a rubber duck (or the Rust Discord) often helps you figure it out yourself.*
