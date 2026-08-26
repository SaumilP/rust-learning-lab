# Gotchas for Python Developers

Things that will surprise you coming from Python. Learn from my mistakes!

## 1. Variables are immutable by default

```rust
let x = 5;
x = 6;  // ❌ ERROR: cannot assign twice to immutable variable
```

**What you expected** (Python behavior):
```python
x = 5
x = 6  # ✅ Just works
```

**The fix**:
```rust
let mut x = 5;
x = 6;  // ✅ OK now
```

**Why it's different**: Rust defaults to safety. Mutation must be explicit. This prevents accidental mutations.

---

## 2. Moving consumes values

```rust
let data = vec![1, 2, 3];
let data2 = data;  // data is moved
println!("{:?}", data);  // ❌ ERROR: value borrowed after move
```

**What you expected** (Python behavior):
```python
data = [1, 2, 3]
data2 = data  # Both reference same list
print(data)  # ✅ Works fine
```

**Why it's different**: Python uses references everywhere; Rust transfers ownership by default.

**The fix**:
```rust
// Option 1: Clone
let data = vec![1, 2, 3];
let data2 = data.clone();
println!("{:?}", data);  // ✅ OK

// Option 2: Borrow
let data = vec![1, 2, 3];
let data2 = &data;
println!("{:?}", data);  // ✅ OK
```

---

## 3. String indexing doesn't work like Python

```rust
let s = "hello";
let c = s[0];  // ❌ ERROR: the type `str` cannot be indexed by `{integer}`
```

**What you expected** (Python behavior):
```python
s = "hello"
c = s[0]  # 'h'
```

**Why it's different**: Strings are UTF-8. A byte index might be in the middle of a character.

**The fix**:
```rust
let s = "hello";

// Get nth character
let c = s.chars().nth(0);  // Some('h')

// Iterate characters
for c in s.chars() {
    println!("{}", c);
}

// Get byte (if you really need it)
let b = s.as_bytes()[0];  // 104 (ASCII 'h')
```

---

## 4. No default parameter values

```rust
fn greet(name: &str = "World") {  // ❌ ERROR: default parameters not allowed
    println!("Hello, {}!", name);
}
```

**What you expected** (Python behavior):
```python
def greet(name="World"):
    print(f"Hello, {name}!")

greet()  # "Hello, World!"
greet("Alice")  # "Hello, Alice!"
```

**The fix**:
```rust
// Option 1: Overloading with different names
fn greet_default() {
    greet("World");
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Option 2: Option type
fn greet(name: Option<&str>) {
    let name = name.unwrap_or("World");
    println!("Hello, {}!", name);
}

// Usage
greet(Some("Alice"));
greet(None);
```

---

## 5. No `None` for optional values

```rust
fn find_user(id: i32) -> User {
    if id == 1 {
        return User { name: "Alice" };
    }
    return None;  // ❌ ERROR: expected `User`, found `()`
}
```

**What you expected** (Python behavior):
```python
def find_user(id):
    if id == 1:
        return {"name": "Alice"}
    return None  # ✅ Works
```

**The fix**:
```rust
fn find_user(id: i32) -> Option<User> {
    if id == 1 {
        return Some(User { name: String::from("Alice") });
    }
    None
}

// Must handle None
match find_user(2) {
    Some(user) => println!("{}", user.name),
    None => println!("Not found"),
}
```

---

## 6. Integer division truncates (same as Python 3)

```rust
let result = 5 / 2;
println!("{}", result);  // 2, not 2.5
```

This is actually the same as Python 3, but worth mentioning!

**The fix**:
```rust
let result = 5.0 / 2.0;  // 2.5
// Or
let result = 5 as f64 / 2 as f64;  // 2.5
```

---

## 7. Can't modify while iterating

```rust
let mut numbers = vec![1, 2, 3];
for num in &numbers {
    numbers.push(*num * 2);  // ❌ ERROR: cannot borrow `numbers` as mutable
}
```

**What you expected** (Python behavior):
```python
numbers = [1, 2, 3]
for num in numbers:
    numbers.append(num * 2)  # ✅ Works (but might cause infinite loop!)
```

**Why it's different**: Rust prevents iterator invalidation at compile time.

**The fix**:
```rust
// Collect what you need first
let numbers = vec![1, 2, 3];
let doubled: Vec<_> = numbers.iter()
    .map(|&n| n * 2)
    .collect();
let mut result = numbers;
result.extend(doubled);
```

---

## 8. No truthiness - must be explicit bool

```rust
let x = 5;
if x {  // ❌ ERROR: expected `bool`, found `i32`
    println!("truthy");
}
```

