# Quick Reference: C/C++ to Rust

A side-by-side comparison cheat sheet for C/C++ developers learning Rust.

## Basic Syntax

### Variables

| C/C++ | Rust | Notes |
|-------|------|-------|
| `int x = 5;` | `let x = 5;` | Immutable by default |
| `int x = 5; x = 6;` | `let mut x = 5; x = 6;` | Explicit `mut` for mutability |
| `const int MAX = 100;` | `const MAX: i32 = 100;` | Compile-time constant |
| `static int counter = 0;` | `static COUNTER: i32 = 0;` | Global variable |
| `auto x = 5;` | `let x = 5;` | Type inference |
| `int x;` | N/A | Must initialize in Rust |

### Types

| C/C++ | Rust | Notes |
|-------|------|-------|
| `int8_t, int16_t, int32_t, int64_t` | `i8, i16, i32, i64` | Signed integers |
| `uint8_t, uint16_t, uint32_t, uint64_t` | `u8, u16, u32, u64` | Unsigned integers |
| `size_t` | `usize` | Pointer-sized unsigned |
| `ssize_t` | `isize` | Pointer-sized signed |
| `float` | `f32` | 32-bit float |
| `double` | `f64` | 64-bit float |
| `bool` | `bool` | Boolean |
| `char` | `char` | Unicode scalar value (4 bytes!) |
| `void` | `()` | Unit type |

### Functions

| C/C++ | Rust |
|-------|------|
| `int add(int a, int b) { return a + b; }` | `fn add(a: i32, b: i32) -> i32 { a + b }` |
| `void print() { }` | `fn print() { }` |
| `int* get_value()` | `fn get_value() -> Box<i32>` |
| `void process(const int& x)` | `fn process(x: &i32)` |
| `void modify(int& x)` | `fn modify(x: &mut i32)` |

### Control Flow

| C/C++ | Rust |
|-------|------|
| `if (x > 0) { }` | `if x > 0 { }` |
| `if (x > 0) { } else { }` | `if x > 0 { } else { }` |
| `int y = x > 0 ? 1 : 0;` | `let y = if x > 0 { 1 } else { 0 };` |
| `while (x > 0) { }` | `while x > 0 { }` |
| `for (int i = 0; i < 10; i++) { }` | `for i in 0..10 { }` |
| `for (auto& item : vec) { }` | `for item in &vec { }` |
| `switch (x) { case 1: break; }` | `match x { 1 => {}, _ => {} }` |

## Memory Management

### Stack Allocation

| C/C++ | Rust |
|-------|------|
| `int x = 5;` | `let x = 5;` |
| `int arr[10];` | `let arr = [0; 10];` |
| `struct Point { int x, y; }; Point p = {1, 2};` | `struct Point { x: i32, y: i32 } let p = Point { x: 1, y: 2 };` |

### Heap Allocation

| C/C++ | Rust |
|-------|------|
| `int* p = (int*)malloc(sizeof(int));` | `let p = Box::new(5);` |
| `free(p);` | Automatic (`Drop` trait) |
| `int* p = new int(5);` | `let p = Box::new(5);` |
| `delete p;` | Automatic |
| `int* arr = new int[10];` | `let arr = vec![0; 10];` |
| `delete[] arr;` | Automatic |

### Smart Pointers

| C/C++ | Rust | Notes |
|-------|------|-------|
| `std::unique_ptr<T>` | `Box<T>` | Exclusive ownership |
| `std::shared_ptr<T>` (single-thread) | `Rc<T>` | Shared ownership |
| `std::shared_ptr<T>` (multi-thread) | `Arc<T>` | Atomic shared ownership |
| `std::weak_ptr<T>` | `Weak<T>` | Non-owning reference |
| `T*` (raw pointer) | `*const T` / `*mut T` | Requires `unsafe` |

### References

| C/C++ | Rust |
|-------|------|
| `int& ref = x;` | `let ref_x = &x;` |
| `const int& ref = x;` | `let ref_x = &x;` |
| `int& ref = x; ref = 5;` | `let ref_x = &mut x; *ref_x = 5;` |
| `int* const p = &x;` | `let p = &x;` |
| `const int* p = &x;` | `let p = &x;` |
| `int* p = &x;` | `let p = &mut x;` |

## Strings

| C/C++ | Rust |
|-------|------|
| `const char* s = "hello";` | `let s = "hello";` (type: `&str`) |
| `char s[] = "hello";` | `let s = String::from("hello");` |
| `std::string s = "hello";` | `let s = String::from("hello");` |
| `std::string s("hello");` | `let s = "hello".to_string();` |
| `s.c_str()` | `s.as_str()` |
| `s.length()` | `s.len()` |
| `s.empty()` | `s.is_empty()` |
| `s + " world"` | `s + " world"` or `format!("{} world", s)` |
| `s.substr(0, 5)` | `&s[0..5]` |
| `s.find("ll")` | `s.find("ll")` returns `Option<usize>` |

## Collections

### Vectors/Arrays

