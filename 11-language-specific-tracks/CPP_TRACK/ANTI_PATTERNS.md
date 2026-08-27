# Common Anti-Patterns for C/C++ Developers

This guide covers mistakes C/C++ developers commonly make when learning Rust. These aren't bugs—your code will compile—but you're fighting the language instead of working with it.

## 1. Overusing `unsafe`

### The Trap

Coming from C/C++, you might reach for `unsafe` too quickly when the borrow checker rejects your code.

```rust
// ❌ BAD: Using unsafe to bypass borrow checker
fn process_data(data: &mut Vec<i32>) {
    unsafe {
        let ptr = data.as_mut_ptr();
        *ptr.offset(0) = 42;
        *ptr.offset(1) = 43;
    }
}

// ✅ GOOD: Just use safe Rust
fn process_data(data: &mut Vec<i32>) {
    if data.len() >= 2 {
        data[0] = 42;
        data[1] = 43;
    }
}

// Or better yet
fn process_data(data: &mut Vec<i32>) {
    for (i, value) in data.iter_mut().take(2).enumerate() {
        *value = 42 + i as i32;
    }
}
```

**Why it's wrong:**
- You lose all safety guarantees
- The borrow checker is catching real issues
- Safe Rust is almost always possible

**When unsafe is justified:**
- FFI with C/C++ libraries
- Implementing low-level data structures (VecDeque, HashMap)
- SIMD or inline assembly
- Performance-critical code after profiling

**Rule of thumb:** Less than 1% of your code should be unsafe. If you're using it more, you're doing it wrong.

### Real Example

```rust
// ❌ BAD: Using raw pointers for linked list
struct Node {
    value: i32,
    next: *mut Node,  // Raw pointer to avoid borrow checker
}

impl Node {
    unsafe fn append(&mut self, value: i32) {
        let new_node = Box::into_raw(Box::new(Node {
            value,
            next: std::ptr::null_mut(),
        }));

        let mut current = self as *mut Node;
        while !(*current).next.is_null() {
            current = (*current).next;
        }
        (*current).next = new_node;
    }
}

// ✅ GOOD: Use safe abstractions
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn append(&mut self, value: i32) {
        match self.next {
            Some(ref mut next) => next.append(value),
            None => {
                self.next = Some(Box::new(Node {
                    value,
                    next: None,
                }));
            }
        }
    }
}

// ✅ BETTER: Use Vec or VecDeque instead of linked list
let mut list = Vec::new();
list.push(42);
list.push(43);
```

## 2. Fighting the Borrow Checker with `.clone()`

### The Trap

When the borrow checker complains, beginners clone everything to make errors go away.

```rust
// ❌ BAD: Cloning to avoid borrow checker
fn process_users(users: &Vec<User>) {
    for user in users {
        let user_clone = user.clone();  // Unnecessary clone!
        send_email(&user_clone);
        log_activity(&user_clone);
    }
}

// ✅ GOOD: Just borrow
fn process_users(users: &Vec<User>) {
    for user in users {
        send_email(user);
        log_activity(user);
    }
}

// Another example
// ❌ BAD: Cloning strings unnecessarily
fn get_filename(path: &str) -> String {
    let owned = path.to_string();  // Clone!
    owned.split('/').last().unwrap().to_string()  // Another clone!
}

// ✅ GOOD: Work with string slices
fn get_filename(path: &str) -> &str {
    path.split('/').last().unwrap()
}
```

**Why it's wrong:**
- Performance cost (heap allocations, copying data)
- Indicates you're not understanding ownership
- Hides actual design issues

**When cloning is appropriate:**
- Storing data in multiple places (legitimately need copies)
- Moving data into threads
- Breaking borrow checker cycles (rare)
- After profiling shows it's not a bottleneck

**Instead of cloning:**
- Use references (`&T`)
- Restructure your code to avoid conflicting borrows
- Use smart pointers (`Rc<T>`, `Arc<T>`) when truly need shared ownership

### Real Example

```rust
// ❌ BAD: Cloning to share data
struct App {
    config: Config,
}

impl App {
    fn process(&self) {
        let config_clone = self.config.clone();
        thread::spawn(move || {
            do_work(config_clone);
        });
    }
}

// ✅ GOOD: Use Arc for shared ownership
use std::sync::Arc;

struct App {
    config: Arc<Config>,
}

impl App {
    fn process(&self) {
        let config = Arc::clone(&self.config);
        thread::spawn(move || {
            do_work(&config);
        });
    }
}
```

