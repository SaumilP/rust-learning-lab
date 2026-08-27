# Rust for C/C++ Developers

Welcome! 👋

If you're reading this, you already know how to write fast, low-level code. You understand memory layouts, cache lines, and what actually happens on the hardware. You've debugged segfaults, fought with dangling pointers, and probably have strong opinions about manual memory management.

Rust was designed for developers like you.

## Why this track exists

I spent years writing C++ for high-performance systems before discovering Rust. The languages share the same problem space: you need control over memory, you can't tolerate garbage collection, and every cycle matters.

But Rust solves problems that C/C++ can't:
- **Memory safety without runtime cost**: No more use-after-free, no more double-free, no more data races
- **Modern tooling**: Cargo makes build systems actually pleasant
- **Fearless refactoring**: Change a type, compiler finds every affected line
- **Zero-cost abstractions**: High-level code that compiles to the same assembly as hand-written C

This track shows you how to think in Rust when you're coming from C/C++.

## What makes Rust appealing for C/C++ developers

### Memory safety without garbage collection

You already manage memory manually. Rust lets you keep that control while eliminating entire classes of bugs:

**C/C++ problems Rust prevents:**
- Use-after-free (compiler catches it)
- Double-free (ownership system prevents it)
- Null pointer dereferences (no null pointers!)
- Buffer overflows (bounds checking by default)
- Data races (impossible to compile code with data races)
- Iterator invalidation (borrow checker prevents it)

**Without:**
- Reference counting overhead (unless you explicitly use Rc/Arc)
- Garbage collection pauses
- Runtime overhead
- Tracing allocations at runtime

### RAII, but enforced by the compiler

C++ developers know RAII. Rust takes it further:

```cpp
// C++: RAII works if you remember to use it
{
    std::unique_ptr<Resource> r = std::make_unique<Resource>();
    // Automatically cleaned up
}

// But you can still shoot yourself in the foot
Resource* raw = new Resource();
// Forgot to delete? Memory leak!
// Deleted twice? Crash!
```

```rust
// Rust: RAII is the *only* way
{
    let r = Resource::new();
    // Automatically cleaned up, no other option
}

// Can't leak memory by accident
// Can't double-free
// Can't use after free
```

### Modern language features

**Coming from C:**
- Algebraic data types (enums with data)
- Pattern matching (better than switch)
- Iterators (zero-cost, composable)
- Generics (like C++ templates, but cleaner)
- Module system (no header files!)
- Package manager (cargo is incredible)

**Coming from Modern C++ (C++11+):**
- Move semantics (similar to C++11, but simpler)
- Smart pointers (Box = unique_ptr, Rc = shared_ptr, but safer)
- Lambda functions (closures with explicit capture)
- Concepts (traits are similar but more powerful)
- No implicit conversions (explicit is better)

### Performance guarantees

```rust
// These compile to the same assembly:

// Low-level
let mut sum = 0;
for i in 0..vec.len() {
    sum += vec[i];
}

// High-level
let sum: i32 = vec.iter().sum();
```

Zero-cost abstractions aren't a promise—they're a guarantee.

### Fearless concurrency

**C/C++:**
```cpp
// Data race waiting to happen
std::vector<int> data;
std::thread t1([&]() { data.push_back(1); });
std::thread t2([&]() { data.push_back(2); });
// Undefined behavior! But compiles fine.
```

**Rust:**
```rust
// This won't compile
let mut data = vec![];
thread::spawn(|| data.push(1));  // ERROR: can't share mut data
thread::spawn(|| data.push(2));  // Compiler prevents data race
```

The compiler *prevents* data races. Not at runtime—at compile time.

## What feels different

1. **The borrow checker** - Takes time to learn, but prevents real bugs

2. **No null pointers** - Use `Option<T>` instead

3. **Explicit error handling** - No exceptions, use `Result<T, E>`

4. **No inheritance** - Composition and traits instead

5. **Move semantics by default** - Like C++11, but more aggressive

6. **Immutable by default** - Must explicitly use `mut`

7. **No header files** - Modules and visibility modifiers instead

## How to use this track

1. **Read LEARNING_PATH.md** - Understand the progression

2. **Study FUNDAMENTALS_COMPARISON.md** - See C/C++ patterns in Rust

3. **Build the mini-projects**:
   - Memory Allocator (understanding ownership)
   - Concurrent Data Structure (safe concurrency)
   - Systems Tool (real-world application)

4. **Reference the guides**:
   - **DESIGN_PATTERNS_GUIDE.md** - RAII, smart pointers, factory patterns
   - **ANTI_PATTERNS.md** - Common C/C++ developer mistakes
   - **CHEAT_SHEET.md** - Quick C/C++ → Rust lookup
   - **GOTCHAS.md** - Surprises for C/C++ developers
   - **TIPS_AND_TRICKS.md** - Productivity shortcuts

