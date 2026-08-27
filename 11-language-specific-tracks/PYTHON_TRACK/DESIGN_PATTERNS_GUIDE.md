# Design Patterns: Python to Rust Translation

How Python patterns translate to Rust. Some patterns work similarly, some work differently, and some aren't needed at all!

## Important Note

Rust doesn't have classes, so many object-oriented patterns look different. Instead of forcing Python patterns onto Rust, understand the *intent* and see how Rust achieves it (often more simply).

---

## Python-Specific Patterns

### 1. Decorator Pattern

**Python version**:

```python
# Python: Function decorators
import time
from functools import wraps

def timing_decorator(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        start = time.time()
        result = func(*args, **kwargs)
        end = time.time()
        print(f"{func.__name__} took {end - start:.2f}s")
        return result
    return wrapper

@timing_decorator
def slow_function():
    time.sleep(1)
    return "done"

slow_function()  # Prints: slow_function took 1.00s
```

**Rust equivalent (derive macros)**:

Rust doesn't have runtime decorators, but has compile-time macros:

```rust
// Rust: Procedural macros (advanced)
// For simple cases, use wrapper functions:

use std::time::Instant;

fn with_timing<F, R>(name: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("{} took {:.2?}", name, duration);
    result
}

// Usage
let result = with_timing("slow_function", || {
    std::thread::sleep(std::time::Duration::from_secs(1));
    "done"
});
```

**Rust alternative (derive macros for structs)**:

```rust
// Derive macros are like class decorators
#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
}

// Automatically implements Debug, Clone, PartialEq traits
```

### 2. Context Manager (`with` statement)

**Python version**:

```python
# Python: Context managers
class Database:
    def __enter__(self):
        print("Opening database")
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        print("Closing database")

with Database() as db:
    print("Using database")
# Automatically calls __exit__
```

**Rust equivalent (RAII - Resource Acquisition Is Initialization)**:

```rust
// Rust: Automatic cleanup with Drop trait
struct Database;

impl Database {
    fn new() -> Self {
        println!("Opening database");
        Database
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        println!("Closing database");
    }
}

// Usage
{
    let db = Database::new();
    println!("Using database");
}  // Automatically calls drop() here
```

**Rust is better**: Cleanup is guaranteed and happens deterministically when the value goes out of scope. No need to remember `with`.

### 3. Iterator/Generator Pattern

**Python version**:

```python
# Python: Generators
def fibonacci():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b

# Usage
for i, num in enumerate(fibonacci()):
    if i >= 10:
        break
    print(num)
```

**Rust equivalent**:

```rust
// Rust: Iterator trait
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let result = self.curr;
        self.curr = self.next;
        self.next += result;
        Some(result)
    }
}

// Usage
for num in Fibonacci::new().take(10) {
    println!("{}", num);
}
```

**Key difference**: Rust iterators are lazy like Python generators, but type-safe and zero-cost.

### 4. Property Decorators

**Python version**:

```python
# Python: Properties
class Circle:
    def __init__(self, radius):
        self._radius = radius

    @property
    def radius(self):
        return self._radius

    @radius.setter
    def radius(self, value):
        if value < 0:
            raise ValueError("Radius must be positive")
        self._radius = value

    @property
    def area(self):
        return 3.14159 * self._radius ** 2

circle = Circle(5)
print(circle.radius)  # 5
print(circle.area)    # 78.53975
circle.radius = 10
```

**Rust equivalent**:

```rust
// Rust: Just use methods (no properties)
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }

    // Getter
    fn radius(&self) -> f64 {
        self.radius
    }

    // Setter
    fn set_radius(&mut self, radius: f64) -> Result<(), String> {
        if radius < 0.0 {
            return Err("Radius must be positive".to_string());
        }
        self.radius = radius;
        Ok(())
    }

    // Computed property
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

// Usage
let mut circle = Circle::new(5.0);
println!("{}", circle.radius());  // 5
println!("{}", circle.area());    // 78.53981633974483
circle.set_radius(10.0)?;
```

**Rust philosophy**: No magic. Methods are explicit.

---

## Classic Gang of Four Patterns

### 5. Builder Pattern

**Python version**:

```python
# Python: Builder (less common, but used)
class HttpClientBuilder:
    def __init__(self):
        self.timeout = 30
        self.retries = 3
        self.headers = {}

    def with_timeout(self, timeout):
        self.timeout = timeout
        return self

    def with_retries(self, retries):
        self.retries = retries
        return self

    def build(self):
        return HttpClient(self.timeout, self.retries, self.headers)

client = (HttpClientBuilder()
    .with_timeout(60)
    .with_retries(5)
    .build())
```

