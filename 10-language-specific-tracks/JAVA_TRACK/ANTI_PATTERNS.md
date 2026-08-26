# Anti-Patterns: Common Mistakes Java Developers Make

This guide covers the mistakes I made (and every Java developer I know made) when learning Rust. Save yourself some time and learn from our pain.

## 1. Cloning Everything

### The mistake

```rust
// ❌ ANTI-PATTERN: Clone all the things!
fn process_users(users: Vec<User>) -> Vec<String> {
    let mut names = Vec::new();
    for user in users {
        let user_clone = user.clone();  // Unnecessary!
        let name_clone = user_clone.name.clone();  // Also unnecessary!
        names.push(name_clone);
    }
    names
}
```

### Why Java developers do this

In Java, you pass references everywhere and don't think about it. The first time you hit a borrow checker error in Rust, your instinct is to `.clone()` until it compiles.

### The correct approach

```rust
// ✅ CORRECT: Borrow instead of clone
fn process_users(users: &[User]) -> Vec<String> {
    users.iter()
        .map(|user| user.name.clone())  // Only clone what you need
        .collect()
}

// Even better if you can return references:
fn get_names(users: &[User]) -> Vec<&str> {
    users.iter()
        .map(|user| user.name.as_str())
        .collect()
}
```

### When to clone

- When you actually need owned data (storing in a collection, sending to another thread)
- When the data is cheap to clone (`i32`, small structs)
- When borrowing would make the code significantly more complex

### When NOT to clone

- Just to satisfy the borrow checker (fix the actual problem instead)
- For large data structures passed around locally
- Inside hot loops (performance killer)

---

## 2. Using `.unwrap()` Everywhere

### The mistake

```rust
// ❌ ANTI-PATTERN: Unwrap and pray
fn load_config() -> Config {
    let contents = fs::read_to_string("config.json")
        .unwrap();  // Panic if file doesn't exist!

    let config: Config = serde_json::from_str(&contents)
        .unwrap();  // Panic if JSON is invalid!

    config
}
```

### Why Java developers do this

In Java, you're used to catching exceptions somewhere up the call stack—or just letting them bubble up. `.unwrap()` feels like "I'll deal with this later."

But `.unwrap()` is basically a `panic!`, which is like `System.exit(1)`. Your whole program crashes.

### The correct approach

```rust
// ✅ CORRECT: Proper error handling
use std::error::Error;

fn load_config() -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string("config.json")?;  // Propagate error
    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}

// Usage
match load_config() {
    Ok(config) => println!("Loaded: {:?}", config),
    Err(e) => eprintln!("Failed to load config: {}", e),
}
```

### When `.unwrap()` is OK

- In `main()` for quick prototypes
- In tests (test failures are fine)
- When you've proven it can't fail (but add a comment explaining why)

```rust
// OK: We just inserted this key, so it must exist
map.insert("key", value);
let value = map.get("key").unwrap();  // Can't fail
```

### Alternatives to `.unwrap()`

- `.expect("meaningful error message")` - Better for debugging
- `?` operator - Propagate errors to caller
- `.unwrap_or(default)` - Provide a fallback
- `.unwrap_or_else(|| compute_default())` - Lazy fallback
- Pattern matching - Handle each case explicitly

---

## 3. Fighting the Borrow Checker with Bad Architecture

### The mistake

```rust
// ❌ ANTI-PATTERN: Trying to hold multiple mutable references
struct GameState {
    players: Vec<Player>,
    current_player: &mut Player,  // ERROR: Can't do this!
}

// Or this:
fn update_game(state: &mut GameState) {
    let player = &mut state.players[0];
    player.health -= 10;

    // Later in same function:
    let other_player = &mut state.players[1];  // ERROR: Already borrowed!
}
```

### Why Java developers do this

In Java, you can have as many references to an object as you want, and mutation just works. In Rust, the borrow checker says "no."

### The correct approach

**Option 1: Use indices instead of references**

```rust
// ✅ CORRECT: Store index instead of reference
struct GameState {
    players: Vec<Player>,
    current_player_idx: usize,
}

impl GameState {
    fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player_idx]
    }
}
```

**Option 2: Split borrows**

```rust
// ✅ CORRECT: Borrow individual fields
fn update_game(players: &mut [Player], current_idx: usize) {
    players[0].health -= 10;
    players[1].health += 5;  // OK: Different indices
}
```

**Option 3: Interior mutability (when you really need it)**

```rust
use std::cell::RefCell;

struct GameState {
    players: Vec<RefCell<Player>>,
}

fn update_game(state: &GameState) {
    state.players[0].borrow_mut().health -= 10;
    state.players[1].borrow_mut().health += 5;
    // Runtime borrow checking instead of compile-time
}
```

### The lesson

