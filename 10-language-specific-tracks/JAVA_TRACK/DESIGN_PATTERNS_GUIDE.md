# Design Patterns: Java to Rust Translation

If you're coming from Java, you've probably memorized the Gang of Four patterns. The good news: most of them translate to Rust. The better news: some are way simpler in Rust.

## Important note before we start

Rust doesn't have classes or inheritance. This means some patterns work differently—and honestly, some patterns you used in Java aren't even necessary in Rust.

Don't try to force Java patterns onto Rust. Instead, understand the *intent* of each pattern, then see how Rust achieves that intent (often more simply).

---

## Creational Patterns

### Builder Pattern

**Java version**:

```java
// Java: Builder with fluent API
public class HttpClient {
    private final Duration timeout;
    private final int maxRetries;
    private final String userAgent;

    private HttpClient(Builder builder) {
        this.timeout = builder.timeout;
        this.maxRetries = builder.maxRetries;
        this.userAgent = builder.userAgent;
    }

    public static class Builder {
        private Duration timeout = Duration.ofSeconds(30);
        private int maxRetries = 3;
        private String userAgent = "MyApp/1.0";

        public Builder timeout(Duration timeout) {
            this.timeout = timeout;
            return this;
        }

        public Builder maxRetries(int maxRetries) {
            this.maxRetries = maxRetries;
            return this;
        }

        public Builder userAgent(String userAgent) {
            this.userAgent = userAgent;
            return this;
        }

        public HttpClient build() {
            return new HttpClient(this);
        }
    }
}

// Usage
HttpClient client = new HttpClient.Builder()
    .timeout(Duration.ofSeconds(10))
    .maxRetries(5)
    .build();
```

**Rust version (basic)**:

```rust
use std::time::Duration;

pub struct HttpClient {
    timeout: Duration,
    max_retries: u32,
    user_agent: String,
}

pub struct HttpClientBuilder {
    timeout: Duration,
    max_retries: u32,
    user_agent: String,
}

impl HttpClientBuilder {
    pub fn new() -> Self {
        HttpClientBuilder {
            timeout: Duration::from_secs(30),
            max_retries: 3,
            user_agent: String::from("MyApp/1.0"),
        }
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self  // Return self for chaining
    }

    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    pub fn build(self) -> HttpClient {
        HttpClient {
            timeout: self.timeout,
            max_retries: self.max_retries,
            user_agent: self.user_agent,
        }
    }
}

// Usage
let client = HttpClientBuilder::new()
    .timeout(Duration::from_secs(10))
    .max_retries(5)
    .build();
```

**Rust version (with typestate pattern)**:

This is a Rust-specific improvement that uses the type system to enforce required fields:

```rust
// Phantom types for builder states
struct MissingTimeout;
struct HasTimeout;

pub struct HttpClientBuilder<T> {
    timeout: Option<Duration>,
    max_retries: u32,
    user_agent: String,
    _marker: std::marker::PhantomData<T>,
}

impl HttpClientBuilder<MissingTimeout> {
    pub fn new() -> Self {
        HttpClientBuilder {
            timeout: None,
            max_retries: 3,
            user_agent: String::from("MyApp/1.0"),
            _marker: std::marker::PhantomData,
        }
    }

    // timeout() transitions to HasTimeout state
    pub fn timeout(self, timeout: Duration) -> HttpClientBuilder<HasTimeout> {
        HttpClientBuilder {
            timeout: Some(timeout),
            max_retries: self.max_retries,
            user_agent: self.user_agent,
            _marker: std::marker::PhantomData,
        }
    }
}

impl HttpClientBuilder<HasTimeout> {
    // Only HasTimeout can call build()
    pub fn build(self) -> HttpClient {
        HttpClient {
            timeout: self.timeout.unwrap(),
            max_retries: self.max_retries,
            user_agent: self.user_agent,
        }
    }
}

// Won't compile without timeout!
let client = HttpClientBuilder::new()
    .timeout(Duration::from_secs(10))
    .build();
```

**Key difference**: Rust can enforce required fields at compile time with the typestate pattern.

---

### Factory Pattern

**Java version**:

