# Traits and Polymorphism - Key Takeaways

## Defining Traits

```rust
trait Animal {
    fn make_sound(&self) -> String;
    fn name(&self) -> &str;
}
```

## Implementing Traits

```rust
impl Animal for Dog {
    fn make_sound(&self) -> String {
        "Woof!".to_string()
    }

    fn name(&self) -> &str {
        &self.name
    }
}
```

## Default Implementation

```rust
trait Shape {
    fn area(&self) -> f64;

    fn describe(&self) -> String {
        format!("Area: {}", self.area())
    }
}
```

## Trait Bounds

```rust
// Single bound
fn print_it<T: Display>(t: T) {
    println!("{}", t);
}

// Multiple bounds
fn compare<T: Display + Clone>(t: T) {
    println!("{}", t);
    let t2 = t.clone();
}

// Where clause
fn process<T>(t: T)
where
    T: Clone + Default,
{
}
```

## Trait Objects

```rust
// Trait object reference
let animal: &dyn Animal = &dog;

// Trait object box
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(dog),
    Box::new(cat),
];

// Use trait object
for animal in animals {
    println!("{}", animal.make_sound());
}
```

## Common Patterns

| Pattern | Use |
|---------|-----|
| Generic trait bounds | Compile-time specialization |
| Trait objects | Runtime polymorphism |
| Associated types | Type-safe generic traits |
| Default methods | Reduce boilerplate |
| Multiple traits | Composition |

## Associated Types

```rust
trait Container {
    type Item;

    fn get(&self) -> &Self::Item;
}

impl Container for Vec<String> {
    type Item = String;

    fn get(&self) -> &Self::Item {
        &self[0]
    }
}
```

## Static vs Dynamic Dispatch

```rust
// Static dispatch (compile time)
// Generic specialized for each type
fn process<T: Display>(t: T) {
    println!("{}", t);
}

// Dynamic dispatch (runtime)
// Single binary code, runtime type lookup
fn process(t: &dyn Display) {
    println!("{}", t);
}
```

## Common Trait Bounds

| Trait | Meaning |
|-------|---------|
| `Display` | Can print with {} |
| `Debug` | Can print with {:?} |
| `Clone` | Can explicitly copy |
| `Copy` | Automatically copied |
| `Default` | Has default value |
| `Eq / PartialEq` | Can compare |
| `Ord / PartialOrd` | Can order |
| `Hash` | Can hash |
| `Iterator` | Produces sequence |

## Important Notes

✓ Traits define behavior, not data
✓ Multiple traits can be implemented by one type
✓ Multiple traits can be required by one bound
✓ Trait objects erase type information
✓ Use generics when you know types at compile time
✓ Use trait objects when you need runtime polymorphism
✓ Associated types replace some generic parameters
✓ Implement common traits on your types

## Common Mistakes

1. ❌ Missing trait bound
   ```rust
   fn print<T>(t: T) {
       println!("{}", t);  // ERROR: no Display
   }
   ```

2. ❌ Unsized trait object
   ```rust
   fn process(t: dyn Trait) {}  // ERROR: unsized
   fn process(t: &dyn Trait) {}  // Correct
   ```

3. ❌ Orphan rule violation
   ```rust
   impl Display for String {}  // ERROR: both external
   ```

4. ❌ Type mismatch in objects
   ```rust
   let v: Vec<dyn Trait> = vec![];  // Mixed types
   ```

5. ❌ Forgetting self parameter
   ```rust
   trait T {
       fn method();  // Should be &self or &mut self
   }
   ```

## Quick Reference

```rust
// Define trait
trait Drawable {
    fn draw(&self);
}

// Implement
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle");
    }
}

// Trait bound (static dispatch)
fn process<T: Drawable>(item: &T) {
    item.draw();
}

// Trait object (dynamic dispatch)
fn process(item: &dyn Drawable) {
    item.draw();
}

// Multiple traits
trait Drawable: Clone {}

// Associated types
trait Container {
    type Item;
    fn get(&self) -> &Self::Item;
}

// Where clause
fn process<T>(t: T)
where
    T: Drawable + Clone,
{
}
```

## When to Use Each

| Situation | Choice |
|-----------|--------|
| Know all types at compile time | Generic + trait bound |
| Need multiple types at runtime | Trait object (dyn) |
| Performance critical | Static dispatch (generic) |
| Flexible plugin system | Trait object |
| API boundaries | Often trait object |

## Dispatching Strategies

**Static Dispatch (Generics):**
- Compile-time specialization
- No runtime overhead
- Larger binary
- Monomorphization

**Dynamic Dispatch (Trait Objects):**
- Runtime type checking
- Runtime overhead
- Smaller binary
- Virtual method tables

## Related Concepts

- Generics (using traits in generic code)
- Advanced trait patterns (GAT, HRTB)
- Async/await (futures, streams)
- Derive macros (auto-implementing traits)