If you're fighting the borrow checker, your architecture probably has a problem. The borrow checker is trying to tell you that your design has potential race conditions or use-after-free bugs.

---

## 4. Overusing `Box<dyn Trait>` (Runtime Polymorphism)

### The mistake

```rust
// ❌ ANTI-PATTERN: Boxing everything for "flexibility"
fn process_items(items: Vec<Box<dyn Item>>) {
    for item in items {
        item.process();  // Virtual method call (slow)
    }
}

// Or worse:
fn transform(value: Box<dyn Any>) -> Box<dyn Any> {
    // Lost all type safety!
}
```

### Why Java developers do this

In Java, everything is a pointer and polymorphism is runtime by default. It feels natural to do the same in Rust.

### The problem

- `Box<dyn Trait>` uses dynamic dispatch (virtual method calls)
- Heap allocation for every object
- Can't inline or optimize
- Loses type information

### The correct approach

**Use generics for static dispatch**:

```rust
// ✅ CORRECT: Generics (compile-time polymorphism)
fn process_items<T: Item>(items: Vec<T>) {
    for item in items {
        item.process();  // Direct call (fast, can inline)
    }
}

// Or if you need different types:
fn process_item<T: Item>(item: &T) {
    item.process();
}

// Call with different types:
process_item(&ConcreteItem1);
process_item(&ConcreteItem2);
```

**Use enums when you know the variants**:

```rust
// ✅ CORRECT: Enum (zero-cost, exhaustive)
enum Item {
    TypeA(TypeA),
    TypeB(TypeB),
    TypeC(TypeC),
}

impl Item {
    fn process(&self) {
        match self {
            Item::TypeA(a) => a.process(),
            Item::TypeB(b) => b.process(),
            Item::TypeC(c) => c.process(),
        }
    }
}
```

### When `Box<dyn Trait>` is OK

- Plugin systems (loading code at runtime)
- When you genuinely need a collection of different types
- When generics would explode compile times
- FFI boundaries

---

## 5. Ignoring Lifetimes Until They Bite You

### The mistake

```rust
// ❌ ANTI-PATTERN: Returning references to local data
fn get_default_name() -> &str {
    let name = String::from("Default");
    &name  // ERROR: Returning reference to local variable!
}

// Or trying to store references without lifetimes:
struct User {
    name: &str,  // ERROR: Missing lifetime parameter
}
```

### Why Java developers do this

Java doesn't have lifetimes. Objects live on the heap and are garbage collected. The concept of "this reference can't outlive that data" doesn't exist in Java.

### The correct approach

**Option 1: Return owned data**:

```rust
// ✅ CORRECT: Return owned String
fn get_default_name() -> String {
    String::from("Default")
}
```

**Option 2: Use static lifetime for constants**:

```rust
// ✅ CORRECT: String literal has 'static lifetime
fn get_default_name() -> &'static str {
    "Default"
}
```

**Option 3: Add explicit lifetimes when needed**:

```rust
// ✅ CORRECT: Explicit lifetime
struct User<'a> {
    name: &'a str,  // This reference lives as long as 'a
}

fn create_user<'a>(name: &'a str) -> User<'a> {
    User { name }
}
```

### The lesson

Lifetimes are about ownership and validity. If the compiler complains about lifetimes, ask yourself: "Who owns this data, and how long does it live?"

---

## 6. String Confusion: String vs &str

### The mistake

```rust
// ❌ ANTI-PATTERN: Confusing String and &str
fn process(s: String) {
    println!("{}", s);
}

fn main() {
    let text = "hello";
    process(text);  // ERROR: expected String, found &str

    let text = String::from("hello");
    process(text);
    process(text);  // ERROR: value moved in previous call
}
```

### Why Java developers do this

Java just has `String`. You don't think about ownership or allocation. In Rust, `String` (owned) and `&str` (borrowed) serve different purposes.

### The correct approach

**General rule**:
- Use `&str` for function parameters (accepts both `String` and `&str`)
- Use `String` for owned data (fields, return values when needed)

```rust
// ✅ CORRECT: Accept &str to be flexible
fn process(s: &str) {
    println!("{}", s);
}

fn main() {
    let text = "hello";  // &str
    process(text);  // OK

    let owned = String::from("hello");
    process(&owned);  // OK: &String coerces to &str
    process(&owned);  // OK: Can use owned again
}
```

**When you need ownership**:

```rust
// ✅ CORRECT: Take String when you need ownership
fn store_name(name: String) -> User {
    User { name }  // Move String into User
}
```

**Converting between them**:

```rust
let str_ref: &str = "hello";
let owned: String = str_ref.to_string();  // or .to_owned()

let owned: String = String::from("hello");
let str_ref: &str = &owned;  // Borrow as &str
```

---

## 7. Not Using Iterators