| C/C++ | Rust |
|-------|------|
| `std::vector<int> v;` | `let mut v = Vec::new();` |
| `std::vector<int> v = {1, 2, 3};` | `let v = vec![1, 2, 3];` |
| `v.push_back(4);` | `v.push(4);` |
| `v.pop_back();` | `v.pop();` |
| `v.size()` | `v.len()` |
| `v.empty()` | `v.is_empty()` |
| `v[0]` | `v[0]` |
| `v.at(0)` | `v.get(0)` returns `Option<&T>` |
| `v.clear()` | `v.clear()` |

### Hash Maps

| C/C++ | Rust |
|-------|------|
| `std::map<K, V> m;` | `use std::collections::BTreeMap; let mut m = BTreeMap::new();` |
| `std::unordered_map<K, V> m;` | `use std::collections::HashMap; let mut m = HashMap::new();` |
| `m[key] = value;` | `m.insert(key, value);` |
| `m[key]` | `m[&key]` (panics if missing) or `m.get(&key)` returns `Option` |
| `m.find(key)` | `m.get(&key)` |
| `m.erase(key)` | `m.remove(&key)` |
| `m.count(key)` | `m.contains_key(&key)` |

### Sets

| C/C++ | Rust |
|-------|------|
| `std::set<T> s;` | `use std::collections::BTreeSet; let mut s = BTreeSet::new();` |
| `std::unordered_set<T> s;` | `use std::collections::HashSet; let mut s = HashSet::new();` |
| `s.insert(value);` | `s.insert(value);` |
| `s.erase(value);` | `s.remove(&value);` |
| `s.count(value)` | `s.contains(&value)` |

## Error Handling

| C/C++ | Rust |
|-------|------|
| `if (result < 0) { /* error */ }` | `match result { Ok(v) => {}, Err(e) => {} }` |
| `throw std::runtime_error("error");` | `return Err("error".to_string());` |
| `try { } catch (const std::exception& e) { }` | `match result { Ok(_) => {}, Err(e) => {} }` |
| `NULL` | `None` |
| `nullptr` | `None` |
| `if (ptr != NULL) { *ptr }` | `if let Some(val) = option { val }` |

## Structs and Classes

### Defining

| C/C++ | Rust |
|-------|------|
| `struct Point { int x; int y; };` | `struct Point { x: i32, y: i32 }` |
| `class Point { public: int x, y; };` | `struct Point { x: i32, y: i32 }` |
| `class Point { private: int x; };` | `struct Point { x: i32 }` (fields private by default) |

### Methods

| C/C++ | Rust |
|-------|------|
| `class Point { int get_x() { return x; } };` | `impl Point { fn get_x(&self) -> i32 { self.x } }` |
| `void set_x(int val) { x = val; }` | `fn set_x(&mut self, val: i32) { self.x = val; }` |
| `Point() : x(0), y(0) { }` | `fn new() -> Self { Point { x: 0, y: 0 } }` |
| `~Point() { }` | `impl Drop for Point { fn drop(&mut self) { } }` |

### Inheritance vs Composition

| C/C++ | Rust |
|-------|------|
| `class Dog : public Animal { };` | No inheritance, use traits + composition |
| `virtual void speak() = 0;` | `trait Animal { fn speak(&self); }` |
| `void speak() override { }` | `impl Animal for Dog { fn speak(&self) { } }` |

## Generics/Templates

| C/C++ | Rust |
|-------|------|
| `template<typename T> T max(T a, T b)` | `fn max<T: Ord>(a: T, b: T) -> T` |
| `template<typename T> class Vec { };` | `struct Vec<T> { }` |
| `template<typename T> requires Addable<T>` | `fn add<T: Add>(a: T, b: T)` (C++20 concepts) |

## Concurrency

### Threads

| C/C++ | Rust |
|-------|------|
| `std::thread t([]() { });` | `thread::spawn(\|\| { });` |
| `t.join();` | `t.join().unwrap();` |
| `std::mutex<int> m;` | `let m = Mutex::new(0);` |
| `std::lock_guard<std::mutex<int>> lock(m);` | `let lock = m.lock().unwrap();` |
| `std::atomic<int> a;` | `use std::sync::atomic::AtomicI32; let a = AtomicI32::new(0);` |

### Channels

| C/C++ | Rust |
|-------|------|
| N/A (varies by library) | `let (tx, rx) = mpsc::channel();` |
| N/A | `tx.send(value).unwrap();` |
| N/A | `let val = rx.recv().unwrap();` |

## Macros

| C/C++ | Rust |
|-------|------|
| `#define MAX(a, b) ((a) > (b) ? (a) : (b))` | `macro_rules! max { ($a:expr, $b:expr) => { if $a > $b { $a } else { $b } } }` |
| `#ifdef DEBUG` | `#[cfg(debug_assertions)]` |
| `assert(x > 0);` | `assert!(x > 0);` |
| `printf("x = %d\n", x);` | `println!("x = {}", x);` |
| `fprintf(stderr, "error\n");` | `eprintln!("error");` |

## Common Operations

