# Fundamentals Comparison: C/C++ vs Rust

This guide provides a detailed comparison of fundamental concepts between C/C++ and Rust. If you're coming from C/C++, you already understand low-level programming—this guide shows you how Rust achieves the same goals with different (and safer) mechanisms.

## 1. Memory Management

### C: Manual malloc/free

```c
// C - Manual memory management
#include <stdlib.h>
#include <string.h>

typedef struct {
    char* name;
    int age;
} Person;

Person* create_person(const char* name, int age) {
    Person* p = malloc(sizeof(Person));
    if (p == NULL) return NULL;

    p->name = malloc(strlen(name) + 1);
    if (p->name == NULL) {
        free(p);
        return NULL;
    }

    strcpy(p->name, name);
    p->age = age;
    return p;
}

void free_person(Person* p) {
    if (p != NULL) {
        free(p->name);
        free(p);
    }
}

// Usage
Person* p = create_person("Alice", 30);
// ... use p ...
free_person(p);  // Don't forget!
// What if you use p after free? Use-after-free!
// What if you forget to free? Memory leak!
// What if you free twice? Crash!
```

**Problems:**
- Easy to forget to free
- Easy to use after free
- Easy to double-free
- No compiler help

### C++: RAII with smart pointers

```cpp
// Modern C++ - RAII
#include <memory>
#include <string>

class Person {
    std::string name;
    int age;

public:
    Person(std::string name, int age) : name(std::move(name)), age(age) {}
    // Destructor automatically called
};

// Automatic cleanup
{
    auto p = std::make_unique<Person>("Alice", 30);
    // ... use p ...
} // p automatically freed

// But you can still shoot yourself
Person* raw = new Person("Bob", 25);
// Forgot to delete? Memory leak!

// Or get raw pointers from smart pointers
auto p = std::make_unique<Person>("Charlie", 35);
Person* raw = p.get();
p.reset();  // Freed!
// raw is now dangling - use-after-free possible!
```

**Better, but:**
- Can still leak if you use `new` directly
- Can still get dangling pointers from `.get()`
- No protection against data races

### Rust: Ownership system

```rust
// Rust - Ownership enforced by compiler
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }
}

fn main() {
    let p = Person::new("Alice".to_string(), 30);
    // ... use p ...
    // Automatically dropped, no other option

    // Can't forget to free - compiler does it
    // Can't use after free - compiler prevents it
    // Can't double-free - ownership rules prevent it
}

// Moving ownership
fn take_ownership(p: Person) {
    println!("{}", p.name);
    // p dropped here
}

fn main() {
    let p = Person::new("Bob".to_string(), 25);
    take_ownership(p);
    // println!("{}", p.name);  // ERROR: value moved
    // Compiler prevents use-after-move
}
```

**Key differences:**
- **Ownership**: Every value has exactly one owner
- **Move semantics by default**: Passing a value transfers ownership
- **Automatic cleanup**: When owner goes out of scope, value is dropped
- **Compiler enforced**: Can't violate these rules

## 2. Pointers vs References

### C/C++: Raw pointers

```cpp
// C++ pointers can do anything
int x = 42;
int* ptr = &x;

// Mutable pointer to mutable data
*ptr = 43;

// Can point to freed memory
int* dangling = new int(5);
delete dangling;
// dangling still points to freed memory!

// NULL pointers everywhere
int* maybe = get_value();
if (maybe != NULL) {
    *maybe = 10;
}
// Forgot to check? Segfault!

// Multiple mutable aliases
int* p1 = &x;
int* p2 = &x;
*p1 = 1;
*p2 = 2;
// Both point to same data, no protection
```

### Rust: Safe references

```rust
// Rust references have strict rules
let mut x = 42;

// Immutable reference
let r1 = &x;
let r2 = &x;
println!("{} {}", r1, r2);  // Multiple immutable refs OK

// Mutable reference
let r3 = &mut x;
*r3 = 43;

// But can't have both at once!
let r1 = &x;
let r2 = &mut x;  // ERROR: cannot borrow as mutable while borrowed as immutable

// No dangling references
fn dangle() -> &i32 {
    let x = 5;
    &x  // ERROR: returns reference to local variable
}

// No null references - they don't exist
// Use Option instead (covered later)
```

**Borrowing rules (enforced at compile time):**
1. You can have any number of immutable references (`&T`)
2. OR exactly one mutable reference (`&mut T`)
3. References must always be valid (no dangling)