**What you expected** (Python behavior):
```python
x = 5
if x:  # ✅ Truthy value
    print("truthy")
```

**The fix**:
```rust
let x = 5;
if x != 0 {  // ✅ Explicit comparison
    println!("not zero");
}

let s = "hello";
if !s.is_empty() {  // ✅ Explicit check
    println!("not empty");
}
```

---

## 9. Closures capture by reference by default

```rust
let mut x = 5;
let add_one = || x += 1;  // Captures &mut x
println!("{}", x);  // ❌ ERROR: x is borrowed by closure
add_one();
```

**What you expected** (Python behavior):
```python
x = 5
def add_one():
    global x
    x += 1

print(x)  # Works (though you need global)
add_one()
```

**The fix**:
```rust
let mut x = 5;
{
    let add_one = || x += 1;
    add_one();
}  // Closure dropped, borrow ends
println!("{}", x);  // ✅ OK now
```

---

## 10. Match must be exhaustive

```rust
let x = Some(5);
match x {
    Some(val) => println!("{}", val),
    // ❌ ERROR: non-exhaustive patterns: `None` not covered
}
```

**What you expected**: Python doesn't require exhaustive matching.

**The fix**:
```rust
match x {
    Some(val) => println!("{}", val),
    None => println!("nothing"),  // ✅ Handle all cases
}

// Or use wildcard
match x {
    Some(val) => println!("{}", val),
    _ => {},  // ✅ Catch-all
}
```

---

## 11. `print` is a macro, not a function

```rust
print("Hello");  // ❌ ERROR: cannot find function `print`
```

**What you expected** (Python behavior):
```python
print("Hello")  # ✅ Function call
```

**The fix**:
```rust
println!("Hello");  // ✅ Macro with !
print!("Hello");    // ✅ Without newline
```

---

## 12. No list multiplication

```rust
let zeros = [0] * 5;  // ❌ ERROR: can't multiply array by integer
```

**What you expected** (Python behavior):
```python
zeros = [0] * 5  # [0, 0, 0, 0, 0]
```

**The fix**:
```rust
let zeros = vec![0; 5];  // ✅ vec![value; count]
let zeros = [0; 5];      // ✅ Fixed-size array
```

---

## 13. Iterators are consumed

```rust
let numbers = vec![1, 2, 3];
let iter = numbers.iter();
let sum: i32 = iter.sum();
let count = iter.count();  // ❌ ERROR: value used after being moved
```

**What you expected** (Python behavior):
```python
numbers = [1, 2, 3]
total = sum(numbers)
count = len(numbers)  # ✅ Can use multiple times
```

**Why it's different**: Iterators are consumed when you call terminal operations.

**The fix**:
```rust
let numbers = vec![1, 2, 3];
let sum: i32 = numbers.iter().sum();
let count = numbers.len();  // ✅ Use original vec

// Or create new iterator
let sum: i32 = numbers.iter().sum();
let count = numbers.iter().count();  // ✅ New iterator
```

---

## 14. No negative indexing

```rust
let items = vec![1, 2, 3];
let last = items[-1];  // ❌ ERROR: can't index with negative number
```

**What you expected** (Python behavior):
```python
items = [1, 2, 3]
last = items[-1]  # 3
```

**The fix**:
```rust
let items = vec![1, 2, 3];
let last = items[items.len() - 1];  // ✅ Manual calculation
let last = items.last();  // ✅ Returns Option<&T>
```

---

## 15. Type must be specified for collect()

```rust
let numbers = (0..5).collect();  // ❌ ERROR: cannot infer type
```

**What you expected**: Python infers the type.

**The fix**:
```rust
// Option 1: Type annotation
let numbers: Vec<i32> = (0..5).collect();

// Option 2: Turbofish
let numbers = (0..5).collect::<Vec<i32>>();
```

---

## Quick Tips to Remember

1. **Immutable by default** - Add `mut` when needed
2. **Moving transfers ownership** - Clone or borrow instead
3. **Strings are UTF-8** - Use `.chars()` not indexing
4. **No None** - Use `Option<T>` explicitly
5. **Explicit bools** - No truthiness, use comparisons
6. **Match is exhaustive** - Handle all cases
7. **Macros use `!`** - `println!()`, `vec![]`, etc.
8. **Iterators are consumed** - Create new one if needed
9. **No negative indexing** - Use `.last()` or calculate
10. **Type inference needs hints** - Sometimes need `::<T>` or type annotation

---

**Remember**: These "gotchas" aren't bugs—they're Rust preventing entire classes of bugs that Python catches at runtime (or doesn't catch at all!).

Most of these will become natural after a few weeks of writing Rust.