### File I/O

| C/C++ | Rust |
|-------|------|
| `FILE* f = fopen("file.txt", "r");` | `let f = File::open("file.txt")?;` |
| `fclose(f);` | Automatic (Drop) |
| `std::ifstream f("file.txt");` | `let f = File::open("file.txt")?;` |
| `std::ofstream f("file.txt");` | `let f = File::create("file.txt")?;` |
| `getline(f, line);` | `use std::io::BufRead; reader.read_line(&mut line)?;` |

### Command Line Arguments

| C/C++ | Rust |
|-------|------|
| `int main(int argc, char* argv[])` | `use std::env; let args: Vec<String> = env::args().collect();` |
| `argv[1]` | `args[1]` or `args.get(1)` |

### Casting

| C/C++ | Rust |
|-------|------|
| `(int)x` | `x as i32` |
| `static_cast<int>(x)` | `x as i32` |
| `reinterpret_cast<int*>(p)` | `p as *const i32` (unsafe) |
| `dynamic_cast<Derived*>(p)` | Pattern matching on enums |

## Memory Safety

| C/C++ Undefined Behavior | Rust Prevention |
|---------------------------|-----------------|
| Use after free | Ownership system prevents |
| Double free | Ownership (can't free twice) |
| Null pointer dereference | No null pointers, use `Option<T>` |
| Dangling pointers | Lifetime checking |
| Buffer overflow | Bounds checking |
| Data races | Borrow checker + Send/Sync traits |
| Iterator invalidation | Borrow checker prevents |

## Common Patterns

### NULL Checking

```cpp
// C/C++
int* ptr = get_value();
if (ptr != NULL) {
    *ptr = 42;
}
```

```rust
// Rust
if let Some(value) = get_value() {
    // use value
}

// Or
match get_value() {
    Some(value) => { /* use value */ },
    None => { /* handle missing */ },
}
```

### Error Propagation

```cpp
// C++
int process() {
    int result = step1();
    if (result < 0) return result;

    result = step2();
    if (result < 0) return result;

    return 0;
}
```

```rust
// Rust
fn process() -> Result<(), Error> {
    step1()?;
    step2()?;
    Ok(())
}
```

### RAII

```cpp
// C++
{
    std::lock_guard<std::mutex> lock(m);
    // Critical section
} // Automatically unlocked
```

```rust
// Rust
{
    let lock = m.lock().unwrap();
    // Critical section
} // Automatically unlocked (Drop)
```

### Iteration

```cpp
// C++
for (auto it = vec.begin(); it != vec.end(); ++it) {
    std::cout << *it << "\n";
}
```

```rust
// Rust
for item in &vec {
    println!("{}", item);
}
```

## Quick Translation Examples

### Example 1: Simple Function

```cpp
// C++
int add(int a, int b) {
    return a + b;
}
```

```rust
// Rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Example 2: Struct with Methods

```cpp
// C++
class Rectangle {
    int width, height;
public:
    Rectangle(int w, int h) : width(w), height(h) {}

    int area() const {
        return width * height;
    }
};
```

```rust
// Rust
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        Rectangle { width, height }
    }

    fn area(&self) -> i32 {
        self.width * self.height
    }
}
```

### Example 3: Vector Processing

```cpp
// C++
std::vector<int> filter_even(const std::vector<int>& numbers) {
    std::vector<int> result;
    for (int n : numbers) {
        if (n % 2 == 0) {
            result.push_back(n);
        }
    }
    return result;
}
```

```rust
// Rust
fn filter_even(numbers: &Vec<i32>) -> Vec<i32> {
    numbers.iter()
        .filter(|&&n| n % 2 == 0)
        .copied()
        .collect()
}
```

## Key Differences Summary

1. **Ownership**: Rust tracks who owns data (C/C++ doesn't)
2. **No null**: Use `Option<T>` instead
3. **No exceptions**: Use `Result<T, E>` instead
4. **Immutable by default**: Add `mut` to make mutable
5. **No inheritance**: Use traits and composition
6. **Move by default**: Not copy (unlike C++)
7. **Expression-based**: `if`, `match` return values
8. **Pattern matching**: More powerful than `switch`
9. **Lifetimes**: Explicit borrow relationships
10. **Macros**: Hygenic macros, not text substitution

## When to Use What

| Use Case | C/C++ | Rust |
|----------|-------|------|
| System programming | ✓ | ✓ |
| Embedded (bare metal) | ✓ | ✓ (`#![no_std]`) |
| Game engines | ✓ | Growing |
| Operating systems | ✓ (C) | Growing |
| Web services | △ | ✓ |
| CLI tools | △ | ✓ |
| Need GC-free performance | ✓ | ✓ |
| Safety-critical | C (with tools) | ✓ |
| Large legacy codebase | ✓ | Gradual migration possible (FFI) |
| Need mature ecosystem | ✓ | Growing rapidly |

---

**Remember:** When translating C/C++ to Rust, don't just convert syntax—embrace Rust's ownership model and idioms for cleaner, safer code.
