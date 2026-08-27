# Design Patterns Guide: C/C++ to Rust

This guide shows how common C/C++ design patterns translate to Rust. Some patterns are unnecessary in Rust, some are simpler, and some work differently due to ownership rules.

## 1. RAII Patterns

RAII (Resource Acquisition Is Initialization) is optional in C++ but mandatory in Rust.

### C++: Manual RAII

```cpp
// C++ RAII - File handle
class FileHandle {
    FILE* file;
    bool is_open;

public:
    FileHandle(const char* path, const char* mode)
        : file(fopen(path, mode)), is_open(file != nullptr) {
        if (!is_open) {
            throw std::runtime_error("Failed to open file");
        }
    }

    ~FileHandle() {
        if (is_open && file) {
            fclose(file);
        }
    }

    // Disable copying
    FileHandle(const FileHandle&) = delete;
    FileHandle& operator=(const FileHandle&) = delete;

    // Enable moving
    FileHandle(FileHandle&& other) noexcept
        : file(other.file), is_open(other.is_open) {
        other.is_open = false;
        other.file = nullptr;
    }

    FILE* get() { return file; }
};

// Usage
{
    FileHandle f("data.txt", "r");
    // ... use f.get() ...
} // Automatically closed
```

### Rust: Built-in RAII

```rust
// Rust RAII - automatic with Drop trait
use std::fs::File;
use std::io::Write;

// Custom resource
struct FileHandle {
    file: File,
    path: String,
}

impl FileHandle {
    fn new(path: &str) -> std::io::Result<Self> {
        Ok(FileHandle {
            file: File::create(path)?,
            path: path.to_string(),
        })
    }

    fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.file.write_all(data)
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        println!("Closing file: {}", self.path);
        // file is automatically closed when dropped
    }
}

// Usage
{
    let mut f = FileHandle::new("data.txt").unwrap();
    f.write(b"Hello").unwrap();
} // Automatically dropped and closed

// Scoped guard pattern
struct Guard<F: FnOnce()> {
    cleanup: Option<F>,
}

impl<F: FnOnce()> Guard<F> {
    fn new(cleanup: F) -> Self {
        Guard {
            cleanup: Some(cleanup),
        }
    }
}

impl<F: FnOnce()> Drop for Guard<F> {
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup();
        }
    }
}

// Usage
{
    let _guard = Guard::new(|| println!("Cleaning up!"));
    // ... do work ...
} // Prints "Cleaning up!" when guard is dropped
```

## 2. Smart Pointer Patterns

### C++: Unique ownership with unique_ptr

```cpp
// C++ unique_ptr pattern
#include <memory>

class Node {
public:
    int value;
    std::unique_ptr<Node> next;

    Node(int val) : value(val), next(nullptr) {}
};

class LinkedList {
    std::unique_ptr<Node> head;

public:
    void push(int value) {
        auto new_node = std::make_unique<Node>(value);
        new_node->next = std::move(head);
        head = std::move(new_node);
    }

    std::unique_ptr<Node> pop() {
        if (!head) return nullptr;

        auto old_head = std::move(head);
        head = std::move(old_head->next);
        return old_head;
    }
};
```

### Rust: Box for heap allocation

```rust
// Rust Box<T> - similar to unique_ptr
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            next: None,
        }
    }
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<Box<Node>> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node
        })
    }
}
```

### C++: Shared ownership with shared_ptr

```cpp
// C++ shared_ptr - reference counted
#include <memory>
#include <vector>

class Resource {
public:
    std::string name;
    Resource(std::string n) : name(n) {
        std::cout << "Resource " << name << " created\n";
    }
    ~Resource() {
        std::cout << "Resource " << name << " destroyed\n";
    }
};

std::vector<std::shared_ptr<Resource>> resources;

void share_resource() {
    auto r = std::make_shared<Resource>("Shared");
    resources.push_back(r);  // Increment ref count
    // r goes out of scope, but resource not freed (still referenced)
}

// Need to clear resources to free
// resources.clear();
```

### Rust: Rc for shared ownership (single-threaded)

```rust
// Rust Rc<T> - reference counted (single-threaded)
use std::rc::Rc;

struct Resource {
    name: String,
}

impl Resource {
    fn new(name: &str) -> Self {
        println!("Resource {} created", name);
        Resource {
            name: name.to_string(),
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Resource {} destroyed", self.name);
    }
}

fn main() {
    let r1 = Rc::new(Resource::new("Shared"));
    let r2 = Rc::clone(&r1);  // Increment ref count
    let r3 = Rc::clone(&r1);

    println!("Ref count: {}", Rc::strong_count(&r1));  // 3

    drop(r2);
    println!("Ref count: {}", Rc::strong_count(&r1));  // 2

    // r1 and r3 dropped at end, resource freed when count reaches 0
}

// Arc<T> for thread-safe reference counting
use std::sync::Arc;
use std::thread;

fn share_across_threads() {
    let r = Arc::new(Resource::new("Thread-safe"));
    let r1 = Arc::clone(&r);
    let r2 = Arc::clone(&r);

    thread::spawn(move || {
        println!("Thread 1: {}", r1.name);
    });

    thread::spawn(move || {
        println!("Thread 2: {}", r2.name);
    });

    // r dropped when main thread ends
}
```