```java
// Java factory
public interface Animal {
    void makeSound();
}

public class Dog implements Animal {
    public void makeSound() {
        System.out.println("Woof!");
    }
}

public class Cat implements Animal {
    public void makeSound() {
        System.out.println("Meow!");
    }
}

public class AnimalFactory {
    public static Animal createAnimal(String type) {
        switch (type) {
            case "dog": return new Dog();
            case "cat": return new Cat();
            default: throw new IllegalArgumentException("Unknown type");
        }
    }
}

// Usage
Animal animal = AnimalFactory.createAnimal("dog");
animal.makeSound();
```

**Rust version (enum-based)**:

```rust
// Rust: Use enums instead of inheritance
pub enum Animal {
    Dog,
    Cat,
}

impl Animal {
    pub fn make_sound(&self) {
        match self {
            Animal::Dog => println!("Woof!"),
            Animal::Cat => println!("Meow!"),
        }
    }
}

pub fn create_animal(kind: &str) -> Option<Animal> {
    match kind {
        "dog" => Some(Animal::Dog),
        "cat" => Some(Animal::Cat),
        _ => None,  // Return None instead of throwing
    }
}

// Usage
if let Some(animal) = create_animal("dog") {
    animal.make_sound();
}
```

**Rust version (trait objects, when you need different data)**:

```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog { name: String }
struct Cat { name: String }

impl Animal for Dog {
    fn make_sound(&self) {
        println!("{} says Woof!", self.name);
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("{} says Meow!", self.name);
    }
}

fn create_animal(kind: &str, name: String) -> Option<Box<dyn Animal>> {
    match kind {
        "dog" => Some(Box::new(Dog { name })),
        "cat" => Some(Box::new(Cat { name })),
        _ => None,
    }
}

// Usage
if let Some(animal) = create_animal("dog", "Buddy".to_string()) {
    animal.make_sound();
}
```

**Key difference**: Prefer enums in Rust. Use trait objects only when you need different data or external extensibility.

---

### Singleton Pattern

**Java version**:

```java
// Java singleton (thread-safe)
public class Database {
    private static volatile Database instance;

    private Database() {
        // Private constructor
    }

    public static Database getInstance() {
        if (instance == null) {
            synchronized (Database.class) {
                if (instance == null) {
                    instance = new Database();
                }
            }
        }
        return instance;
    }
}
```

**Rust version (lazy_static)**:

```rust
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DATABASE: Mutex<Database> = Mutex::new(Database::new());
}

struct Database {
    // fields
}

impl Database {
    fn new() -> Self {
        Database { /* initialize */ }
    }

    fn query(&self) {
        // database operations
    }
}

// Usage
fn example() {
    let db = DATABASE.lock().unwrap();
    db.query();
}
```

**Rust version (once_cell, more modern)**:

```rust
use std::sync::Mutex;
use once_cell::sync::Lazy;

static DATABASE: Lazy<Mutex<Database>> = Lazy::new(|| {
    Mutex::new(Database::new())
});

// Usage is the same
```

**Key difference**: In Rust, singletons are usually just statics with lazy initialization. No double-checked locking needed.

---

## Structural Patterns

### Adapter Pattern

**Java version**:

```java
// Java adapter
interface MediaPlayer {
    void play(String filename);
}

class Mp3Player {
    void playMp3(String filename) {
        System.out.println("Playing MP3: " + filename);
    }
}

class MediaAdapter implements MediaPlayer {
    private Mp3Player mp3Player;

    public MediaAdapter(Mp3Player player) {
        this.mp3Player = player;
    }

    public void play(String filename) {
        mp3Player.playMp3(filename);
    }
}
```

**Rust version**:

```rust
// Target trait
trait MediaPlayer {
    fn play(&self, filename: &str);
}

// Adaptee (external type we don't control)
struct Mp3Player;

impl Mp3Player {
    fn play_mp3(&self, filename: &str) {
        println!("Playing MP3: {}", filename);
    }
}

// Adapter
struct MediaAdapter {
    mp3_player: Mp3Player,
}

impl MediaPlayer for MediaAdapter {
    fn play(&self, filename: &str) {
        self.mp3_player.play_mp3(filename);
    }
}

// Usage
let adapter = MediaAdapter { mp3_player: Mp3Player };
adapter.play("song.mp3");
```

**Rust bonus (newtype pattern)**:

If you just need to add a trait to an existing type:

```rust
struct Mp3Adapter(Mp3Player);

impl MediaPlayer for Mp3Adapter {
    fn play(&self, filename: &str) {
        self.0.play_mp3(filename);
    }
}

// Even more concise!
```

---

### Decorator Pattern

**Java version**:

```java
// Java decorator
interface Coffee {
    double cost();
    String description();
}

class SimpleCoffee implements Coffee {
    public double cost() { return 2.0; }
    public String description() { return "Simple coffee"; }
}

class MilkDecorator implements Coffee {
    private Coffee coffee;

    public MilkDecorator(Coffee coffee) {
        this.coffee = coffee;
    }

    public double cost() {
        return coffee.cost() + 0.5;
    }

    public String description() {
        return coffee.description() + ", milk";
    }
}

// Usage
Coffee coffee = new SimpleCoffee();
coffee = new MilkDecorator(coffee);
```

**Rust version (composition)**:

```rust
trait Coffee {
    fn cost(&self) -> f64;
    fn description(&self) -> String;
}

struct SimpleCoffee;

impl Coffee for SimpleCoffee {
    fn cost(&self) -> f64 { 2.0 }
    fn description(&self) -> String {
        String::from("Simple coffee")
    }
}

struct MilkDecorator<T: Coffee> {
    coffee: T,
}

impl<T: Coffee> Coffee for MilkDecorator<T> {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.5
    }

    fn description(&self) -> String {
        format!("{}, milk", self.coffee.description())
    }
}

// Usage
let coffee = SimpleCoffee;
let coffee = MilkDecorator { coffee };
println!("{}: ${}", coffee.description(), coffee.cost());
```

**Rust alternative (builder pattern)**:

Often in Rust, you'd just use a builder:

```rust
struct Coffee {
    has_milk: bool,
    has_sugar: bool,
    base_cost: f64,
}

impl Coffee {
    fn new() -> Self {
        Coffee { has_milk: false, has_sugar: false, base_cost: 2.0 }
    }

    fn with_milk(mut self) -> Self {
        self.has_milk = true;
        self
    }

    fn with_sugar(mut self) -> Self {
        self.has_sugar = true;
        self
    }

    fn cost(&self) -> f64 {
        let mut cost = self.base_cost;
        if self.has_milk { cost += 0.5; }
        if self.has_sugar { cost += 0.3; }
        cost
    }
}

// Cleaner usage
let coffee = Coffee::new().with_milk().with_sugar();
```

---

## Behavioral Patterns

### Strategy Pattern

**Java version**:

```java
// Java strategy
interface PaymentStrategy {
    void pay(double amount);
}

class CreditCardPayment implements PaymentStrategy {
    public void pay(double amount) {
        System.out.println("Paid " + amount + " with credit card");
    }
}

class PayPalPayment implements PaymentStrategy {
    public void pay(double amount) {
        System.out.println("Paid " + amount + " with PayPal");
    }
}

class ShoppingCart {
    private PaymentStrategy paymentStrategy;

    public void setPaymentStrategy(PaymentStrategy strategy) {
        this.paymentStrategy = strategy;
    }

    public void checkout(double amount) {
        paymentStrategy.pay(amount);
    }
}
```

**Rust version (trait objects)**:

```rust
trait PaymentStrategy {
    fn pay(&self, amount: f64);
}

struct CreditCardPayment;
struct PayPalPayment;

impl PaymentStrategy for CreditCardPayment {
    fn pay(&self, amount: f64) {
        println!("Paid {} with credit card", amount);
    }
}

impl PaymentStrategy for PayPalPayment {
    fn pay(&self, amount: f64) {
        println!("Paid {} with PayPal", amount);
    }
}

struct ShoppingCart {
    payment_strategy: Box<dyn PaymentStrategy>,
}

impl ShoppingCart {
    fn new(payment_strategy: Box<dyn PaymentStrategy>) -> Self {
        ShoppingCart { payment_strategy }
    }

    fn checkout(&self, amount: f64) {
        self.payment_strategy.pay(amount);
    }
}

// Usage
let cart = ShoppingCart::new(Box::new(CreditCardPayment));
cart.checkout(100.0);
```

**Rust alternative (enums, often better)**:

