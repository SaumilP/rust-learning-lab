# Java to Rust Cheat Sheet

Quick reference for "how do I do X in Rust?" when you know how to do it in Java.

## Basic Syntax

| Java | Rust | Notes |
|------|------|-------|
| `public class Foo {}` | `pub struct Foo {}` | No classes, use structs |
| `public void method() {}` | `pub fn method() {}` | Functions, not methods (unless in `impl`) |
| `public static final int X = 5;` | `pub const X: i32 = 5;` | Constants are `const` |
| `int x = 5;` | `let x = 5;` | Immutable by default |
| `int x = 5; x = 6;` | `let mut x = 5; x = 6;` | Mutable requires `mut` |
| `// comment` | `// comment` | Same |
| `/* block */` | `/* block */` | Same |
| `/** javadoc */` | `/// doc comment` | Docs use `///` |

## Types

| Java | Rust | Notes |
|------|------|-------|
| `byte` | `i8` | 8-bit signed |
| `short` | `i16` | 16-bit signed |
| `int` | `i32` | 32-bit signed (default) |
| `long` | `i64` | 64-bit signed |
| `float` | `f32` | 32-bit float |
| `double` | `f64` | 64-bit float (default) |
| `boolean` | `bool` | true/false |
| `char` | `char` | 4-byte Unicode |
| `String` | `String` | Owned string |
| `String` (as param) | `&str` | Borrowed string slice |
| `Integer` (boxed) | `i32` | No boxing |
| `null` | `Option::None` | No null in Rust |
| `Object` | `dyn Any` or generics | Rarely needed |

## Collections

| Java | Rust | Notes |
|------|------|-------|
| `ArrayList<T>` | `Vec<T>` | Growable array |
| `HashMap<K,V>` | `HashMap<K,V>` | Hash map |
| `HashSet<T>` | `HashSet<T>` | Hash set |
| `LinkedList<T>` | `LinkedList<T>` | Linked list (rarely used) |
| `T[]` | `[T; N]` or `&[T]` | Array or slice |
| `List.of(1,2,3)` | `vec![1,2,3]` | Literal syntax |

### Collection operations

| Java | Rust | Notes |
|------|------|-------|
| `list.add(x)` | `vec.push(x)` | Add element |
| `list.get(i)` | `vec[i]` or `vec.get(i)` | Access by index |
| `list.size()` | `vec.len()` | Length |
| `list.isEmpty()` | `vec.is_empty()` | Check if empty |
| `list.clear()` | `vec.clear()` | Remove all |
| `map.put(k, v)` | `map.insert(k, v)` | Insert |
| `map.get(k)` | `map.get(&k)` | Returns `Option<&V>` |
| `map.containsKey(k)` | `map.contains_key(&k)` | Check existence |

## Control Flow

| Java | Rust |
|------|------|
| `if (condition) { }` | `if condition { }` | No parentheses |
| `if (x) { } else { }` | `if x { } else { }` | Same structure |
| `condition ? a : b` | `if condition { a } else { b }` | Ternary as if expression |
| `while (condition) { }` | `while condition { }` | No parentheses |
| `for (int i = 0; i < n; i++)` | `for i in 0..n { }` | Range syntax |
| `for (Item item : items)` | `for item in &items { }` | Iterate by reference |
| `switch (x) { case 1: ... }` | `match x { 1 => ..., }` | Match is more powerful |
| `break;` | `break;` | Same |
| `continue;` | `continue;` | Same |
| `return x;` | `return x;` or just `x` | Last expression returns |

## Functions

| Java | Rust |
|------|------|
| `void method()` | `fn method()` | No return type |
| `int method()` | `fn method() -> i32` | Return type after `->` |
| `method(int x, String s)` | `fn method(x: i32, s: &str)` | Types after param |
| `public static void main` | `fn main()` | Entry point |
| `method(args...)` | `method(&args)` | Varargs use slices |

