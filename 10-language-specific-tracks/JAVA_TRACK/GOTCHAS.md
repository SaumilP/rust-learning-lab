# Gotchas for Java Developers

Things that will trip you up coming from Java. I learned these the hard way so you don't have to.

## 1. Variables are immutable by default

```rust
let x = 5;
x = 6;  // ❌ ERROR: cannot assign twice to immutable variable
```

**What you expected** (Java behavior):
```java
int x = 5;
x = 6;  // ✅ Works fine
```

**The fix**:
```rust
let mut x = 5;
x = 6;  // ✅ OK now
```

**Why it's different**: Rust defaults to immutable for safety. Mutation must be explicit.

---

## 2. Can't have multiple mutable references

```rust
let mut v = vec![1, 2, 3];
let r1 = &mut v;
let r2 = &mut v;  // ❌ ERROR: cannot borrow `v` as mutable more than once
```

**What you expected** (Java behavior):
```java
List<Integer> v = new ArrayList<>();
List<Integer> r1 = v;
List<Integer> r2 = v;  // ✅ Both refer to same list
```

**Why it's different**: Rust prevents data races at compile time. Only one mutable reference OR multiple immutable references allowed at a time.

**The fix**:
```rust
// Option 1: Use them sequentially
let r1 = &mut v;
// use r1
drop(r1);  // End borrow
let r2 = &mut v;  // OK now

// Option 2: Use Arc<Mutex<T>> for shared mutable state
use std::sync::{Arc, Mutex};
let v = Arc::new(Mutex::new(vec![1, 2, 3]));
```

---

## 3. Moving values consumes them

```rust
let s = String::from("hello");
let t = s;  // s is moved to t
println!("{}", s);  // ❌ ERROR: value borrowed after move
```

**What you expected** (Java behavior):
```java
String s = "hello";
String t = s;  // Both reference same string
System.out.println(s);  // ✅ Works fine
```

**Why it's different**: Rust transfers ownership by default. After moving, the original variable is invalid.

**The fix**:
```rust
// Option 1: Clone if you need both
let s = String::from("hello");
let t = s.clone();
println!("{} {}", s, t);  // ✅ Both valid

// Option 2: Borrow instead
let s = String::from("hello");
let t = &s;  // Borrow, not move
println!("{} {}", s, t);  // ✅ Both valid
```

---

## 4. Can't return references to local variables

```rust
fn get_name() -> &str {
    let name = String::from("Alice");
    &name  // ❌ ERROR: returns a reference to data owned by current function
}
```

**What you expected** (Java behavior):
```java
String getName() {
    String name = "Alice";
    return name;  // ✅ Objects live on heap, GC handles it
}
```

**Why it's different**: Local variables are dropped when function returns. Reference would be dangling.

**The fix**:
```rust
// Option 1: Return owned data
fn get_name() -> String {
    String::from("Alice")  // ✅ Caller owns it now
}

// Option 2: Use static lifetime
fn get_name() -> &'static str {
    "Alice"  // ✅ String literal lives forever
}
```

---

## 5. Integer division truncates

```rust
let result = 5 / 2;
println!("{}", result);  // Prints 2, not 2.5
```

**What you expected**: Same as Java, actually! But worth mentioning.

**The fix**:
```rust
let result = 5.0 / 2.0;  // ✅ 2.5
// Or
let result = 5 as f64 / 2 as f64;  // ✅ 2.5
```

---

## 6. No automatic type coercion

```rust
let x: i32 = 5;
let y: i64 = x;  // ❌ ERROR: expected i64, found i32
```

**What you expected** (Java behavior):
```java
int x = 5;
long y = x;  // ✅ Automatic widening
```

**Why it's different**: Rust requires explicit conversions to prevent accidental bugs.

**The fix**:
```rust
let x: i32 = 5;
let y: i64 = x as i64;  // ✅ Explicit cast
// Or
let y: i64 = x.into();  // ✅ Using Into trait
```

---

## 7. Strings are not iterable by default

```rust
let s = "hello";
for c in s {  // ❌ ERROR: `&str` is not an iterator
    println!("{}", c);
}
```

**What you expected** (Java behavior):
```java
String s = "hello";
for (char c : s.toCharArray()) {  // Works with toCharArray()
    System.out.println(c);
}
```

**Why it's different**: Rust strings are UTF-8, so you need to specify if you want bytes or chars.

**The fix**:
```rust
let s = "hello";

// Iterate over chars
for c in s.chars() {  // ✅
    println!("{}", c);
}

// Or bytes
for b in s.bytes() {  // ✅
    println!("{}", b);
}
```

---

## 8. Match must be exhaustive

```rust
let x = Some(5);
match x {
    Some(val) => println!("{}", val),
    // ❌ ERROR: non-exhaustive patterns: `None` not covered
}
```

**What you expected** (Java behavior):
```java
// Switch doesn't require default in old Java
switch(x) {
    case 1: System.out.println("one");
}
```

