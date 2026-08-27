# Learning Path for Python Developers

This guide maps out a practical learning path for Python developers transitioning to Rust. Coming from Python, you'll find some concepts familiar and others completely new.

## The mental model shift

The biggest difference isn't syntax—it's how you think about your programs. Python lets you prototype quickly and think at a high level. Rust makes you think about how the computer actually executes your code.

### From dynamic to static typing

In Python, types are optional hints. In Rust, types are enforced contracts.

```python
# Python: Types are suggestions
def process(items: list) -> list:
    return [x * 2 for x in items]

# This runs fine even though types don't match
result = process("hello")  # TypeError at runtime
```

```rust
// Rust: Types are verified at compile time
fn process(items: &[i32]) -> Vec<i32> {
    items.iter().map(|x| x * 2).collect()
}

// This won't compile - caught before you run it
// let result = process("hello");  // ERROR: expected &[i32], found &str
```

**The shift**: Types aren't documentation—they're guarantees the compiler enforces.

### From garbage collection to ownership

Python has a garbage collector that tracks references and cleans up memory. Rust has ownership rules enforced at compile time.

```python
# Python: References everywhere, GC cleans up
data = [1, 2, 3]
reference1 = data
reference2 = data
data.append(4)  # All references see the change
```

```rust
// Rust: One owner, explicit borrowing
let mut data = vec![1, 2, 3];
let reference1 = &data;      // Immutable borrow
// let reference2 = &mut data;  // ERROR: can't have mutable borrow while immutable exists
data.push(4);  // ERROR: can't mutate while borrowed
```

**The shift**: Think about who owns data and who's just borrowing it.

### From exceptions to Result types

Python uses exceptions for error handling. Rust uses `Result<T, E>` types.

```python
# Python: Exceptions can hide anywhere
def divide(a, b):
    return a / b  # Might raise ZeroDivisionError

try:
    result = divide(10, 0)
except ZeroDivisionError:
    print("Can't divide by zero")
```

