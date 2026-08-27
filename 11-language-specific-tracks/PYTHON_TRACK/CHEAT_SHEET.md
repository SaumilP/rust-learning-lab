# Python to Rust Cheat Sheet

Quick reference for "how do I do X in Rust?" when you know Python.

## Basic Syntax

| Python | Rust | Notes |
|--------|------|-------|
| `def func():` | `fn func() {}` | Functions |
| `def func(x: int) -> int:` | `fn func(x: i32) -> i32 {}` | With types |
| `x = 5` | `let x = 5;` | Immutable variable |
| `x = 5; x = 6` | `let mut x = 5; x = 6;` | Mutable variable |
| `# comment` | `// comment` | Single-line comment |
| `"""docstring"""` | `/// doc comment` | Documentation |
| `pass` | `{}` or `()` | Empty block/unit |

## Types

| Python | Rust | Notes |
|--------|------|-------|
| `int` | `i32`, `i64`, `isize` | Signed integers |
| `int` (arbitrary precision) | `i128` or `BigInt` crate | Large numbers |
| `float` | `f32`, `f64` | Floating point |
| `bool` | `bool` | Boolean |
| `str` | `&str` (borrowed) or `String` (owned) | Strings |
| `None` | `None` (in `Option<T>`) | Null value |
| `list` | `Vec<T>` | Dynamic array |
| `tuple` | `(T, U, V)` | Fixed-size tuple |
| `dict` | `HashMap<K, V>` | Hash map |
| `set` | `HashSet<T>` | Hash set |
| `bytes` | `Vec<u8>` or `&[u8]` | Byte arrays |

## Collections

### Lists vs Vec

| Python | Rust | Notes |
|--------|------|-------|
| `lst = []` | `let mut v = Vec::new();` | Empty list/vec |
| `lst = [1, 2, 3]` | `let v = vec![1, 2, 3];` | With values |
| `lst.append(x)` | `v.push(x);` | Add element |
| `lst.pop()` | `v.pop()` | Remove & return last (returns Option) |
| `lst.insert(i, x)` | `v.insert(i, x);` | Insert at index |
| `lst[i]` | `v[i]` or `v.get(i)` | Access by index |
| `len(lst)` | `v.len()` | Length |
| `lst.clear()` | `v.clear();` | Remove all |
| `x in lst` | `v.contains(&x)` | Check membership |
| `lst.extend(other)` | `v.extend(other);` | Extend with iterator |

### Dicts vs HashMap

| Python | Rust | Notes |
|--------|------|-------|
| `d = {}` | `let mut m = HashMap::new();` | Empty dict/map |
| `d = {"a": 1}` | Use `HashMap::from([...])` | With values |
| `d[key] = value` | `m.insert(key, value);` | Set value |
| `d[key]` | `m[&key]` | Get (panics if missing) |
| `d.get(key)` | `m.get(&key)` | Get (returns Option) |
| `key in d` | `m.contains_key(&key)` | Check key |
| `d.keys()` | `m.keys()` | Get keys |
| `d.values()` | `m.values()` | Get values |
| `d.items()` | `m.iter()` | Key-value pairs |
| `del d[key]` | `m.remove(&key);` | Remove key |

## Control Flow

| Python | Rust |
|--------|------|
| `if condition:` | `if condition {` |
| `elif other:` | `} else if other {` |
| `else:` | `} else {` |
| `x if condition else y` | `if condition { x } else { y }` |
| `while condition:` | `while condition {` |
| `for item in items:` | `for item in &items {` |
| `for i in range(n):` | `for i in 0..n {` |
| `for i, item in enumerate(items):` | `for (i, item) in items.iter().enumerate() {` |
| `break` | `break;` |
| `continue` | `continue;` |
| `return x` | `return x;` or just `x` (last expression) |

## Functions

| Python | Rust |
|--------|------|
| `def func():` | `fn func() {` |
| `def func(x):` | `fn func(x: Type) {` (must specify type) |
| `def func(x: int) -> int:` | `fn func(x: i32) -> i32 {` |
| `def func(*args):` | Use slice `&[T]` |
| `def func(**kwargs):` | No direct equivalent |
| `lambda x: x * 2` | `\|x\| x * 2` |
| `return None` | `return;` or no return |