## 3. Using `unwrap()` Everywhere

### The Trap

C/C++ developers are used to just dereferencing pointers. In Rust, they `unwrap()` every `Option` and `Result`.

```rust
// ❌ BAD: unwrap everywhere
fn read_config(path: &str) -> Config {
    let contents = std::fs::read_to_string(path).unwrap();  // Panic if file doesn't exist!
    let config: Config = serde_json::from_str(&contents).unwrap();  // Panic if invalid JSON!
    config
}

// ✅ GOOD: Proper error handling
fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}

// Or with context
use anyhow::{Context, Result};

fn read_config(path: &str) -> Result<Config> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path))?;

    let config = serde_json::from_str(&contents)
        .context("Failed to parse config JSON")?;

    Ok(config)
}
```

**Why it's wrong:**
- Your program panics (crashes) instead of handling errors gracefully
- Makes debugging harder
- Not idiomatic Rust

**When unwrap is okay:**
- Examples and prototypes
- Tests
- When you know for certain it can't fail (but add a comment explaining why!)
- `expect()` with a message is better: `value.expect("guaranteed by X")`

**Better alternatives:**
- `?` operator for propagating errors
- `match` for handling both cases
- `unwrap_or()`, `unwrap_or_else()` for defaults
- `ok()` to convert `Result` to `Option`

### Real Example

```rust
// ❌ BAD: Unwrap with no safety net
fn parse_port(s: &str) -> u16 {
    s.parse::<u16>().unwrap()  // Panics on invalid input!
}

// ✅ GOOD: Return Result
fn parse_port(s: &str) -> Result<u16, std::num::ParseIntError> {
    s.parse::<u16>()
}

// ✅ ALSO GOOD: Provide default
fn parse_port(s: &str) -> u16 {
    s.parse::<u16>().unwrap_or(8080)
}

// ✅ BEST: Validate with context
fn parse_port(s: &str) -> Result<u16, String> {
    let port: u16 = s.parse()
        .map_err(|_| format!("Invalid port number: {}", s))?;

    if port < 1024 {
        return Err(format!("Port {} is reserved", port));
    }

    Ok(port)
}
```

## 4. Not Using Iterators

### The Trap

C/C++ developers write manual loops because that's what they know.

```rust
// ❌ BAD: Manual loops everywhere
fn sum_even_squares(numbers: &Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..numbers.len() {
        if numbers[i] % 2 == 0 {
            result += numbers[i] * numbers[i];
        }
    }
    result
}

// ✅ GOOD: Iterator chain
fn sum_even_squares(numbers: &Vec<i32>) -> i32 {
    numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum()
}

// Another example
// ❌ BAD: Manual string building
fn join_strings(strings: &Vec<String>) -> String {
    let mut result = String::new();
    for i in 0..strings.len() {
        result.push_str(&strings[i]);
        if i < strings.len() - 1 {
            result.push_str(", ");
        }
    }
    result
}

// ✅ GOOD: Use iterator methods
fn join_strings(strings: &Vec<String>) -> String {
    strings.join(", ")
}
```

**Why it's wrong:**
- Verbose and error-prone
- Misses optimization opportunities
- Not idiomatic Rust

**Benefits of iterators:**
- Zero-cost abstractions (compile to same assembly as manual loops)
- Chainable transformations
- Lazy evaluation
- More functional, declarative style
- Compiler optimizes better

### Real Example

```rust
// ❌ BAD: Manual filtering and collecting
fn get_adult_names(people: &Vec<Person>) -> Vec<String> {
    let mut result = Vec::new();
    for person in people {
        if person.age >= 18 {
            result.push(person.name.clone());
        }
    }
    result
}

// ✅ GOOD: Iterator chain
fn get_adult_names(people: &Vec<Person>) -> Vec<String> {
    people.iter()
        .filter(|p| p.age >= 18)
        .map(|p| p.name.clone())
        .collect()
}

// ✅ EVEN BETTER: Avoid cloning if possible
fn get_adult_names(people: &Vec<Person>) -> Vec<&str> {
    people.iter()
        .filter(|p| p.age >= 18)
        .map(|p| p.name.as_str())
        .collect()
}
```

## 5. Using String When &str Would Work

### The Trap

C developers use `char*`, C++ developers use `std::string`. Rust has two types, and choosing wrong costs performance.