```rust
enum PaymentStrategy {
    CreditCard,
    PayPal,
}

impl PaymentStrategy {
    fn pay(&self, amount: f64) {
        match self {
            PaymentStrategy::CreditCard => {
                println!("Paid {} with credit card", amount);
            }
            PaymentStrategy::PayPal => {
                println!("Paid {} with PayPal", amount);
            }
        }
    }
}

struct ShoppingCart {
    payment_strategy: PaymentStrategy,
}

// Simpler and faster!
```

---

### Observer Pattern

**Java version**:

```java
// Java observer
interface Observer {
    void update(String message);
}

class Subject {
    private List<Observer> observers = new ArrayList<>();

    public void attach(Observer observer) {
        observers.add(observer);
    }

    public void notifyObservers(String message) {
        for (Observer observer : observers) {
            observer.update(message);
        }
    }
}
```

**Rust version (callbacks)**:

```rust
type Callback = Box<dyn Fn(&str)>;

struct Subject {
    observers: Vec<Callback>,
}

impl Subject {
    fn new() -> Self {
        Subject { observers: Vec::new() }
    }

    fn attach<F>(&mut self, callback: F)
    where
        F: Fn(&str) + 'static,
    {
        self.observers.push(Box::new(callback));
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

**Rust alternative (channels)**:

For async systems, channels are often better:

```rust
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    // Observer 1
    tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("Observer 1: {}", msg);
        }
    });

    // Observer 2
    tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("Observer 2: {}", msg);
        }
    });

    // Notify
    tx.send("Hello!").unwrap();
}
```

---

### Command Pattern

**Java version**:

```java
// Java command
interface Command {
    void execute();
}

class Light {
    void turnOn() { System.out.println("Light on"); }
    void turnOff() { System.out.println("Light off"); }
}

class TurnOnCommand implements Command {
    private Light light;

    public TurnOnCommand(Light light) {
        this.light = light;
    }

    public void execute() {
        light.turnOn();
    }
}

class RemoteControl {
    private Command command;

    public void setCommand(Command command) {
        this.command = command;
    }

    public void pressButton() {
        command.execute();
    }
}
```

**Rust version**:

```rust
trait Command {
    fn execute(&self);
}

struct Light;

impl Light {
    fn turn_on(&self) {
        println!("Light on");
    }

    fn turn_off(&self) {
        println!("Light off");
    }
}

struct TurnOnCommand {
    light: Light,
}

impl Command for TurnOnCommand {
    fn execute(&self) {
        self.light.turn_on();
    }
}

struct RemoteControl {
    command: Box<dyn Command>,
}

impl RemoteControl {
    fn press_button(&self) {
        self.command.execute();
    }
}
```

**Rust alternative (closures)**:

```rust
struct RemoteControl<F>
where
    F: Fn(),
{
    command: F,
}

impl<F> RemoteControl<F>
where
    F: Fn(),
{
    fn new(command: F) -> Self {
        RemoteControl { command }
    }

    fn press_button(&self) {
        (self.command)();
    }
}

// Usage
let light = Light;
let remote = RemoteControl::new(|| light.turn_on());
remote.press_button();
```

Much simpler with closures!

---

## Summary: Patterns That Change in Rust

| Pattern | Java Approach | Rust Approach |
|---------|---------------|---------------|
| **Factory** | Classes + inheritance | Enums or trait objects |
| **Builder** | Nested class | Separate struct, or typestate |
| **Singleton** | Double-checked locking | Lazy static |
| **Adapter** | Wrapper class | Newtype pattern |
| **Decorator** | Wrapper chain | Composition or builder |
| **Strategy** | Interface + classes | Traits, enums, or closures |
| **Observer** | Interface + list | Callbacks or channels |
| **Command** | Interface + classes | Traits or closures |

## Key takeaways

1. **Enums are powerful** - Many patterns become simple enum matches
2. **Traits > Inheritance** - Composition beats hierarchies
3. **Closures replace simple patterns** - Command and Strategy often don't need full structs
4. **Type system helps** - Typestate pattern enforces correctness at compile time
5. **Channels for async** - Observer pattern often uses message passing

Don't force Java patterns onto Rust. Think about the problem, then use Rust's tools to solve it elegantly.

Next: Check out ANTI_PATTERNS.md to see common mistakes Java developers make!
