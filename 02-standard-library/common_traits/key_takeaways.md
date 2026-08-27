# Key Takeaways: Common Traits

## Quick Reference

### Core Traits

```rust
// Copying and Cloning
#[derive(Copy, Clone)]
struct MyType;

// Comparison
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct MyType;

// Hashing
#[derive(Hash)]
struct MyType;

// Display/Debug
#[derive(Debug)]
struct MyType;

impl fmt::Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "...")
    }
}

// Default values
#[derive(Default)]
struct MyType;
```

### Trait Bounds

```rust
// Single bound
fn func<T: Debug>(value: T) { }

// Multiple bounds
fn func<T: Debug + Clone>(value: T) { }

// Where clause
fn func<T>(value: T)
where
    T: Debug + Clone,
{ }

// Trait object
let obj: &dyn Debug = &value;
```

## Essential Concepts

### 1. Copy vs Clone

| Trait | Behavior | Cost | When |
|-------|----------|------|------|
| `Copy` | Implicit copying | Zero-cost bitwise copy | Primitives |
| `Clone` | Explicit deep copy | Can be expensive | Owned data |

```rust
// Copy - automatic
let x: i32 = 5;
let y = x;  // Copied

// Clone - explicit
let s1 = String::from("hello");
let s2 = s1.clone();  // Explicit deep copy
```

**Key rule**: If type is `Copy`, it can't implement `Drop`

### 2. Equality and Ordering

| Trait | Operator | Notes |
|-------|----------|-------|
| `PartialEq` | `==`, `!=` | May not be total (NaN) |
| `Eq` | Complete PartialEq | Requires reflexivity |
| `PartialOrd` | `<`, `>`, `<=`, `>=` | Partial ordering |
| `Ord` | Total ordering | Requires Eq + PartialOrd |

```rust
// For HashMap/HashSet keys
#[derive(Hash, Eq, PartialEq)]
struct Key { id: u32 }

// For sorting
#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Item { priority: u32 }

let mut items = vec![...];
items.sort();  // Uses Ord
```

### 3. Derive Macro Traits

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct MyType {
    field: i32,
}
```

**Common derivable traits:**
- `Debug` - `{:?}` printing
- `Clone` - Explicit copying
- `Copy` - Implicit copying (primitives)
- `Default` - Default values
- `PartialEq` - `==` comparison
- `Eq` - Total equality
- `PartialOrd` - Partial ordering
- `Ord` - Total ordering
- `Hash` - HashMap/HashSet key

### 4. Hash + Eq Requirement

For HashMap and HashSet:

```rust
// ✅ MUST implement both
#[derive(Hash, Eq, PartialEq)]
struct Id { value: u32 }

let mut map = HashMap::new();
map.insert(Id { value: 1 }, "data");

// ❌ ERROR: Vec can't be HashMap key
let mut map: HashMap<Vec<i32>, String> = HashMap::new();
map.insert(vec![1, 2], "value");  // Compile error
```

### 5. Display vs Debug

| Trait | Format | Use Case | Derive? |
|-------|--------|----------|---------|
| `Display` | User-friendly | End users | Manual |
| `Debug` | Programmer-friendly | Debugging | `#[derive]` |

```rust
use std::fmt;

#[derive(Debug)]
struct Point { x: i32, y: i32 }

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 1, y: 2 };
println!("{}", p);   // (1, 2) - Display
println!("{:?}", p); // Point { x: 1, y: 2 } - Debug
```

### 6. Trait Bounds

```rust
// Function bounds
fn print_debug<T: Debug>(value: &T) {
    println!("{:?}", value);
}

// Multiple bounds
fn process<T: Clone + Debug + PartialEq>(value: &T) { }

// Where clause (preferred for complex)
fn process<T>(value: &T)
where
    T: Clone + Debug + PartialEq + Ord,
{ }
```

### 7. Trait Objects

```rust
// Trait object - dynamic dispatch
let obj: &dyn Debug = &value;

// Owned trait object
let obj: Box<dyn Debug> = Box::new(value);

// In collections
let items: Vec<Box<dyn Handler>> = vec![
    Box::new(ConsoleHandler),
    Box::new(FileHandler),
];
```

## Common Patterns

### Pattern 1: Deriving traits
```rust
#[derive(Debug, Clone, PartialEq)]
struct Person { name: String, age: u32 }
```

### Pattern 2: Custom Display
```rust
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (age {})", self.name, self.age)
    }
}
```

### Pattern 3: Generic with bounds
```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max { max = item; }
    }
    max
}
```

### Pattern 4: Trait object collection
```rust
let handlers: Vec<Box<dyn Handler>> = vec![
    Box::new(type1),
    Box::new(type2),
];
```

### Pattern 5: Default implementation
```rust
trait Animal {
    fn speak(&self);

    fn introduce(&self) {  // Default
        println!("Hello!");
        self.speak();
    }
}
```

## Checklist: When to Derive

