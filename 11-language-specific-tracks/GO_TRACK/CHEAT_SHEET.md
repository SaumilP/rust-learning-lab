# Cheat Sheet: Go to Rust Quick Reference

Quick lookup for translating Go patterns to Rust. Bookmark this page!

## Table of Contents

1. [Variables & Types](#variables--types)
2. [Functions](#functions)
3. [Control Flow](#control-flow)
4. [Collections](#collections)
5. [Structs & Methods](#structs--methods)
6. [Interfaces & Traits](#interfaces--traits)
7. [Error Handling](#error-handling)
8. [Concurrency](#concurrency)
9. [Pointers & References](#pointers--references)
10. [Common Patterns](#common-patterns)

---

## Variables & Types

| Go | Rust | Notes |
|----|------|-------|
| `var x int = 5` | `let x: i32 = 5;` | Explicit type |
| `x := 5` | `let x = 5;` | Type inference |
| `var x int` | `let x: i32 = 0;` | Must initialize in Rust |
| `x = 10` (reassign) | `let mut x = 5; x = 10;` | Must use `mut` |
| `const MAX = 100` | `const MAX: i32 = 100;` | Uppercase convention |
| `string` | `String` or `&str` | `&str` for parameters |
| `[]byte` | `Vec<u8>` or `&[u8]` | |
| `int`, `int32`, `int64` | `i32`, `i64`, `isize` | |
| `uint`, `uint32`, `uint64` | `u32`, `u64`, `usize` | |
| `float32`, `float64` | `f32`, `f64` | |
| `bool` | `bool` | |
| `rune` | `char` | UTF-8 in both |
| `nil` | `None` (in `Option<T>`) | Must be explicit |

---

## Functions

| Go | Rust |
|----|------|
| `func add(a int, b int) int` | `fn add(a: i32, b: i32) -> i32` |
| `func swap(a, b int) (int, int)` | `fn swap(a: i32, b: i32) -> (i32, i32)` |
| `func process() (int, error)` | `fn process() -> Result<i32, Error>` |
| `func(x int) int { return x * 2 }` | `\|x: i32\| x * 2` or `\|x\| x * 2` |
| `defer cleanup()` | Trust `Drop` trait (RAII) |
| `panic("error")` | `panic!("error")` |
| `recover()` | `std::panic::catch_unwind` (rare) |

### Example: Multiple Return Values

```go
// Go
func divide(a, b int) (int, error) {
    if b == 0 {
        return 0, errors.New("division by zero")
    }
    return a / b, nil
}

result, err := divide(10, 2)
if err != nil {
    log.Fatal(err)
}
```

```rust
// Rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("division by zero".to_string());
    }
    Ok(a / b)
}

let result = divide(10, 2)?;  // Propagates error
// Or
match divide(10, 2) {
    Ok(r) => println!("{}", r),
    Err(e) => eprintln!("{}", e),
}
```

---

## Control Flow

| Go | Rust |
|----|------|
| `if x > 0 { }` | `if x > 0 { }` |
| `if x := getValue(); x > 0 { }` | `if let x = get_value() { if x > 0 { } }` |
| `for i := 0; i < 10; i++ { }` | `for i in 0..10 { }` |
| `for i, v := range items { }` | `for (i, v) in items.iter().enumerate() { }` |
| `for v := range items { }` | `for v in items.iter() { }` or `for v in &items { }` |
| `for { }` (infinite) | `loop { }` |
| `for condition { }` | `while condition { }` |
| `switch` | `match` |
| `break` | `break` |
| `continue` | `continue` |

### Example: Switch/Match

```go
// Go
switch x {
case 1:
    fmt.Println("one")
case 2, 3:
    fmt.Println("two or three")
default:
    fmt.Println("other")
}
```

```rust
// Rust
match x {
    1 => println!("one"),
    2 | 3 => println!("two or three"),
    _ => println!("other"),
}

// Or with if
if x == 1 {
    println!("one");
} else if x == 2 || x == 3 {
    println!("two or three");
} else {
    println!("other");
}
```

---

## Collections

| Go | Rust |
|----|------|
| `[]int{1, 2, 3}` | `vec![1, 2, 3]` |
| `make([]int, 0, 10)` | `Vec::with_capacity(10)` |
| `append(slice, item)` | `vec.push(item)` |
| `len(slice)` | `vec.len()` |
| `slice[i]` | `vec[i]` or `vec.get(i)` |
| `slice[start:end]` | `&vec[start..end]` |
| `map[string]int{}` | `HashMap<String, i32>::new()` |
| `m[key] = value` | `m.insert(key, value);` |
| `value := m[key]` | `let value = m.get(&key);` (returns `Option`) |
| `value, ok := m[key]` | `if let Some(v) = m.get(&key) { }` |
| `delete(m, key)` | `m.remove(&key);` |

### Example: Maps

```go
// Go
m := make(map[string]int)
m["age"] = 30

if value, ok := m["age"]; ok {
    fmt.Println(value)
}
```

```rust
// Rust
use std::collections::HashMap;

let mut m = HashMap::new();
m.insert("age".to_string(), 30);

if let Some(value) = m.get("age") {
    println!("{}", value);
}

// Or with match
match m.get("age") {
    Some(v) => println!("{}", v),
    None => println!("not found"),
}
```

---

## Structs & Methods

| Go | Rust |
|----|------|
| `type Person struct { Name string }` | `struct Person { name: String }` |
| `p := Person{Name: "Alice"}` | `let p = Person { name: "Alice".to_string() };` |
| `p.Name` | `p.name` |
| `func (p *Person) SetName(n string)` | `impl Person { fn set_name(&mut self, n: String) }` |
| `func (p Person) GetName() string` | `impl Person { fn get_name(&self) -> &str }` |

### Example: Methods

```go
// Go
type Counter struct {
    count int
}

func (c *Counter) Increment() {
    c.count++
}

func (c Counter) Value() int {
    return c.count
}
```

```rust
// Rust
struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn value(&self) -> i32 {
        self.count
    }
}
```

---

## Interfaces & Traits

| Go | Rust |
|----|------|
| `type Reader interface { Read() }` | `trait Reader { fn read(&mut self); }` |
| Implicit satisfaction | `impl Reader for MyType { }` |
| `var r Reader = &MyType{}` | `let r: Box<dyn Reader> = Box::new(MyType);` |
| `func process(r Reader)` | `fn process<T: Reader>(r: T)` or `fn process(r: &dyn Reader)` |

### Example: Interface/Trait

```go
// Go
type Writer interface {
    Write([]byte) (int, error)
}

func save(w Writer, data []byte) {
    w.Write(data)
}

// Any type with Write method satisfies Writer
```

```rust
// Rust
trait Writer {
    fn write(&mut self, data: &[u8]) -> io::Result<usize>;
}

fn save<W: Writer>(w: &mut W, data: &[u8]) {
    w.write(data).unwrap();
}

// Must explicitly implement
impl Writer for MyType {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        // Implementation
    }
}
```

---

## Error Handling

| Go | Rust |
|----|------|
| `if err != nil { return err }` | `let x = fallible()?;` |
| `errors.New("msg")` | `Err("msg".to_string())` |
| `fmt.Errorf("failed: %w", err)` | `.context("failed")?` (anyhow) |
| `_, err := operation()` | `operation()?` |
| `panic(err)` | `panic!("{}", err)` or `.unwrap()` |

### Example: Error Propagation

```go
// Go
func readConfig() (Config, error) {
    data, err := os.ReadFile("config.json")
    if err != nil {
        return Config{}, err
    }

    var config Config
    err = json.Unmarshal(data, &config)
    if err != nil {
        return Config{}, err
    }

    return config, nil
}
```

```rust
// Rust
fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("config.json")?;
    let config = serde_json::from_str(&data)?;
    Ok(config)
}

// Or with anyhow
use anyhow::{Context, Result};

fn read_config() -> Result<Config> {
    let data = fs::read_to_string("config.json")
        .context("failed to read config")?;
    let config = serde_json::from_str(&data)
        .context("failed to parse JSON")?;
    Ok(config)
}
```

---

## Concurrency

| Go | Rust |
|----|------|
| `go func() { }()` | `thread::spawn(\|\| { });` or `tokio::spawn(async { })` |
| `ch := make(chan int)` | `let (tx, rx) = mpsc::channel();` |
| `ch <- value` | `tx.send(value).unwrap();` |
| `value := <-ch` | `let value = rx.recv().unwrap();` |
| `close(ch)` | `drop(tx)` (drop all senders) |
| `select { case <-ch1: ... }` | `tokio::select! { v = rx1.recv() => ... }` |
| `sync.Mutex` | `std::sync::Mutex` |
| `sync.RWMutex` | `std::sync::RwLock` |
| `sync.WaitGroup` | Manual with channels or use `join_all` |
| `context.Context` | Channels for cancellation |

### Example: Goroutines vs Threads

```go
// Go
func main() {
    var wg sync.WaitGroup

    for i := 0; i < 10; i++ {
        wg.Add(1)
        go func(id int) {
            defer wg.Done()
            fmt.Println(id)
        }(i)
    }

    wg.Wait()
}
```

```rust
// Rust (threads)
use std::thread;

fn main() {
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                println!("{}", i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

// Rust (async with Tokio)
#[tokio::main]
async fn main() {
    let tasks: Vec<_> = (0..10)
        .map(|i| {
            tokio::spawn(async move {
                println!("{}", i);
            })
        })
        .collect();

    for task in tasks {
        task.await.unwrap();
    }
}
```

---

## Pointers & References

| Go | Rust |
|----|------|
| `&x` (take address) | `&x` (borrow) |
| `*ptr` (dereference) | `*ptr` (dereference, rare) |
| `*int` (pointer type) | `&i32` (reference) or `&mut i32` (mutable ref) |
| `new(int)` | `Box::new(0)` |
| `x == nil` | `x.is_none()` (for `Option`) |

### Key Differences

```go
// Go: Pointers are nullable
var p *int = nil
if p != nil {
    fmt.Println(*p)
}
```

```rust
// Rust: References can't be null, use Option
let p: Option<&i32> = None;
if let Some(val) = p {
    println!("{}", val);
}

// Regular references are always valid
let x = 5;
let r = &x;  // Always points to valid data
println!("{}", r);
```

---

## Common Patterns

### Reading Files

```go
// Go
data, err := os.ReadFile("file.txt")
if err != nil {
    log.Fatal(err)
}
fmt.Println(string(data))
```

```rust
// Rust
use std::fs;

let data = fs::read_to_string("file.txt")
    .expect("failed to read file");
println!("{}", data);
```

### HTTP Request

```go
// Go
resp, err := http.Get("https://example.com")
if err != nil {
    log.Fatal(err)
}
defer resp.Body.Close()

body, err := io.ReadAll(resp.Body)
```

```rust
// Rust (with reqwest)
let body = reqwest::get("https://example.com")
    .await?
    .text()
    .await?;
```

### JSON Parsing

```go
// Go
var user User
err := json.Unmarshal(data, &user)
if err != nil {
    log.Fatal(err)
}
```

```rust
// Rust (with serde)
let user: User = serde_json::from_str(&data)?;
```

### String Formatting

```go
// Go
msg := fmt.Sprintf("Hello, %s! You are %d years old.", name, age)
```

```rust
// Rust
let msg = format!("Hello, {}! You are {} years old.", name, age);
```

### Printing

| Go | Rust |
|----|------|
| `fmt.Println(x)` | `println!("{}", x);` |
| `fmt.Printf("%d\n", x)` | `println!("{}", x);` |
| `fmt.Printf("%v\n", x)` | `println!("{:?}", x);` (Debug) |
| `log.Println(x)` | `eprintln!("{}", x);` or use `log` crate |

---

## Type Conversions

| Go | Rust |
|----|------|
| `int32(x)` | `x as i32` (for primitives) |
| `string(bytes)` | `String::from_utf8(bytes)?` |
| `[]byte(str)` | `str.as_bytes()` or `str.into_bytes()` |
| `strconv.Atoi(s)` | `s.parse::<i32>()?` |
| `strconv.Itoa(i)` | `i.to_string()` |

---

## Quick Reminders

### Ownership Rules
1. Each value has one owner
2. When owner goes out of scope, value is dropped
3. Can have many `&T` OR one `&mut T` at a time

### Common Types
- `String` = owned string (like `[]byte` in Go)
- `&str` = string slice (use for parameters)
- `Vec<T>` = growable array (like Go slice)
- `&[T]` = slice (borrow of array/Vec)
- `Option<T>` = value or None (like nullable in Go)
- `Result<T, E>` = success or error

### Iterator Shortcuts
- `.iter()` = iterate with `&T`
- `.iter_mut()` = iterate with `&mut T`
- `.into_iter()` = iterate with `T` (consumes)
- `.map()` = transform each element
- `.filter()` = keep matching elements
- `.collect()` = build collection from iterator

### Error Handling
- `?` = propagate error (like Go's `if err != nil { return err }`)
- `.unwrap()` = panic if error (use only for prototyping)
- `.expect("msg")` = panic with message
- `.unwrap_or(default)` = use default if error
- `match` = handle specific error cases

---

## Pro Tips

1. **Default to `&str` for parameters**: `fn process(s: &str)` works with both `String` and `&str`

2. **Use `?` for error propagation**: Much cleaner than Go's `if err != nil`

3. **Iterator chains are free**: They compile to same code as manual loops

4. **Clone when learning**: `.clone()` gets you unstuck while learning ownership

5. **Read compiler errors**: They're actually helpful and suggest fixes

6. **Use `cargo clippy`**: Like `go vet` but stricter and more helpful

---

This cheat sheet covers 90% of what you'll need day-to-day. For deeper dives, see the other guides in this track!