## String Operations

| Python | Rust | Notes |
|--------|------|-------|
| `s = "hello"` | `let s = "hello";` | String literal (&str) |
| `s = str(x)` | `let s = x.to_string();` | Convert to string |
| `f"Hello {name}"` | `format!("Hello {}", name)` | String formatting |
| `s.upper()` | `s.to_uppercase()` | Uppercase |
| `s.lower()` | `s.to_lowercase()` | Lowercase |
| `s.split()` | `s.split_whitespace()` | Split on whitespace |
| `s.split(sep)` | `s.split(sep)` | Split on separator |
| `sep.join(items)` | `items.join(sep)` | Join strings |
| `s.strip()` | `s.trim()` | Remove whitespace |
| `s.startswith(prefix)` | `s.starts_with(prefix)` | Check prefix |
| `s.endswith(suffix)` | `s.ends_with(suffix)` | Check suffix |
| `s.replace(old, new)` | `s.replace(old, new)` | Replace substring |
| `len(s)` | `s.len()` | Length in bytes |
| `s[i]` | `s.chars().nth(i)` | Get character |

## List Comprehensions vs Iterators

| Python | Rust |
|--------|------|
| `[x * 2 for x in items]` | `items.iter().map(\|x\| x * 2).collect()` |
| `[x for x in items if x > 0]` | `items.iter().filter(\|&x\| x > 0).copied().collect()` |
| `[f(x) for x in items if p(x)]` | `items.iter().filter(\|x\| p(x)).map(\|x\| f(x)).collect()` |
| `sum(items)` | `items.iter().sum()` |
| `max(items)` | `items.iter().max()` |
| `min(items)` | `items.iter().min()` |
| `any(condition for x in items)` | `items.iter().any(\|x\| condition)` |
| `all(condition for x in items)` | `items.iter().all(\|x\| condition)` |
| `enumerate(items)` | `items.iter().enumerate()` |
| `zip(list1, list2)` | `list1.iter().zip(list2.iter())` |

## File I/O

| Python | Rust |
|--------|------|
| `with open(path) as f:` | `let f = File::open(path)?;` |
| `  data = f.read()` | `let mut data = String::new(); f.read_to_string(&mut data)?;` |
| `Path(path).read_text()` | `fs::read_to_string(path)?;` |
| `Path(path).write_text(data)` | `fs::write(path, data)?;` |
| `Path(path).exists()` | `Path::new(path).exists()` |
| `os.path.join(a, b)` | `Path::new(a).join(b)` |

## Error Handling

| Python | Rust |
|--------|------|
| `try:` | `match result {` |
| `  risky()` | `  Ok(val) => { /* use val */ }` |
| `except Error as e:` | `  Err(e) => { /* handle error */ }` |
| `  handle(e)` | `}` |
| `raise Exception(msg)` | `return Err(Error::new(msg));` |
| `raise` | `panic!()` (only for unrecoverable) |
| `assert condition` | `assert!(condition);` |

## Classes vs Structs

| Python | Rust |
|--------|------|
| `class Person:` | `struct Person {` |
| `  def __init__(self, name):` | `impl Person { fn new(name: String) -> Self {` |
| `    self.name = name` | `  Person { name }` |
| `  def greet(self):` | `fn greet(&self) {` |
| `    print(f"Hi {self.name}")` | `  println!("Hi {}", self.name);` |
| `@property` | Just a method (no properties) |
| `@staticmethod` | Associated function: `impl Type { fn name() {} }` |
| `@classmethod` | Associated function with Self |

## Common Patterns

### Iteration

```python
# Python
for item in items:
    print(item)

for i, item in enumerate(items):
    print(f"{i}: {item}")
```

```rust
// Rust
for item in &items {
    println!("{}", item);
}

for (i, item) in items.iter().enumerate() {
    println!("{}: {}", i, item);
}
```

### Filtering and Mapping

```python
# Python
result = [x * 2 for x in numbers if x > 0]
```

