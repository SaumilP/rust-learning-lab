# Fundamentals: Python vs Rust

This guide shows how fundamental concepts differ between Python and Rust. Each section has Python code you know, followed by the Rust equivalent, and then an explanation of key differences.

## Table of Contents

1. [Dynamic vs Static Typing](#dynamic-vs-static-typing)
2. [Mutable Objects vs Ownership](#mutable-objects-vs-ownership)
3. [None vs Option](#none-vs-option)
4. [Exceptions vs Result](#exceptions-vs-result)
5. [Duck Typing vs Traits](#duck-typing-vs-traits)
6. [List Comprehensions vs Iterators](#list-comprehensions-vs-iterators)
7. [Async/Await: asyncio vs Tokio](#asyncawait-asyncio-vs-tokio)

---

## Dynamic vs Static Typing

This is the most visible difference. Python checks types at runtime; Rust checks at compile time.

### Python approach: Runtime type checking

```python
# Python: Types are optional hints
def greet(name: str) -> str:
    return f"Hello, {name}!"

# Type hints don't prevent this
result = greet(123)  # Works! Returns "Hello, 123!"
result = greet(None)  # Works! Returns "Hello, None!"

# Runtime error only if you do something invalid
def get_length(items: list) -> int:
    return len(items)

get_length("hello")  # Works! Strings have len()
get_length(123)      # TypeError at runtime
```

**What's happening**:
- Type hints are documentation (mypy can check them, but it's optional)
- Duck typing: if it quacks like a duck, it's a duck
- Errors happen when you run the code
- Very flexible for prototyping

### Rust approach: Compile-time type checking

```rust
// Rust: Types are enforced by compiler
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// These won't compile
// greet(123);     // ERROR: expected &str, found integer
// greet(None);    // ERROR: expected &str, found Option<_>

// Correct usage
let result = greet("Alice");  // OK

// Generic function for types that implement a trait
fn get_length<T>(items: &[T]) -> usize {
    items.len()
}

// get_length("hello");  // ERROR: expected slice, found &str
get_length(&[1, 2, 3]);   // OK: slice of integers
```

**What's happening**:
- Types must be known at compile time
- No duck typing—types must explicitly implement traits
- Errors are caught before you run the code
- Type inference reduces verbosity (Rust infers a lot!)

### Key differences

| Aspect | Python | Rust |
|--------|--------|------|
| **Type checking** | Runtime | Compile-time |
| **Type hints** | Optional documentation | Required, enforced |
| **Duck typing** | Yes, anything goes | No, explicit trait implementation |
| **Errors** | At runtime | Before you run |
| **Flexibility** | Very high | Moderate (but type inference helps) |
| **Safety** | Runtime crashes possible | Many errors caught early |

### When type inference helps

```rust
// Rust infers types from usage (like Python!)
let numbers = vec![1, 2, 3];  // Inferred as Vec<i32>
let doubled = numbers.iter()
    .map(|x| x * 2)           // Inferred that x is &i32
    .collect::<Vec<i32>>();

// Often you don't need type annotations
let name = "Alice";           // Inferred as &str
let age = 30;                 // Inferred as i32
let scores = vec![95, 87, 92]; // Inferred as Vec<i32>
```

---

## Mutable Objects vs Ownership

Python has mutable objects everywhere. Rust has ownership rules.

### Python approach: Mutable references

```python
# Python: Objects are mutable by default
data = {"count": 0}

def increment(d):
    d["count"] += 1  # Modifies original

increment(data)
print(data["count"])  # 1

# Multiple references to same object
list1 = [1, 2, 3]
list2 = list1  # Both refer to same list
list2.append(4)
print(list1)  # [1, 2, 3, 4] - modified through list2
```

**What's happening**:
- Everything is a reference (no copies unless you ask)
- Mutations are visible through all references
- GC cleans up when no more references exist
- Easy to share and modify data

### Rust approach: Ownership and borrowing

```rust
// Rust: One owner, explicit borrowing
use std::collections::HashMap;

fn increment(d: &mut HashMap<String, i32>) {
    *d.entry("count".to_string()).or_insert(0) += 1;
}

let mut data = HashMap::new();
data.insert("count".to_string(), 0);
increment(&mut data);  // Borrow mutably
println!("{}", data["count"]);  // 1

// Moving vs borrowing
let list1 = vec![1, 2, 3];
let list2 = list1;  // list1 is MOVED, not copied
// println!("{:?}", list1);  // ERROR: value borrowed after move

// To share, you borrow
let list1 = vec![1, 2, 3];
let list2 = &list1;  // Borrow immutably
println!("{:?}", list1);  // OK: list1 still owns it
```

**What's happening**:
- Each value has exactly one owner
- When owner goes out of scope, value is dropped
- Borrowing (&T or &mut T) lets you use without owning
- One mutable borrow OR many immutable borrows (never both)

### Key differences

| Aspect | Python | Rust |
|--------|--------|------|
| **Default** | Mutable references | Immutable ownership |
| **Copying** | Rare (use copy.deepcopy) | Explicit with .clone() |
| **Sharing** | Easy (references everywhere) | Explicit (borrowing rules) |
| **Mutation** | Anytime, anywhere | Only with &mut |
| **Memory cleanup** | GC determines when | Owner determines when |

### Ownership rules visualized

```python
# Python mental model
data = [1, 2, 3]
ref1 = data  # Points to same list
ref2 = data  # Points to same list
ref1.append(4)  # All see the change
```

```rust
// Rust mental model
let data = vec![1, 2, 3];  // data owns the Vec

// Option 1: Move ownership
let moved = data;  // data no longer owns it, moved does

// Option 2: Borrow immutably (many allowed)
let data = vec![1, 2, 3];
let borrow1 = &data;
let borrow2 = &data;  // OK: multiple immutable borrows

// Option 3: Borrow mutably (only one allowed)
let mut data = vec![1, 2, 3];
let borrow = &mut data;
borrow.push(4);
// let borrow2 = &mut data;  // ERROR: already borrowed
```

---

## None vs Option

Python has `None` that can hide anywhere. Rust has `Option<T>` in the type system.

### Python approach: None everywhere

```python
# Python: Any object can be None
def find_user(user_id: int) -> dict:
    # Might return None (but type hint doesn't say so!)
    if user_id == 1:
        return {"name": "Alice", "age": 30}
    return None  # Easy to forget to check

# Usage (easy to forget None check)
user = find_user(2)
print(user["name"])  # AttributeError: 'NoneType' object has no attribute '__getitem__'

# Proper usage
user = find_user(2)
if user is not None:
    print(user["name"])
else:
    print("User not found")
```

**Problems**:
- `None` can hide anywhere
- Type hints don't enforce checking
- Easy to forget None check → runtime crash
- `NoneType` errors are common in production

### Rust approach: Option<T>

```rust
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn find_user(user_id: i32) -> Option<User> {
    if user_id == 1 {
        Some(User {
            name: String::from("Alice"),
            age: 30,
        })
    } else {
        None
    }
}

// Must handle None case - compiler enforces it
match find_user(2) {
    Some(user) => println!("Name: {}", user.name),
    None => println!("User not found"),
}

// Or use if let for single case
if let Some(user) = find_user(1) {
    println!("Name: {}", user.name);
}

// Or provide default
let user = find_user(2).unwrap_or_else(|| User {
    name: String::from("Guest"),
    age: 0,
});
```

**Option is an enum**:

```rust
enum Option<T> {
    Some(T),  // Has a value
    None,     // No value
}
```

### Key differences

| Aspect | Python | Rust |
|--------|--------|------|
| **Null handling** | None can be anywhere | Option<T> is explicit |
| **Type system** | Types don't show None | Option is in the type |
| **Compiler** | Doesn't check None | Forces handling of None |
| **Runtime errors** | AttributeError, TypeError | Compile error if not handled |
| **Convenience methods** | None | .unwrap(), .unwrap_or(), .map(), etc. |

### Helpful Option methods

```rust
let maybe_number: Option<i32> = Some(42);

// Unwrap (panics if None - use carefully!)
let value = maybe_number.unwrap();

// Unwrap with default
let value = maybe_number.unwrap_or(0);

// Map over option (like Python's map on optional)
let doubled = maybe_number.map(|x| x * 2);  // Some(84)

// Chain operations
let result = Some(5)
    .map(|x| x * 2)
    .filter(|x| x > &5)
    .unwrap_or(0);  // 10
```

---

## Exceptions vs Result

Python uses try/except. Rust uses `Result<T, E>`.

### Python approach: Exceptions

```python
# Python: Exceptions for errors
import json

def load_config(path: str) -> dict:
    with open(path) as f:  # Might raise FileNotFoundError
        return json.load(f)  # Might raise JSONDecodeError

# Usage
try:
    config = load_config("config.json")
    print(config["database"]["host"])
except FileNotFoundError:
    print("Config file not found")
except json.JSONDecodeError:
    print("Invalid JSON")
except KeyError as e:
    print(f"Missing config key: {e}")
```

**Problems**:
- Exceptions can be anywhere (not in function signature)
- Easy to miss catching an exception
- Control flow via exceptions is invisible
- Performance cost for exceptional paths

### Rust approach: Result type

```rust
use std::fs;
use serde_json::{self, Value};

fn load_config(path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;  // Propagates error
    let config: Value = serde_json::from_str(&contents)?;
    Ok(config)
}

// Usage with explicit matching
match load_config("config.json") {
    Ok(config) => {
        if let Some(host) = config["database"]["host"].as_str() {
            println!("Host: {}", host);
        }
    }
    Err(e) => println!("Error: {}", e),
}

// Or with ? operator in a function that returns Result
fn main_app() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config("config.json")?;
    let host = config["database"]["host"]
        .as_str()
        .ok_or("Missing host")?;
    println!("Host: {}", host);
    Ok(())
}
```

**Result is an enum**:

```rust
enum Result<T, E> {
    Ok(T),   // Success value
    Err(E),  // Error value
}
```

### Key differences

| Aspect | Python | Rust |
|--------|--------|------|
| **Error visibility** | Hidden | In type signature |
| **Handling** | Optional (try/except) | Compiler enforces |
| **Control flow** | Exception jumps stack | Return values |
| **Performance** | Stack unwinding cost | Zero cost (just an enum) |
| **Type safety** | Exception type not in signature | Error type in Result<T, E> |

### The ? operator (like exception propagation)

```rust
// Without ?
fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = fs::File::open("username.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// With ? (same as above, but cleaner!)
fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut s = String::new();
    fs::File::open("username.txt")?
        .read_to_string(&mut s)?;
    Ok(s)
}
```

---

## Duck Typing vs Traits

Python uses duck typing. Rust uses explicit traits.

### Python approach: Duck typing

```python
# Python: If it walks like a duck...
class Duck:
    def quack(self):
        print("Quack!")

class Person:
    def quack(self):
        print("I'm imitating a duck!")

# Function accepts anything with quack()
def make_it_quack(thing):
    thing.quack()  # Works if thing has quack()

make_it_quack(Duck())
make_it_quack(Person())
# make_it_quack(123)  # AttributeError at runtime
```

**What's happening**:
- No explicit interface declaration
- Any object with the right methods works
- Very flexible
- Errors only at runtime

### Rust approach: Traits

```rust
// Rust: Explicit trait definition
trait Quackable {
    fn quack(&self);
}

struct Duck;
struct Person;

// Explicit implementation
impl Quackable for Duck {
    fn quack(&self) {
        println!("Quack!");
    }
}

impl Quackable for Person {
    fn quack(&self) {
        println!("I'm imitating a duck!");
    }
}

// Function requires Quackable trait
fn make_it_quack<T: Quackable>(thing: &T) {
    thing.quack();
}

// Or with trait objects (dynamic dispatch)
fn make_it_quack_dyn(thing: &dyn Quackable) {
    thing.quack();
}

// Usage
make_it_quack(&Duck);
make_it_quack(&Person);
// make_it_quack(&123);  // ERROR at compile time
```

### Key differences

| Aspect | Python | Rust |
|--------|--------|------|
| **Interfaces** | Implicit (duck typing) | Explicit (traits) |
| **Type checking** | Runtime | Compile-time |
| **Dispatch** | Always dynamic | Static (generics) or dynamic (trait objects) |
| **Performance** | Virtual calls | Can be inlined with generics |
| **Safety** | Runtime errors | Compile errors |

### Python protocols (PEP 544) vs Rust traits

```python
# Python typing.Protocol (closer to Rust traits)
from typing import Protocol

class Drawable(Protocol):
    def draw(self) -> None: ...

class Circle:
    def draw(self) -> None:
        print("Drawing circle")

def render(shape: Drawable) -> None:
    shape.draw()

render(Circle())  # OK with static type checker
```

```rust
// Rust trait (similar concept, but enforced)
trait Drawable {
    fn draw(&self);
}

struct Circle;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle");
    }
}

fn render<T: Drawable>(shape: &T) {
    shape.draw();
}

render(&Circle);  // Verified at compile time
```

---

## List Comprehensions vs Iterators

Python has comprehensions. Rust has iterator chains.

### Python approach: Comprehensions

```python
# Python: List comprehensions
numbers = [1, 2, 3, 4, 5]

# Filter and map
doubled = [x * 2 for x in numbers if x % 2 == 0]
print(doubled)  # [4, 8]

# Dict comprehension
squared = {x: x**2 for x in numbers}
print(squared)  # {1: 1, 2: 4, 3: 9, 4: 16, 5: 25}

# Nested
matrix = [[i * j for j in range(3)] for i in range(3)]

# Generator (lazy)
squares = (x**2 for x in range(1000000))
```

**What's happening**:
- Concise syntax for transformations
- List comprehensions are eager (create full list)
- Generator expressions are lazy
- Very readable for simple cases

### Rust approach: Iterator chains

```rust
// Rust: Iterator chains
let numbers = vec![1, 2, 3, 4, 5];

// Filter and map
let doubled: Vec<i32> = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * 2)
    .collect();
println!("{:?}", doubled);  // [4, 8]

// Into HashMap
use std::collections::HashMap;
let squared: HashMap<i32, i32> = numbers.iter()
    .map(|&x| (x, x * x))
    .collect();
println!("{:?}", squared);  // {1: 1, 2: 4, 3: 9, 4: 16, 5: 25}

// Nested (flatten)
let matrix: Vec<Vec<i32>> = (0..3)
    .map(|i| (0..3).map(|j| i * j).collect())
    .collect();

// Lazy iterator (like generator)
let squares = (0..1000000).map(|x| x * x);
// Nothing computed until you consume it
```

**What's happening**:
- Method chaining instead of comprehension syntax
- Iterators are lazy (like Python generators)
- Must call `.collect()` to materialize
- Very composable

### Key differences

| Aspect | Python | Rust |
|--------|--------|------|
| **Syntax** | [x for x in items] | items.iter().map() |
| **Lazy** | Generators only | All iterators |
| **Type inference** | Dynamic | Static (may need type hints) |
| **Chaining** | Limited | Very composable |
| **Performance** | Depends | Zero-cost abstraction |

### Common iterator patterns

```rust
let numbers = vec![1, 2, 3, 4, 5];

// Sum
let total: i32 = numbers.iter().sum();

// Find first match
let first_even = numbers.iter().find(|&&x| x % 2 == 0);

// Any/all
let has_even = numbers.iter().any(|&x| x % 2 == 0);
let all_positive = numbers.iter().all(|&x| x > 0);

// Take first n
let first_three: Vec<_> = numbers.iter().take(3).collect();

// Skip n
let last_two: Vec<_> = numbers.iter().skip(3).collect();

// Enumerate
for (i, value) in numbers.iter().enumerate() {
    println!("{}: {}", i, value);
}

// Zip
let names = vec!["Alice", "Bob", "Charlie"];
let ages = vec![30, 25, 35];
let paired: Vec<_> = names.iter().zip(ages.iter()).collect();
```

---

## Async/Await: asyncio vs Tokio

Both languages have async/await! Syntax is similar, but runtime differs.

### Python approach: asyncio

```python
# Python: asyncio event loop
import asyncio
import aiohttp

async def fetch_url(url: str) -> str:
    async with aiohttp.ClientSession() as session:
        async with session.get(url) as response:
            return await response.text()

async def main():
    urls = ["http://example.com", "http://example.org"]
    tasks = [fetch_url(url) for url in urls]
    results = await asyncio.gather(*tasks)
    for result in results:
        print(result[:100])

# Run event loop
asyncio.run(main())
```

**What's happening**:
- GIL limits to single-threaded execution
- Event loop schedules coroutines
- await suspends until result ready
- Good for I/O-bound tasks

### Rust approach: Tokio

```rust
// Rust: Tokio async runtime
use tokio;
use reqwest;

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    Ok(text)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec!["http://example.com", "http://example.org"];

    let tasks: Vec<_> = urls.iter()
        .map(|url| tokio::spawn(fetch_url(url)))
        .collect();

    for task in tasks {
        let result = task.await??;
        println!("{}", &result[..100.min(result.len())]);
    }

    Ok(())
}
```

**What's happening**:
- No GIL—true multi-threading possible
- Tokio runtime schedules futures
- await suspends until result ready
- Scales to 100,000+ concurrent tasks

### Key differences

| Aspect | Python | Rust |
|--------|--------|------|
| **GIL** | Yes (single-threaded) | No (multi-threaded) |
| **Runtime** | Built-in (asyncio) | External crate (tokio) |
| **Syntax** | async/await | async/await (same!) |
| **Scaling** | 1,000s of tasks | 100,000s of tasks |
| **Error handling** | Exceptions | Result types |
| **Type safety** | Dynamic | Static (Future<Output=T>) |

### Concurrent operations comparison

```python
# Python: Gather concurrent tasks
results = await asyncio.gather(
    fetch_data(1),
    fetch_data(2),
    fetch_data(3)
)
```

```rust
// Rust: Join concurrent tasks
let (result1, result2, result3) = tokio::join!(
    fetch_data(1),
    fetch_data(2),
    fetch_data(3)
);
```

---

## Summary

Quick reference table:

| Concept | Python | Rust | Mental Shift |
|---------|--------|------|--------------|
| **Types** | Dynamic | Static | Compile-time verification |
| **None** | Everywhere | Option<T> | Make "nothing" explicit |
| **Errors** | Exceptions | Result<T, E> | Errors are values |
| **Interfaces** | Duck typing | Traits | Explicit implementation |
| **Lists** | Comprehensions | Iterators | Method chains |
| **Mutability** | Default | Opt-in with mut | Immutable by default |
| **Memory** | GC | Ownership | Think about who owns what |
| **Async** | asyncio + GIL | Tokio (no GIL) | True parallelism |

The biggest mental shift: Rust makes you think about how computers actually work. Python abstracts that away. Both are valuable for different problems.

Next: Check out DESIGN_PATTERNS_GUIDE.md to see how Python patterns translate to Rust!