## Learning timeline

**Week 1-2**: Fighting the borrow checker (everyone does this)
**Week 3-4**: Understanding ownership deeply
**Month 2**: Writing idiomatic Rust
**Month 3**: Appreciating the safety guarantees
**Month 6**: Never want to go back to C++

## What you'll miss from C/C++

Be realistic about tradeoffs:

**From C:**
- **Simplicity**: C is simpler (fewer features)
- **Compile times**: C is faster to compile
- **Total control**: Rust has guardrails

**From C++:**
- **Mature ecosystem**: C++ has more libraries (for now)
- **Template metaprogramming**: Rust macros are different
- **Compile times**: C++ modules are faster (but C++ without modules is slower)
- **Familiarity**: You know where everything is in C++

## What you'll gain

**Over C:**
- Memory safety without GC
- Modern features (enums, pattern matching, traits)
- Package manager (cargo is amazing)
- Great tooling (rust-analyzer, clippy)
- No undefined behavior

**Over C++:**
- No data races (compiler enforced)
- No null pointer dereferences
- Simpler language (no ADL, no template gotchas)
- Faster compile times than C++ without modules
- Better error messages
- Fearless refactoring

## C/C++ vs Rust philosophy

| C | C++ | Rust |
|---|-----|------|
| "Trust the programmer" | "Zero-overhead principle" | "Fearless concurrency" |
| "Minimal language" | "You don't pay for what you don't use" | "Zero-cost safety" |
| "Portable assembly" | "Abstractions without cost" | "If it compiles, it's probably correct" |

## Performance comparison

**Rust matches C/C++ performance:**
- Same memory model (stack/heap)
- No garbage collection
- Zero-cost abstractions
- LLVM backend (like Clang)
- Inline assembly support
- Control over memory layout

**Often faster in practice:**
- Harder to write slow code by accident
- Better defaults (overflow checks in debug)
- Iterator optimization
- No null pointer checks in hot loops

## FFI: Working with existing C/C++ code

```rust
// Calling C from Rust
extern "C" {
    fn c_function(x: i32) -> i32;
}

unsafe {
    let result = c_function(42);
}

// Exposing Rust to C
#[no_mangle]
pub extern "C" fn rust_function(x: i32) -> i32 {
    x * 2
}
```

You can gradually migrate C/C++ codebases to Rust.

## Quick comparison

**Memory allocation:**
```c
// C
int* ptr = malloc(sizeof(int));
*ptr = 42;
free(ptr);

// C++
auto ptr = std::make_unique<int>(42);

// Rust
let ptr = Box::new(42);
// Automatically freed
```

**Error handling:**
```c
// C
int result = operation();
if (result < 0) {
    fprintf(stderr, "Error: %s\n", strerror(errno));
    return -1;
}

// C++
try {
    operation();
} catch (const std::exception& e) {
    std::cerr << "Error: " << e.what() << std::endl;
}

// Rust
match operation() {
    Ok(val) => println!("Success: {}", val),
    Err(e) => eprintln!("Error: {}", e),
}
```

**Concurrency:**
```cpp
// C++
std::thread t1([]() { work1(); });
std::thread t2([]() { work2(); });
t1.join();
t2.join();

// Rust
let t1 = thread::spawn(|| work1());
let t2 = thread::spawn(|| work2());
t1.join().unwrap();
t2.join().unwrap();
```

## Common questions from C/C++ developers

**Q: Is Rust as fast as C/C++?**
A: Yes. Same LLVM backend, same memory model, zero-cost abstractions. Benchmarks are competitive.

**Q: Can I use Rust in embedded systems?**
A: Yes. `#![no_std]` mode for bare metal. Used in production on microcontrollers.

**Q: Can I call C libraries from Rust?**
A: Yes. FFI is well-supported. Many Rust crates wrap C libraries.

**Q: What about compile times?**
A: Slower than C, comparable to C++ with templates. Incremental compilation helps.

**Q: Is unsafe Rust as powerful as C?**
A: Yes. Unsafe blocks let you do anything C can do.

**Q: Why can't I just write correct C/C++?**
A: You can. But even experts make mistakes. Rust catches them at compile time.

## Ready?

Head to **LEARNING_PATH.md** and let's get started.

You already know how to write fast code. Rust will teach you to write *safe* fast code. And once you experience fearless refactoring and impossible-to-compile data races, you might not want to go back.

Welcome to Rust! 🦀

---

*Fun fact: The Rust compiler itself was originally written in C++ (via OCaml), then rewritten in Rust. Many core Rust developers come from C++ backgrounds, including the language designer.*
