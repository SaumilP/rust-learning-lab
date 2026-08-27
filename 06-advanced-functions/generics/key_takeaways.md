# Generics - Key Takeaways

## Generic Functions

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

## Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

## Generic Enums

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Trait Bounds

```rust
// Single bound
fn print<T: Display>(t: T) {}

// Multiple bounds
fn process<T: Clone + Display>(t: T) {}

// Where clause
fn work<T>(t: T)
where
    T: Clone + Display,
{
}
```

## Multiple Type Parameters

```rust
struct Container<T, U> {
    first: T,
    second: U,
}

fn process<T, U>(t: T, u: U)
where
    T: Clone,
    U: Display,
{
}
```

## Generic Implementation

```rust
// All types
impl<T> Container<T> {
    fn new() -> Self { }
}

// Only types with Clone
impl<T: Clone> Container<T> {
    fn duplicate(&self) -> Self { }
}

// Specific types
impl Container<String> {
    fn to_uppercase(&self) -> String { }
}
```

## Lifetime Generics

```rust
struct Wrapper<'a, T> {
    reference: &'a T,
}

fn borrow<'a, T>(t: &'a T) -> &'a T {
    t
}
```

## Important Patterns

| Pattern | Use |
|---------|-----|
| `<T>` | Generic type parameter |
| `<T, U>` | Multiple types |
| `<T: Trait>` | Bounded generic |
| `<T: T1 + T2>` | Multiple bounds |
| `where T: Trait` | Complex bounds |
| `<'a>` | Lifetime parameter |

## Monomorphization

```
Generic code:
fn double<T: Mul>(t: T) -> T { t * t }

Compile time becomes:
fn double_i32(t: i32) -> i32 { t * t }
fn double_f64(t: f64) -> f64 { t * t }
```

## Common Trait Bounds

| Bound | Means |
|-------|-------|
| `Clone` | Can explicitly copy |
| `Copy` | Automatically copied |
| `Display` | Can print with {} |
| `Debug` | Can print with {:?} |
| `PartialOrd` | Can compare |
| `Eq` | Can test equality |
| `Hash` | Can hash |

## Generic Containers

```rust
struct Box<T> {
    item: T,
}

impl<T> Box<T> {
    fn new(item: T) -> Self {
        Box { item }
    }

    fn get(&self) -> &T {
        &self.item
    }

    fn map<U, F>(self, f: F) -> Box<U>
    where
        F: FnOnce(T) -> U,
    {
        Box {
            item: f(self.item),
        }
    }
}
```

## Generic Trait Implementation

```rust
trait Container {
    type Item;
    fn get(&self) -> &Self::Item;
}

impl<T> Container for Vec<T> {
    type Item = T;
    fn get(&self) -> &Self::Item {
        &self[0]
    }
}
```

## Important Notes

✓ Generics have zero runtime cost (monomorphization)
✓ Type checking happens at compile time
✓ Add only necessary trait bounds
✓ Where clauses improve readability for complex bounds
✓ Multiple type parameters are allowed
✓ Lifetimes are implicit when possible
✓ Generic code is instantiated once per concrete type

## Common Mistakes

1. ❌ Too broad generic
   ```rust
   fn process<T>(t: T) {
       println!("{}", t);  // ERROR
   }
   ```

2. ❌ Missing bounds
   ```rust
   fn clone_it<T>(t: T) {
       t.clone()  // ERROR: no Clone
   }
   ```

3. ❌ Mismatched types
   ```rust
   fn pair<T>(a: T, b: T) {
       (a, b)
   }
   pair(5, "hello");  // ERROR: types don't match
   ```

4. ❌ Over-constraining
   ```rust
   struct S<T: Clone + Debug + Display + Serialize> {
       item: T,  // Too many constraints
   }
   ```

5. ❌ Lifetime mismatch
   ```rust
   fn choose<'a>(a: &'a T, b: &T) -> &'a T {
       if true { a } else { b }  // ERROR: b lifetime
   }
   ```

## Quick Reference

```rust
// Generic function
fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// Generic struct
struct Pair<T, U> {
    first: T,
    second: U,
}

// Generic impl
impl<T: Clone> Pair<T, T> {
    fn clone_both(&self) -> (T, T) {
        (self.first.clone(), self.second.clone())
    }
}

// Where clause
fn process<T>(t: T)
where
    T: Clone + std::fmt::Display,
{
}

// Associated types
trait Container {
    type Item;
    fn item(&self) -> &Self::Item;
}
```

## Static vs Dynamic Dispatch

**Static (Generics):**
- Compile time specialization
- Zero runtime cost
- Larger binary

**Dynamic (Trait Objects):**
- Runtime polymorphism
- Runtime overhead
- Smaller binary

## When to Add Bounds

```rust
// No bounds - too generic
fn func<T>(t: T) { }  // Can't use T

// Add what you need
fn func<T: Clone>(t: T) {
    let t2 = t.clone();  // OK
}

// Multiple bounds
fn func<T: Clone + Display>(t: T) {
    let t2 = t.clone();
    println!("{}", t);  // OK
}
```

## Related Concepts

- Traits (bounding generics)
- Lifetimes (generic parameters)
- Procedural macros (generic code generation)
- Type system (advanced patterns)

