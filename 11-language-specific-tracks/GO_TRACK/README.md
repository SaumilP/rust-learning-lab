# Rust for Go Developers

Hey Gopher! 👋 Welcome to Rust.

If you're reading this, you probably appreciate Go's simplicity and performance. Maybe you've hit some limitations with the garbage collector, or maybe you're curious about that "fearless concurrency" thing everyone talks about.

## Why this track exists

I spent three years writing Go before picking up Rust. The languages share a lot: both are systems languages, both care about performance, and both have great tooling.

But they make different tradeoffs. Go optimizes for simplicity and fast compile times. Rust optimizes for safety and zero-cost abstractions.

This track shows you how to think in Rust when you're coming from Go.

## What makes Rust appealing for Go developers

**Memory safety without GC**
- No garbage collection pauses (your 99th percentile latency drops dramatically)
- Deterministic cleanup (you know exactly when resources are freed)
- Lower memory footprint (no GC overhead)
- Better cache locality (explicit data layout)

**More expressive type system**
- Algebraic data types (enums are powerful)
- Generics with zero runtime cost (monomorphization vs interface{})
- Trait system (interfaces, but more powerful)
- Exhaustive pattern matching (compiler enforces all cases)

**Similar concurrency, different approach**
- async/await instead of goroutines
- Channels work similarly (but are more type-safe)
- Compiler prevents data races
- Scales to 100,000+ concurrent tasks

**Zero-cost abstractions**
- Generic code compiles to specialized versions
- No interface dispatch unless you ask for it
- Iterators optimize to the same code as loops
- Abstractions you don't pay for at runtime

## What feels different

1. **No garbage collector** - You think about ownership instead of letting GC handle it

2. **Explicit lifetimes** - Sometimes you need to tell the compiler how long references live

3. **No `nil`** - Rust uses `Option<T>` instead, preventing null pointer dereference panics

4. **Immutable by default** - Variables don't change unless marked `mut`

5. **Trait implementation is explicit** - You must `impl Trait for Type` (Go's interfaces are implicit)

## How to use this track

1. **Read LEARNING_PATH.md** - Understand the progression

2. **Study FUNDAMENTALS_COMPARISON.md** - See Go patterns in Rust

3. **Build the mini-projects**:
   - CLI Tool (traits + error handling)
   - Worker Pool (channels + concurrency)
   - Service Discovery (async + networking)

4. **Reference the guides**:
   - **CANONICAL_RUST_LINKS.md** - Canonical runnable Rust lessons for each comparison
   - **MVP_CHECKS.md** - Short migration challenges that test the mental-model shift

## Learning timeline

**Week 1-2**: Ownership will feel restrictive (why can't I just pass a pointer?)
**Week 3-4**: Traits start making sense (they're like interfaces but different)
**Month 2**: You appreciate the compile-time guarantees
**Month 3**: You write concurrent code that's provably safe

## What you'll miss from Go

Be realistic about tradeoffs:

- **Compile times** - Go is faster to compile (Rust is getting better though)
- **Simplicity** - Go is easier to learn and has fewer concepts
- **Error handling** - `if err != nil` is more verbose but simpler than `Result<T, E>`
- **Implicit interfaces** - Go's duck typing is more flexible
- **Goroutines** - Starting a goroutine is dead simple

## What you'll gain

- **Confidence** - If it compiles, it's much more likely to be correct
- **Performance** - No GC pauses, better memory layout
- **Safety** - No data races, no null pointer panics
- **Expressiveness** - Enums, pattern matching, powerful type system
- **Memory control** - You decide what goes on the stack vs heap

## Go vs Rust philosophy

| Go Says | Rust Says |
|---------|-----------|
| "A little copying is better than a little dependency" | "Zero-cost abstractions are better" |
| "Simple is better than complex" | "Correct is better than simple" |
| "Errors are values" | "Errors are values (but types enforce handling)" |
| "Share memory by communicating" | "Share memory by communicating (or don't share at all)" |
| "Don't communicate by sharing memory" | "Can't communicate by sharing mutable memory (compiler prevents it)" |

## Side-by-side quick look

**Goroutines vs async tasks**:
```go
// Go
go func() {
    result := doWork()
    ch <- result
}()
```

```rust
// Rust
tokio::spawn(async {
    let result = do_work().await;
    tx.send(result).unwrap();
});
```

**Error handling**:
```go
// Go
result, err := doSomething()
if err != nil {
    return err
}
```

```rust
// Rust
let result = do_something()?;
// ? operator propagates error
```

**Interfaces vs traits**:
```go
// Go (implicit)
type Reader interface {
    Read(p []byte) (n int, err error)
}

// Any type with Read() satisfies Reader
```

```rust
// Rust (explicit)
trait Reader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

impl Reader for MyType {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // implementation
    }
}
```

## Common questions from Go developers

**Q: Why do I need lifetimes?**
A: Go has GC so references are always valid. Rust doesn't, so the compiler needs to verify references don't outlive the data they point to.

**Q: Can I just use `Arc<Mutex<T>>` for everything like Go's pointers?**
A: You can, but you're giving up Rust's main benefits. Learn borrowing first.

**Q: Is async/await as easy as goroutines?**
A: Different, not harder. Goroutines are simpler to start. Rust async gives you more control and better performance.

**Q: Where's my `defer`?**
A: Rust has RAII (Drop trait). Cleanup happens automatically when values go out of scope. Even better than defer!

## Ready?

Head to **LEARNING_PATH.md** and let's get started.

Go taught you to write simple, concurrent programs. Rust will teach you to write safe, fast, concurrent programs. You'll appreciate both.

Welcome to Rust! 🦀

---

*Fun fact: Many Rust developers came from Go, and many still use both. They're great tools for different problems.*
