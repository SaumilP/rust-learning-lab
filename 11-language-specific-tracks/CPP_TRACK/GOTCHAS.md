# Gotchas for C/C++ Developers

This guide covers surprising differences and common "wait, what?" moments when coming from C/C++ to Rust.

## 1. Move Semantics Are Fundamentally Different

### The Gotcha

In C++11+, `std::move` creates a moved-from object in "valid but unspecified state." In Rust, moved values are **gone**.

```cpp
// C++ - moved-from objects still exist
std::string s1 = "hello";
std::string s2 = std::move(s1);

// s1 still exists! You can even use it
std::cout << "Length: " << s1.length();  // Might be 0, might not
s1 = "new value";  // Can reassign
std::cout << s1;   // Works
```

```rust
// Rust - moved values are GONE
let s1 = String::from("hello");
let s2 = s1;  // s1 is moved

// println!("{}", s1);  // ERROR: value used after move
// s1 doesn't exist in the type system anymore
// Can't use it, can't assign to it, it's dead

// But you CAN shadow it (new variable with same name)
let s1 = String::from("new value");  // OK: new variable
println!("{}", s1);  // Works
```

**Why it matters:** In Rust, the compiler prevents use-after-move. In C++, you can compile code that uses moved-from objects (undefined behavior territory).

### Another Example

```cpp
// C++
std::vector<int> v1 = {1, 2, 3};
std::vector<int> v2 = std::move(v1);

if (!v1.empty()) {  // Might be true or false!
    v1.push_back(4);  // Might work!
}
```

```rust
// Rust
let v1 = vec![1, 2, 3];
let v2 = v1;

// if !v1.is_empty() {  // ERROR: can't use v1
//     v1.push(4);
// }
```

## 2. Mutable References Are Exclusive

### The Gotcha

In C/C++, you can have multiple pointers/references to the same mutable data. Rust doesn't allow this.

```cpp
// C++ - multiple mutable references are fine
int x = 42;
int& r1 = x;
int& r2 = x;

r1 = 10;
r2 = 20;  // Both modify x, no compiler error
```

```rust
// Rust - can't have multiple mutable references
let mut x = 42;
let r1 = &mut x;
// let r2 = &mut x;  // ERROR: cannot borrow `x` as mutable more than once

*r1 = 10;
// println!("{}", r2);
```

**The Rule:**
- Many immutable references (`&T`) **OR**
- Exactly one mutable reference (`&mut T`)
- **Never both at the same time**

**Why:** Prevents data races at compile time. If you can mutate, you have exclusive access.

### Practical Impact

```cpp
// C++ - Iterator invalidation (undefined behavior)
std::vector<int> v = {1, 2, 3};
for (int& item : v) {
    if (item > 1) {
        v.push_back(item * 2);  // UB: modifying while iterating!
    }
}
```

```rust
// Rust - won't compile!
let mut v = vec![1, 2, 3];
for item in &v {  // Immutable borrow
    if *item > 1 {
        // v.push(item * 2);  // ERROR: can't mutate while borrowed
    }
}

// Fix: collect changes first
let mut v = vec![1, 2, 3];
let to_add: Vec<i32> = v.iter()
    .filter(|&&item| item > 1)
    .map(|&item| item * 2)
    .collect();

v.extend(to_add);  // Now we can modify
```

## 3. Integer Overflow Behavior

### The Gotcha

In C/C++, signed integer overflow is **undefined behavior**. In Rust, it's **defined** but different in debug vs release.

```c
// C - signed overflow is UB
int x = INT_MAX;
x++;  // Undefined behavior! Might be INT_MIN, might be anything!

// Compiler can assume this never happens and optimize based on that
```

```cpp
// C++ - same, undefined behavior
int x = std::numeric_limits<int>::max();
x++;  // UB
```

```rust
// Rust - well-defined behavior
let mut x: i32 = i32::MAX;

// In DEBUG mode: panics (crashes with error message)
// In RELEASE mode: wraps to i32::MIN (two's complement)
x += 1;

// Explicit control over behavior
let y = x.wrapping_add(1);   // Always wraps
let z = x.checked_add(1);    // Returns None on overflow
let w = x.saturating_add(1); // Clamps to MAX
let v = x.overflowing_add(1); // Returns (result, did_overflow)
```