## Object-Oriented

| Java | Rust | Notes |
|------|------|-------|
| `class Foo { }` | `struct Foo { }` | Data structure |
| `class Foo { void method() }` | `impl Foo { fn method() }` | Methods in `impl` block |
| `new Foo()` | `Foo::new()` | Convention, not syntax |
| `this.field` | `self.field` | `self` not `this` |
| `interface Foo { }` | `trait Foo { }` | Traits are interfaces |
| `class Bar implements Foo` | `impl Foo for Bar { }` | Implementing traits |
| `extends Parent` | No inheritance | Use composition |
| `@Override` | No annotation | Compiler checks automatically |
| `super.method()` | No super | No inheritance |
| `instanceof Foo` | N/A | Use `match` or `Any` |

## Generics

| Java | Rust | Notes |
|------|------|-------|
| `<T>` | `<T>` | Same syntax |
| `<T extends Comparable>` | `<T: Ord>` | Trait bounds |
| `<T extends Foo & Bar>` | `<T: Foo + Bar>` | Multiple bounds |
| `List<?>` | `Vec<_>` | Type inference |
| `List<? extends T>` | No direct equivalent | Use trait objects |
| `List<? super T>` | No equivalent | Not needed |

## Error Handling

| Java | Rust | Notes |
|------|------|-------|
| `throw new Exception(msg)` | `Err(error)` or `panic!(msg)` | Errors are values |
| `throws IOException` | `-> Result<T, io::Error>` | Return type shows error |
| `try { } catch (E e) { }` | `match result { Ok(_) => ..., Err(e) => ... }` | Pattern matching |
| `try { } finally { }` | No finally | Use RAII/Drop trait |
| `Optional<T>` | `Option<T>` | Same concept |
| `opt.orElse(x)` | `opt.unwrap_or(x)` | Provide default |
| `opt.map(f)` | `opt.map(f)` | Same! |
| `opt.flatMap(f)` | `opt.and_then(f)` | Chain operations |

## Null Safety

