# Design Patterns: Go to Rust Translation

If you're coming from Go, you know that Go favors simplicity over fancy patterns. The good news: Rust shares this philosophy. The better news: Rust's type system lets you express patterns more safely.

## Important note before we start

Go achieves simplicity through minimalism—few features, simple composition. Rust achieves safety through its type system—ownership, traits, and zero-cost abstractions.

Don't try to force Go patterns onto Rust verbatim. Instead, understand the *intent* of each pattern, then see how Rust achieves that intent (often with stronger guarantees).

---

## Table of Contents

- [Design Patterns: Go to Rust Translation](#design-patterns-go-to-rust-translation)
  - [Important note before we start](#important-note-before-we-start)
  - [Table of Contents](#table-of-contents)
  - [Creational Patterns](#creational-patterns)
    - [Builder Pattern](#builder-pattern)
    - [Functional Options Pattern](#functional-options-pattern)
    - [Factory Pattern](#factory-pattern)
  - [Structural Patterns](#structural-patterns)
    - [Interface Composition](#interface-composition)
    - [Struct Embedding vs Trait Composition](#struct-embedding-vs-trait-composition)
  - [Behavioral Patterns](#behavioral-patterns)
    - [Strategy Pattern](#strategy-pattern)
    - [Iterator Pattern](#iterator-pattern)
  - [Concurrency Patterns](#concurrency-patterns)
    - [Worker Pool](#worker-pool)
    - [Fan-Out/Fan-In](#fan-outfan-in)
    - [Pipeline Pattern](#pipeline-pattern)
    - [Context Pattern](#context-pattern)
  - [Error Handling Patterns](#error-handling-patterns)
    - [Error Wrapping](#error-wrapping)
  - [Summary: Patterns in Rust vs Go](#summary-patterns-in-rust-vs-go)
    - [Patterns that work similarly](#patterns-that-work-similarly)
    - [Patterns that differ significantly](#patterns-that-differ-significantly)
    - [Rust-specific advantages](#rust-specific-advantages)
    - [Go-specific advantages](#go-specific-advantages)
  - [Key Takeaways](#key-takeaways)

---

## Creational Patterns

### Builder Pattern

Both Go and Rust use builders for complex object construction, but with different syntax.

**🐹 Go version**:

```go
// Go: Builder with method chaining
package main

type Server struct {
    host    string
    port    int
    timeout int
    tls     bool
}

type ServerBuilder struct {
    host    string
    port    int
    timeout int
    tls     bool
}

func NewServerBuilder() *ServerBuilder {
    return &ServerBuilder{
        host:    "localhost",
        port:    8080,
        timeout: 30,
        tls:     false,
    }
}

func (b *ServerBuilder) Host(host string) *ServerBuilder {
    b.host = host
    return b
}

func (b *ServerBuilder) Port(port int) *ServerBuilder {
    b.port = port
    return b
}

func (b *ServerBuilder) Timeout(timeout int) *ServerBuilder {
    b.timeout = timeout
    return b
}

func (b *ServerBuilder) EnableTLS() *ServerBuilder {
    b.tls = true
    return b
}

func (b *ServerBuilder) Build() *Server {
    return &Server{
        host:    b.host,
        port:    b.port,
        timeout: b.timeout,
        tls:     b.tls,
    }
}

// Usage
func main() {
    server := NewServerBuilder().
        Host("api.example.com").
        Port(443).
        EnableTLS().
        Build()
}
```

**🦀 Rust version**:

```rust
// Rust: Builder with move semantics
pub struct Server {
    host: String,
    port: u16,
    timeout: u64,
    tls: bool,
}

pub struct ServerBuilder {
    host: String,
    port: u16,
    timeout: u64,
    tls: bool,
}

impl ServerBuilder {
    pub fn new() -> Self {
        ServerBuilder {
            host: String::from("localhost"),
            port: 8080,
            timeout: 30,
            tls: false,
        }
    }

    // Methods take self by value (move semantics)
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn enable_tls(mut self) -> Self {
        self.tls = true;
        self
    }

    pub fn build(self) -> Server {
        Server {
            host: self.host,
            port: self.port,
            timeout: self.timeout,
            tls: self.tls,
        }
    }
}

// Usage
fn main() {
    let server = ServerBuilder::new()
        .host("api.example.com")
        .port(443)
        .enable_tls()
        .build();
}
```

**Key differences**:
- Go: Builder methods return `*ServerBuilder` (pointer)
- Rust: Builder methods take `self` by value (move semantics)
- Rust: Can use `impl Into<String>` for flexible input types
- Rust: Builder is consumed on `build()` (can't reuse)

---

### Functional Options Pattern

This is a very popular Go pattern. Rust has a different approach.

**🐹 Go version**:

```go
// Go: Functional options pattern
package main

import "time"

type Server struct {
    host    string
    port    int
    timeout time.Duration
}

type Option func(*Server)

func WithHost(host string) Option {
    return func(s *Server) {
        s.host = host
    }
}

func WithPort(port int) Option {
    return func(s *Server) {
        s.port = port
    }
}

func WithTimeout(timeout time.Duration) Option {
    return func(s *Server) {
        s.timeout = timeout
    }
}

func NewServer(opts ...Option) *Server {
    s := &Server{
        host:    "localhost",
        port:    8080,
        timeout: 30 * time.Second,
    }

    for _, opt := range opts {
        opt(s)
    }

    return s
}

// Usage
func main() {
    server := NewServer(
        WithHost("api.example.com"),
        WithPort(443),
        WithTimeout(60 * time.Second),
    )
}
```

**Rust version (using builder)**:

```rust
// Rust: Builder pattern is preferred over functional options
use std::time::Duration;

pub struct Server {
    host: String,
    port: u16,
    timeout: Duration,
}

impl Server {
    pub fn builder() -> ServerBuilder {
        ServerBuilder::new()
    }
}

pub struct ServerBuilder {
    host: String,
    port: u16,
    timeout: Duration,
}

impl ServerBuilder {
    pub fn new() -> Self {
        ServerBuilder {
            host: String::from("localhost"),
            port: 8080,
            timeout: Duration::from_secs(30),
        }
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> Server {
        Server {
            host: self.host,
            port: self.port,
            timeout: self.timeout,
        }
    }
}

// Usage
fn main() {
    let server = Server::builder()
        .host("api.example.com")
        .port(443)
        .timeout(Duration::from_secs(60))
        .build();
}
```

**Why Rust uses builders instead of functional options**:
- Type safety: Builder methods are type-checked at compile time
- Ownership: Clear ownership transfer with move semantics
- Performance: Zero runtime overhead
- Discoverability: IDE autocomplete works better

---

### Factory Pattern

**🐹 Go version**:

```go
// Go: Factory function
package main

type Database interface {
    Query(string) ([]byte, error)
    Close() error
}

type PostgresDB struct {
    conn string
}

func (p *PostgresDB) Query(q string) ([]byte, error) {
    // Implementation
    return nil, nil
}

func (p *PostgresDB) Close() error {
    return nil
}

type MySQLDB struct {
    conn string
}

func (m *MySQLDB) Query(q string) ([]byte, error) {
    // Implementation
    return nil, nil
}

func (m *MySQLDB) Close() error {
    return nil
}

// Factory function
func NewDatabase(dbType string, conn string) (Database, error) {
    switch dbType {
    case "postgres":
        return &PostgresDB{conn: conn}, nil
    case "mysql":
        return &MySQLDB{conn: conn}, nil
    default:
        return nil, fmt.Errorf("unknown database type: %s", dbType)
    }
}

// Usage
func main() {
    db, err := NewDatabase("postgres", "localhost:5432")
    if err != nil {
        log.Fatal(err)
    }
    defer db.Close()
}
```

**🦀 Rust version**:

```rust
// Rust: Factory with trait objects or enums
use std::io;

trait Database {
    fn query(&self, query: &str) -> io::Result<Vec<u8>>;
    fn close(&mut self) -> io::Result<()>;
}

struct PostgresDB {
    conn: String,
}

impl Database for PostgresDB {
    fn query(&self, query: &str) -> io::Result<Vec<u8>> {
        // Implementation
        Ok(vec![])
    }

    fn close(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct MySQLDB {
    conn: String,
}

impl Database for MySQLDB {
    fn query(&self, query: &str) -> io::Result<Vec<u8>> {
        // Implementation
        Ok(vec![])
    }

    fn close(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// Factory function returning trait object
fn new_database(db_type: &str, conn: String) -> Result<Box<dyn Database>, String> {
    match db_type {
        "postgres" => Ok(Box::new(PostgresDB { conn })),
        "mysql" => Ok(Box::new(MySQLDB { conn })),
        _ => Err(format!("unknown database type: {}", db_type)),
    }
}

// Alternative: Using enum (often better in Rust)
enum DatabaseType {
    Postgres(PostgresDB),
    MySQL(MySQLDB),
}

impl DatabaseType {
    fn new(db_type: &str, conn: String) -> Result<Self, String> {
        match db_type {
            "postgres" => Ok(DatabaseType::Postgres(PostgresDB { conn })),
            "mysql" => Ok(DatabaseType::MySQL(MySQLDB { conn })),
            _ => Err(format!("unknown database type: {}", db_type)),
        }
    }

    fn query(&self, query: &str) -> io::Result<Vec<u8>> {
        match self {
            DatabaseType::Postgres(db) => db.query(query),
            DatabaseType::MySQL(db) => db.query(query),
        }
    }
}

// Usage
fn main() {
    // Using trait object
    let mut db = new_database("postgres", "localhost:5432".to_string()).unwrap();

    // Using enum (often preferred)
    let db = DatabaseType::new("postgres", "localhost:5432".to_string()).unwrap();
}
```

**Key differences**:
- Go: Returns interface value
- Rust: Returns `Box<dyn Trait>` (trait object) or use enum
- Rust: Enum approach is often better (no heap allocation, exhaustive matching)

---

## Structural Patterns

### Interface Composition

**🐹 Go version**:

```go
// Go: Interface composition
package main

type Reader interface {
    Read(p []byte) (n int, err error)
}

type Writer interface {
    Write(p []byte) (n int, err error)
}

type Closer interface {
    Close() error
}

// Composed interface
type ReadWriteCloser interface {
    Reader
    Writer
    Closer
}

// Any type implementing all three satisfies ReadWriteCloser
```

**🦀 Rust version**:

```rust
// Rust: Trait bounds composition
use std::io::{self, Read, Write};

// Individual traits
trait Reader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

trait Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
}

trait Closer {
    fn close(&mut self) -> io::Result<()>;
}

// Composed trait
trait ReadWriteCloser: Reader + Writer + Closer {}

// Any type implementing all three automatically implements ReadWriteCloser
impl<T> ReadWriteCloser for T where T: Reader + Writer + Closer {}

// Alternative: Use trait bounds directly
fn process<T: Reader + Writer + Closer>(mut rwc: T) {
    // Use T as reader, writer, and closer
}
```

**Key differences**:
- Go: Interface embedding is implicit
- Rust: Trait bounds are explicit with `+`
- Rust: Blanket implementation with `impl<T> Trait for T where T: ...`

---

### Struct Embedding vs Trait Composition

**🐹 Go version**:

```go
// Go: Struct embedding
package main

import "fmt"

type Animal struct {
    Name string
}

func (a *Animal) Speak() {
    fmt.Println(a.Name, "makes a sound")
}

type Dog struct {
    Animal  // Embedded
    Breed string
}

func (d *Dog) Speak() {
    fmt.Println(d.Name, "barks")
}

func main() {
    dog := Dog{
        Animal: Animal{Name: "Buddy"},
        Breed:  "Labrador",
    }

    dog.Speak()         // Calls Dog.Speak
    dog.Animal.Speak()  // Calls Animal.Speak
    fmt.Println(dog.Name)  // Promoted field
}
```

**🦀 Rust version**:

```rust
// Rust: Explicit composition (no embedding)
struct Animal {
    name: String,
}

impl Animal {
    fn speak(&self) {
        println!("{} makes a sound", self.name);
    }
}

struct Dog {
    animal: Animal,  // Explicit field
    breed: String,
}

impl Dog {
    fn speak(&self) {
        println!("{} barks", self.animal.name);
    }

    fn animal_speak(&self) {
        self.animal.speak();
    }
}

fn main() {
    let dog = Dog {
        animal: Animal { name: String::from("Buddy") },
        breed: String::from("Labrador"),
    };

    dog.speak();          // Calls Dog::speak
    dog.animal_speak();   // Calls Animal::speak
    println!("{}", dog.animal.name);  // Must access explicitly
}

// Using Deref for automatic dereferencing (use sparingly)
use std::ops::Deref;

impl Deref for Dog {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}

// Now can access animal fields directly
fn use_deref() {
    let dog = Dog {
        animal: Animal { name: String::from("Buddy") },
        breed: String::from("Labrador"),
    };

    println!("{}", dog.name);  // Derefs to dog.animal.name
}
```

**Key differences**:
- Go: Field promotion is automatic
- Rust: Composition is explicit (clearer, but more verbose)
- Rust: Can use `Deref` trait for automatic dereferencing (but use sparingly)

---

## Behavioral Patterns

### Strategy Pattern

**🐹 Go version**:

```go
// Go: Strategy with interfaces
package main

type PaymentStrategy interface {
    Pay(amount float64) error
}

type CreditCard struct {
    number string
}

func (c *CreditCard) Pay(amount float64) error {
    // Process credit card payment
    return nil
}

type PayPal struct {
    email string
}

func (p *PayPal) Pay(amount float64) error {
    // Process PayPal payment
    return nil
}

type Checkout struct {
    strategy PaymentStrategy
}

func (c *Checkout) SetStrategy(strategy PaymentStrategy) {
    c.strategy = strategy
}

func (c *Checkout) Process(amount float64) error {
    return c.strategy.Pay(amount)
}

// Usage
func main() {
    checkout := &Checkout{}

    checkout.SetStrategy(&CreditCard{number: "1234-5678"})
    checkout.Process(100.00)

    checkout.SetStrategy(&PayPal{email: "user@example.com"})
    checkout.Process(50.00)
}
```

**🦀 Rust version**:

```rust
// Rust: Strategy with trait objects or generics
trait PaymentStrategy {
    fn pay(&self, amount: f64) -> Result<(), String>;
}

struct CreditCard {
    number: String,
}

impl PaymentStrategy for CreditCard {
    fn pay(&self, amount: f64) -> Result<(), String> {
        // Process credit card payment
        Ok(())
    }
}

struct PayPal {
    email: String,
}

impl PaymentStrategy for PayPal {
    fn pay(&self, amount: f64) -> Result<(), String> {
        // Process PayPal payment
        Ok(())
    }
}

// Using trait objects (dynamic dispatch)
struct Checkout {
    strategy: Box<dyn PaymentStrategy>,
}

impl Checkout {
    fn new(strategy: Box<dyn PaymentStrategy>) -> Self {
        Checkout { strategy }
    }

    fn set_strategy(&mut self, strategy: Box<dyn PaymentStrategy>) {
        self.strategy = strategy;
    }

    fn process(&self, amount: f64) -> Result<(), String> {
        self.strategy.pay(amount)
    }
}

// Alternative: Using enums (often better)
enum Payment {
    CreditCard(CreditCard),
    PayPal(PayPal),
}

impl Payment {
    fn pay(&self, amount: f64) -> Result<(), String> {
        match self {
            Payment::CreditCard(cc) => cc.pay(amount),
            Payment::PayPal(pp) => pp.pay(amount),
        }
    }
}

// Usage
fn main() {
    // Using trait objects
    let mut checkout = Checkout::new(Box::new(CreditCard {
        number: String::from("1234-5678"),
    }));
    checkout.process(100.0).unwrap();

    checkout.set_strategy(Box::new(PayPal {
        email: String::from("user@example.com"),
    }));
    checkout.process(50.0).unwrap();

    // Using enum (often preferred)
    let payment = Payment::CreditCard(CreditCard {
        number: String::from("1234-5678"),
    });
    payment.pay(100.0).unwrap();
}
```

**Key differences**:
- Go: Interface values are natural
- Rust: Choose between trait objects (`Box<dyn Trait>`) or enums
- Rust: Enums are often better (no heap allocation, exhaustive matching)

---

### Iterator Pattern

**🐹 Go version**:

```go
// Go: Manual iterator
package main

type Iterator interface {
    Next() bool
    Value() int
}

type SliceIterator struct {
    data  []int
    index int
}

func NewSliceIterator(data []int) *SliceIterator {
    return &SliceIterator{data: data, index: -1}
}

func (it *SliceIterator) Next() bool {
    it.index++
    return it.index < len(it.data)
}

func (it *SliceIterator) Value() int {
    return it.data[it.index]
}

// Usage
func main() {
    iter := NewSliceIterator([]int{1, 2, 3, 4, 5})

    for iter.Next() {
        println(iter.Value())
    }
}
```

**🦀 Rust version**:

```rust
// Rust: Built-in Iterator trait
struct SliceIterator<'a> {
    data: &'a [i32],
    index: usize,
}

impl<'a> SliceIterator<'a> {
    fn new(data: &'a [i32]) -> Self {
        SliceIterator { data, index: 0 }
    }
}

impl<'a> Iterator for SliceIterator<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let item = &self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// Usage
fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let iter = SliceIterator::new(&data);

    for value in iter {
        println!("{}", value);
    }

    // Or use built-in iterator
    for value in data.iter() {
        println!("{}", value);
    }

    // Iterator combinators
    let doubled: Vec<i32> = data.iter()
        .map(|x| x * 2)
        .collect();

    let sum: i32 = data.iter().sum();
}
```

**Key differences**:
- Go: Manual iteration pattern
- Rust: Standardized `Iterator` trait with many combinators
- Rust: Iterators are lazy and composable
- Rust: for-in loops work with anything implementing `Iterator`

---

## Concurrency Patterns

### Worker Pool

**🐹 Go version**:

```go
// Go: Worker pool with goroutines
package main

import (
    "fmt"
    "sync"
)

func worker(id int, jobs <-chan int, results chan<- int) {
    for job := range jobs {
        fmt.Printf("Worker %d processing job %d\n", id, job)
        results <- job * 2
    }
}

func main() {
    const numWorkers = 3
    const numJobs = 10

    jobs := make(chan int, numJobs)
    results := make(chan int, numJobs)

    // Start workers
    var wg sync.WaitGroup
    for w := 1; w <= numWorkers; w++ {
        wg.Add(1)
        go func(id int) {
            defer wg.Done()
            worker(id, jobs, results)
        }(w)
    }

    // Send jobs
    for j := 1; j <= numJobs; j++ {
        jobs <- j
    }
    close(jobs)

    // Wait for completion
    go func() {
        wg.Wait()
        close(results)
    }()

    // Collect results
    for result := range results {
        fmt.Println("Result:", result)
    }
}
```

**🦀 Rust version**:

```rust
// Rust: Worker pool with threads
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    const NUM_WORKERS: usize = 3;
    const NUM_JOBS: usize = 10;

    let (job_tx, job_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();

    let job_rx = Arc::new(Mutex::new(job_rx));

    // Start workers
    let mut handles = vec![];
    for id in 0..NUM_WORKERS {
        let job_rx = Arc::clone(&job_rx);
        let result_tx = result_tx.clone();

        let handle = thread::spawn(move || {
            loop {
                let job = job_rx.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        println!("Worker {} processing job {}", id, job);
                        result_tx.send(job * 2).unwrap();
                    }
                    Err(_) => break,  // Channel closed
                }
            }
        });
        handles.push(handle);
    }

    // Send jobs
    for j in 1..=NUM_JOBS {
        job_tx.send(j).unwrap();
    }
    drop(job_tx);  // Close job channel

    // Wait for workers to finish
    for handle in handles {
        handle.join().unwrap();
    }
    drop(result_tx);  // Close result channel

    // Collect results
    for result in result_rx {
        println!("Result: {}", result);
    }
}

// Async version with Tokio
use tokio::sync::mpsc;

#[tokio::main]
async fn async_worker_pool() {
    const NUM_WORKERS: usize = 3;
    const NUM_JOBS: usize = 10;

    let (job_tx, mut job_rx) = mpsc::channel(100);
    let (result_tx, mut result_rx) = mpsc::channel(100);

    // Start workers
    for id in 0..NUM_WORKERS {
        let mut job_rx = job_rx.clone();
        let result_tx = result_tx.clone();

        tokio::spawn(async move {
            while let Some(job) = job_rx.recv().await {
                println!("Worker {} processing job {}", id, job);
                result_tx.send(job * 2).await.unwrap();
            }
        });
    }
    drop(job_rx);  // Drop original receiver

    // Send jobs
    for j in 1..=NUM_JOBS {
        job_tx.send(j).await.unwrap();
    }
    drop(job_tx);  // Close channel

    drop(result_tx);  // Close result channel

    // Collect results
    while let Some(result) = result_rx.recv().await {
        println!("Result: {}", result);
    }
}
```

**Key differences**:
- Go: Goroutines are lightweight, easy to spawn
- Rust: Choose between threads (OS threads) or async tasks
- Rust: Must use `Arc<Mutex<T>>` for shared receiver in thread version
- Rust: Async version is more similar to Go's goroutines

---

### Fan-Out/Fan-In

**🐹 Go version**:

```go
// Go: Fan-out/fan-in pattern
package main

import (
    "fmt"
    "sync"
)

func producer(nums ...int) <-chan int {
    out := make(chan int)
    go func() {
        defer close(out)
        for _, n := range nums {
            out <- n
        }
    }()
    return out
}

func square(in <-chan int) <-chan int {
    out := make(chan int)
    go func() {
        defer close(out)
        for n := range in {
            out <- n * n
        }
    }()
    return out
}

func merge(channels ...<-chan int) <-chan int {
    var wg sync.WaitGroup
    out := make(chan int)

    output := func(c <-chan int) {
        defer wg.Done()
        for n := range c {
            out <- n
        }
    }

    wg.Add(len(channels))
    for _, c := range channels {
        go output(c)
    }

    go func() {
        wg.Wait()
        close(out)
    }()

    return out
}

func main() {
    in := producer(1, 2, 3, 4, 5)

    // Fan out
    c1 := square(in)
    c2 := square(in)

    // Fan in
    for n := range merge(c1, c2) {
        fmt.Println(n)
    }
}
```

**Rust version (async)**:

```rust
// Rust: Fan-out/fan-in with async
use tokio::sync::mpsc;
use tokio::task;

async fn producer(nums: Vec<i32>) -> mpsc::Receiver<i32> {
    let (tx, rx) = mpsc::channel(10);

    task::spawn(async move {
        for n in nums {
            tx.send(n).await.unwrap();
        }
    });

    rx
}

async fn square(mut input: mpsc::Receiver<i32>) -> mpsc::Receiver<i32> {
    let (tx, rx) = mpsc::channel(10);

    task::spawn(async move {
        while let Some(n) = input.recv().await {
            tx.send(n * n).await.unwrap();
        }
    });

    rx
}

async fn merge(mut channels: Vec<mpsc::Receiver<i32>>) -> mpsc::Receiver<i32> {
    let (tx, rx) = mpsc::channel(10);

    for mut channel in channels {
        let tx = tx.clone();
        task::spawn(async move {
            while let Some(n) = channel.recv().await {
                tx.send(n).await.unwrap();
            }
        });
    }

    rx
}

#[tokio::main]
async fn main() {
    let input = producer(vec![1, 2, 3, 4, 5]).await;

    // Fan out (would need to split channel, simplified here)
    let c1 = square(input).await;

    // Collect results
    let mut output = c1;
    while let Some(n) = output.recv().await {
        println!("{}", n);
    }
}
```

**Key differences**:
- Go: Channel-based patterns are idiomatic
- Rust: Async channels with Tokio work similarly
- Rust: Must manage channel ownership explicitly

---

### Pipeline Pattern

**🐹 Go version**:

```go
// Go: Pipeline pattern
package main

import "fmt"

func gen(nums ...int) <-chan int {
    out := make(chan int)
    go func() {
        defer close(out)
        for _, n := range nums {
            out <- n
        }
    }()
    return out
}

func sq(in <-chan int) <-chan int {
    out := make(chan int)
    go func() {
        defer close(out)
        for n := range in {
            out <- n * n
        }
    }()
    return out
}

func main() {
    // Set up pipeline: gen -> sq -> sq
    for n := range sq(sq(gen(1, 2, 3, 4))) {
        fmt.Println(n)
    }
}
```

**🦀 Rust version**:

```rust
// Rust: Iterator pipeline (zero cost!)
fn main() {
    let result: Vec<i32> = vec![1, 2, 3, 4]
        .into_iter()
        .map(|n| n * n)
        .map(|n| n * n)
        .collect();

    for n in result {
        println!("{}", n);
    }
}

// Async pipeline
use tokio::sync::mpsc;
use tokio::task;

async fn gen(nums: Vec<i32>) -> mpsc::Receiver<i32> {
    let (tx, rx) = mpsc::channel(10);
    task::spawn(async move {
        for n in nums {
            tx.send(n).await.unwrap();
        }
    });
    rx
}

async fn sq(mut input: mpsc::Receiver<i32>) -> mpsc::Receiver<i32> {
    let (tx, rx) = mpsc::channel(10);
    task::spawn(async move {
        while let Some(n) = input.recv().await {
            tx.send(n * n).await.unwrap();
        }
    });
    rx
}

#[tokio::main]
async fn async_pipeline() {
    let mut output = sq(sq(gen(vec![1, 2, 3, 4]).await).await).await;

    while let Some(n) = output.recv().await {
        println!("{}", n);
    }
}
```

**Key differences**:
- Go: Channel-based pipelines
- Rust: Iterator pipelines are zero-cost (preferred for CPU-bound)
- Rust: Async pipelines for I/O-bound work

---

### Context Pattern

**🐹 Go version**:

```go
// Go: Context for cancellation
package main

import (
    "context"
    "fmt"
    "time"
)

func worker(ctx context.Context, id int) {
    for {
        select {
        case <-ctx.Done():
            fmt.Printf("Worker %d cancelled: %v\n", id, ctx.Err())
            return
        default:
            fmt.Printf("Worker %d working...\n", id)
            time.Sleep(500 * time.Millisecond)
        }
    }
}

func main() {
    ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
    defer cancel()

    for i := 0; i < 3; i++ {
        go worker(ctx, i)
    }

    <-ctx.Done()
    fmt.Println("Main: context cancelled")
    time.Sleep(1 * time.Second)  // Wait for workers to finish
}
```

**🦀 Rust version**:

```rust
// Rust: Using tokio::select! for cancellation
use tokio::time::{sleep, Duration};
use tokio::sync::broadcast;
use tokio::task;

async fn worker(id: i32, mut cancel_rx: broadcast::Receiver<()>) {
    loop {
        tokio::select! {
            _ = cancel_rx.recv() => {
                println!("Worker {} cancelled", id);
                return;
            }
            _ = sleep(Duration::from_millis(500)) => {
                println!("Worker {} working...", id);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (cancel_tx, _) = broadcast::channel(1);

    let mut handles = vec![];
    for i in 0..3 {
        let cancel_rx = cancel_tx.subscribe();
        let handle = task::spawn(async move {
            worker(i, cancel_rx).await;
        });
        handles.push(handle);
    }

    // Cancel after 2 seconds
    sleep(Duration::from_secs(2)).await;
    let _ = cancel_tx.send(());
    println!("Main: sent cancellation signal");

    // Wait for all workers
    for handle in handles {
        handle.await.unwrap();
    }
}
```

**Key differences**:
- Go: `context.Context` is standard for cancellation
- Rust: No standard context; use channels or `tokio::select!`
- Rust: Cancellation is explicit via channels

---

## Error Handling Patterns

### Error Wrapping

**🐹 Go version**:

```go
// Go: Error wrapping with %w
package main

import (
    "fmt"
    "os"
)

func readConfig(path string) error {
    _, err := os.ReadFile(path)
    if err != nil {
        return fmt.Errorf("failed to read config: %w", err)
    }
    return nil
}

func loadApp() error {
    if err := readConfig("config.yaml"); err != nil {
        return fmt.Errorf("failed to load app: %w", err)
    }
    return nil
}

func main() {
    if err := loadApp(); err != nil {
        fmt.Printf("Error: %v\n", err)
        // Error: failed to load app: failed to read config: open config.yaml: no such file or directory
    }
}
```

**🦀 Rust version**:

```rust
// Rust: Error context with anyhow or custom errors
use std::fs;
use std::io;

// Using anyhow for quick error handling
use anyhow::{Context, Result};

fn read_config(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .context("failed to read config")
}

fn load_app() -> Result<()> {
    read_config("config.yaml")
        .context("failed to load app")?;
    Ok(())
}

fn main() {
    if let Err(e) = load_app() {
        eprintln!("Error: {:?}", e);
        // Error: failed to load app
        // Caused by: failed to read config
        // Caused by: No such file or directory (os error 2)
    }
}

// Custom error types with thiserror
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("failed to read config")]
    ConfigRead(#[from] io::Error),

    #[error("failed to load app: {0}")]
    LoadFailed(String),
}

fn read_config_custom(path: &str) -> Result<String, AppError> {
    Ok(fs::read_to_string(path)?)
}
```

**Key differences**:
- Go: `fmt.Errorf` with `%w` for wrapping
- Rust: Use `anyhow` for quick error handling or `thiserror` for custom errors
- Rust: `.context()` adds context to errors

---

## Summary: Patterns in Rust vs Go

### Patterns that work similarly

1. **Builder**: Both languages use builders, Rust with move semantics
2. **Strategy**: Both use interfaces/traits, Rust can also use enums
3. **Iterator**: Rust has a more powerful standard Iterator trait
4. **Worker Pool**: Similar patterns, Rust uses threads or async

### Patterns that differ significantly

1. **Functional Options** (Go) → **Builder** (Rust): Rust prefers builders
2. **Struct Embedding** (Go) → **Explicit Composition** (Rust): No field promotion
3. **Context** (Go) → **Channels/select!** (Rust): No standard context type
4. **Interface composition** (Go) → **Trait bounds** (Rust): Explicit with `+`

### Rust-specific advantages

1. **Enums**: Algebraic data types make many patterns simpler
2. **Iterators**: Zero-cost, composable, lazy evaluation
3. **Ownership**: Prevents data races at compile time
4. **Type system**: Stronger guarantees, less runtime errors

### Go-specific advantages

1. **Simplicity**: Fewer concepts to learn
2. **Goroutines**: Dead simple concurrency
3. **Interface satisfaction**: Implicit, very flexible
4. **Context**: Standard pattern for cancellation and deadlines

---

## Key Takeaways

1. **Don't translate literally**: Understand the intent, then use Rust's strengths
2. **Enums are powerful**: Often better than trait objects
3. **Ownership changes patterns**: Embrace it, don't fight it
4. **Iterators are free**: Use them instead of manual loops
5. **Choose the right tool**: Threads for CPU, async for I/O

The goal isn't to write Go in Rust—it's to write idiomatic Rust that solves the same problems Go solved, but with Rust's safety guarantees.