**🦀 Rust version**:

```rust
// Rust: Builder pattern (very common)
struct HttpClient {
    timeout: u64,
    retries: u32,
}

struct HttpClientBuilder {
    timeout: u64,
    retries: u32,
}

impl HttpClientBuilder {
    fn new() -> Self {
        HttpClientBuilder {
            timeout: 30,
            retries: 3,
        }
    }

    fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self  // Return self for chaining
    }

    fn retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    fn build(self) -> HttpClient {
        HttpClient {
            timeout: self.timeout,
            retries: self.retries,
        }
    }
}

// Usage
let client = HttpClientBuilder::new()
    .timeout(60)
    .retries(5)
    .build();
```

**Very similar!** Builder pattern is idiomatic in both languages.

### 6. Factory Pattern

**Python version**:

```python
# Python: Factory
class Animal:
    def speak(self):
        pass

class Dog(Animal):
    def speak(self):
        return "Woof!"

class Cat(Animal):
    def speak(self):
        return "Meow!"

def create_animal(animal_type):
    if animal_type == "dog":
        return Dog()
    elif animal_type == "cat":
        return Cat()
    else:
        raise ValueError(f"Unknown animal: {animal_type}")

animal = create_animal("dog")
print(animal.speak())
```

**Rust version (enum-based, better!)**:

```rust
// Rust: Use enums instead of inheritance
enum Animal {
    Dog,
    Cat,
}

impl Animal {
    fn speak(&self) -> &str {
        match self {
            Animal::Dog => "Woof!",
            Animal::Cat => "Meow!",
        }
    }
}

fn create_animal(animal_type: &str) -> Option<Animal> {
    match animal_type {
        "dog" => Some(Animal::Dog),
        "cat" => Some(Animal::Cat),
        _ => None,  // Type-safe "unknown"
    }
}

// Usage
if let Some(animal) = create_animal("dog") {
    println!("{}", animal.speak());
}
```

**Rust is better**: Enums are type-safe and exhaustive. Compiler ensures you handle all cases.

### 7. Observer Pattern

**Python version**:

```python
# Python: Observer
class Subject:
    def __init__(self):
        self._observers = []

    def attach(self, observer):
        self._observers.append(observer)

    def notify(self, message):
        for observer in self._observers:
            observer.update(message)

class Observer:
    def update(self, message):
        print(f"Received: {message}")

subject = Subject()
subject.attach(Observer())
subject.notify("Hello!")
```

**Rust version (callbacks)**:

```rust
// Rust: Callbacks with closures
struct Subject {
    observers: Vec<Box<dyn Fn(&str)>>,
}

impl Subject {
    fn new() -> Self {
        Subject { observers: Vec::new() }
    }

    fn attach<F>(&mut self, observer: F)
    where
        F: Fn(&str) + 'static,
    {
        self.observers.push(Box::new(observer));
    }

    fn notify(&self, message: &str) {
        for observer in &self.observers {
            observer(message);
        }
    }
}

// Usage
let mut subject = Subject::new();
subject.attach(|msg| println!("Observer 1: {}", msg));
subject.attach(|msg| println!("Observer 2: {}", msg));
subject.notify("Hello!");
```

**Rust alternative (channels for async systems)**:

```rust
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    // Spawn observers
    tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("Observer 1: {}", msg);
        }
    });

    tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("Observer 2: {}", msg);
        }
    });

    // Send event
    tx.send("Hello!").unwrap();
}
```

### 8. Strategy Pattern

**Python version**:

```python
# Python: Strategy with classes
class PaymentStrategy:
    def pay(self, amount):
        pass

class CreditCard(PaymentStrategy):
    def pay(self, amount):
        print(f"Paid ${amount} with credit card")

class PayPal(PaymentStrategy):
    def pay(self, amount):
        print(f"Paid ${amount} with PayPal")

class ShoppingCart:
    def __init__(self, strategy: PaymentStrategy):
        self.strategy = strategy

    def checkout(self, amount):
        self.strategy.pay(amount)

cart = ShoppingCart(CreditCard())
cart.checkout(100)
```

**Rust version (trait objects)**:

```rust
// Rust: Trait for strategy
trait PaymentStrategy {
    fn pay(&self, amount: f64);
}

struct CreditCard;
struct PayPal;

impl PaymentStrategy for CreditCard {
    fn pay(&self, amount: f64) {
        println!("Paid ${} with credit card", amount);
    }
}

impl PaymentStrategy for PayPal {
    fn pay(&self, amount: f64) {
        println!("Paid ${} with PayPal", amount);
    }
}

struct ShoppingCart {
    strategy: Box<dyn PaymentStrategy>,
}

impl ShoppingCart {
    fn new(strategy: Box<dyn PaymentStrategy>) -> Self {
        ShoppingCart { strategy }
    }

    fn checkout(&self, amount: f64) {
        self.strategy.pay(amount);
    }
}

// Usage
let cart = ShoppingCart::new(Box::new(CreditCard));
cart.checkout(100.0);
```

**Rust alternative (enums, often better)**:

```rust
// Simpler and faster with enums
enum PaymentStrategy {
    CreditCard,
    PayPal,
}

impl PaymentStrategy {
    fn pay(&self, amount: f64) {
        match self {
            PaymentStrategy::CreditCard => {
                println!("Paid ${} with credit card", amount);
            }
            PaymentStrategy::PayPal => {
                println!("Paid ${} with PayPal", amount);
            }
        }
    }
}

struct ShoppingCart {
    strategy: PaymentStrategy,
}

// Simpler, faster (static dispatch), type-safe
```

### 9. Singleton Pattern

**Python version**:

```python
# Python: Singleton
class Database:
    _instance = None

    def __new__(cls):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
        return cls._instance

db1 = Database()
db2 = Database()
assert db1 is db2  # Same instance
```

**🦀 Rust version**:

```rust
// Rust: Lazy static
use once_cell::sync::Lazy;
use std::sync::Mutex;

static DATABASE: Lazy<Mutex<Database>> = Lazy::new(|| {
    Mutex::new(Database::new())
});

struct Database {
    // fields
}

impl Database {
    fn new() -> Self {
        Database { /* initialize */ }
    }
}

// Usage
fn example() {
    let db = DATABASE.lock().unwrap();
    // Use db
}
```

**Rust alternative**: Often just use module-level statics or pass instances around.

---

## Pythonic Patterns in Rust

### 10. List Comprehensions → Iterator Chains

**Python**:

```python
# List comprehension
squares = [x**2 for x in range(10) if x % 2 == 0]

# Generator expression
squares_gen = (x**2 for x in range(10) if x % 2 == 0)
```

**Rust**:

```rust
// Iterator chain (lazy like generators)
let squares: Vec<i32> = (0..10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .collect();

// Lazy iterator (like generator expression)
let squares_iter = (0..10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x);
// Not evaluated until consumed
```

### 11. Dictionary Comprehensions → Iterator + collect

**Python**:

```python
# Dict comprehension
squared = {x: x**2 for x in range(5)}
```

**Rust**:

```rust
use std::collections::HashMap;

let squared: HashMap<i32, i32> = (0..5)
    .map(|x| (x, x * x))
    .collect();
```

### 12. Multiple Return Values → Tuples

**Python**:

```python
def get_user():
    return "Alice", 30

name, age = get_user()
```

**Rust**:

```rust
fn get_user() -> (&'static str, u32) {
    ("Alice", 30)
}

let (name, age) = get_user();
```

---

## Summary: Pattern Translation

| Pattern | Python Approach | Rust Approach |
|---------|----------------|---------------|
| **Decorator** | `@decorator` | Wrapper functions or macros |
| **Context Manager** | `with` statement | RAII (Drop trait) |
| **Iterator** | Generators with `yield` | Iterator trait |
| **Property** | `@property` | Getter/setter methods |
| **Builder** | Method chaining | Method chaining (same!) |
| **Factory** | Classes + inheritance | Enums or trait objects |
| **Observer** | Callback lists | Closures or channels |
| **Strategy** | Classes | Traits or enums |
| **Singleton** | `__new__` override | Lazy static |

## Key Takeaways

1. **RAII replaces context managers** - Automatic, deterministic cleanup
2. **Enums replace inheritance** - Type-safe, exhaustive matching
3. **Traits replace duck typing** - Explicit, compile-time checked
4. **Iterators are lazy** - Like Python generators, but type-safe
5. **Macros replace decorators** - Compile-time code generation
6. **Closures are powerful** - Often simpler than full trait implementations

## Philosophy Differences

| Python Says | Rust Says |
|-------------|-----------|
| "Duck typing is flexible" | "Traits are explicit and safe" |
| "Magic methods (__enter__, __exit__)" | "Explicit traits (Drop, Iterator)" |
| "Runtime flexibility" | "Compile-time guarantees" |
| "Simple is better" | "Correct is better" |

Don't try to write Python in Rust. Learn the Rust way—it's often simpler once you understand it!

Next: Build the mini-projects to practice these patterns!
