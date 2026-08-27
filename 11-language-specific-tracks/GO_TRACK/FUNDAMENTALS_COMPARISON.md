# Fundamentals: Go vs Rust

This guide shows how fundamental concepts differ between Go and Rust. Each section has Go code you're familiar with, followed by the Rust equivalent, and then an explanation of the key differences.

## Table of Contents

1. [Memory Management: GC vs Ownership](#memory-management-gc-vs-ownership)
2. [nil vs Option](#nil-vs-option)
3. [Error Handling: if err != nil vs Result](#error-handling-if-err-nil-vs-result)
4. [Interfaces vs Traits](#interfaces-vs-traits)
5. [Goroutines vs Async/Await](#goroutines-vs-asyncawait)
6. [Channels](#channels)
7. [defer vs Drop (RAII)](#defer-vs-drop-raii)
8. [Type System: interface{} vs Generics](#type-system-interface-vs-generics)
9. [Mutability: Mutable by Default vs Explicit mut](#mutability-mutable-by-default-vs-explicit-mut)
10. [Struct Embedding vs Composition](#struct-embedding-vs-composition)

---

## Memory Management: GC vs Ownership

This is the biggest conceptual difference. Go uses garbage collection. Rust uses ownership.

### Go approach: Garbage collection

```go
// Go: Allocate freely, GC handles cleanup
type Person struct {
    Name string
    Age  int
}

func example() {
    alice := &Person{Name: "Alice", Age: 30}
    bob := alice       // Both point to same memory
    charlie := alice   // More pointers, no problem!

    // All three variables reference the same Person
    alice.Name = "Alicia"
    fmt.Println(bob.Name)  // Alicia

    // When function returns, GC will eventually clean up
}
```

**What's happening**:
- Allocate with `new` or `&Type{}`
- Multiple pointers can reference the same data
- Garbage collector tracks references
- Memory freed when no more references exist (eventually)
- GC pauses can happen unpredictably
- You never think about *when* memory is freed

### Rust approach: Ownership

```rust
// Rust: One owner, explicit transfers
struct Person {
    name: String,
    age: u32,
}

fn example() {
    let alice = Person { name: String::from("Alice"), age: 30 };
    let bob = alice;  // Ownership MOVED to bob

    // println!("{}", alice.name);  // ERROR! alice no longer valid
    println!("{}", bob.name);       // OK, bob owns it

    // When bob goes out of scope, Person is dropped immediately
}
```

**What's happening**:
- Each value has exactly one owner
- Assigning moves ownership (previous owner becomes invalid)
- When owner goes out of scope, value is dropped (freed) immediately
- No garbage collector—cleanup is deterministic
- No GC pauses ever

### When you need multiple references (borrowing)

```rust
// Rust: Borrowing instead of ownership
fn example() {
    let alice = Person { name: String::from("Alice"), age: 30 };

    // Borrow immutably (read-only access)
    print_person(&alice);
    print_person(&alice);  // Can borrow multiple times

    // alice still owns the data
    println!("{}", alice.name);
}

fn print_person(person: &Person) {
    println!("{}", person.name);
}
```

### Key differences

| Aspect | Go | Rust |
|--------|-----|------|
| **Multiple references?** | Yes, freely | Yes, via borrowing |
| **Cleanup timing** | Unpredictable (GC decides) | Predictable (when owner drops) |
| **Runtime cost** | GC pauses | Zero overhead |
| **Memory layout control** | Limited | Full control |
| **Mental model** | "Allocate and forget" | "Who owns this?" |

---

## nil vs Option

Go's `nil` can appear anywhere. Rust's `Option` makes absence explicit.

### Go approach: nil everywhere

```go
// Go: nil can hide anywhere
type User struct {
    Name  string
    Email string
}

func findUser(id int) *User {
    if id == 1 {
        return &User{Name: "Alice", Email: "alice@example.com"}
    }
    return nil  // Might return nil
}

// Easy to forget nil check
func main() {
    user := findUser(42)
    fmt.Println(user.Name)  // Panic: nil pointer dereference!

    // Must remember to check
    user = findUser(42)
    if user != nil {
        fmt.Println(user.Name)
    }
}
```

**What's happening**:
- `nil` is a valid value for any pointer, interface, slice, map, channel, or function type
- Forgetting to check causes runtime panics
- Very common source of bugs
- No compile-time enforcement

### Rust approach: Option type

```rust
// Rust: Option makes absence explicit in type signature
struct User {
    name: String,
    email: String,
}

fn find_user(id: u32) -> Option<User> {
    if id == 1 {
        Some(User {
            name: String::from("Alice"),
            email: String::from("alice@example.com"),
        })
    } else {
        None
    }
}

fn main() {
    let user = find_user(42);

    // Can't access directly—must handle the Option
    // println!("{}", user.name);  // ERROR! user is Option<User>, not User

    // Must explicitly handle None case
    match user {
        Some(u) => println!("{}", u.name),
        None => println!("User not found"),
    }

    // Or use if let
    if let Some(u) = find_user(42) {
        println!("{}", u.name);
    }

    // Or unwrap_or for defaults
    let user = find_user(42).unwrap_or(User {
        name: String::from("Guest"),
        email: String::from("guest@example.com"),
    });
}
```

**Option is an enum**:

```rust
enum Option<T> {
    Some(T),   // Has a value
    None,      // No value
}
```

### Key differences

| Aspect | Go | Rust |
|--------|-----|------|
| **Null representation** | `nil` | `None` (enum variant) |
| **Type safety** | Any pointer can be nil | Explicit in type signature |
| **Forgotten check** | Runtime panic | Compile error |
| **Checking** | `if x != nil` | `match`, `if let`, or methods |
| **Default values** | Manual check | `unwrap_or`, `unwrap_or_else` |

---

## Error Handling: if err != nil vs Result

Both Go and Rust make errors explicit values, but Rust enforces handling at compile time.

### Go approach: if err != nil

```go
// Go: Errors are values, but easy to ignore
package main

import (
    "errors"
    "fmt"
    "os"
)

func readFile(path string) (string, error) {
    data, err := os.ReadFile(path)
    if err != nil {
        return "", err
    }
    return string(data), nil
}

func parseConfig(path string) (string, error) {
    content, err := readFile(path)
    if err != nil {
        return "", fmt.Errorf("failed to read config: %w", err)
    }
    // Process content...
    return content, nil
}

// Can forget to check error (compiles but dangerous)
func dangerous() {
    content, _ := readFile("config.txt")  // Ignoring error!
    fmt.Println(content)
}
```

**What's happening**:
- Functions return `(result, error)`
- Must manually check `if err != nil`
- Easy to ignore errors with `_`
- No compile-time enforcement
- Verbose but explicit

### Rust approach: Result type

```rust
use std::fs;
use std::io;

// Rust: Errors are in the return type
fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn parse_config(path: &str) -> Result<String, String> {
    // Using match for explicit handling
    let content = match read_file(path) {
        Ok(s) => s,
        Err(e) => return Err(format!("failed to read config: {}", e)),
    };
    // Process content...
    Ok(content)
}

// Using ? operator (like Go's if err != nil { return err })
fn parse_config_concise(path: &str) -> Result<String, io::Error> {
    let content = read_file(path)?;  // Returns early if Err
    Ok(content)
}

// Can't forget to check error—won't compile
fn main() {
    // let content = read_file("config.txt");  // ERROR! Result not handled

    // Must handle the Result
    match read_file("config.txt") {
        Ok(content) => println!("{}", content),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

**Result is an enum**:

```rust
enum Result<T, E> {
    Ok(T),    // Success value
    Err(E),   // Error value
}
```

### The ? operator

The `?` operator is Rust's equivalent to Go's error propagation pattern:

```go
// Go pattern
if err != nil {
    return err
}
```

```rust
// Rust equivalent with ?
let value = fallible_operation()?;
// If Err, returns early. If Ok, unwraps the value.
```

### Key differences

| Aspect | Go | Rust |
|--------|-----|------|
| **Error visibility** | In return values | In type signature |
| **Handling requirement** | Optional (can use `_`) | Mandatory (compiler enforces) |
| **Propagation** | Manual `if err != nil { return }` | `?` operator |
| **Performance** | Return value | Zero cost (just an enum) |
| **Type safety** | Error type is `error` interface | Error type is generic |

---

## Interfaces vs Traits

Go's interfaces are implicit—any type with the right methods satisfies the interface. Rust's traits are explicit.

### Go approach: Implicit interfaces

```go
// Go: Interfaces are satisfied implicitly
package main

import "fmt"

type Reader interface {
    Read(p []byte) (n int, err error)
}

type File struct {
    data []byte
}

// File implements Reader implicitly—no declaration needed
func (f *File) Read(p []byte) (n int, err error) {
    n = copy(p, f.data)
    return n, nil
}

func process(r Reader) {
    buf := make([]byte, 100)
    n, _ := r.Read(buf)
    fmt.Printf("Read %d bytes\n", n)
}

func main() {
    f := &File{data: []byte("hello")}
    process(f)  // File satisfies Reader automatically
}
```

**What's happening**:
- Interfaces defined separately from implementation
- Any type with matching methods satisfies the interface
- No explicit declaration needed
- Very flexible—can make third-party types satisfy your interfaces
- Duck typing: "if it walks like a duck..."

### Rust approach: Explicit traits

```rust
// Rust: Traits must be explicitly implemented
use std::io;

trait Reader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

struct File {
    data: Vec<u8>,
}

// Must explicitly declare "impl Reader for File"
impl Reader for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = buf.len().min(self.data.len());
        buf[..n].copy_from_slice(&self.data[..n]);
        Ok(n)
    }
}

fn process<R: Reader>(reader: &mut R) {
    let mut buf = vec![0u8; 100];
    let n = reader.read(&mut buf).unwrap();
    println!("Read {} bytes", n);
}

fn main() {
    let mut f = File { data: b"hello".to_vec() };
    process(&mut f);  // Type-checked at compile time
}
```

**What's happening**:
- Traits defined separately
- Must explicitly `impl Trait for Type`
- Compiler checks all trait requirements
- Cannot implement external traits for external types (orphan rule)
- More verbose but more explicit

### Trait bounds and generics

```rust
// Rust: Trait bounds enable generic programming
use std::fmt::Display;

fn print_it<T: Display>(value: T) {
    println!("{}", value);
}

// Multiple trait bounds
fn process<T: Reader + Display>(item: T) {
    // T must implement both Reader and Display
}

// Where clauses for complex bounds
fn complex<T, U>(a: T, b: U)
where
    T: Reader + Display,
    U: Clone + Send,
{
    // Complex trait requirements
}
```

### Key differences

| Aspect | Go | Rust |
|--------|-----|------|
| **Declaration** | Implicit | Explicit (`impl Trait`) |
| **Checking** | Runtime (interface assertions) | Compile-time |
| **Flexibility** | Very high (duck typing) | High (but requires declaration) |
| **Third-party types** | Can satisfy your interfaces | Cannot (orphan rule) |
| **Dispatch** | Dynamic (interface values) | Static (generics) or dynamic (`dyn Trait`) |

---

## Goroutines vs Async/Await

Go's goroutines are simple and runtime-managed. Rust's async is explicit and gives you more control.

### Go approach: Goroutines

```go
// Go: Super simple concurrency
package main

import (
    "fmt"
    "time"
)

func doWork(id int) {
    time.Sleep(1 * time.Second)
    fmt.Printf("Work %d done\n", id)
}

func main() {
    // Just add "go" and you're concurrent!
    for i := 0; i < 10; i++ {
        go doWork(i)
    }

    time.Sleep(2 * time.Second)  // Wait for goroutines
}
```

**What's happening**:
- `go func()` spawns a goroutine
- Runtime schedules goroutines on OS threads
- Very lightweight (2KB stack, grows as needed)
- Can have millions of goroutines
- Simple mental model

### Rust approach: Async/await

```rust
// Rust: More explicit async
use tokio::time::{sleep, Duration};

async fn do_work(id: u32) {
    sleep(Duration::from_secs(1)).await;
    println!("Work {} done", id);
}

#[tokio::main]  // Sets up async runtime
async fn main() {
    let mut tasks = vec![];

    for i in 0..10 {
        // Spawn async tasks
        let task = tokio::spawn(async move {
            do_work(i).await;
        });
        tasks.push(task);
    }

    // Wait for all tasks
    for task in tasks {
        task.await.unwrap();
    }
}
```

**What's happening**:
- Functions marked `async` return `Future`
- Must `.await` to execute async functions
- Need an async runtime (Tokio, async-std)
- Tasks are lightweight (similar to goroutines)
- More explicit, more control

### Threads vs Async

```rust
// Rust also has OS threads (like Go's goroutines, but heavier)
use std::thread;
use std::time::Duration;

fn main() {
    let mut handles = vec![];

    for i in 0..10 {
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            println!("Work {} done", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

### Key differences

| Aspect | Go | Rust |
|--------|-----|------|
| **Starting** | `go func()` | `tokio::spawn(async {})` or `thread::spawn` |
| **Syntax** | Implicit | Explicit (`async`/`.await`) |
| **Runtime** | Built-in | Choose your runtime (Tokio, etc.) |
| **Lightweight** | Yes (goroutines) | Yes (async tasks) |
| **When to use** | Default for concurrency | Async for I/O, threads for CPU |

---

## Channels

Both Go and Rust have channels for communication between concurrent tasks. They work similarly but with different syntax.

### Go approach: Channels

```go
// Go: Channels are first-class citizens
package main

import "fmt"

func main() {
    // Unbuffered channel
    ch := make(chan int)

    go func() {
        ch <- 42  // Send
    }()

    value := <-ch  // Receive
    fmt.Println(value)

    // Buffered channel
    buffered := make(chan string, 3)
    buffered <- "a"
    buffered <- "b"
    buffered <- "c"

    close(buffered)

    // Receive until closed
    for msg := range buffered {
        fmt.Println(msg)
    }
}
```

**What's happening**:
- `make(chan T)` creates unbuffered channel
- `make(chan T, n)` creates buffered channel
- `<-` operator for send and receive
- `close(ch)` closes channel
- Can range over channels
- Very ergonomic

### Rust approach: Channels

```rust
// Rust: Channels from std::sync::mpsc or async libraries
use std::sync::mpsc;
use std::thread;

fn main() {
    // Create channel (multi-producer, single-consumer)
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(42).unwrap();  // Send
    });

    let value = rx.recv().unwrap();  // Receive
    println!("{}", value);

    // Async channels with Tokio
    use_async_channels();
}

#[tokio::main]
async fn use_async_channels() {
    use tokio::sync::mpsc;

    let (tx, mut rx) = mpsc::channel(3);  // Buffered

    tokio::spawn(async move {
        tx.send("a").await.unwrap();
        tx.send("b").await.unwrap();
        tx.send("c").await.unwrap();
        // Sender dropped, channel closes
    });

    // Receive until closed
    while let Some(msg) = rx.recv().await {
        println!("{}", msg);
    }
}
```

### Key differences

| Aspect | Go | Rust |
|--------|-----|------|
| **Syntax** | `<-` operator | `.send()` / `.recv()` methods |
| **Built-in** | Yes | std::sync::mpsc (sync), or crate (async) |
| **Type** | Can send/receive on same channel | Split into sender/receiver |
| **Closing** | Explicit `close(ch)` | Drop sender(s) |
| **Buffering** | `make(chan T, n)` | `mpsc::channel()` or `mpsc::channel(n)` |

---

## defer vs Drop (RAII)

Go's `defer` schedules cleanup code. Rust uses RAII—cleanup happens automatically when values go out of scope.

### Go approach: defer

```go
// Go: defer for cleanup
package main

import (
    "fmt"
    "os"
)

func processFile(path string) error {
    file, err := os.Open(path)
    if err != nil {
        return err
    }
    defer file.Close()  // Deferred until function returns

    // Work with file...
    // file.Close() called automatically on return

    return nil
}

func example() {
    defer fmt.Println("3")
    defer fmt.Println("2")
    defer fmt.Println("1")
    fmt.Println("0")
    // Output: 0, 1, 2, 3 (LIFO order)
}
```

**What's happening**:
- `defer` schedules function call for when surrounding function returns
- Multiple defers execute in LIFO order
- Explicit—you write the cleanup code
- Runs even if function panics
- Very clear and readable

### Rust approach: Drop trait (RAII)

```rust
// Rust: Drop trait for automatic cleanup
use std::fs::File;
use std::io;

fn process_file(path: &str) -> io::Result<()> {
    let file = File::open(path)?;

    // Work with file...

    // file.close() called automatically when file goes out of scope
    Ok(())
}  // <- Drop runs here

// Custom Drop implementation
struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Cleaning up {}", self.name);
    }
}

fn example() {
    let r1 = Resource { name: String::from("resource1") };
    let r2 = Resource { name: String::from("resource2") };
    println!("Resources created");
    // Drop runs in reverse order: r2, then r1
}
// Output:
// Resources created
// Cleaning up resource2
// Cleaning up resource1
```

### Manual cleanup with drop()

```rust
fn main() {
    let resource = Resource { name: String::from("early") };

    // Force early drop
    drop(resource);

    println!("Resource already cleaned up");
}
```

### Key differences

| Aspect | Go | Rust |
|--------|-----|------|
| **Syntax** | `defer func()` | Automatic (Drop trait) |
| **When** | Function return | Scope exit |
| **Order** | LIFO (stacked defers) | LIFO (reverse declaration order) |
| **Explicitness** | Very explicit | Implicit (but predictable) |
| **Custom cleanup** | Write defer manually | Implement Drop trait |

---

## Type System: interface{} vs Generics

Go historically used `interface{}` (now `any`) for generic code. Rust has powerful generics with zero runtime cost.

### Go approach: interface{} and any

```go
// Go: interface{} can hold any type
package main

import "fmt"

func printAnything(value interface{}) {
    fmt.Println(value)
}

func main() {
    printAnything(42)
    printAnything("hello")
    printAnything(true)

    // Need type assertion to get value back
    var x interface{} = "hello"
    s := x.(string)  // Type assertion
    fmt.Println(s)

    // Safe type assertion
    if s, ok := x.(string); ok {
        fmt.Println(s)
    }
}

// Go 1.18+ generics
func printGeneric[T any](value T) {
    fmt.Println(value)
}

func max[T comparable](a, b T) T {
    if a > b {  // Requires comparable constraint
        return a
    }
    return b
}
```

**What's happening**:
- `interface{}` (or `any`) can hold any type
- Need type assertions to get concrete type back
- Runtime overhead (type information stored)
- Go 1.18+ has generics (but simpler than Rust's)

### Rust approach: Generics with traits

```rust
// Rust: Generics with trait bounds
use std::fmt::Display;

fn print_anything<T: Display>(value: T) {
    println!("{}", value);
}

fn main() {
    print_anything(42);
    print_anything("hello");
    print_anything(true);
}

// Multiple trait bounds
fn process<T: Display + Clone>(value: T) {
    let copy = value.clone();
    println!("{}", copy);
}

// Where clauses for complex bounds
fn complex<T, U>(a: T, b: U)
where
    T: Display + Clone,
    U: PartialOrd + Copy,
{
    // Use a and b
}

// Generic structs
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
}
```

### Monomorphization

Rust generates specialized code for each concrete type:

```rust
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// Compiler generates:
// fn add_i32(a: i32, b: i32) -> i32 { a + b }
// fn add_f64(a: f64, b: f64) -> f64 { a + b }
// etc.
```

### Key differences

| Aspect | Go | Rust |
|--------|-----|------|
| **Generic syntax** | `[T any]` or `interface{}` | `<T>` with trait bounds |
| **Runtime cost** | interface{} has overhead | Zero cost (monomorphization) |
| **Type safety** | Need type assertions | Checked at compile time |
| **Expressiveness** | Simple constraints | Powerful trait system |
| **Code generation** | Single implementation | Specialized per type |

---

## Mutability: Mutable by Default vs Explicit mut

Go variables and pointers are mutable by default. Rust variables are immutable unless marked `mut`.

### Go approach: Mutable by default

```go
// Go: Everything is mutable
package main

import "fmt"

func main() {
    // Variables can be reassigned
    x := 5
    x = 10

    // Struct fields can be changed
    type Person struct {
        Name string
        Age  int
    }

    person := Person{Name: "Alice", Age: 30}
    person.Name = "Bob"  // Mutation is default
    person.Age = 31

    // Pointers allow mutation
    p := &person
    p.Name = "Charlie"

    // Slices and maps are mutable
    nums := []int{1, 2, 3}
    nums[0] = 99
    nums = append(nums, 4)
}
```

**What's happening**:
- All variables can be reassigned
- All fields can be modified
- Mutability is the default
- Immutability requires special types or conventions

### Rust approach: Immutable by default

```rust
// Rust: Explicit mutability
fn main() {
    // Immutable by default
    let x = 5;
    // x = 10;  // ERROR! x is immutable

    let mut y = 5;  // mut makes it mutable
    y = 10;         // OK

    // Structs
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    // person.name = String::from("Bob");  // ERROR! person is immutable

    let mut person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    person.name = String::from("Bob");  // OK—person is mut
    person.age = 31;

    // Vectors
    let nums = vec![1, 2, 3];
    // nums.push(4);  // ERROR! nums is immutable

    let mut nums = vec![1, 2, 3];
    nums.push(4);   // OK
    nums[0] = 99;   // OK
}
```

### Borrowing enforces single writer OR multiple readers

```rust
fn main() {
    let mut data = vec![1, 2, 3];

    let r1 = &data;     // Immutable borrow
    let r2 = &data;     // Multiple readers OK
    println!("{:?} {:?}", r1, r2);

    // let w = &mut data;  // ERROR! Can't borrow mutably while
                           // immutable borrows exist

    // After r1 and r2 are done...
    let w = &mut data;    // Now we can get mutable access
    w.push(4);
}
```

### Key differences

| Aspect | Go | Rust |
|--------|-----|------|
| **Default** | Mutable | Immutable |
| **Keyword** | None | `mut` |
| **Enforcement** | None (convention only) | Compile-time (borrow checker) |
| **Shared mutable state** | Allowed (needs manual sync) | Prevented at compile time |
| **Mental model** | "Everything changes" | "What needs to change?" |

---

## Struct Embedding vs Composition

Go uses struct embedding for code reuse. Rust uses explicit composition and traits.

### Go approach: Struct embedding

```go
// Go: Embed structs to compose behavior
package main

import "fmt"

type Animal struct {
    Name string
}

func (a Animal) Speak() {
    fmt.Println(a.Name, "makes a sound")
}

type Dog struct {
    Animal  // Embedded struct
    Breed string
}

func (d Dog) Speak() {
    fmt.Println(d.Name, "barks")
}

func main() {
    dog := Dog{
        Animal: Animal{Name: "Buddy"},
        Breed:  "Golden Retriever",
    }

    dog.Speak()         // Calls Dog.Speak
    dog.Animal.Speak()  // Calls Animal.Speak

    // Fields from Animal are promoted
    fmt.Println(dog.Name)  // Accesses dog.Animal.Name
}
```

**What's happening**:
- Embedded struct fields are promoted
- Embedded methods are available on outer struct
- Can override embedded methods
- Feels like inheritance but is composition

### Rust approach: Explicit composition

```rust
// Rust: Explicit composition with traits
struct Animal {
    name: String,
}

trait Speak {
    fn speak(&self);
}

impl Speak for Animal {
    fn speak(&self) {
        println!("{} makes a sound", self.name);
    }
}

struct Dog {
    animal: Animal,  // Explicit composition
    breed: String,
}

impl Speak for Dog {
    fn speak(&self) {
        println!("{} barks", self.animal.name);
    }
}

impl Dog {
    fn animal_speak(&self) {
        self.animal.speak();
    }
}

fn main() {
    let dog = Dog {
        animal: Animal { name: String::from("Buddy") },
        breed: String::from("Golden Retriever"),
    };

    dog.speak();          // Calls Dog::speak
    dog.animal_speak();   // Calls Animal::speak via explicit method

    // No field promotion—must access explicitly
    println!("{}", dog.animal.name);
}
```

### Trait-based composition

```rust
// Deref trait for automatic dereferencing
use std::ops::Deref;

struct Dog {
    animal: Animal,
    breed: String,
}

impl Deref for Dog {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}

fn main() {
    let dog = Dog {
        animal: Animal { name: String::from("Buddy") },
        breed: String::from("Golden Retriever"),
    };

    // Can access animal.name through Deref
    println!("{}", dog.name);  // Derefs to dog.animal.name
}
```

### Key differences

| Aspect | Go | Rust |
|--------|-----|------|
| **Mechanism** | Struct embedding | Explicit composition |
| **Field promotion** | Automatic | Manual (or via Deref) |
| **Method forwarding** | Automatic | Manual (or via traits) |
| **Clarity** | Implicit behavior | Explicit behavior |
| **Flexibility** | Quick and easy | More verbose, more control |

---

## Summary: Making the Transition

### What's harder in Rust

1. **Ownership and borrowing**—The borrow checker is the biggest learning curve
2. **Explicit mutability**—Must think about what can change
3. **No nil**—Must handle `Option` and `Result` explicitly
4. **Traits are explicit**—Must declare implementations
5. **Longer compile times**—Rust optimizes heavily

### What's easier in Rust

1. **No GC pauses**—Predictable, consistent latency
2. **Fearless refactoring**—Compiler catches breaking changes
3. **No nil panics**—Option type prevents null pointer errors
4. **Data race freedom**—Impossible to have data races that compile
5. **Zero-cost abstractions**—Generics don't cost runtime performance

### The mental model shift

| Go mindset | Rust mindset |
|------------|--------------|
| "GC handles memory" | "Who owns this data?" |
| "Pass pointers freely" | "Borrow or move?" |
| "nil checks at runtime" | "Option enforced at compile time" |
| "if err != nil everywhere" | "? operator for propagation" |
| "Interfaces just work" | "Explicitly implement traits" |
| "go func() for concurrency" | "async/await or threads?" |
| "defer for cleanup" | "Drop handles it automatically" |

### Tips for learning

1. **Don't fight the borrow checker**—Redesign your approach to fit Rust's model
2. **Start with immutability**—Only add `mut` when needed
3. **Use Option and Result**—Don't reach for `unwrap()` in production code
4. **Read compiler errors carefully**—They're incredibly helpful
5. **Clone liberally at first**—Optimize after you understand ownership

### Common gotchas for Go developers

1. **String vs &str**—`String` is owned, `&str` is borrowed (like Go's `[]byte` vs `string`)
2. **No nil anywhere**—Must use `Option<T>` explicitly
3. **Can't share mutable data**—Borrow checker prevents it (use `Arc<Mutex<T>>` or channels)
4. **Imports are different**—`use` instead of `import`, paths work differently
5. **No defer in loops**—Use `drop()` explicitly or restructure code
6. **Iterators are lazy**—Must call `.collect()` or consume them

The shift from Go to Rust is significant, but the payoff is worth it: no GC pauses, fearless concurrency, and the compiler prevents entire classes of bugs that you'd catch at runtime in Go.