**Why it matters:**
- Debug builds catch overflow bugs
- Release builds have defined behavior (wrapping)
- You can choose the overflow strategy explicitly

### Unsigned is Different

```c
// C - unsigned overflow is defined (wraps)
unsigned int x = UINT_MAX;
x++;  // Defined: wraps to 0
```

```rust
// Rust - same in release, but panics in debug
let mut x: u32 = u32::MAX;
x += 1;  // Debug: panic, Release: wraps to 0
```

## 4. String vs &str Confusion

### The Gotcha

Rust has two string types, and choosing wrong causes allocation overhead or lifetime issues.

```cpp
// C++ - one type
std::string s1 = "hello";
std::string s2 = s1;  // Copy
const std::string& s3 = s1;  // Reference
```

```rust
// Rust - two types that interact
let s1: &str = "hello";        // String slice (borrowed)
let s2: String = s1.to_string(); // Owned string (heap allocated)
let s3: &str = &s2;            // Borrow from String

// "hello" is &str (lives in binary)
// String::from("hello") is String (heap allocated)
```

**When to use:**
- **`&str`** - Function parameters (unless you need ownership), string literals, slices
- **`String`** - When you need to own/modify the string, build dynamically, return new strings

**Common mistake:**

```rust
// ❌ BAD: Taking String when &str would work
fn print(s: String) {
    println!("{}", s);
}

let name = String::from("Alice");
print(name);
// name is moved, can't use anymore!

// ✅ GOOD: Take &str
fn print(s: &str) {
    println!("{}", s);
}

let name = String::from("Alice");
print(&name);  // Still own name
print("Bob");  // Can pass literals too!
```

**Type coercion:**

```rust
// String can deref to &str
let s = String::from("hello");
let slice: &str = &s;  // Automatic deref coercion

// But &str can't become String without allocation
let s: &str = "hello";
let owned: String = s.to_string();  // Allocates
```

## 5. `char` is 4 Bytes (Unicode Scalar Value)

### The Gotcha

In C/C++, `char` is 1 byte (ASCII or UTF-8 code unit). In Rust, `char` is 4 bytes (Unicode scalar value).

```c
// C - char is 1 byte
char c = 'a';
sizeof(c);  // 1
```

```cpp
// C++ - same
char c = 'a';
sizeof(c);  // 1
```

```rust
// Rust - char is 4 bytes!
let c = 'a';
std::mem::size_of_val(&c);  // 4

let emoji = '😀';  // This is a char!
let chinese = '中';  // This too!
```

**Implications:**

```rust
// String is UTF-8 (variable-length)
let s = "hello";
s.len();  // 5 bytes

let s = "😀";
s.len();  // 4 bytes (emoji is 4 bytes in UTF-8)

// Indexing doesn't work like C!
let s = "hello";
// let c = s[0];  // ERROR: can't index into str

// Must iterate properly
for c in s.chars() {  // Iterates Unicode scalar values
    println!("{}", c);
}

for b in s.bytes() {  // Iterates bytes
    println!("{}", b);
}
```

**Practical issue:**

```rust
// This looks like it should work...
let s = String::from("hello");
// let c: char = s[0];  // ERROR: can't index

// Get first char
let c: Option<char> = s.chars().next();

// Get byte at index
let b: u8 = s.as_bytes()[0];
```

## 6. Arrays Have Fixed Size (in the Type!)

### The Gotcha

In C/C++, array size is often erased. In Rust, array size is **part of the type**.

```c
// C - array decays to pointer
void func(int arr[10]) {  // Actually int*
    sizeof(arr);  // Size of pointer, not array!
}
```

```cpp
// C++ - same issue
void func(int arr[10]) {  // Still just int*
    // Size lost
}

// Better with templates
template<size_t N>
void func(int (&arr)[N]) {
    // N is compile-time known
}
```

```rust
// Rust - size is part of the type!
fn func(arr: [i32; 10]) {
    // arr is EXACTLY 10 elements
    // Type is [i32; 10]
}

fn func2(arr: [i32; 5]) {
    // Different type! [i32; 5] != [i32; 10]
}

// Use slices for dynamic size
fn func3(arr: &[i32]) {
    // Any length
    let len = arr.len();
}
```

**Practical impact:**