**Why it's different**: Rust forces you to handle all cases to prevent bugs.

**The fix**:
```rust
match x {
    Some(val) => println!("{}", val),
    None => println!("nothing"),  // ✅ Handle None
}

// Or use wildcard
match x {
    Some(val) => println!("{}", val),
    _ => {},  // ✅ Catch-all
}
```

---

## 9. Closures capture by reference by default

```rust
let mut x = 5;
let add = || x += 1;  // Captures &mut x
// println!("{}", x);  // ❌ ERROR: x is borrowed by closure
add();
```

**What you expected** (Java behavior):
```java
int x = 5;
Runnable add = () -> x++;  // ❌ Actually won't compile either (final)
```

**Why it's different**: Rust closures can capture by reference, mutable reference, or value.

**The fix**:
```rust
// Option 1: Use closure first
let mut x = 5;
let add = || x += 1;
add();
println!("{}", x);  // ✅ OK after closure is done

// Option 2: Move into closure
let x = 5;
let add = move || {
    let mut x = x;  // Owned copy
    x += 1;
};
```

---

## 10. Array length is part of the type

```rust
let arr1: [i32; 3] = [1, 2, 3];
let arr2: [i32; 4] = [1, 2, 3, 4];
// arr1 and arr2 are DIFFERENT TYPES
```

**What you expected** (Java behavior):
```java
int[] arr1 = {1, 2, 3};
int[] arr2 = {1, 2, 3, 4};
// Both are type int[]
```

**Why it's different**: Fixed-size arrays have the size in the type for optimization.

**The fix**: Use slices or `Vec` for variable-length data:
```rust
let arr1 = vec![1, 2, 3];
let arr2 = vec![1, 2, 3, 4];
// Both are Vec<i32>
```

---

## 11. No implicit boolean conversion

```rust
let x = 5;
if x {  // ❌ ERROR: expected bool, found integer
    println!("truthy");
}
```

**What you expected**: Other languages allow truthy/falsy values.

**Why it's different**: Rust requires explicit boolean expressions.

**The fix**:
```rust
let x = 5;
if x != 0 {  // ✅ Explicit comparison
    println!("not zero");
}
```

---

## 12. Lifetime annotations look scary but aren't

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

**What you expected**: No such thing in Java!

**Why it exists**: Tells compiler how long the returned reference lives.

**What it means**: "The returned reference lives as long as the shortest input reference."

**You don't always need them**:
```rust
// Compiler infers lifetime
fn first(x: &str, y: &str) -> &str {
    x  // Obviously returns x's lifetime
}
```

---

## 13. No method overloading (by parameter type)

```rust
fn print(x: i32) { }
fn print(x: String) { }  // ❌ ERROR: duplicate definition
```

**What you expected** (Java behavior):
```java
void print(int x) { }
void print(String x) { }  // ✅ Overloading works
```

**Why it's different**: Rust doesn't support method overloading by parameter type.

**The fix**: Use different names or generics:
```rust
fn print_int(x: i32) { }
fn print_string(x: String) { }

// Or generics
fn print<T: Display>(x: T) { }
```

---

## 14. Copy vs Clone

```rust
let x = 5;
let y = x;  // ✅ x is still valid (i32 is Copy)

let s = String::from("hello");
let t = s;  // ❌ s is no longer valid (String is not Copy)
```

**What's happening**:
- Types that implement `Copy` (like integers) are copied automatically
- Types that don't (like `String`) are moved

**Simple types that are Copy**: i32, f64, bool, char, tuples of Copy types

**Types that aren't Copy**: String, Vec, HashMap, Box (anything with heap data)

**The fix**: Know which types are Copy. For non-Copy types, use `.clone()` if you need both:
```rust
let s = String::from("hello");
let t = s.clone();  // ✅ Both valid
```

---

## 15. Underscore variables still consume

```rust
let s = String::from("hello");
let _ = s;  // s is still moved!
println!("{}", s);  // ❌ ERROR: value moved
```

**What you expected**: `_` means "ignore", right?

**Why it's different**: `let _ = x` still moves/consumes x.

**The fix**: Use `_var_name` to ignore the unused warning but not consume:
```rust
let s = String::from("hello");
let _s = s;  // Moved, but suppresses warning
// or just don't bind it
```

---

## Quick Tips to Remember

1. **Default to immutable** - Add `mut` only when needed
2. **One mutable reference OR many immutable** - Never both
3. **Moving transfers ownership** - Clone if you need both
4. **Can't return local references** - Return owned data instead
5. **Match must be exhaustive** - Handle all cases
6. **Strings are UTF-8** - Use `.chars()` or `.bytes()` to iterate
7. **Array length is in type** - Use `Vec` for flexibility
8. **No null** - Use `Option<T>` instead
9. **Explicit type conversions** - No automatic coercion
10. **Read compiler errors** - They're actually helpful!

---

**Remember**: These "gotchas" aren't bugs—they're features preventing entire classes of bugs in your code. The initial frustration is worth it for the runtime safety!
