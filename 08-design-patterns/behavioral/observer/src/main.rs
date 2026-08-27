// Observer Pattern in Rust
// Defines a one-to-many dependency between objects
// When one object changes state, all dependents are notified

use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};

fn main() {
    println!("=== Observer Pattern in Rust ===\n");

    // APPROACH 1: Callback-based (traditional)
    println!("1. CALLBACK-BASED OBSERVER:\n");

    let mut stock = Stock::new("RUST", 100.0);

    stock.attach(Box::new(|name, price| {
        println!("   Observer 1: {} price changed to ${}", name, price);
    }));

    stock.attach(Box::new(|name, price| {
        if price > 110.0 {
            println!("   Observer 2: Alert! {} exceeded $110", name);
        }
    }));

    stock.set_price(105.0);
    stock.set_price(115.0);

    println!("\n   ✓ Callbacks notify all observers\n");

    // APPROACH 2: Channel-based (Rust idiomatic)
    println!("2. CHANNEL-BASED OBSERVER:\n");

    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    let mut news = NewsAgency::new();
    news.subscribe(tx1);
    news.subscribe(tx2);

    news.publish("Rust 2.0 released!");

    println!("   Subscriber 1: {}", rx1.recv().unwrap());
    println!("   Subscriber 2: {}", rx2.recv().unwrap());

    news.publish("New async features added");

    println!("   Subscriber 1: {}", rx1.recv().unwrap());
    println!("   Subscriber 2: {}", rx2.recv().unwrap());

    println!("\n   ✓ Channels provide safe message passing\n");

    // APPROACH 3: Trait-based observers
    println!("3. TRAIT-BASED OBSERVER:\n");

    let mut weather = WeatherStation::new();

    weather.attach(Box::new(PhoneDisplay));
    weather.attach(Box::new(WebDisplay));
    weather.attach(Box::new(AlertSystem));

    weather.set_temperature(75.0);
    weather.set_temperature(95.0);

    println!("\n   ✓ Trait objects allow different observer types\n");

    // APPROACH 4: Event system
    println!("4. EVENT SYSTEM:\n");

    let mut event_bus = EventBus::new();

    event_bus.subscribe(EventType::UserLogin, Box::new(|data| {
        println!("   Logging: User logged in - {}", data);
    }));

    event_bus.subscribe(EventType::UserLogin, Box::new(|data| {
        println!("   Analytics: Recording login - {}", data);
    }));

    event_bus.subscribe(EventType::UserLogout, Box::new(|data| {
        println!("   Cleanup: User logged out - {}", data);
    }));

    event_bus.emit(EventType::UserLogin, "user123");
    event_bus.emit(EventType::UserLogout, "user123");

    println!("\n   ✓ Event bus for decoupled communication\n");

    // APPROACH 5: Property change notification
    println!("5. PROPERTY CHANGE NOTIFICATION:\n");

    let user = ObservableUser::new("Alice", 25);
    let user_clone = user.clone();

    std::thread::spawn(move || {
        loop {
            let changed = user_clone.lock().check_changes();
            if changed {
                println!("   Change detected in thread!");
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
            if user_clone.lock().name == "Bob" {
                break;
            }
        }
    });

    std::thread::sleep(std::time::Duration::from_millis(50));
    user.lock().set_name("Bob");

    std::thread::sleep(std::time::Duration::from_millis(200));

    println!("\n   ✓ Thread-safe property observation\n");
}

// ========== APPROACH 1: Callback-Based ==========

type Observer = Box<dyn Fn(&str, f64) + Send>;

struct Stock {
    name: String,
    price: f64,
    observers: Vec<Observer>,
}

impl Stock {
    fn new(name: &str, price: f64) -> Self {
        Stock {
            name: name.to_string(),
            price,
            observers: Vec::new(),
        }
    }

    fn attach(&mut self, observer: Observer) {
        self.observers.push(observer);
    }

    fn set_price(&mut self, price: f64) {
        self.price = price;
        self.notify();
    }

    fn notify(&self) {
        for observer in &self.observers {
            observer(&self.name, self.price);
        }
    }
}

// ========== APPROACH 2: Channel-Based ==========

struct NewsAgency {
    subscribers: Vec<Sender<String>>,
}

impl NewsAgency {
    fn new() -> Self {
        NewsAgency {
            subscribers: Vec::new(),
        }
    }

    fn subscribe(&mut self, subscriber: Sender<String>) {
        self.subscribers.push(subscriber);
    }

    fn publish(&self, news: &str) {
        for subscriber in &self.subscribers {
            let _ = subscriber.send(news.to_string());
        }
    }
}

// ========== APPROACH 3: Trait-Based ==========

trait WeatherObserver {
    fn update(&self, temperature: f64);
}

struct PhoneDisplay;
struct WebDisplay;
struct AlertSystem;

impl WeatherObserver for PhoneDisplay {
    fn update(&self, temperature: f64) {
        println!("   Phone: Temperature is {}°F", temperature);
    }
}

impl WeatherObserver for WebDisplay {
    fn update(&self, temperature: f64) {
        println!("   Web: Current temp: {}°F", temperature);
    }
}

impl WeatherObserver for AlertSystem {
    fn update(&self, temperature: f64) {
        if temperature > 90.0 {
            println!("   Alert: High temperature warning! {}°F", temperature);
        }
    }
}

struct WeatherStation {
    temperature: f64,
    observers: Vec<Box<dyn WeatherObserver>>,
}

impl WeatherStation {
    fn new() -> Self {
        WeatherStation {
            temperature: 70.0,
            observers: Vec::new(),
        }
    }

    fn attach(&mut self, observer: Box<dyn WeatherObserver>) {
        self.observers.push(observer);
    }

    fn set_temperature(&mut self, temp: f64) {
        self.temperature = temp;
        self.notify_observers();
    }

    fn notify_observers(&self) {
        for observer in &self.observers {
            observer.update(self.temperature);
        }
    }
}

// ========== APPROACH 4: Event System ==========

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EventType {
    UserLogin,
    UserLogout,
}

type EventHandler = Box<dyn Fn(&str)>;

struct EventBus {
    handlers: std::collections::HashMap<EventType, Vec<EventHandler>>,
}

impl EventBus {
    fn new() -> Self {
        EventBus {
            handlers: std::collections::HashMap::new(),
        }
    }

    fn subscribe(&mut self, event_type: EventType, handler: EventHandler) {
        self.handlers
            .entry(event_type)
            .or_insert_with(Vec::new)
            .push(handler);
    }

    fn emit(&self, event_type: EventType, data: &str) {
        if let Some(handlers) = self.handlers.get(&event_type) {
            for handler in handlers {
                handler(data);
            }
        }
    }
}

// ========== APPROACH 5: Property Change ==========

#[derive(Clone)]
struct ObservableUser {
    inner: Arc<Mutex<UserData>>,
}

struct UserData {
    name: String,
    age: u32,
    changed: bool,
}

impl ObservableUser {
    fn new(name: &str, age: u32) -> Self {
        ObservableUser {
            inner: Arc::new(Mutex::new(UserData {
                name: name.to_string(),
                age,
                changed: false,
            })),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<UserData> {
        self.inner.lock().unwrap()
    }
}

impl UserData {
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
        self.changed = true;
    }

    fn check_changes(&mut self) -> bool {
        if self.changed {
            self.changed = false;
            true
        } else {
            false
        }
    }
}

/*
Observer Pattern Summary:
=========================

WHEN TO USE:
- One-to-many relationships
- State changes need to notify multiple objects
- Decoupled communication
- Event-driven architecture
- Reactive programming

RUST APPROACHES:

1. CALLBACKS:
   - Simple closures
   - Direct notification
   - Good for simple cases

2. CHANNELS (Idiomatic):
   - mpsc::channel
   - Thread-safe
   - Decoupled
   - Best for async/concurrent

3. TRAIT OBJECTS:
   - Different observer types
   - Traditional OOP style
   - Flexible behavior

4. EVENT BUS:
   - Multiple event types
   - Many-to-many
   - Centralized management

BENEFITS:
- Loose coupling
- Dynamic subscriptions
- Broadcast communication
- Separation of concerns

TRADEOFFS:
- Callbacks: Not thread-safe without Arc
- Channels: Requires explicit receiving
- Trait objects: Heap allocation
- Event bus: More complex

COMPARED TO OTHER LANGUAGES:
Java:    Observable/Observer interfaces
C#:      Events and delegates
JavaScript: EventEmitter, Observables
Rust:    Channels (preferred) or callbacks

RUST ADVANTAGES:
- Channels are thread-safe
- No null observer issues
- Type-safe event handling
- Compile-time guarantees

BEST PRACTICES:
- Prefer channels for async/threading
- Use callbacks for simple sync cases
- Consider event bus for complex systems
- Avoid circular references
- Use weak references when needed (Arc/Weak)

COMMON USE CASES:
- UI updates (MVC pattern)
- Event handling
- Publish-subscribe systems
- Stock tickers
- News feeds
- Sensor data
- Real-time updates

Run this:
    cargo run

Experiment:
    - Add more observers
    - Try removing observers
    - Implement unsubscribe
    - Compare channel vs callback performance
    - Build a real-time notification system
*/