## 3. Factory Pattern

### C++: Virtual factory

```cpp
// C++ factory with inheritance
class Animal {
public:
    virtual void speak() = 0;
    virtual ~Animal() = default;
};

class Dog : public Animal {
public:
    void speak() override {
        std::cout << "Woof!\n";
    }
};

class Cat : public Animal {
public:
    void speak() override {
        std::cout << "Meow!\n";
    }
};

class AnimalFactory {
public:
    static std::unique_ptr<Animal> create(const std::string& type) {
        if (type == "dog") {
            return std::make_unique<Dog>();
        } else if (type == "cat") {
            return std::make_unique<Cat>();
        }
        return nullptr;
    }
};

// Usage
auto animal = AnimalFactory::create("dog");
if (animal) {
    animal->speak();
}
```

### Rust: Enum-based factory

```rust
// Rust factory - enums are better than inheritance
enum Animal {
    Dog,
    Cat,
    Bird { species: String },
}

impl Animal {
    fn create(type_name: &str) -> Option<Self> {
        match type_name {
            "dog" => Some(Animal::Dog),
            "cat" => Some(Animal::Cat),
            "bird" => Some(Animal::Bird {
                species: "Sparrow".to_string(),
            }),
            _ => None,
        }
    }

    fn speak(&self) {
        match self {
            Animal::Dog => println!("Woof!"),
            Animal::Cat => println!("Meow!"),
            Animal::Bird { species } => println!("{} chirps!", species),
        }
    }
}

// Usage
if let Some(animal) = Animal::create("dog") {
    animal.speak();
}

// Or trait-based factory (when you need dynamic dispatch)
trait Speak {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

struct AnimalFactory;

impl AnimalFactory {
    fn create(type_name: &str) -> Option<Box<dyn Speak>> {
        match type_name {
            "dog" => Some(Box::new(Dog)),
            "cat" => Some(Box::new(Cat)),
            _ => None,
        }
    }
}
```

## 4. Builder Pattern

### C++: Builder with method chaining

```cpp
// C++ builder pattern
class HttpRequest {
    std::string url;
    std::string method;
    std::map<std::string, std::string> headers;
    std::string body;

public:
    class Builder {
        HttpRequest request;

    public:
        Builder& url(const std::string& url) {
            request.url = url;
            return *this;
        }

        Builder& method(const std::string& method) {
            request.method = method;
            return *this;
        }

        Builder& header(const std::string& key, const std::string& value) {
            request.headers[key] = value;
            return *this;
        }

        Builder& body(const std::string& body) {
            request.body = body;
            return *this;
        }

        HttpRequest build() {
            return std::move(request);
        }
    };

    static Builder builder() {
        return Builder();
    }

    void send() {
        std::cout << method << " " << url << "\n";
    }
};

// Usage
auto request = HttpRequest::builder()
    .url("https://api.example.com")
    .method("POST")
    .header("Content-Type", "application/json")
    .body("{\"key\": \"value\"}")
    .build();
request.send();
```

### Rust: Builder with ownership

```rust
// Rust builder pattern
struct HttpRequest {
    url: String,
    method: String,
    headers: std::collections::HashMap<String, String>,
    body: String,
}

struct HttpRequestBuilder {
    url: Option<String>,
    method: String,
    headers: std::collections::HashMap<String, String>,
    body: String,
}

impl HttpRequestBuilder {
    fn new() -> Self {
        HttpRequestBuilder {
            url: None,
            method: "GET".to_string(),
            headers: std::collections::HashMap::new(),
            body: String::new(),
        }
    }

    fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    fn method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }

    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    fn build(self) -> Result<HttpRequest, String> {
        let url = self.url.ok_or("URL is required")?;

        Ok(HttpRequest {
            url,
            method: self.method,
            headers: self.headers,
            body: self.body,
        })
    }
}

impl HttpRequest {
    fn builder() -> HttpRequestBuilder {
        HttpRequestBuilder::new()
    }

    fn send(&self) {
        println!("{} {}", self.method, self.url);
    }
}

// Usage
let request = HttpRequest::builder()
    .url("https://api.example.com")
    .method("POST")
    .header("Content-Type", "application/json")
    .body(r#"{"key": "value"}"#)
    .build()
    .unwrap();

request.send();

// Or using derive_builder crate
// #[derive(Builder)]
// struct HttpRequest { ... }
```