- [ ] Need `==` comparison? → Add `PartialEq`
- [ ] Use as HashMap key? → Add `Hash + Eq + PartialEq`
- [ ] Need sorting? → Add `Ord + PartialOrd + Eq + PartialEq`
- [ ] Need printing? → Add `Debug`
- [ ] Need user display? → Implement `Display` manually
- [ ] Need copying? → Add `Copy + Clone` (primitives only)
- [ ] Need default value? → Add `Default`

## Error Prevention

### ❌ DON'T: Use non-hashable type as HashMap key
```rust
let mut map: HashMap<Vec<i32>, String> = HashMap::new();
map.insert(vec![1, 2], "value");  // ERROR: Vec not Hash
```

### ✅ DO: Use hashable types
```rust
#[derive(Hash, Eq, PartialEq)]
struct Key { id: u32 }

let mut map = HashMap::new();
map.insert(Key { id: 1 }, "value");  // OK
```

### ❌ DON'T: Forget trait bounds
```rust
fn sort_items<T>(items: &mut Vec<T>) {
    items.sort();  // ERROR: T not Ord
}
```

### ✅ DO: Add trait bounds
```rust
fn sort_items<T: Ord>(items: &mut Vec<T>) {
    items.sort();  // OK
}
```

### ❌ DON'T: Conflicting derives and impls
```rust
#[derive(Clone)]
struct MyType;

impl Clone for MyType { }  // ERROR: duplicate
```

### ✅ DO: Use one approach
```rust
#[derive(Clone)]
struct MyType;
// or manually implement
```

### ❌ DON'T: Forget dyn for trait objects
```rust
let obj: &Animal = &dog;  // ERROR
```

### ✅ DO: Use dyn keyword
```rust
let obj: &dyn Animal = &dog;  // OK
```

## Trait Quick Reference

| Trait | Method | Purpose | Derive? |
|-------|--------|---------|---------|
| `Copy` | (implicit copy) | Zero-cost copy | Yes |
| `Clone` | `.clone()` | Explicit copy | Yes |
| `PartialEq` | `==`, `!=` | Equality | Yes |
| `Eq` | (extends PartialEq) | Total equality | Yes |
| `PartialOrd` | `<`, `>`, etc | Comparison | Yes |
| `Ord` | Compare | Total ordering | Yes |
| `Hash` | (for HashMap) | Hashing | Yes |
| `Debug` | `{:?}` | Debug print | Yes |
| `Display` | `{}` | User print | Manual |
| `Default` | `::default()` | Default value | Yes |

## Performance Considerations

### Copy (zero-cost)
- For small primitives (i32, f64, bool, char)
- Stack-allocated only
- Implicit, fast copying

### Clone (potentially expensive)
- For owned types (String, Vec)
- Explicit, controlled copying
- Deep copy of all data

### Trait Objects (dynamic dispatch)
- Vtable lookup at runtime
- Slower than generics
- Use when types are unknown at compile time

## Related Concepts

- **Generics** - Traits work with generic types
- **Structs** - Traits implement for types
- **Functions** - Trait bounds in signatures
- **Error Handling** - From trait for conversions

## Time Estimates

- Reading this takeaway: 10-15 minutes
- Reviewing patterns: 10 minutes
- Practice drills: 20-30 minutes
- Total: 40-55 minutes

## Practice Questions

1. What's the difference between Copy and Clone?
2. When should you derive vs manually implement a trait?
3. What does a trait bound do in a function signature?
4. Why do HashMap keys need Hash + Eq?
5. How do trait objects enable polymorphism?
6. What's the difference between Display and Debug?
7. When would you use where clause vs inline bounds?
8. How do you implement a trait for multiple types?

## Common Implementations

**Hashable struct for HashMap/HashSet:**
```rust
#[derive(Hash, Eq, PartialEq)]
struct Key { id: u32 }
```

**Comparable struct for sorting:**
```rust
#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Item { priority: u32 }
```

**With Display and Debug:**
```rust
#[derive(Debug)]
struct Data { field: i32 }

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Data({})", self.field)
    }
}
```

**Generic with bounds:**
```rust
fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

**Trait object polymorphism:**
```rust
let handlers: Vec<Box<dyn Handler>> = vec![
    Box::new(Type1),
    Box::new(Type2),
];
```

## Real-World Scenarios

| Scenario | Solution | Traits |
|----------|----------|--------|
| Use custom type as HashMap key | Derive traits | Hash, Eq, PartialEq |
| Sort custom type | Implement ordering | Ord, PartialOrd, Eq |
| Print custom type nicely | Implement Display | Display, Debug |
| Generic function for any comparable | Use trait bounds | PartialOrd, Ord |
| Flexible collection of different types | Use trait objects | dyn TraitName |
| Avoid copying expensive data | Use Clone | Clone |
| Copy efficiently (primitives only) | Derive Copy | Copy |

---

**Status**: Quick reference guide
**Importance**: ⭐⭐⭐⭐⭐ (Critical)
**Difficulty**: Intermediate
**Part of**: Module 02 - Standard Library