```rust
let arr1: [i32; 3] = [1, 2, 3];
let arr2: [i32; 5] = [1, 2, 3, 4, 5];

// Can't pass to same function!
fn process(arr: [i32; 3]) {
    // Only accepts [i32; 3]
}

process(arr1);  // OK
// process(arr2);  // ERROR: different type

// Use slices instead
fn process(arr: &[i32]) {
    // Accepts any length
}

process(&arr1);  // OK
process(&arr2);  // OK
```

## 7. Macros Are Hygienic (No Capture Issues)

### The Gotcha

C/C++ macros are text substitution. Rust macros are syntax-aware and hygienic.

```c
// C - macro captures variables
#define MAX(a, b) ((a) > (b) ? (a) : (b))

int x = 5;
int max = MAX(x++, 10);  // Evaluates x++ twice! UB!
```

```cpp
// C++ - same issue
#define SQUARE(x) ((x) * (x))

int y = SQUARE(++x);  // ++x evaluated twice!
```

```rust
// Rust macros don't have this issue
macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
}

let mut x = 5;
let m = max!(x, 10);  // x NOT evaluated multiple times
```

**Variable capture:**

```c
// C - macros can accidentally capture
#define LOG(msg) printf("%d: %s\n", __LINE__, msg)

int __LINE__ = 42;  // Name collision!
LOG("test");  // Might use our variable!
```

```rust
// Rust macros are hygienic - can't capture
macro_rules! log {
    ($msg:expr) => {
        println!("{}: {}", line!(), $msg)
    };
}

let line = 42;  // No collision, different scopes
log!("test");  // Uses correct line!()
```

## 8. No Implicit Conversions

### The Gotcha

C/C++ converts types implicitly. Rust requires explicit conversions.

```cpp
// C++ - implicit conversions everywhere
int x = 5;
double d = x;  // Implicit int -> double

float f = 3.14;
int i = f;  // Implicit float -> int (truncates)

bool b = 42;  // Implicit int -> bool (non-zero = true)

void* p = &x;  // Implicit int* -> void*
```

```rust
// Rust - must be explicit
let x: i32 = 5;
let d: f64 = x as f64;  // Explicit cast

let f: f32 = 3.14;
let i: i32 = f as i32;  // Explicit truncation

// let b: bool = 42;  // ERROR: no implicit conversion

let x: i32 = 5;
// let p: *const () = &x;  // ERROR: different types
let p: *const i32 = &x;
let p: *const () = p as *const ();  // Explicit cast
```

**No integer promotion:**

```cpp
// C++ - integer promotion
short s = 32767;
short result = s + 1;  // Promoted to int, then assigned
```

```rust
// Rust - no implicit promotion
let s: i16 = 32767;
// let result: i16 = s + 1;  // Overflow in debug mode!

// Must handle overflow explicitly
let result: i16 = s.wrapping_add(1);
```

## 9. Lifetimes Prevent Dangling References

### The Gotcha

C/C++ lets you return dangling pointers/references. Rust prevents this at compile time.

```cpp
// C++ - dangling reference compiles!
int& dangerous() {
    int x = 5;
    return x;  // WARNING (if enabled), but compiles!
}

int main() {
    int& ref = dangerous();
    std::cout << ref;  // Undefined behavior!
}
```

```rust
// Rust - won't compile!
fn dangerous() -> &i32 {
    let x = 5;
    &x  // ERROR: returns reference to local variable
}

// Error message:
// this function's return type contains a borrowed value,
// but there is no value for it to be borrowed from
```

**The fix:**

```rust
// Return owned value
fn safe() -> i32 {
    let x = 5;
    x  // Move ownership
}

// Or take reference as parameter
fn safe2(x: &i32) -> &i32 {
    x  // Return the same reference (lifetime is tied to input)
}
```

## 10. Copy vs Clone

### The Gotcha

In C++, classes have copy constructors. In Rust, types are `Copy` XOR require explicit `.clone()`.

```cpp
// C++ - everything copyable by default
struct Point {
    int x, y;
};

Point p1 = {1, 2};
Point p2 = p1;  // Implicit copy
// Both p1 and p2 valid
```