```rust
// Rust: Errors are in the type signature
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Must handle the error
match divide(10.0, 0.0) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

**The shift**: Errors are values you handle explicitly, not exceptions you might catch.

## Your learning path

Here's the order I recommend based on helping dozens of Python developers learn Rust:

### Phase 1: Static typing and basics (Week 1-2)

**Goal**: Get comfortable with explicit types and basic syntax

1. **Type annotations** (2-3 days)
   - Basic types: i32, f64, bool, char
   - Type inference (Rust infers a lot, like Python)
   - Explicit type annotations when needed
   - This is easier than you think!

2. **Ownership basics** (3-4 days)
   - The three ownership rules
   - Move semantics (this is new!)
   - Borrowing with & and &mut
   - This is the hard part—take your time

3. **Structs and enums** (2-3 days)
   - Defining data structures (like dataclasses)
   - Pattern matching (better than if-elif)
   - Option and Result types (no more None!)

**Milestone**: Build a simple CLI tool that processes data

**Resources**:
- Rust Book chapters 1-6
- Rustlings exercises
- FUNDAMENTALS_COMPARISON.md (in this track)

### Phase 2: Collections and iterators (Week 3-4)

**Goal**: Work with data structures fluently

1. **Collections** (2-3 days)
   - Vec<T> (like list)
   - HashMap<K, V> (like dict)
   - String vs &str (confusing at first, important!)

2. **Iterators** (3-4 days)
   - Iterator trait (like Python iterators)
   - Iterator combinators (like map, filter)
   - Lazy evaluation (just like Python generators)
   - collect() to materialize results

3. **Error handling** (2-3 days)
   - Result<T, E> in depth
   - The ? operator (like exception propagation)
   - Custom error types
   - When to panic vs return error

**Milestone**: Build the Data Processing Pipeline project (MINI_PROJECTS/01-data-pipeline)

**Resources**:
- Rust Book chapters 8-9
- Iterator documentation
- DESIGN_PATTERNS_GUIDE.md for iterator patterns

### Phase 3: Traits and abstractions (Month 2)

**Goal**: Write reusable, generic code

1. **Traits** (4-5 days)
   - Defining traits (like Python protocols)
   - Implementing traits for types
   - Trait bounds (like TypeVar with protocols)
   - Common traits: Debug, Clone, Display

2. **Generics** (3-4 days)
   - Generic functions and structs
   - Trait bounds on generics
   - Where clauses for complex bounds
   - Zero-cost abstractions

3. **Modules and crates** (2-3 days)
   - Organizing code (like Python modules)
   - Visibility rules (pub)
   - Using external crates (like pip packages)
   - Publishing crates (optional)

**Milestone**: Build the REST API Server project (MINI_PROJECTS/02-rest-api-server)

**Resources**:
- Rust Book chapters 10, 7
- Traits documentation
- CHEAT_SHEET.md for quick lookups

### Phase 4: Async and concurrency (Month 3)

**Goal**: Write concurrent code safely

1. **Async basics** (4-5 days)
   - Understanding async/await (familiar from Python!)
   - Tokio runtime (like asyncio event loop)
   - Future trait
   - When to use async vs threads

2. **Concurrent patterns** (4-5 days)
   - Channels for communication
   - Arc and Mutex for shared state
   - Avoiding data races (compiler helps!)
   - Parallel iterators with rayon

3. **Advanced async** (3-4 days)
   - Streams (like async iterators)
   - Concurrent task spawning
   - Async file I/O
   - Error handling in async code

**Milestone**: Build the Async File Processor project (MINI_PROJECTS/03-async-file-processor)

**Resources**:
- Tokio tutorial
- Rust Book chapter 16
- FUNDAMENTALS_COMPARISON.md async section

## Common stumbling blocks for Python developers

Based on watching Python developers learn Rust, here's where people get stuck:

### Week 1: "Why is everything so explicit?"

Coming from Python's duck typing, Rust's explicitness feels heavy.

**What helps**:
- Type inference reduces annotations (Rust infers more than you think)
- Explicit types catch bugs at compile time (no more late-night AttributeError)
- IDE support shows you what types are inferred

### Week 2-3: "The borrow checker is impossible"

You'll try to do something that worked in Python, and the compiler will say no.

**What helps**:
- Read the error messages—they explain the problem
- Start with cloning when stuck (optimize later)
- Understand borrowing rules deeply before fighting them
- Remember: the compiler is preventing bugs that would happen at runtime

### Month 1: "String vs &str is confusing"

This trips up everyone coming from Python's simple `str`.

**Simple rule**:
- Use `&str` for function parameters (flexible)
- Use `String` for owned data (struct fields, returns)
- `&str` is a view into string data
- `String` owns the data

### Month 2: "When do I use which collection?"

Python's list and dict cover 90% of use cases. Rust has more options.

**Start with**:
- `Vec<T>` for dynamic arrays (like list)
- `HashMap<K, V>` for key-value (like dict)
- `HashSet<T>` for unique values (like set)
- `&[T]` for borrowed slices (like list views)

### Month 3: "Lifetimes make no sense"

Lifetimes are Rust's way of proving references are valid.

**What helps**:
- Many lifetimes are elided (inferred)
- Explicit lifetimes are rare in practice
- Think of them as "this reference lives as long as that data"
- They prevent use-after-free bugs Python's GC hides

## Progress checkpoints

How do you know you're making progress?

**You're getting somewhere when**:
- You read Rust code and understand what's happening
- Compiler errors start making sense
- You fix borrow checker errors without googling
- You appreciate that types caught a bug

**You're proficient when**:
- You write Rust code on the first try that compiles
- You know when to clone vs borrow intuitively
- You can implement traits for custom types
- You contribute to Rust projects

**You're experienced when**:
- You design APIs around ownership
- You use async/await fluently
- You optimize without premature optimization
- The type system feels helpful, not restrictive

## Tips for staying motivated

Learning Rust is harder than learning most languages. Here's how to stick with it:

1. **Build things** - Don't just read. Code cements understanding.

2. **Embrace the struggle** - If it's hard, you're learning. Everyone struggled with ownership.

3. **Compare to Python** - But don't expect Python patterns to work. Rust solves different problems differently.

4. **Celebrate wins** - First program that compiles on first try? Milestone!

5. **Use the community** - Rust Discord #beginners is incredibly helpful.

6. **Remember why you started** - Performance? Safety? Deploy complexity? Keep that in mind.

## What to focus on each week

Here's a realistic weekly breakdown:

**Week 1**: Types and ownership
- Don't try to understand everything
- Focus on basic types and move semantics
- Do Rustlings exercises

**Week 2**: Borrowing and structs
- Practice with &T and &mut T
- Build small programs
- Use Option and Result

**Week 3-4**: Collections and iterators
- Replace loops with iterator chains
- Learn common iterator methods
- Complete Data Pipeline project

**Week 5-6**: Traits and generics
- Implement traits for your types
- Write generic functions
- Start REST API project

**Week 7-8**: Error handling and modules
- Custom error types
- Organize code into modules
- Proper error propagation

**Week 9-12**: Async and concurrency
- Tokio async runtime
- Async functions and await
- Complete Async File Processor project

## Comparison to Python learning curve

Learning curve comparison:

```
Python:
Week 1-2: ███████████████████ (Very productive quickly)
Month 2:  ████████████████████ (Expert-level productivity)
Month 6:  ████████████████████ (Mastery)

Rust:
Week 1-2: ███░░░░░░░░░░░░░░░░ (Fighting compiler)
Month 2:  ██████████░░░░░░░░░ (Getting productive)
Month 6:  ████████████████░░░ (Proficient)
Year 1:   ████████████████████ (Mastery)
```

Python is faster to learn. Rust is more frontloaded. But Rust's upfront investment pays off in fewer production bugs and better performance.

## Next steps

Ready to dive in? Here's your immediate action plan:

1. **Read FUNDAMENTALS_COMPARISON.md** - This is crucial! It shows Python patterns side-by-side with Rust.

2. **Install Rust and tools** - Follow GETTING_STARTED.md

3. **Do Rustlings** - Interactive exercises (`cargo install rustlings`)

4. **Start Mini-Project 01** - Data Pipeline (MINI_PROJECTS/01-data-pipeline)

5. **Keep CHEAT_SHEET.md handy** - Quick Python → Rust reference

6. **Join Rust Discord** - Get help when stuck

## Mindset shifts

| Python thinking | Rust thinking |
|-----------------|---------------|
| "Just try it and see if it works" | "Make it compile correctly first" |
| "Types are optional documentation" | "Types are compiler-verified contracts" |
| "Exceptions will be caught somewhere" | "Handle errors explicitly here" |
| "GC will clean up" | "Ownership determines cleanup" |
| "Performance? Use NumPy" | "Performance is built-in" |

Remember: Python optimizes for developer time. Rust optimizes for correctness and runtime performance. Both are valid choices for different problems.

Good luck! 🦀

---

*Pro tip: Keep writing Python for exploratory work. Use Rust when you need the correctness and performance guarantees. Many developers use both.*