## 5. Iterator Pattern

### C++: STL iterators

```cpp
// C++ iterators
#include <vector>
#include <algorithm>

std::vector<int> numbers = {1, 2, 3, 4, 5};

// Manual iteration
for (auto it = numbers.begin(); it != numbers.end(); ++it) {
    std::cout << *it << " ";
}

// Range-based for
for (const auto& num : numbers) {
    std::cout << num << " ";
}

// Algorithms
auto result = std::find_if(numbers.begin(), numbers.end(),
                          [](int n) { return n > 3; });

std::transform(numbers.begin(), numbers.end(), numbers.begin(),
              [](int n) { return n * 2; });
```

### Rust: Powerful iterator chains

```rust
// Rust iterators - zero-cost abstractions
let numbers = vec![1, 2, 3, 4, 5];

// Basic iteration
for num in &numbers {
    println!("{}", num);
}

// Iterator chains
let result: Vec<i32> = numbers
    .iter()
    .filter(|&&x| x > 2)
    .map(|&x| x * 2)
    .collect();

// Custom iterator
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// Usage
let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();

// Iterators are lazy - nothing happens until consumed
let iter = vec![1, 2, 3]
    .iter()
    .map(|x| {
        println!("Processing {}", x);  // Not called yet!
        x * 2
    });

// Now it runs
let _result: Vec<_> = iter.collect();
```

## 6. Observer Pattern

### C++: Callbacks and signals

```cpp
// C++ observer with callbacks
#include <functional>
#include <vector>

class Observable {
    std::vector<std::function<void(int)>> observers;

public:
    void subscribe(std::function<void(int)> observer) {
        observers.push_back(observer);
    }

    void notify(int value) {
        for (auto& observer : observers) {
            observer(value);
        }
    }
};

// Usage
Observable observable;
observable.subscribe([](int val) {
    std::cout << "Observer 1: " << val << "\n";
});
observable.subscribe([](int val) {
    std::cout << "Observer 2: " << val << "\n";
});
observable.notify(42);
```

### Rust: Trait-based observers

```rust
// Rust observer pattern
trait Observer {
    fn update(&self, value: i32);
}

struct ConcreteObserver {
    id: String,
}

impl Observer for ConcreteObserver {
    fn update(&self, value: i32) {
        println!("Observer {}: {}", self.id, value);
    }
}

struct Observable {
    observers: Vec<Box<dyn Observer>>,
}

impl Observable {
    fn new() -> Self {
        Observable {
            observers: Vec::new(),
        }
    }

    fn subscribe(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn notify(&self, value: i32) {
        for observer in &self.observers {
            observer.update(value);
        }
    }
}

// Usage
let mut observable = Observable::new();
observable.subscribe(Box::new(ConcreteObserver {
    id: "1".to_string(),
}));
observable.subscribe(Box::new(ConcreteObserver {
    id: "2".to_string(),
}));
observable.notify(42);

// Or with closures (channels)
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    for value in rx {
        println!("Received: {}", value);
    }
});

tx.send(42).unwrap();
tx.send(100).unwrap();
```

## 7. Strategy Pattern

### C++: Function pointers or virtuals

```cpp
// C++ strategy with virtual functions
class SortStrategy {
public:
    virtual void sort(std::vector<int>& data) = 0;
    virtual ~SortStrategy() = default;
};

class QuickSort : public SortStrategy {
public:
    void sort(std::vector<int>& data) override {
        // ... quicksort implementation ...
        std::cout << "QuickSort\n";
    }
};

class MergeSort : public SortStrategy {
public:
    void sort(std::vector<int>& data) override {
        // ... mergesort implementation ...
        std::cout << "MergeSort\n";
    }
};

class Sorter {
    std::unique_ptr<SortStrategy> strategy;

public:
    void set_strategy(std::unique_ptr<SortStrategy> s) {
        strategy = std::move(s);
    }

    void sort(std::vector<int>& data) {
        strategy->sort(data);
    }
};
```

### Rust: Trait objects or closures

```rust
// Rust strategy with traits
trait SortStrategy {
    fn sort(&self, data: &mut Vec<i32>);
}

struct QuickSort;
struct MergeSort;

impl SortStrategy for QuickSort {
    fn sort(&self, data: &mut Vec<i32>) {
        println!("QuickSort");
        // ... implementation ...
    }
}

impl SortStrategy for MergeSort {
    fn sort(&self, data: &mut Vec<i32>) {
        println!("MergeSort");
        // ... implementation ...
    }
}

struct Sorter {
    strategy: Box<dyn SortStrategy>,
}

impl Sorter {
    fn new(strategy: Box<dyn SortStrategy>) -> Self {
        Sorter { strategy }
    }

    fn sort(&self, data: &mut Vec<i32>) {
        self.strategy.sort(data);
    }
}

// Usage
let mut data = vec![3, 1, 4, 1, 5];
let sorter = Sorter::new(Box::new(QuickSort));
sorter.sort(&mut data);

// Or simpler with closures
struct SorterWithClosure<F>
where
    F: Fn(&mut Vec<i32>),
{
    strategy: F,
}

impl<F> SorterWithClosure<F>
where
    F: Fn(&mut Vec<i32>),
{
    fn new(strategy: F) -> Self {
        SorterWithClosure { strategy }
    }

    fn sort(&self, data: &mut Vec<i32>) {
        (self.strategy)(data);
    }
}

// Usage
let sorter = SorterWithClosure::new(|data| {
    data.sort();
    println!("Sorted with closure");
});
```