```rust
// Rust
let result: Vec<_> = numbers.iter()
    .filter(|&&x| x > 0)
    .map(|&x| x * 2)
    .collect();
```

### Reading JSON

```python
# Python
import json

with open("data.json") as f:
    data = json.load(f)
```

```rust
// Rust
use serde_json;
use std::fs;

let data: serde_json::Value =
    serde_json::from_str(&fs::read_to_string("data.json")?)?;
```

### HTTP Requests

```python
# Python
import requests

response = requests.get("https://api.example.com/data")
data = response.json()
```

```rust
// Rust (async with reqwest)
use reqwest;

let response = reqwest::get("https://api.example.com/data").await?;
let data: serde_json::Value = response.json().await?;
```

## Async/Await

| Python | Rust |
|--------|------|
| `async def func():` | `async fn func() {` |
| `await coroutine()` | `coroutine().await` |
| `asyncio.gather(*tasks)` | `tokio::join!(task1, task2, ...)` |
| `asyncio.create_task(coro)` | `tokio::spawn(async_block)` |
| `asyncio.run(main())` | `#[tokio::main] async fn main()` |
| `asyncio.sleep(1)` | `tokio::time::sleep(Duration::from_secs(1)).await` |

## Important Rust Concepts Not in Python

### Ownership

```rust
// Value has one owner
let s1 = String::from("hello");
let s2 = s1;  // s1 is moved, no longer valid

// To use both, borrow or clone
let s1 = String::from("hello");
let s2 = &s1;  // Borrow
let s3 = s1.clone();  // Clone
```

### Option Type

```rust
// Instead of None checks
let maybe_value: Option<i32> = Some(5);

match maybe_value {
    Some(v) => println!("Value: {}", v),
    None => println!("No value"),
}

// Or
if let Some(v) = maybe_value {
    println!("Value: {}", v);
}
```

### Result Type

```rust
// Instead of try/except
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Use with ?
let result = divide(10.0, 2.0)?;
```

## Common Crates (like pip packages)

| Use Case | Python Package | Rust Crate |
|----------|----------------|------------|
| **Serialization** | `json`, `pickle` | `serde`, `serde_json` |
| **HTTP client** | `requests` | `reqwest` |
| **HTTP server** | `flask`, `fastapi` | `actix-web`, `axum` |
| **Async runtime** | `asyncio` | `tokio`, `async-std` |
| **CLI parsing** | `argparse`, `click` | `clap`, `structopt` |
| **Regex** | `re` | `regex` |
| **Dates/times** | `datetime` | `chrono` |
| **Random** | `random` | `rand` |
| **Testing** | `pytest` | Built-in `cargo test` |
| **Logging** | `logging` | `log`, `env_logger` |
| **Environment** | `os.environ` | `std::env` |

## Type Conversions

```rust
// String conversions
let s: String = "hello".to_string();
let s: String = String::from("hello");
let slice: &str = &s;  // String to &str

// Number conversions
let x: i32 = 42;
let y: f64 = x as f64;  // Cast
let z: i64 = x.into();  // Into trait

// Parsing
let num: i32 = "42".parse()?;
let num = "42".parse::<i32>()?;

// To string
let s = 42.to_string();
let s = format!("{}", 42);
```

## Macro Equivalents

| Python | Rust |
|--------|------|
| `print("Hello")` | `println!("Hello");` |
| `print(f"x = {x}")` | `println!("x = {}", x);` |
| `print(x, y, z)` | `println!("{} {} {}", x, y, z);` |
| `assert condition` | `assert!(condition);` |
| `assert x == y` | `assert_eq!(x, y);` |

## Quick Tips

1. **Use `&str` for parameters**, `String` for owned data
2. **`?` operator** propagates errors (like Python exception propagation)
3. **`.collect()`** materializes iterators (like list comprehensions)
4. **Match exhaustiveness** - compiler ensures you handle all cases
5. **Immutable by default** - add `mut` when you need mutation
6. **No null** - use `Option<T>` instead
7. **Explicit error handling** - `Result<T, E>` instead of exceptions

---

**Remember**: When stuck, check the Rust docs or ask on Discord! The community is very helpful to Python developers making the transition.