```rust
// Rust - depends on the type
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1;  // Copy (because we derived Copy)
// Both p1 and p2 valid

// But String is NOT Copy
let s1 = String::from("hello");
let s2 = s1;  // Move, not copy!
// s1 is gone

// Must explicitly clone
let s1 = String::from("hello");
let s2 = s1.clone();  // Explicit copy
// Both valid
```

**Rule:**
- Types with heap allocation can't be `Copy` (String, Vec, Box, etc.)
- Simple types can be `Copy` (integers, floats, tuples of Copy types, etc.)
- If you want both copies, use `.clone()` explicitly

**Affects function calls:**

```rust
#[derive(Clone)]
struct Data {
    buffer: Vec<u8>,
}

fn process(data: Data) {  // Takes ownership
    // ...
}

let d = Data { buffer: vec![1, 2, 3] };
process(d);
// d is moved, can't use anymore

// To keep it
let d = Data { buffer: vec![1, 2, 3] };
process(d.clone());  // Explicit clone
// d still valid
```

## 11. Match Must Be Exhaustive

### The Gotcha

C/C++ `switch` doesn't require all cases. Rust `match` must handle everything.

```cpp
// C++ - missing cases is fine
enum Color { Red, Green, Blue };

Color c = Red;
switch (c) {
    case Red:
        std::cout << "Red\n";
        break;
    case Green:
        std::cout << "Green\n";
        break;
    // Missing Blue - compiles fine
}
```

```rust
// Rust - must handle all cases
enum Color {
    Red,
    Green,
    Blue,
}

let c = Color::Red;
match c {
    Color::Red => println!("Red"),
    Color::Green => println!("Green"),
    // ERROR: pattern `Blue` not covered
}

// Must add catch-all or all cases
match c {
    Color::Red => println!("Red"),
    Color::Green => println!("Green"),
    Color::Blue => println!("Blue"),
}

// Or use _ for "everything else"
match c {
    Color::Red => println!("Red"),
    _ => println!("Not red"),
}
```

**Helps catch bugs:**

```rust
enum Result {
    Ok,
    Err,
    Timeout,  // Added later
}

// All matches must be updated!
match result {
    Result::Ok => handle_ok(),
    Result::Err => handle_err(),
    // Compiler error: missing Timeout
    // Forces you to handle new case
}
```

## 12. No Null Pointers (Use Option)

### The Gotcha

NULL exists everywhere in C/C++. Rust has no null pointers.

```c
// C - NULL pointers everywhere
int* ptr = NULL;
if (ptr != NULL) {
    *ptr = 42;
}
// Forgot check? Segfault!
```

```cpp
// C++ - nullptr
int* ptr = nullptr;
if (ptr != nullptr) {
    *ptr = 42;
}
// Same issue
```

```rust
// Rust - no null pointers exist!
// let ptr: &i32 = ???;  // Can't be null!

// Use Option for "might not exist"
let maybe: Option<i32> = None;

// Compiler forces you to check
match maybe {
    Some(value) => println!("{}", value),
    None => println!("No value"),
}

// Or use if let
if let Some(value) = maybe {
    println!("{}", value);
}
```

**Tony Hoare called null his "billion-dollar mistake." Rust eliminates it entirely.**

## Summary Table

| C/C++ Behavior | Rust Equivalent | Gotcha |
|----------------|-----------------|--------|
| Moved-from objects exist | Moved values are gone | Can't use after move |
| Multiple mutable refs OK | Exclusive mutable borrow | Iterator invalidation prevented |
| Signed overflow is UB | Panics (debug) or wraps (release) | Defined behavior |
| One string type | `String` vs `&str` | Different use cases |
| `char` is 1 byte | `char` is 4 bytes (Unicode) | String indexing doesn't work |
| Array size erased | Array size in type | `[i32; 3]` != `[i32; 5]` |
| Text-substitution macros | Hygienic macros | No variable capture issues |
| Implicit conversions | Explicit `as` casts | Must be intentional |
| Dangling refs compile | Lifetime errors | Prevents use-after-free |
| Implicit copy | `Copy` trait or explicit `.clone()` | Clear ownership |
| Switch can skip cases | Match must be exhaustive | Catches missing cases |
| NULL pointers | `Option<T>` | No null pointer dereferences |

**Remember:** These aren't bugs or limitations—they're safety features that prevent entire classes of errors C/C++ developers deal with daily.