```rust
// ❌ BAD: Taking String when you only need to read
fn print_greeting(name: String) {  // Takes ownership!
    println!("Hello, {}!", name);
}

let name = String::from("Alice");
print_greeting(name);
// name is moved, can't use it anymore!

// ✅ GOOD: Use &str
fn print_greeting(name: &str) {  // Just borrows
    println!("Hello, {}!", name);
}

let name = String::from("Alice");
print_greeting(&name);  // Still own name
print_greeting("Bob");  // Can pass string literals too!

// Another example
// ❌ BAD: Returning String when &str would work
fn get_first_word(text: String) -> String {
    text.split_whitespace()
        .next()
        .unwrap_or("")
        .to_string()  // Unnecessary allocation!
}

// ✅ GOOD: Work with string slices
fn get_first_word(text: &str) -> &str {
    text.split_whitespace()
        .next()
        .unwrap_or("")
}
```

**Rule of thumb:**
- **Function parameters:** Use `&str` (unless you need ownership)
- **Return values:** Use `String` if creating new data, `&str` if returning slice
- **Struct fields:** Use `String` (unless you have lifetimes)

**Why it matters:**
- `String` allocates on heap
- `&str` is just a pointer and length
- `&str` is more flexible (accepts `String`, string literals, slices)

### Real Example

```rust
// ❌ BAD: Unnecessary String allocations
struct Config {
    host: String,
    port: String,  // Why is port a String?!
}

fn build_url(config: Config) -> String {  // Takes ownership for no reason
    format!("http://{}:{}", config.host, config.port)
}

// ✅ GOOD: Use appropriate types
struct Config {
    host: String,
    port: u16,  // Port is a number!
}

fn build_url(config: &Config) -> String {  // Just borrow
    format!("http://{}:{}", config.host, config.port)
}
```

## 6. Ignoring Rust's Enums

### The Trap

C/C++ developers use classes and inheritance when Rust enums would be better.

```rust
// ❌ BAD: Using Option-like struct
struct MaybeValue {
    has_value: bool,
    value: i32,  // Garbage if has_value is false!
}

fn get_value(flag: bool) -> MaybeValue {
    if flag {
        MaybeValue { has_value: true, value: 42 }
    } else {
        MaybeValue { has_value: false, value: 0 }
    }
}

// ✅ GOOD: Use Option
fn get_value(flag: bool) -> Option<i32> {
    if flag {
        Some(42)
    } else {
        None
    }
}

// Another example
// ❌ BAD: Error code in struct
struct Result {
    success: bool,
    value: i32,
    error_msg: String,  // Wasted space if success is true
}

// ✅ GOOD: Use Result enum
type MyResult = Result<i32, String>;

fn divide(a: i32, b: i32) -> MyResult {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

**Rust enums are powerful:**
- Can hold different data in each variant
- Compiler ensures you handle all cases
- Zero-cost abstraction (same size as largest variant + discriminant)

### Real Example

```rust
// ❌ BAD: Trait objects when enum would work
trait Message {
    fn send(&self);
}

struct EmailMessage { /* ... */ }
struct SmsMessage { /* ... */ }

impl Message for EmailMessage { /* ... */ }
impl Message for SmsMessage { /* ... */ }

fn process(msg: Box<dyn Message>) {  // Dynamic dispatch
    msg.send();
}

// ✅ GOOD: Use enum for known variants
enum Message {
    Email { to: String, subject: String, body: String },
    Sms { to: String, text: String },
}

impl Message {
    fn send(&self) {
        match self {
            Message::Email { to, subject, body } => {
                println!("Sending email to {}: {}", to, subject);
            }
            Message::Sms { to, text } => {
                println!("Sending SMS to {}: {}", to, text);
            }
        }
    }
}

fn process(msg: Message) {  // No dynamic dispatch, faster!
    msg.send();
}
```

## 7. Treating Rust Like C++ with Move Semantics

### The Trap

C++11 developers think Rust's move is like `std::move`, but it's fundamentally different.

```cpp
// C++: moved-from objects are "valid but unspecified"
std::string s1 = "hello";
std::string s2 = std::move(s1);
// s1 is in valid but unspecified state
// Can still use s1, might be empty, might not!
std::cout << s1.size();  // Compiles! Might be 0, might be 5, who knows?
```

```rust
// Rust: moved-from variables are GONE
let s1 = String::from("hello");
let s2 = s1;  // s1 is moved
// println!("{}", s1);  // ERROR: value used after move
// s1 doesn't exist anymore in the type system
```

**Key difference:**
- **C++:** Move is a runtime operation, moved-from object still exists
- **Rust:** Move is a compile-time transfer of ownership, original is gone

### Real Example

```rust
// ❌ BAD: Trying to use after move
fn process_vec(v: Vec<i32>) {
    let v2 = v;  // v is moved
    println!("Original: {:?}", v);  // ERROR!
    println!("New: {:?}", v2);
}