**This prevents:**
- Data races (impossible to have concurrent mutation)
- Use-after-free (can't outlive the data)
- Iterator invalidation (can't modify while iterating)

## 3. Smart Pointers

### C++: unique_ptr and shared_ptr

```cpp
// C++ unique_ptr - exclusive ownership
#include <memory>

auto p1 = std::make_unique<int>(42);
// auto p2 = p1;  // ERROR: can't copy
auto p2 = std::move(p1);  // OK: transfer ownership
// p1 is now nullptr

// C++ shared_ptr - reference counted
auto s1 = std::make_shared<int>(42);
auto s2 = s1;  // OK: increment reference count
// Both s1 and s2 point to same data
// Freed when last shared_ptr is destroyed

// But you can still get raw pointers
int* raw = s1.get();
s1.reset();
s2.reset();
// raw is now dangling!

// Thread safety issues
auto s = std::make_shared<std::vector<int>>();
std::thread t1([s]() { s->push_back(1); });  // Data race!
std::thread t2([s]() { s->push_back(2); });  // Undefined behavior!
```

### Rust: Box, Rc, Arc

```rust
// Rust Box<T> - heap allocation, unique ownership
let b1 = Box::new(42);
// let b2 = b1;  // Moves, b1 is now invalid
// println!("{}", b1);  // ERROR: value moved

// Rust Rc<T> - reference counted (single-threaded)
use std::rc::Rc;

let r1 = Rc::new(42);
let r2 = Rc::clone(&r1);  // Increment reference count
println!("{} {}", r1, r2);  // Both valid

// Arc<T> - atomic reference counted (thread-safe)
use std::sync::Arc;
use std::thread;

let a = Arc::new(vec![1, 2, 3]);
let a1 = Arc::clone(&a);
let a2 = Arc::clone(&a);

thread::spawn(move || {
    println!("{:?}", a1);
});

thread::spawn(move || {
    println!("{:?}", a2);
});

// But Arc doesn't allow mutation!
let a = Arc::new(vec![1, 2, 3]);
// a.push(4);  // ERROR: cannot mutate through Arc

// Need Arc<Mutex<T>> for shared mutable state
use std::sync::Mutex;

let data = Arc::new(Mutex::new(vec![1, 2, 3]));
let d1 = Arc::clone(&data);
let d2 = Arc::clone(&data);

thread::spawn(move || {
    d1.lock().unwrap().push(4);  // Safe mutation
});

thread::spawn(move || {
    d2.lock().unwrap().push(5);  // Mutex prevents data race
});
```

**Comparison:**

| C++ | Rust | Notes |
|-----|------|-------|
| `std::unique_ptr<T>` | `Box<T>` | Exclusive ownership, heap allocation |
| `std::shared_ptr<T>` (single-thread) | `Rc<T>` | Reference counted, single-threaded |
| `std::shared_ptr<T>` (multi-thread) | `Arc<T>` | Atomic reference counted |
| `std::shared_ptr<std::mutex<T>>` | `Arc<Mutex<T>>` | Shared mutable state |

**Key difference:** Rust's type system prevents you from getting raw pointers that outlive the smart pointer.

## 4. NULL vs Option

### C: NULL pointers

```c
// C - NULL can appear anywhere
int* get_value(int flag) {
    if (flag > 0) {
        int* result = malloc(sizeof(int));
        *result = 42;
        return result;
    }
    return NULL;  // Indicate failure
}

// Usage
int* val = get_value(0);
if (val != NULL) {
    printf("%d\n", *val);
    free(val);
}
// Forgot to check? Segfault!
```

### C++: std::optional (C++17)

```cpp
// Modern C++ - optional
#include <optional>

std::optional<int> get_value(int flag) {
    if (flag > 0) {
        return 42;
    }
    return std::nullopt;
}

// Usage
auto val = get_value(0);
if (val.has_value()) {
    std::cout << *val << std::endl;
}

// But raw pointers still exist
int* ptr = get_pointer();
// Might be NULL, might not, who knows?
```

### Rust: Option<T>

```rust
// Rust - no null pointers exist
fn get_value(flag: i32) -> Option<i32> {
    if flag > 0 {
        Some(42)
    } else {
        None
    }
}

// Must handle both cases
match get_value(0) {
    Some(val) => println!("{}", val),
    None => println!("No value"),
}

// Or use if let
if let Some(val) = get_value(1) {
    println!("{}", val);
}

// Or unwrap (panics if None)
let val = get_value(1).unwrap();

// Or provide default
let val = get_value(0).unwrap_or(0);

// Chaining operations
let result = get_value(1)
    .map(|x| x * 2)
    .filter(|x| x > &50)
    .unwrap_or(0);
```

**Key difference:** Rust has no null pointers. `Option<T>` makes absence explicit in the type system, and the compiler forces you to handle both cases.

## 5. Error Handling

### C: errno and return codes

```c
// C - errno and return codes
#include <stdio.h>
#include <errno.h>

int read_file(const char* path, char* buffer, size_t size) {
    FILE* f = fopen(path, "r");
    if (f == NULL) {
        return -1;  // Error, check errno
    }

    size_t read = fread(buffer, 1, size, f);
    if (read != size) {
        if (ferror(f)) {
            fclose(f);
            return -1;  // Error
        }
    }

    fclose(f);
    return 0;  // Success
}

// Usage
char buffer[1024];
if (read_file("data.txt", buffer, 1024) < 0) {
    fprintf(stderr, "Error: %s\n", strerror(errno));
}
```

**Problems:**
- Easy to forget to check return codes
- errno is global state (not thread-safe)
- No type safety

### C++: Exceptions

```cpp
// C++ - exceptions
#include <fstream>
#include <stdexcept>

void read_file(const std::string& path) {
    std::ifstream file(path);
    if (!file) {
        throw std::runtime_error("Failed to open file");
    }

    // ... read file ...

    // What if this throws?
    process_data();  // Might throw, but not visible in signature
}

// Usage
try {
    read_file("data.txt");
} catch (const std::exception& e) {
    std::cerr << "Error: " << e.what() << std::endl;
}
```

**Problems:**
- Exceptions are invisible (not in function signature)
- Performance overhead
- Can't tell which functions throw
- Exception safety is hard

### Rust: Result<T, E>

```rust
// Rust - Result type
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;  // ? propagates error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Usage: must handle both cases
match read_file("data.txt") {
    Ok(contents) => println!("{}", contents),
    Err(e) => eprintln!("Error: {}", e),
}

// Or propagate error
fn process() -> Result<(), io::Error> {
    let contents = read_file("data.txt")?;  // Returns early if error
    println!("{}", contents);
    Ok(())
}

// Or unwrap (panics on error)
let contents = read_file("data.txt").unwrap();

// Or provide default
let contents = read_file("data.txt").unwrap_or_else(|_| String::new());
```

**Advantages:**
- Errors are in the type signature
- Compiler forces you to handle errors
- No runtime overhead (zero-cost)
- Can't forget to check errors

## 6. RAII (Resource Acquisition Is Initialization)

### C++: RAII is optional

```cpp
// C++ RAII - works if you use it
class FileHandle {
    FILE* file;
public:
    FileHandle(const char* path) : file(fopen(path, "r")) {
        if (!file) throw std::runtime_error("Failed to open");
    }

    ~FileHandle() {
        if (file) fclose(file);  // Automatic cleanup
    }

    // Prevent copying
    FileHandle(const FileHandle&) = delete;
    FileHandle& operator=(const FileHandle&) = delete;
};

// Usage
{
    FileHandle f("data.txt");
    // ... use f ...
}  // Automatically closed

// But you can still use raw FILE*
FILE* f = fopen("data.txt", "r");
// ... use f ...
// Forgot to fclose? Resource leak!
```

### Rust: RAII is the only way

```rust
// Rust - RAII is enforced
use std::fs::File;

fn main() {
    {
        let f = File::open("data.txt").unwrap();
        // ... use f ...
    }  // File automatically closed (Drop called)

    // Can't forget to close
    // Can't close twice
    // Can't use after close
}

// Custom Drop implementation
struct Resource {
    id: i32,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Cleaning up resource {}", self.id);
    }
}

fn main() {
    let r1 = Resource { id: 1 };
    let r2 = Resource { id: 2 };
    println!("Resources created");
}  // r2 dropped first, then r1 (LIFO order)

// Output:
// Resources created
// Cleaning up resource 2
// Cleaning up resource 1
```

**Drop order:**
- Variables are dropped in reverse order of creation (LIFO)
- Fields are dropped in declaration order
- Automatic, no way to forget

## 7. Templates vs Generics

### C++: Templates

```cpp
// C++ templates - duck typing
template<typename T>
T add(T a, T b) {
    return a + b;  // Assumes T has operator+
}

// Specialization
template<typename T>
class Vector {
    T* data;
    size_t size;
public:
    void push(T value) { /* ... */ }
    T get(size_t index) { return data[index]; }
};

// Template metaprogramming
template<int N>
struct Factorial {
    static const int value = N * Factorial<N-1>::value;
};

template<>
struct Factorial<0> {
    static const int value = 1;
};

// Constraints (C++20)
template<typename T>
concept Addable = requires(T a, T b) {
    { a + b } -> std::same_as<T>;
};

template<Addable T>
T add_constrained(T a, T b) {
    return a + b;
}
```

**Issues:**
- Error messages are cryptic
- Compilation times can be slow
- No separation between interface and implementation

### Rust: Generics with traits

```rust
// Rust generics - trait bounds
use std::ops::Add;

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// Trait definition
trait Summary {
    fn summarize(&self) -> String;
}

// Generic function with trait bound
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// Multiple trait bounds
fn process<T: Summary + Clone>(item: T) {
    let copy = item.clone();
    println!("{}", copy.summarize());
}

// Where clauses for complex bounds
fn complex<T, U>(t: T, u: U)
where
    T: Summary + Clone,
    U: Summary + Copy,
{
    // ...
}

// Trait objects for dynamic dispatch
fn make_summary() -> Box<dyn Summary> {
    Box::new(Article {
        title: "News".to_string(),
        content: "Content".to_string(),
    })
}
```

**Advantages:**
- Clear trait bounds
- Better error messages
- Compile-time guarantees
- Explicit about capabilities

**Comparison:**

| Feature | C++ Templates | Rust Generics |
|---------|---------------|---------------|
| Type checking | Duck typing | Trait bounds required |
| Error messages | Cryptic | Clear |
| Constraints | Concepts (C++20) | Traits |
| Specialization | Yes | Limited |
| Metaprogramming | Yes (complex) | Macros |

## 8. Move Semantics

### C++: Move semantics (C++11+)

```cpp
// C++11 move semantics
#include <vector>
#include <string>

class Resource {
    std::vector<int> data;
public:
    // Move constructor
    Resource(Resource&& other) noexcept
        : data(std::move(other.data)) {
        // other.data is now empty
    }

    // Move assignment
    Resource& operator=(Resource&& other) noexcept {
        data = std::move(other.data);
        return *this;
    }
};

// Usage
Resource r1;
Resource r2 = std::move(r1);  // Explicit move
// r1 is in "valid but unspecified state"
// Can still use r1! Might be empty, might not

std::vector<std::string> vec;
vec.push_back(std::string("hello"));  // Automatic move

// But copies still happen by default
Resource r3 = r2;  // Copy! Might be expensive
```

### Rust: Move by default

```rust
// Rust - move semantics by default
struct Resource {
    data: Vec<i32>,
}

fn main() {
    let r1 = Resource { data: vec![1, 2, 3] };
    let r2 = r1;  // Move, not copy

    // println!("{:?}", r1.data);  // ERROR: value moved

    // r1 is gone, can't use it anymore
    println!("{:?}", r2.data);  // OK
}

// Explicit copy with Clone
#[derive(Clone)]
struct Data {
    value: i32,
}

fn main() {
    let d1 = Data { value: 42 };
    let d2 = d1.clone();  // Explicit copy

    println!("{}", d1.value);  // OK, d1 still valid
    println!("{}", d2.value);  // OK
}

// Copy trait for implicit copy (only for cheap types)
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;  // Implicit copy

    println!("{} {}", p1.x, p2.x);  // Both valid
}
```

**Key differences:**
- **C++**: Copy by default, move with `std::move`
- **Rust**: Move by default, copy with `.clone()` or `Copy` trait
- **C++**: Moved-from objects are "valid but unspecified"
- **Rust**: Moved-from variables are inaccessible (compiler error)

## 9. Const Correctness vs Immutability

### C++: const keyword

```cpp
// C++ const
const int x = 42;
// x = 43;  // ERROR

// Const pointers
const int* p1 = &x;      // Pointer to const int
int* const p2 = &x;      // Const pointer to int
const int* const p3 = &x; // Const pointer to const int

// Const methods
class Counter {
    int count;
public:
    int get() const { return count; }  // Const method
    void increment() { count++; }      // Non-const method
};

const Counter c;
c.get();        // OK
// c.increment();  // ERROR: can't call non-const method on const object

// But const is shallow
class Wrapper {
    int* data;
public:
    void modify() const {
        *data = 42;  // OK! const doesn't protect pointed-to data
    }
};

// mutable keyword
class Cache {
    mutable std::map<int, int> cache;
public:
    int get(int key) const {
        cache[key] = compute(key);  // OK: mutable allows modification
        return cache[key];
    }
};
```

### Rust: Immutability by default

```rust
// Rust - immutable by default
let x = 42;
// x = 43;  // ERROR: cannot assign twice to immutable variable

// Explicit mutability
let mut y = 42;
y = 43;  // OK

// References
let r1 = &x;      // Immutable reference
let r2 = &mut y;  // Mutable reference

// Can't have multiple mutable references
let r3 = &mut y;  // ERROR: cannot borrow as mutable more than once

// Structs
struct Counter {
    count: i32,
}

impl Counter {
    fn get(&self) -> i32 {
        self.count
    }

    fn increment(&mut self) {
        self.count += 1;
    }
}

let mut c = Counter { count: 0 };
c.increment();  // OK: c is mutable
println!("{}", c.get());

let immut = Counter { count: 0 };
// immut.increment();  // ERROR: cannot borrow as mutable

// Interior mutability with Cell/RefCell
use std::cell::RefCell;

struct Cache {
    data: RefCell<Vec<i32>>,
}

impl Cache {
    fn add(&self, value: i32) {  // Takes &self, not &mut self
        self.data.borrow_mut().push(value);  // Runtime borrow checking
    }
}
```

**Comparison:**

| C++ | Rust |
|-----|------|
| Mutable by default | Immutable by default |
| `const` keyword | `mut` keyword |
| Shallow const | Deep immutability |
| `mutable` for exceptions | `Cell`/`RefCell` for interior mutability |
| Const checked at compile time | Immutability checked at compile time |

## 10. Undefined Behavior

### C/C++: Easy to invoke UB

```cpp
// C++ undefined behavior examples
int* dangling() {
    int x = 5;
    return &x;  // UB: returning pointer to local
}

void use_after_free() {
    int* p = new int(42);
    delete p;
    std::cout << *p;  // UB: use after free
}

void buffer_overflow() {
    int arr[5];
    arr[10] = 42;  // UB: out of bounds
}

void data_race() {
    int x = 0;
    std::thread t1([&]() { x++; });
    std::thread t2([&]() { x++; });
    // UB: data race on x
}

void signed_overflow() {
    int x = INT_MAX;
    x++;  // UB: signed integer overflow
}

void null_deref() {
    int* p = nullptr;
    *p = 42;  // UB: null dereference
}
```

### Rust: UB is prevented

```rust
// Rust prevents these at compile time
fn dangling() -> &i32 {
    let x = 5;
    &x  // ERROR: returns reference to local variable
}

fn use_after_free() {
    let p = Box::new(42);
    drop(p);
    // println!("{}", p);  // ERROR: value used after move
}

fn buffer_overflow() {
    let arr = [0; 5];
    // arr[10];  // Panic at runtime (not UB), or bounds check optimized away if provably safe
}

fn data_race() {
    let mut x = 0;
    std::thread::spawn(|| {
        x += 1;  // ERROR: can't capture mutable reference across threads
    });
}

fn overflow() {
    let x: i32 = i32::MAX;
    let y = x.wrapping_add(1);  // Explicit wrapping
    let z = x.checked_add(1);   // Returns None on overflow
    // x + 1 panics in debug, wraps in release
}

fn null_deref() {
    let p: Option<i32> = None;
    // *p;  // ERROR: can't dereference Option
    match p {
        Some(val) => println!("{}", val),
        None => println!("No value"),
    }
}
```

**What Rust prevents:**
- Use-after-free (ownership)
- Double-free (ownership)
- Null pointer dereference (no null pointers)
- Dangling pointers (lifetime checking)
- Data races (borrow checker)
- Buffer overflows (bounds checking)
- Iterator invalidation (borrow checker)

**What Rust doesn't prevent:**
- Logic errors
- Deadlocks
- Memory leaks (you can leak with `Rc` cycles)
- Integer overflow (configurable: panic in debug, wrap in release)

## Summary

| Concept | C/C++ | Rust |
|---------|-------|------|
| Memory management | Manual (malloc/free/new/delete) | Automatic (ownership) |
| Default semantics | Copy | Move |
| Null handling | NULL everywhere | Option<T> |
| Error handling | errno/exceptions | Result<T, E> |
| Immutability | Mutable by default | Immutable by default |
| References | No restrictions | Strict borrowing rules |
| Generics | Templates (duck typing) | Traits (explicit bounds) |
| RAII | Optional | Mandatory |
| Undefined behavior | Easy to invoke | Hard to invoke (prevented) |
| Thread safety | Manual synchronization | Compiler enforced |

The biggest mental shift from C/C++ to Rust is accepting that the compiler knows better than you about memory safety. The borrow checker feels restrictive at first, but it's catching real bugs that would be runtime crashes in C/C++.
