# Fundamentals: Java vs Rust

This guide shows how fundamental concepts differ between Java and Rust. Each section has Java code you're familiar with, followed by the Rust equivalent, and then an explanation of the key differences.

## Table of Contents

1. [Memory Management: GC vs Ownership](#memory-management-gc-vs-ownership)
2. [Null Safety: NullPointerException vs Option](#null-safety)
3. [Error Handling: Exceptions vs Result](#error-handling)
4. [Interfaces vs Traits](#interfaces-vs-traits)
5. [Generics: Type Erasure vs Monomorphization](#generics)
6. [Concurrency: Threads vs Async/Await](#concurrency)
7. [Collections: ArrayList vs Vec](#collections)

---

## Memory Management: GC vs Ownership

This is the big one. Understanding this unlocks everything else.

### Java approach: Garbage Collection

```java
// Java: Create objects, GC cleans up automatically
public class Person {
    private String name;

    public Person(String name) {
        this.name = name;
    }
}

public void example() {
    Person alice = new Person("Alice");
    Person bob = alice;  // Both reference the same object

    // Modify through one reference
    // Both alice and bob see the change

    // When method returns, GC will eventually clean up
}
```

**What's happening**:
- Objects live on the heap
- Multiple references can point to the same object
- Garbage collector tracks references and frees memory when no more references exist
- You never think about *when* memory is freed
- GC pauses can happen anytime

### Rust approach: Ownership

```rust
// Rust: One owner, explicit moves
struct Person {
    name: String,
}

fn example() {
    let alice = Person { name: String::from("Alice") };
    let bob = alice;  // alice is MOVED to bob

    // println!("{}", alice.name);  // ERROR! alice no longer valid
    println!("{}", bob.name);       // OK, bob owns it now

    // When bob goes out of scope, memory is freed immediately
}
```

**What's happening**:
- Each value has exactly one owner
- When owner goes out of scope, value is dropped (freed)
- Moving transfers ownership
- No garbage collector needed—cleanup is deterministic

### Key differences

| Aspect | Java | Rust |
|--------|------|------|
| **Who owns data?** | Garbage collector tracks it | One owner, explicit in code |
| **Multiple references?** | Yes, freely | No—need borrowing rules |
| **When is memory freed?** | Unpredictable (GC decides) | Predictable (when owner drops) |
| **Runtime cost?** | GC pauses | Zero overhead |
| **Mental model** | "Create and forget" | "Who owns this?" |

### When you want multiple references (borrowing)

```rust
// Rust: Borrowing instead of ownership
fn example() {
    let alice = Person { name: String::from("Alice") };

    // Borrow immutably (multiple readers OK)
    print_name(&alice);  // &alice is a reference
    print_name(&alice);  // Can borrow multiple times

    // alice still owns the data
    println!("{}", alice.name);
}

fn print_name(person: &Person) {
    println!("{}", person.name);
    // Doesn't own person, just borrows it
}
```

**The rules**:
1. One value, one owner
2. You can have many immutable borrows (`&T`)
3. OR you can have one mutable borrow (`&mut T`)
4. But not both at the same time

This prevents data races at compile time—it's impossible to have one thread writing while another reads.

---

## Null Safety

In Java, any object reference can be `null`. This leads to runtime `NullPointerException` that we've all debugged at 2 AM.

### Java: Null everywhere

```java
// Java: null is always possible
public String getName(Map<String, Person> people, String id) {
    Person person = people.get(id);  // Might return null

    if (person != null) {  // Easy to forget this check
        return person.getName();
    }
    return "Unknown";
}

// Or with Optional (Java 8+)
public Optional<String> getNameOptional(Map<String, Person> people, String id) {
    return Optional.ofNullable(people.get(id))
        .map(Person::getName);
}
```

### Rust: Option type

```rust
// Rust: Option makes "might not exist" explicit
use std::collections::HashMap;

fn get_name(people: &HashMap<String, Person>, id: &str) -> String {
    // get() returns Option<&Person>
    match people.get(id) {
        Some(person) => person.name.clone(),
        None => String::from("Unknown"),
    }
}

// Or more concisely with combinators
fn get_name_concise(people: &HashMap<String, Person>, id: &str) -> String {
    people.get(id)
        .map(|person| person.name.clone())
        .unwrap_or_else(|| String::from("Unknown"))
}
```

**Option is an enum**:

```rust
enum Option<T> {
    Some(T),    // Has a value
    None,       // No value
}
```

### Key differences

| Aspect | Java | Rust |
|--------|------|------|
| **Null values** | Any object can be null | No null—use Option<T> |
| **Type system** | Hidden (not in type) | Explicit (in the type) |
| **Runtime errors** | NullPointerException | Compile error if not handled |
| **Verbosity** | null checks are optional | Must handle None case |

**Why this matters**:

In Java:
```java
String name = person.getName();  // Might explode at runtime
```

In Rust:
```rust
let name: Option<String> = person.get_name();  // Type tells you it might not exist
```

The type system forces you to handle the "missing value" case. No more null pointer surprises.

---

## Error Handling

Java has exceptions (checked and unchecked). Rust has Result types. This is a fundamental difference in philosophy.

### Java: Exceptions

```java
// Java: Exceptions might be hidden
public String readFile(String path) throws IOException {
    // Checked exception—must declare or handle
    return Files.readString(Path.of(path));
}

public int parseInt(String s) {
    // Unchecked exception—can forget to handle
    return Integer.parseInt(s);  // Might throw NumberFormatException
}

// Usage
try {
    String content = readFile("config.txt");
    int value = parseInt(content);
} catch (IOException e) {
    System.err.println("Failed to read: " + e.getMessage());
} catch (NumberFormatException e) {
    System.err.println("Invalid number: " + e.getMessage());
}
```

**Problems with this approach**:
- Unchecked exceptions can hide anywhere
- Easy to forget error handling
- Control flow via exceptions is invisible
- Stack unwinding has runtime cost

### Rust: Result type

```rust
use std::fs;
use std::num::ParseIntError;

// Rust: Errors are explicit in return type
fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

// Usage with explicit matching
fn example_explicit() -> Result<i32, String> {
    let content = match read_file("config.txt") {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to read: {}", e)),
    };

    let value = match parse_int(&content) {
        Ok(v) => v,
        Err(e) => return Err(format!("Invalid number: {}", e)),
    };

    Ok(value)
}

// Usage with ? operator (cleaner)
fn example_concise() -> Result<i32, Box<dyn std::error::Error>> {
    let content = read_file("config.txt")?;  // Propagates error if it occurs
    let value = parse_int(&content)?;
    Ok(value)
}
```

**Result is an enum**:

```rust
enum Result<T, E> {
    Ok(T),     // Success value
    Err(E),    // Error value
}
```

### Key differences

| Aspect | Java | Rust |
|--------|------|------|
| **Error visibility** | Hidden (unchecked) or verbose (checked) | Always in type signature |
| **Handling requirement** | Optional for unchecked | Compiler enforces handling |
| **Control flow** | Invisible (exception jumps) | Explicit (return values) |
| **Performance** | Stack unwinding cost | Zero cost (it's just an enum) |
| **Multiple errors** | Multiple catch blocks | Pattern match or ? operator |

### The ? operator

This is the Rust equivalent of Java's "propagate exception". If you get an `Err`, it returns early. If you get `Ok`, it unwraps the value.

```rust
// These are equivalent:
fn example1() -> Result<String, std::io::Error> {
    let content = match read_file("data.txt") {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    Ok(content)
}

fn example2() -> Result<String, std::io::Error> {
    let content = read_file("data.txt")?;  // Same as above!
    Ok(content)
}
```

Much cleaner than Java's try-catch for simple error propagation.

---

## Interfaces vs Traits

Both Java interfaces and Rust traits define behavior contracts. But traits are more powerful.

### Java: Interfaces

```java
// Java interface
public interface Drawable {
    void draw();
}

// Implementation
public class Circle implements Drawable {
    private double radius;

    public Circle(double radius) {
        this.radius = radius;
    }

    @Override
    public void draw() {
        System.out.println("Drawing circle with radius " + radius);
    }
}

// Usage
public void renderShape(Drawable shape) {
    shape.draw();  // Runtime dispatch (virtual method call)
}
```

### Rust: Traits

```rust
// Rust trait
trait Drawable {
    fn draw(&self);
}

// Implementation
struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

// Usage with generics (static dispatch)
fn render_shape<T: Drawable>(shape: &T) {
    shape.draw();  // Compile-time dispatch (inlined!)
}

// Or with trait objects (dynamic dispatch)
fn render_shape_dyn(shape: &dyn Drawable) {
    shape.draw();  // Runtime dispatch (like Java)
}
```

### Key differences

| Aspect | Java | Rust |
|--------|------|------|
| **Dispatch** | Always runtime (virtual) | Static by default, dynamic opt-in |
| **Performance** | Virtual call overhead | Zero cost with generics |
| **Adding to existing types** | Can't (must own type) | Can! (impl trait for any type) |
| **Default methods** | Yes (Java 8+) | Yes (default implementations) |
| **Associated types** | No | Yes (powerful feature) |

### Extending types you don't own

This is huge. In Rust, you can implement traits for types from other libraries:

```rust
// Rust: Implement Display for a type you didn't write
use std::fmt;

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle(radius: {})", self.radius)
    }
}

// Now you can print it
let circle = Circle { radius: 5.0 };
println!("{}", circle);  // Prints: Circle(radius: 5)
```

You can't do this in Java without wrapper classes.

---

## Generics

Both languages have generics, but they work completely differently under the hood.

### Java: Type Erasure

```java
// Java generics
public class Box<T> {
    private T value;

    public Box(T value) {
        this.value = value;
    }

    public T getValue() {
        return value;
    }
}

// Usage
Box<Integer> intBox = new Box<>(42);
Box<String> strBox = new Box<>("hello");

// At runtime, both are just Box—type info erased
```

**What happens**:
- Generics only exist at compile time
- At runtime, everything is Object
- One class implementation for all types
- Can't get type info at runtime

### Rust: Monomorphization

```rust
// Rust generics
struct Box<T> {
    value: T,
}

impl<T> Box<T> {
    fn new(value: T) -> Self {
        Box { value }
    }

    fn get(&self) -> &T {
        &self.value
    }
}

// Usage
let int_box = Box::new(42);
let str_box = Box::new("hello");
```

**What happens**:
- Compiler generates separate code for each type
- `Box<i32>` and `Box<String>` are different types at runtime
- Zero runtime overhead—direct function calls
- Binary size is larger (tradeoff)

### With trait bounds

```java
// Java: Bounded type parameters
public <T extends Comparable<T>> T max(List<T> items) {
    return items.stream()
        .max(Comparator.naturalOrder())
        .orElse(null);
}
```

```rust
// Rust: Trait bounds
fn max<T: Ord + Clone>(items: &[T]) -> Option<T> {
    items.iter()
        .max()
        .cloned()
}

// Alternative syntax (more readable for complex bounds)
fn max_verbose<T>(items: &[T]) -> Option<T>
where
    T: Ord + Clone,
{
    items.iter().max().cloned()
}
```

### Key differences

| Aspect | Java | Rust |
|--------|------|------|
| **Compile strategy** | Type erasure | Monomorphization |
| **Runtime overhead** | Some (boxing, casts) | None (static dispatch) |
| **Binary size** | Smaller | Larger |
| **Type info at runtime** | Lost | Preserved |
| **Null handling** | Can return null | Return Option<T> |

---

## Concurrency

This is where Rust really shines. The type system prevents data races at compile time.

### Java: Threads and Locks

```java
// Java: Shared mutable state requires locks
public class Counter {
    private int count = 0;

    public synchronized void increment() {  // Must remember synchronized!
        count++;
    }

    public synchronized int getCount() {
        return count;
    }
}

// Usage
Counter counter = new Counter();
ExecutorService executor = Executors.newFixedThreadPool(4);

for (int i = 0; i < 1000; i++) {
    executor.submit(() -> counter.increment());
}

executor.shutdown();
executor.awaitTermination(1, TimeUnit.MINUTES);
System.out.println("Count: " + counter.getCount());
```

**Problems**:
- Easy to forget `synchronized`
- Deadlocks possible
- No compile-time safety
- GC adds unpredictability

### Rust: Fearless Concurrency

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Rust: Type system enforces safety
fn main() {
    // Arc = Atomic Reference Counted (thread-safe shared ownership)
    // Mutex = Mutual exclusion (like synchronized)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Count: {}", *counter.lock().unwrap());
}
```

**If you forget the Mutex**:
```rust
let counter = Arc::new(0);  // Just Arc, no Mutex

// This won't compile!
// ERROR: cannot mutate through shared reference
```

The compiler catches your concurrency bugs.

### Async/Await

```java
// Java: CompletableFuture (verbose)
CompletableFuture<String> future = CompletableFuture.supplyAsync(() -> {
    return fetchData();
});

future.thenAccept(data -> {
    System.out.println("Got: " + data);
});
```

```rust
// Rust: async/await (cleaner)
use tokio;

#[tokio::main]
async fn main() {
    let data = fetch_data().await;
    println!("Got: {}", data);
}

async fn fetch_data() -> String {
    // Async operation
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    String::from("data")
}
```

### Key differences

| Aspect | Java | Rust |
|--------|------|------|
| **Data races** | Possible (runtime error) | Impossible (compile error) |
| **Thread safety** | Manual (synchronized) | Automatic (Send/Sync traits) |
| **Deadlocks** | Possible | Still possible (not magic) |
| **Async model** | OS threads by default | Async tasks (lighter weight) |
| **Scaling** | 1000s of threads | 100,000s of tasks |

---

## Collections

Finally, something that looks similar!

### Java: ArrayList

```java
// Java collections
List<String> names = new ArrayList<>();
names.add("Alice");
names.add("Bob");

for (String name : names) {
    System.out.println(name);
}

// Streams
names.stream()
    .filter(name -> name.startsWith("A"))
    .map(String::toUpperCase)
    .forEach(System.out::println);
```

### Rust: Vec

```rust
// Rust collections
let mut names = Vec::new();
names.push(String::from("Alice"));
names.push(String::from("Bob"));

for name in &names {
    println!("{}", name);
}

// Iterators
names.iter()
    .filter(|name| name.starts_with("A"))
    .map(|name| name.to_uppercase())
    .for_each(|name| println!("{}", name));
```

### Key differences

| Aspect | Java | Rust |
|--------|------|------|
| **Syntax** | Very similar! | Very similar! |
| **Memory** | Heap allocated | Heap allocated |
| **Growth** | Automatic | Automatic |
| **Ownership** | References | Owned values |
| **Iteration** | for-each, streams | for loop, iterators |
| **Laziness** | Streams are lazy | Iterators are lazy |

The main difference is ownership:

```rust
let names = vec![String::from("Alice"), String::from("Bob")];
let first = names[0];  // ERROR: Can't move out of Vec

// Instead:
let first = &names[0];        // Borrow
let first = names[0].clone(); // Clone
```

---

## Summary

Here's the quick reference:

| Concept | Java | Rust | Mental shift |
|---------|------|------|--------------|
| **Memory** | GC manages it | Ownership manages it | Think about who owns what |
| **Null** | Everywhere | Option<T> | Make "nothing" explicit |
| **Errors** | Exceptions | Result<T, E> | Errors are values |
| **Interfaces** | Runtime dispatch | Compile-time dispatch | Zero cost abstractions |
| **Generics** | Type erasure | Monomorphization | Real types at runtime |
| **Concurrency** | Locks (manual) | Ownership (automatic) | Let compiler prevent races |
| **Collections** | Similar | Similar | Watch out for ownership |

The biggest mental shift: Rust makes you think about ownership and lifetimes explicitly. It's more work upfront, but prevents entire classes of bugs.

Next up: Check out the MINI_PROJECTS to practice these concepts!