// ✅ GOOD: Clone if you need both, or just borrow
fn process_vec(v: &Vec<i32>) {
    let v2 = v.clone();  // Explicit clone
    println!("Original: {:?}", v);
    println!("Clone: {:?}", v2);
}

// ✅ BETTER: Just borrow
fn process_vec(v: &Vec<i32>) {
    println!("Vector: {:?}", v);
    // No cloning needed
}
```

## 8. Not Leveraging Zero-Cost Abstractions

### The Trap

Dropping down to "low-level" code when high-level Rust is just as fast.

```rust
// ❌ BAD: Manual unsafe indexing for "performance"
fn sum_array(arr: &[i32]) -> i32 {
    let mut sum = 0;
    unsafe {
        for i in 0..arr.len() {
            sum += *arr.get_unchecked(i);
        }
    }
    sum
}

// ✅ GOOD: High-level iterator - same performance!
fn sum_array(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

// Compiles to same assembly!
```

**Zero-cost abstractions mean:**
- Iterators compile to same code as manual loops
- Bounds checks often optimized away
- Generic code has no runtime overhead
- `Box<T>` is identical to raw pointer

**Don't assume you need unsafe for performance. Profile first!**

## 9. Avoiding Lifetimes

### The Trap

Lifetimes feel scary, so developers avoid them with unnecessary allocations.

```rust
// ❌ BAD: Cloning to avoid lifetimes
struct Parser {
    input: String,  // Owns a copy of the input
}

impl Parser {
    fn new(input: &str) -> Self {
        Parser {
            input: input.to_string(),  // Unnecessary clone!
        }
    }
}

// ✅ GOOD: Use lifetimes
struct Parser<'a> {
    input: &'a str,  // Just borrows the input
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input }
    }
}
```

**When to use lifetimes:**
- Storing references in structs
- Returning references from functions
- When you want to borrow instead of own

**Lifetimes aren't scary:**
- Usually elided (inferred)
- Just make borrowing relationships explicit
- Prevent dangling references at compile time

## 10. Writing C++ Style Code in Rust

### The Trap

Using Rust's syntax but thinking in C++.

```rust
// ❌ BAD: C++ thinking in Rust
struct Data {
    ptr: *mut i32,  // Raw pointer like C++
}

impl Data {
    fn new(value: i32) -> Self {
        Data {
            ptr: Box::into_raw(Box::new(value)),
        }
    }

    fn get(&self) -> i32 {
        unsafe { *self.ptr }
    }

    fn set(&mut self, value: i32) {
        unsafe { *self.ptr = value; }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}

// ✅ GOOD: Idiomatic Rust
struct Data {
    value: i32,  // Just store the value!
}

impl Data {
    fn new(value: i32) -> Self {
        Data { value }
    }

    fn get(&self) -> i32 {
        self.value
    }

    fn set(&mut self, value: i32) {
        self.value = value;
    }
}
// No manual Drop needed!
```

## Summary: Common Anti-Patterns

| Anti-Pattern | Why It's Wrong | Better Approach |
|--------------|----------------|-----------------|
| Overusing `unsafe` | Loses safety guarantees | Use safe Rust, it's more powerful than you think |
| `.clone()` everything | Performance cost, not understanding ownership | Use references, restructure code |
| `.unwrap()` everywhere | Program panics instead of graceful errors | Use `?`, `match`, or `unwrap_or()` |
| Manual loops | Verbose, misses optimizations | Use iterator chains |
| `String` when `&str` works | Unnecessary allocations | Function params: `&str`, returns: `String` if new data |
| Ignoring enums | Missing powerful abstraction | Use enums for sum types, pattern matching |
| Treating move like C++ | Misunderstanding ownership | Moved values are GONE, not "unspecified" |
| Avoiding zero-cost abstractions | Premature optimization | Trust the compiler, profile first |
| Avoiding lifetimes | Unnecessary allocations | Lifetimes are free, cloning is not |
| Writing C++ in Rust syntax | Fighting the language | Learn Rust idioms, embrace the differences |

**The golden rule:** When the borrow checker fights you, don't reach for `unsafe`, `.clone()`, or workarounds. Redesign your code to work with ownership. The borrow checker is trying to help you.