## 8. Singleton Pattern

### C++: Thread-safe singleton

```cpp
// C++11 thread-safe singleton
class Singleton {
    Singleton() = default;

public:
    static Singleton& instance() {
        static Singleton instance;  // Thread-safe since C++11
        return instance;
    }

    Singleton(const Singleton&) = delete;
    Singleton& operator=(const Singleton&) = delete;

    void do_something() {
        std::cout << "Singleton method\n";
    }
};

// Usage
Singleton::instance().do_something();
```

### Rust: Lazy static or OnceLock

```rust
// Rust singleton with lazy_static
use lazy_static::lazy_static;
use std::sync::Mutex;

struct Config {
    setting: String,
}

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config {
        setting: "default".to_string(),
    });
}

// Usage
{
    let mut config = CONFIG.lock().unwrap();
    config.setting = "new value".to_string();
}

// Or with OnceLock (std since Rust 1.70)
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| Config {
        setting: "default".to_string(),
    })
}

// Usage
let config = get_config();
println!("{}", config.setting);
```

## 9. Newtype Pattern

Rust-specific pattern with no direct C++ equivalent.

```rust
// Newtype pattern - wrap type for type safety
struct Meters(f64);
struct Seconds(f64);

// Can't mix these up!
fn calculate_speed(distance: Meters, time: Seconds) -> f64 {
    distance.0 / time.0
}

let d = Meters(100.0);
let t = Seconds(10.0);
let speed = calculate_speed(d, t);

// let wrong = calculate_speed(t, d);  // ERROR: type mismatch!

// Add methods to newtype
impl Meters {
    fn to_feet(&self) -> f64 {
        self.0 * 3.28084
    }
}

// Deref coercion
use std::ops::Deref;

impl Deref for Meters {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Now can use as f64 in many contexts
let m = Meters(5.0);
let doubled = *m * 2.0;  // Deref to f64
```

## 10. Type State Pattern

Rust-specific pattern leveraging zero-cost abstractions.

```rust
// Type state pattern - enforce state at compile time
use std::marker::PhantomData;

struct Locked;
struct Unlocked;

struct Door<State> {
    _state: PhantomData<State>,
}

impl Door<Locked> {
    fn new() -> Self {
        Door {
            _state: PhantomData,
        }
    }

    fn unlock(self, key: &str) -> Door<Unlocked> {
        if key == "correct_key" {
            println!("Door unlocked");
            Door {
                _state: PhantomData,
            }
        } else {
            panic!("Wrong key!");
        }
    }
}

impl Door<Unlocked> {
    fn open(self) {
        println!("Door opened");
    }

    fn lock(self) -> Door<Locked> {
        println!("Door locked");
        Door {
            _state: PhantomData,
        }
    }
}

// Usage
let door = Door::new();                    // Locked
// door.open();                            // ERROR: can't open locked door
let door = door.unlock("correct_key");     // Unlocked
door.open();                               // OK!

let door = Door::new();
let door = door.unlock("correct_key");
let door = door.lock();                    // Locked again
// door.open();                            // ERROR: locked!
```

## Summary

| Pattern | C++ Approach | Rust Approach | Notes |
|---------|-------------|---------------|-------|
| RAII | Optional (destructors) | Mandatory (Drop) | All types use RAII in Rust |
| Factory | Virtual functions | Enums or traits | Enums often better |
| Builder | Method chaining | Method chaining with ownership | Similar concept |
| Iterator | STL iterators | Zero-cost iterator chains | More powerful in Rust |
| Observer | Callbacks/signals | Traits or channels | Channels often cleaner |
| Strategy | Virtual functions | Trait objects or closures | Closures more idiomatic |
| Singleton | Static instance | lazy_static or OnceLock | Thread-safe by default |
| Smart Pointers | unique_ptr/shared_ptr | Box/Rc/Arc | Similar concepts |
| Newtype | typedef/using | Newtype pattern | Type safety in Rust |
| Type State | - | Zero-cost state machine | Rust-specific |

**Key insight:** Many C++ patterns exist to work around language limitations. Rust's ownership system, enums, and traits often provide cleaner solutions.