### The mistake

```rust
// ❌ ANTI-PATTERN: Java-style loops
fn sum_evens(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..numbers.len() {
        if numbers[i] % 2 == 0 {
            sum += numbers[i];
        }
    }
    sum
}
```

### Why Java developers do this

This looks like Java's enhanced for-loop or traditional for-loop. It works, but it's not idiomatic Rust.

### The correct approach

```rust
// ✅ CORRECT: Iterator chain
fn sum_evens(numbers: &[i32]) -> i32 {
    numbers.iter()
        .filter(|&&n| n % 2 == 0)
        .sum()
}
```

**More examples**:

```java
// Java streams
list.stream()
    .filter(x -> x > 0)
    .map(x -> x * 2)
    .collect(Collectors.toList());
```

```rust
// Rust iterators (similar!)
list.iter()
    .filter(|&x| *x > 0)
    .map(|x| x * 2)
    .collect::<Vec<_>>()
```

### Benefits

- More concise and readable
- Composable
- Compiler optimizes better
- Harder to make off-by-one errors

---

## 8. Mutex Everywhere (Overusing Interior Mutability)

### The mistake

```rust
// ❌ ANTI-PATTERN: Mutex for single-threaded code
struct Counter {
    value: Arc<Mutex<i32>>,  // Overkill!
}

impl Counter {
    fn increment(&self) {
        let mut val = self.value.lock().unwrap();
        *val += 1;
    }
}
```

### Why Java developers do this

In Java, everything is mutable by default and you add `synchronized` when you need thread safety. In Rust, you might reach for `Mutex` too quickly.

### The correct approach

**Single-threaded: Just use `mut`**:

```rust
// ✅ CORRECT: Plain old mutability
struct Counter {
    value: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }
}
```

**Multi-threaded: Use `Arc<Mutex<T>>` only when needed**:

```rust
// ✅ CORRECT: Arc<Mutex<T>> for shared mutable state across threads
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let handles: Vec<_> = (0..10).map(|_| {
    let counter = Arc::clone(&counter);
    thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    })
}).collect();
```

**Single-threaded interior mutability: Use `RefCell<T>`**:

```rust
// ✅ CORRECT: RefCell for interior mutability (single-threaded)
use std::cell::RefCell;

struct Logger {
    logs: RefCell<Vec<String>>,  // Can mutate through &self
}

impl Logger {
    fn log(&self, message: String) {
        self.logs.borrow_mut().push(message);
    }
}
```

---

## 9. Premature Optimization

### The mistake

```rust
// ❌ ANTI-PATTERN: Optimizing before measuring
fn process(data: &[u8]) -> Vec<u8> {
    // Using unsafe for "performance" without benchmarking
    unsafe {
        // Complex pointer arithmetic that's barely faster
        // and much more dangerous
    }
}
```

### Why Java developers do this

Coming from a GC language, you're excited about Rust's performance. But safe Rust is already fast!

### The correct approach

1. **Write clear, safe code first**
2. **Measure with benchmarks** (use `criterion` crate)
3. **Optimize only bottlenecks**
4. **Use `cargo clippy` for free optimizations**

```rust
// ✅ CORRECT: Start with clean, safe code
fn process(data: &[u8]) -> Vec<u8> {
    data.iter()
        .map(|&b| b.wrapping_add(1))
        .collect()
}

// Benchmark first, then optimize if needed!
```

---

## 10. Not Reading Compiler Errors

### The mistake

```
error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
```

**Your reaction**: "Ugh, borrow checker is so annoying!" *Randomly adds `clone()` until it compiles*

### The correct approach

**Read the error**. Rust's error messages are actually helpful:

```
error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
  --> src/main.rs:5:5
   |
4  |     let y = &x;
   |             -- immutable borrow occurs here
5  |     x.push(1);
   |     ^^^^^^^^^ mutable borrow occurs here
6  |     println!("{}", y);
   |                    - immutable borrow later used here
```

The compiler is telling you:
1. What the problem is
2. Where it occurred
3. Why it's a problem

Listen to it!

---

## Summary: How to Avoid These Mistakes

1. **Don't clone until you need to** - Try borrowing first
2. **Handle errors properly** - Use `?` operator, not `.unwrap()`
3. **Trust the borrow checker** - It's finding real bugs
4. **Prefer generics over trait objects** - Static dispatch is faster
5. **Learn lifetimes early** - They're not that scary
6. **Use `&str` for parameters** - More flexible than `String`
7. **Use iterators** - They're idiomatic and fast
8. **Don't overuse `Mutex`** - Simple mutability often works
9. **Measure before optimizing** - Safe Rust is already fast
10. **Read compiler errors** - They're teaching you Rust

Next: Check out CHEAT_SHEET.md for quick reference on Java → Rust conversions!
