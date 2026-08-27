# Gotchas: Surprising Differences for Go Developers

Things that will surprise you coming from Go. Some are frustrating, some are delightful—all are worth knowing.

## Table of Contents

1. [Move Semantics Surprises](#move-semantics-surprises)
2. [String vs &str Confusion](#string-vs-str-confusion)
3. [No nil Anywhere](#no-nil-anywhere)
4. [Integer Overflow Behavior](#integer-overflow-behavior)
5. [Loop Variables Are Different](#loop-variables-are-different)
6. [Shadowing Is Allowed](#shadowing-is-allowed)
7. [Macros Look Like Functions](#macros-look-like-functions)
8. [No Implicit Numeric Conversions](#no-implicit-numeric-conversions)
9. [Slices Are References](#slices-are-references)
10. [Return in Closures](#return-in-closures)
11. [Struct Update Syntax](#struct-update-syntax)
12. [Method Calls Auto-Deref](#method-calls-auto-deref)

---

## Move Semantics Surprises

### Gotcha: Assigning Moves, Not Copies

```rust
// Rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED, not copied

// println!("{}", s1);  // ERROR! s1 is no longer valid
println!("{}", s2);     // OK
```

**Why it's surprising**: In Go, assignment copies the pointer but both variables remain valid.

```go
// Go - for comparison
s1 := "hello"
s2 := s1  // Both s1 and s2 are valid
fmt.Println(s1, s2)  // Works fine
```

**What to remember**: Non-Copy types (String, Vec, etc.) move on assignment. Use `.clone()` if you need both.

---

## String vs &str Confusion

### Gotcha: String and &str Are Different Types

```rust
// Rust
fn print_go_string(s: String) {  // Takes ownership
    println!("{}", s);
}

fn print_rust_string(s: &str) {  // Borrows
    println!("{}", s);
}

let owned = String::from("hello");
print_go_string(owned.clone());  // Must clone or can't use owned again
print_rust_string(&owned);       // Can use owned again
print_rust_string("world");      // String literals are &str
```

**Why it's surprising**: Go has one `string` type. Rust has `String` (owned) and `&str` (borrowed).

**What to remember**:
- Function parameters: use `&str`
- Owned string data: use `String`
- String literals: are `&str`
- Conversion: `String::from("hello")` or `"hello".to_string()`

---

## No nil Anywhere

### Gotcha: Can't Check for nil

```rust
// Rust - NO nil!
let x: i32 = 5;
// if x == nil { }  // ERROR! No nil

// Must use Option explicitly
let maybe_x: Option<i32> = Some(5);
if maybe_x.is_some() {
    println!("Has value");
}

// Or with if let
if let Some(value) = maybe_x {
    println!("{}", value);
}
```

**Why it's surprising**: In Go, any pointer, interface, slice, map, or channel can be nil.

```go
// Go - for comparison
var m map[string]int = nil
if m == nil {
    fmt.Println("map is nil")
}
```

**What to remember**: Use `Option<T>` when a value might not exist. Compiler forces you to handle both cases.

---

## Integer Overflow Behavior

### Gotcha: Debug Mode Panics, Release Mode Wraps

```rust
// Rust
let x: u8 = 255;
// let y = x + 1;  // Panics in debug mode!
                   // Wraps to 0 in release mode!

// Explicit wrapping
let y = x.wrapping_add(1);  // Always wraps to 0

// Checked operations
if let Some(y) = x.checked_add(1) {
    println!("{}", y);
} else {
    println!("Overflow!");
}
```

**Why it's surprising**: Go silently wraps on overflow in all modes.

```go
// Go
var x uint8 = 255
y := x + 1  // Always wraps to 0, no panic
```

**What to remember**:
- Debug builds: overflow panics (catches bugs)
- Release builds: overflow wraps (matches Go)
- Use `checked_*` or `wrapping_*` to be explicit

---

## Loop Variables Are Different

### Gotcha: Loop Variables Are Reborrowed, Not New

```rust
// Rust
let mut nums = vec![1, 2, 3];

// This is a BORROW
for num in &nums {
    // num is &i32
    println!("{}", num);
}
// nums still valid here

// This MOVES
for num in nums {
    // num is i32, nums is consumed
    println!("{}", num);
}
// nums is now INVALID

// To mutate in loop
for num in &mut nums {
    *num *= 2;  // Must dereference
}
```

**Why it's surprising**: In Go, range always creates references or copies depending on context.

```go
// Go
nums := []int{1, 2, 3}
for _, num := range nums {
    fmt.Println(num)
}
// nums still valid
```

**What to remember**:
- `for x in collection` moves (consumes collection)
- `for x in &collection` borrows immutably
- `for x in &mut collection` borrows mutably

---

## Shadowing Is Allowed

### Gotcha: Can Redeclare Variables

```rust
// Rust - shadowing is fine!
let x = 5;
let x = x + 1;    // Shadows previous x
let x = x * 2;    // Shadows again
println!("{}", x);  // 12

// Can even change type
let spaces = "   ";
let spaces = spaces.len();  // Now it's a number!
```

**Why it's surprising**: Go doesn't allow redeclaring in the same scope.

```go
// Go - this is an error
x := 5
x := 10  // ERROR: no new variables on left side of :=
```

**What to remember**: Shadowing is idiomatic in Rust. Use it to avoid naming like `x1`, `x2`, `x_final`.

---

## Macros Look Like Functions

### Gotcha: println! Is a Macro, Not a Function

```rust
// Rust
println!("hello");        // Macro (note the !)
vec![1, 2, 3];           // Macro
format!("x = {}", x);    // Macro

// Regular function (no !)
String::from("hello");   // Function
```

**Why it's surprising**: Go doesn't have macros. In Rust, `!` means it's a macro.

**What to remember**:
- `!` = macro (compile-time code generation)
- No `!` = function (runtime call)
- Macros can do things functions can't (like variadic arguments)

---

## No Implicit Numeric Conversions

### Gotcha: Must Explicitly Convert Between Number Types

```rust
// Rust
let x: i32 = 5;
let y: i64 = 10;

// let z = x + y;  // ERROR! Can't add i32 and i64

let z = x as i64 + y;  // Must cast explicitly
```

**Why it's surprising**: Go doesn't auto-convert either, but Rust is stricter.

```go
// Go - also requires explicit conversion
var x int32 = 5
var y int64 = 10
z := int64(x) + y
```

**What to remember**: Use `as` for numeric conversions: `x as i64`

---

## Slices Are References

### Gotcha: &[T] Is Already a Reference

```rust
// Rust
fn process(data: &[i32]) {  // &[i32] is a slice (already a reference!)
    // Don't need &&[i32]
}

let vec = vec![1, 2, 3];
process(&vec);  // &Vec<i32> coerces to &[i32]
```

**Why it's surprising**: In Go, `[]int` is the slice. In Rust, it's `&[i32]`.

```go
// Go
func process(data []int) {
    // []int is already reference-like
}

vec := []int{1, 2, 3}
process(vec)
```

**What to remember**:
- `Vec<T>` = owned, growable array (like Go slice)
- `&[T]` = borrowed slice (like Go slice passed to function)
- `&vec` automatically converts to `&[T]`

---

## Return in Closures

### Gotcha: return Returns from Closure, Not Outer Function

```rust
// Rust
fn find_even(nums: Vec<i32>) -> Option<i32> {
    nums.iter().find(|&&x| {
        if x % 2 == 0 {
            return true;  // Returns from closure, not find_even!
        }
        false
    }).copied()
}

// Better: use implicit return
fn find_even_better(nums: Vec<i32>) -> Option<i32> {
    nums.iter().find(|&&x| x % 2 == 0).copied()
}
```

**Why it's surprising**: In Go, return in a closure returns from the outer function.

```go
// Go
func findEven(nums []int) *int {
    for _, n := range nums {
        if n % 2 == 0 {
            return &n  // Returns from findEven
        }
    }
    return nil
}
```

**What to remember**: Rust closures have their own returns. Use implicit returns or control flow.

---

## Struct Update Syntax

### Gotcha: Struct Spread Syntax Moves

```rust
// Rust
let user1 = User {
    name: String::from("Alice"),
    age: 30,
};

let user2 = User {
    name: String::from("Bob"),
    ..user1  // Copies age, MOVES any non-Copy fields
};

// println!("{}", user1.name);  // ERROR if name was moved
```

**Why it's surprising**: Looks like Go's struct literals but has move semantics.

```go
// Go - for comparison (no spread syntax)
user1 := User{Name: "Alice", Age: 30}
user2 := User{Name: "Bob", Age: user1.Age}
```

**What to remember**: `..other` copies Copy types, moves others. Clone if you need both.

---

## Method Calls Auto-Deref

### Gotcha: Method Calls Automatically Dereference

```rust
// Rust
let s = String::from("hello");
let len = s.len();           // Calls &String::len

let r = &s;
let len = r.len();           // Auto-derefs! Calls &String::len

let rr = &&&&s;
let len = rr.len();          // Auto-derefs multiple times!
```

**Why it's surprising**: In Go, you'd need to manually dereference.

**What to remember**: Method calls auto-deref. `s.method()` tries `&s`, `&&s`, etc. until it works.

---

## Common Misconceptions

### "Rust is just Go with a borrow checker"

**False**. Rust has:
- Powerful enums (algebraic data types)
- Pattern matching
- Zero-cost abstractions
- Trait system (more powerful than interfaces)
- No garbage collector
- Generics with monomorphization

### "I can just use Arc<Mutex<T>> for everything"

**Bad idea**. You lose Rust's benefits:
- Runtime overhead (like Go's pointers + sync)
- Deadlock potential
- Missing zero-cost abstractions

### "Lifetimes are too complex"

**Overblown**. Most code doesn't need explicit lifetimes:
- Lifetime elision handles 90% of cases
- Only needed when returning references
- Compiler suggests the fix

### "String vs &str is unnecessarily confusing"

**There's a reason**:
- `&str` = zero-copy, stack-allocated (fast!)
- `String` = heap-allocated, growable
- Separation gives you control

---

## Delightful Surprises

### Compiler Errors Are Helpful

```rust
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:4:20
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`
3 |     let s2 = s1;
  |              -- value moved here
4 |     println!("{}", s1);
  |                    ^^ value borrowed here after move
  |
  = note: consider cloning the value if performance is not an issue
```

The compiler tells you exactly what's wrong AND suggests fixes!

### If/Match Are Expressions

```rust
// Rust - if is an expression!
let x = if condition { 5 } else { 10 };

// Match returns values
let message = match status {
    200 => "OK",
    404 => "Not Found",
    _ => "Error",
};
```

### Ranges Are Awesome

```rust
for i in 0..10 { }       // 0 to 9
for i in 0..=10 { }      // 0 to 10 (inclusive)
for i in (0..10).rev() { }  // 9 down to 0

let slice = &arr[2..5];  // Elements 2, 3, 4
```

### Iterator Combinators

```rust
let result: Vec<i32> = vec![1, 2, 3, 4, 5]
    .iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 2)
    .collect();
// [4, 8] - and it's zero-cost!
```

---

## Pro Tips to Avoid Gotchas

1. **When in doubt, borrow**: Start with `&T`, only use `T` if you need ownership

2. **Use clippy**: `cargo clippy` catches gotchas before they bite

3. **Read compiler errors**: They're not cryptic like C++, they're helpful

4. **Clone while learning**: `.clone()` gets you unstuck, optimize later

5. **Trust the borrow checker**: It's preventing real bugs, not being pedantic

6. **Use .as_ref()/.as_mut()**: Converts `Option<T>` to `Option<&T>` without moving

7. **Remember ?**: Works on Result AND Option, making error handling clean

8. **Use dbg!() macro**: Better than println! for debugging

Remember: Most gotchas exist because Rust is preventing bugs that would be runtime errors in Go. Once you understand why, they start making sense!