| Java | Rust |
|------|------|
| `if (x != null)` | `if let Some(x) = option { }` |
| `obj.method()` (might NPE) | `obj.method()` (can't be null) |
| `Optional.of(x)` | `Some(x)` |
| `Optional.empty()` | `None` |
| `optional.get()` | `option.unwrap()` |
| `optional.orElse(def)` | `option.unwrap_or(def)` |

## Concurrency

| Java | Rust | Notes |
|------|------|-------|
| `new Thread(() -> { }).start()` | `thread::spawn( || { })` | Spawn thread |
| `synchronized(obj) { }` | `mutex.lock().unwrap()` | Explicit locking |
| `volatile` | `AtomicXxx` types | Atomic operations |
| `AtomicInteger` | `AtomicI32` | Same concept |
| `ExecutorService` | No built-in | Use `rayon` or `tokio` |
| `CompletableFuture<T>` | `impl Future<Output = T>` | Async result |
| `async/await` (Java 21+) | `async/await` | Similar syntax |

## Annotations/Attributes

| Java | Rust | Notes |
|------|------|-------|
| `@Override` | No annotation needed | Compiler checks automatically |
| `@Deprecated` | `#[deprecated]` | Deprecation warning |
| `@Test` | `#[test]` | Test annotation |
| `@SuppressWarnings` | `#[allow(lint_name)]` | Suppress lint |
| Custom annotations | Procedural macros | More complex |

## Common Patterns

### Creating objects

```java
// Java
Person person = new Person("Alice", 30);
```

```rust
// Rust
let person = Person { name: "Alice".to_string(), age: 30 };
// Or with constructor
let person = Person::new("Alice", 30);
```

### Builder pattern

```java
// Java
HttpClient client = HttpClient.newBuilder()
    .timeout(Duration.ofSeconds(30))
    .build();
```

```rust
// Rust
let client = HttpClient::builder()
    .timeout(Duration::from_secs(30))
    .build();
```

### Iterating

```java
// Java
for (String item : list) {
    System.out.println(item);
}

list.forEach(item -> System.out.println(item));
```

```rust
// Rust
for item in &list {
    println!("{}", item);
}

list.iter().for_each(|item| println!("{}", item));
```

### Filtering/Mapping

```java
// Java
List<String> result = list.stream()
    .filter(x -> x.length() > 3)
    .map(String::toUpperCase)
    .collect(Collectors.toList());
```

```rust
// Rust
let result: Vec<String> = list.iter()
    .filter(|x| x.len() > 3)
    .map(|x| x.to_uppercase())
    .collect();
```

### Reading files

```java
// Java
String content = Files.readString(Path.of("file.txt"));
```

```rust
// Rust
let content = fs::read_to_string("file.txt")?;
```

### JSON parsing

```java
// Java (with Jackson/Gson)
ObjectMapper mapper = new ObjectMapper();
Person person = mapper.readValue(json, Person.class);
```

```rust
// Rust (with serde_json)
let person: Person = serde_json::from_str(&json)?;
```

## Quick Conversions

### String conversions

```rust
// &str to String
let owned: String = "hello".to_string();
let owned: String = "hello".to_owned();
let owned = String::from("hello");

// String to &str
let borrowed: &str = &owned;
let borrowed: &str = owned.as_str();

// Formatting
let s = format!("Hello {}", name);  // Like String.format()
```

### Number parsing

```rust
// String to number
let num: i32 = "42".parse()?;  // Returns Result
let num: i32 = "42".parse().unwrap();  // Panics on error

// Number to String
let s = 42.to_string();
let s = format!("{}", 42);
```

### Collection creation

```rust
// Vec
let v = vec![1, 2, 3];
let v: Vec<i32> = Vec::new();

// HashMap
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", "value");

// From iterator
let v: Vec<i32> = (0..10).collect();
```

## Common Traits (Like Interfaces)

| Java Interface | Rust Trait | Notes |
|----------------|-----------|-------|
| `Comparable<T>` | `Ord`, `PartialOrd` | Comparison |
| `Cloneable` | `Clone` | Deep copy |
| `Iterable<T>` | `IntoIterator` | Can be iterated |
| `Iterator<T>` | `Iterator` | Same concept |
| `AutoCloseable` | `Drop` | Cleanup (automatic) |
| `Serializable` | `Serialize` (serde) | Serialization |
| `Runnable` | `Fn()`, `FnMut()`, `FnOnce()` | Callables |

## Memory Management

| Concept | Java | Rust |
|---------|------|------|
| **Allocation** | Always heap (for objects) | Stack by default, heap with `Box` |
| **Deallocation** | Garbage collector | Automatic (ownership) |
| **Sharing** | References everywhere | Ownership or borrowing |
| **Mutability** | Mutable by default | Immutable by default |
| **Null references** | Possible (NPE danger) | Impossible (Option instead) |

## Quick Tips

1. **When to use `&str` vs `String`**:
   - Parameter: `&str` (more flexible)
   - Return value: `String` (if owned) or `&str` (if borrowed)
   - Struct field: `String` (if owned)

2. **When to use `.clone()`**:
   - Only when you need an independent copy
   - Not to satisfy the borrow checker

3. **When to use `.unwrap()`**:
   - In `main()` for prototypes
   - In tests
   - Almost never in library code

4. **Error handling**:
   - Use `?` operator to propagate
   - Return `Result<T, E>` from functions
   - Use `match` or `if let` at boundaries

5. **Mutability**:
   - Default to immutable (`let x`)
   - Only add `mut` when needed (`let mut x`)
   - Prefer transforming data over mutation

---

**Pro tip**: Keep this cheat sheet handy while working on the mini-projects. The patterns will become second nature after a few weeks!
