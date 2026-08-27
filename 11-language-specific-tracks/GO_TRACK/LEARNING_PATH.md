# Learning Path for Go Developers

This guide maps out a practical path for Go developers learning Rust. I've organized it around the mental shifts you'll need to make, not just syntax differences.

## The mental model shift

The biggest challenge isn't learning Rust syntax—it's adjusting from Go's simplicity-first philosophy to Rust's safety-first approach. Here are the core shifts:

### From garbage collection to ownership

In Go, you allocate memory and the GC figures out when to clean it up. You can pass pointers around freely, and everything just works™.

```go
// Go: Pointers everywhere
name := "Alice"
ptr1 := &name
ptr2 := &name  // Multiple pointers, no problem!
```

Rust forces you to think about *ownership*. At any moment, there's exactly one owner of each piece of data. When that owner goes out of scope, the data is cleaned up immediately—no GC pause, no heap scanning, no mark-and-sweep.

```rust
// Rust: One owner
let name = String::from("Alice");
let ptr = name;  // Ownership MOVED, name is now invalid
// println!("{}", name);  // Error! name was moved
```

This feels restrictive at first, but the payoff is huge: no GC pauses, deterministic cleanup, and the compiler prevents entire classes of bugs.

### From `if err != nil` to Result types

Go makes errors explicit as values. You check them, return them, wrap them. Rust does the same, but with a twist: the type system *forces* you to handle them.

```go
// Go: Easy to forget error handling
func readFile(path string) string {
    content, err := os.ReadFile(path)
    // Oops, forgot to check err!
    return string(content)
}
```

```rust
// Rust: Compiler won't let you forget
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)  // Returns Result
}
// If you don't handle the Result, code won't compile
```

### From implicit interfaces to explicit traits

Go's interfaces are implicit—if a type has the right methods, it satisfies the interface. It's flexible and convenient.

```go
// Go: Implicit satisfaction
type Reader interface {
    Read([]byte) (int, error)
}

// Any type with Read() is a Reader
```

Rust's traits are explicit—you must declare that a type implements a trait. This feels more verbose, but it makes dependencies crystal clear and enables powerful features like orphan rules and coherence.

```rust
// Rust: Explicit implementation
trait Reader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

impl Reader for MyType {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Must explicitly implement
    }
}
```

### From goroutines to async/await

Go's goroutines are dead simple: slap a `go` in front of a function call and boom, concurrency. The runtime handles everything.

```go
// Go: Super simple
go doWork()
go doMoreWork()
```

Rust's async is more explicit. You mark functions as `async`, call them with `.await`, and choose a runtime (usually Tokio). It's more ceremony but gives you finer control and better performance.

```rust
// Rust: More explicit
tokio::spawn(async {
    do_work().await;
});
tokio::spawn(async {
    do_more_work().await;
});
```

### From `defer` to Drop (RAII)

Go's `defer` is brilliant for cleanup. You write the cleanup code right next to the setup code.

```go
// Go: Defer for cleanup
file, err := os.Open("data.txt")
if err != nil {
    return err
}
defer file.Close()
```

Rust uses RAII (Resource Acquisition Is Initialization). When a value goes out of scope, its `Drop` implementation runs automatically. It's like `defer`, but automatic and zero-cost.

```rust
// Rust: Drop runs automatically
fn process_file() -> io::Result<()> {
    let file = File::open("data.txt")?;
    // Use file...
    // Close() called automatically when file goes out of scope
    Ok(())
}
```

## Your learning path

Here's the order I recommend:

### Phase 1: Fundamentals (Week 1-2)

**Goal**: Get comfortable with ownership and basic syntax

1. **Basic syntax** (1-2 days)
   - Variables and mutability (`let` vs `let mut`)
   - Functions and control flow
   - Basic types (similar to Go's primitives)
   - If you know Go, this part is easy

2. **Ownership** (3-5 days)
   - The ownership rules (one owner, moves, borrowing)
   - References (&T and &mut T)
   - The borrow checker
   - **This is the hard part**—Go doesn't prepare you for this

3. **Structs and enums** (2-3 days)
   - Creating data structures (like Go structs)
   - Methods (impl blocks, like Go receivers)
   - Enums are much more powerful than Go's iota
   - Pattern matching with `match`

4. **Option and Result** (2-3 days)
   - `Option<T>` instead of nil
   - `Result<T, E>` for error handling
   - The `?` operator (like Go's `if err != nil { return err }`)

**Milestone**: Build a simple CLI tool (see MINI_PROJECTS/01-cli-tool)

**Resources**:
- Rust Book chapters 1-6
- Rustlings exercises on ownership
- FUNDAMENTALS_COMPARISON.md (in this track)

### Phase 2: Traits and collections (Week 3-4)

**Goal**: Write programs that do real work

1. **Collections** (2-3 days)
   - `Vec<T>` (like Go slices, but owned)
   - `HashMap<K, V>` (like Go maps)
   - `String` vs `&str` (important distinction!)
   - Slices (`&[T]`)

2. **Traits in depth** (3-4 days)
   - Defining and implementing traits
   - Trait bounds on generics
   - Common traits (Debug, Clone, Display, From)
   - Trait objects (`dyn Trait`)

3. **Error handling patterns** (2-3 days)
   - Custom error types
   - Error propagation with `?`
   - `anyhow` and `thiserror` crates
   - When to panic vs return errors

**Milestone**: Build the CLI tool with proper error handling

**Resources**:
- Rust Book chapters 7-10
- DESIGN_PATTERNS_GUIDE.md
- CHEAT_SHEET.md for Go → Rust lookups

### Phase 3: Concurrency (Month 2)

**Goal**: Write concurrent programs safely

1. **Threading basics** (3-4 days)
   - `std::thread::spawn` (like goroutines, but explicit)
   - Channels (`mpsc`, similar to Go's channels)
   - `Arc<T>` and `Mutex<T>` (for shared state)
   - Why Rust prevents data races at compile time

2. **Async/await** (5-6 days)
   - Understanding `Future` trait
   - `async fn` and `.await` syntax
   - Tokio runtime (like Go's runtime, but you control it)
   - When to use async vs threads

3. **Practical concurrency** (4-5 days)
   - Worker pools (similar to Go's goroutine pools)
   - Request handling
   - Graceful shutdown
   - Concurrent data structures

**Milestone**: Build the Worker Pool project (MINI_PROJECTS/02-worker-pool)

**Resources**:
- Rust Book chapter 16 (fearless concurrency)
- Tokio tutorial
- FUNDAMENTALS_COMPARISON.md concurrency section
- DESIGN_PATTERNS_GUIDE.md for concurrency patterns

### Phase 4: Advanced patterns (Month 3)

**Goal**: Write idiomatic Rust

1. **Lifetimes** (4-5 days)
   - Why lifetimes exist (Go doesn't need them because of GC)
   - Lifetime annotations (`'a`)
   - Lifetime elision rules
   - The `'static` lifetime

2. **Advanced traits** (4-5 days)
   - Associated types
   - Default implementations
   - Blanket implementations
   - Trait bounds and where clauses

3. **Iterators** (3-4 days)
   - Iterator trait
   - Iterator combinators (map, filter, fold, etc.)
   - Lazy evaluation
   - Writing custom iterators

4. **Smart pointers** (3-4 days)
   - `Box<T>` (heap allocation, like Go's new)
   - `Rc<T>` (reference counting, single-threaded)
   - `Arc<T>` (atomic ref counting, like Go pointers)
   - `RefCell<T>` (interior mutability)

**Milestone**: Build the Service Discovery Client (MINI_PROJECTS/03-service-discovery-client)

**Resources**:
- Rust Book chapters 15, 17, 19
- DESIGN_PATTERNS_GUIDE.md
- ANTI_PATTERNS.md

## Common stumbling blocks for Go developers

Based on conversations with dozens of Gophers who learned Rust:

### Week 1: "Why can't I just copy this pointer?"

Coming from Go, you're used to passing pointers around freely. Rust's borrow checker will reject code that seems perfectly safe.

**What helps**:
- Start with immutable borrows (&T)
- Use one mutable borrow (&mut T) at a time
- When stuck, `.clone()` it (you'll learn to avoid this later)
- Read the compiler errors—they're actually helpful

### Week 2-3: "String vs &str is confusing"

Go has one string type. Rust has two. Here's the simple rule:
- `&str` - Borrowed string slice (like `string` in Go, read-only)
- `String` - Owned, growable string (like `[]byte` in Go)
- Use `&str` when you can, `String` when you need ownership

### Week 4: "Where are my goroutines?"

Starting a goroutine is `go func()`. Starting a Rust async task requires:
1. An async runtime (usually Tokio)
2. `async` functions
3. `.await` calls
4. Understanding when to use `spawn`

It feels heavyweight compared to Go, but you get more control and better performance.

### Month 2: "What are lifetimes for?"

Go's GC means you never think about reference validity. In Rust, you must prove to the compiler that references won't outlive the data they point to.

```rust
fn get_first<'a>(items: &'a [String]) -> &'a str {
    &items[0]  // Returns a reference valid as long as items
}
```

Lifetimes are explicit proof that prevents dangling pointers.

### Month 3: "When do I use Arc vs Rc vs Box?"

Coming from Go where pointers are just pointers:

- `Box<T>` - Single owner, heap-allocated (use this first)
- `Rc<T>` - Multiple owners, single-threaded (like Go pointers, no concurrency)
- `Arc<T>` - Multiple owners, thread-safe (like Go pointers, with sync)
- Raw pointers - Don't use these (you probably don't need them)

## Progress checkpoints

How do you know you're making progress?

**You're getting somewhere when**:
- Borrow checker errors start making sense
- You write ownership-correct code without fighting the compiler
- You read Rust code and predict what will compile
- You stop trying to write Go code in Rust

**You're proficient when**:
- You design APIs around ownership
- You know when to clone vs borrow without thinking
- You use iterators instead of loops naturally
- You contribute to Rust open source projects

**You're experienced when**:
- You write code that compiles on the first try
- Lifetimes feel natural
- You understand the performance implications of your choices
- The borrow checker feels like a helpful assistant, not an adversary

## Tips for staying motivated

Learning Rust is harder than learning Go. Go was designed to be simple. Rust was designed to be safe and fast. Here's how to stick with it:

1. **Build things** - Don't just read. The ownership model only clicks when you write code.

2. **Don't fight the language** - If the borrow checker rejects your Go-style code, redesign it. Don't just add `.clone()` everywhere.

3. **Embrace different philosophies**:
   - Go: "Simple, fast compilation, good enough performance"
   - Rust: "Safe, fast execution, worth the compile time"

4. **Join the community** - The Rust Discord and forums are incredibly helpful.

5. **Celebrate wins** - Your first program that compiles on first try is a milestone!

6. **Use the right tools**:
   - `cargo check` - Fast feedback (like `go build -n`)
   - `cargo clippy` - Catches un-idiomatic code (like `go vet`)
   - `cargo fmt` - Formats code (like `gofmt`)
   - `rust-analyzer` - IDE support (works with VS Code, Neovim, etc.)

## What to focus on each week

Here's a realistic weekly breakdown:

**Week 1**: Ownership and borrowing
- This is where Go developers struggle most
- Do the Rustlings ownership exercises
- Fight with the borrow checker (it's normal!)
- Read error messages carefully

**Week 2**: Structs, enums, pattern matching
- Enums are way more powerful than Go's
- Pattern matching is better than switch
- Build a small project to practice

**Week 3-4**: Traits and error handling
- Traits are interfaces++
- Get comfortable with `Result<T, E>`
- Use the `?` operator for error propagation
- Complete the CLI Tool project

**Week 5-6**: Collections and iterators
- Replace Go-style loops with iterator chains
- Learn when to collect vs iterate
- HashMap and Vec become second nature

**Week 7-8**: Concurrency fundamentals
- Understand Send and Sync traits
- Channels work like Go's, but more type-safe
- Arc/Mutex for shared state (simpler than you think)

**Week 9-12**: Async/await
- Pick up Tokio basics
- Write async functions
- Understand when async is better than threads
- Complete the Worker Pool project

## Comparing your experience levels

If you're comfortable in Go, here's how concepts map:

| Go Experience | Rust Equivalent | Time to Learn |
|---------------|-----------------|---------------|
| Variables & types | Variables & types | 1 day |
| Structs & methods | Structs & impl blocks | 2 days |
| Interfaces | Traits | 1 week |
| Goroutines | Async/await + threads | 2 weeks |
| Channels | mpsc channels | 3 days |
| Error handling | Result<T, E> | 1 week |
| Pointers | Ownership & borrowing | 3 weeks |
| defer | Drop trait (RAII) | 3 days |

## Next steps

Ready to dive in? Here's your immediate action plan:

1. **Read FUNDAMENTALS_COMPARISON.md** - See Go patterns translated to Rust
2. **Install Rust tools** - `rustup`, `rust-analyzer` for your editor
3. **Do Rustlings** - Interactive exercises (`cargo install rustlings`)
4. **Start Mini-Project 01** - CLI Tool (MINI_PROJECTS/01-cli-tool)
5. **Keep CHEAT_SHEET.md handy** - Quick Go → Rust reference

## Managing expectations

**After 1 week**: You'll fight with the borrow checker constantly. This is normal. Every Rust developer went through this.

**After 1 month**: You'll understand ownership intellectually, but still make mistakes. Keep going!

**After 2 months**: The borrow checker errors become predictable. You start writing ownership-correct code.

**After 3 months**: You appreciate the guarantees. You miss the borrow checker when writing Go.

**After 6 months**: You're productive in Rust. You understand why the tradeoffs are worth it.

## The honest truth

Go is simpler and faster to learn. If you're happy with Go, you don't *need* Rust.

But if you want:
- Eliminates entire classes of bugs at compile time
- No GC pauses (predictable latency)
- Memory safety without a runtime
- Fearless refactoring (compiler catches breaking changes)
- True zero-cost abstractions

...then the learning curve is worth it.

## Remember

Go taught you to write simple, concurrent programs.
Rust will teach you to write safe, fast, concurrent programs.

Both are great languages. You'll appreciate both more deeply by learning both.

Welcome to Rust! 🦀

---

*Pro tip: When the borrow checker rejects your code, ask yourself "could this cause a use-after-free or data race in Go?" The answer is usually yes. The borrow checker isn't being pedantic—it's preventing real bugs.*
