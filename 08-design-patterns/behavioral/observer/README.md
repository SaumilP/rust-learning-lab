# Observer Pattern

> Design reference: [C4 component explanation and embedded PlantUML](DESIGN.md).

## Overview

The Observer pattern defines a one-to-many dependency between objects so that when one object changes state, all its dependents are notified automatically. This is the basis for event-driven systems and reactive programming.

## Problem It Solves

- Decoupling objects that need to communicate
- Broadcasting state changes to multiple observers
- Creating event-driven systems
- Implementing publish-subscribe mechanisms
- Avoiding tight coupling between subject and observers

## Implementation in Rust

### Basic Observer Pattern

```rust
use std::cell::RefCell;
use std::rc::Rc;

// Observer trait - anything that wants to be notified
pub trait Observer {
    fn update(&self, subject_state: &str);
}

// Subject that maintains observers
pub struct Subject {
    state: String,
    observers: RefCell<Vec<Rc<dyn Observer>>>,
}

impl Subject {
    pub fn new() -> Self {
        Self {
            state: String::new(),
            observers: RefCell::new(Vec::new()),
        }
    }

    pub fn attach(&self, observer: Rc<dyn Observer>) {
        self.observers.borrow_mut().push(observer);
    }

    pub fn detach(&self, observer: Rc<dyn Observer>) {
        let mut observers = self.observers.borrow_mut();
        observers.retain(|o| {
            !Rc::ptr_eq(o, &observer)
        });
    }

    pub fn get_state(&self) -> &str {
        &self.state
    }

    pub fn set_state(&self, state: String) {
        self.state = state;
        self.notify_observers();
    }

    fn notify_observers(&self) {
        let observers = self.observers.borrow();
        for observer in observers.iter() {
            observer.update(&self.state);
        }
    }
}

// Concrete observers
pub struct ConcreteObserverA;

impl Observer for ConcreteObserverA {
    fn update(&self, subject_state: &str) {
        println!("ConcreteObserverA: React to state: {}", subject_state);
    }
}

pub struct ConcreteObserverB;

impl Observer for ConcreteObserverB {
    fn update(&self, subject_state: &str) {
        println!("ConcreteObserverB: React to state: {}", subject_state);
    }
}

// Usage
fn main() {
    let subject = Rc::new(Subject::new());

    let observer_a = Rc::new(ConcreteObserverA);
    subject.attach(observer_a.clone());

    let observer_b = Rc::new(ConcreteObserverB);
    subject.attach(observer_b);

    subject.set_state("New State 1".to_string());
    subject.set_state("New State 2".to_string());
}
```

### Event System Observer

```rust
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub type EventHandler = Rc<dyn Fn(&Event)>;

#[derive(Clone, Debug)]
pub struct Event {
    pub event_type: String,
    pub data: String,
}

pub struct EventEmitter {
    handlers: RefCell<HashMap<String, Vec<EventHandler>>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            handlers: RefCell::new(HashMap::new()),
        }
    }

    pub fn on<F>(&self, event_type: &str, handler: F)
    where
        F: Fn(&Event) + 'static,
    {
        let handler = Rc::new(handler);
        let mut handlers = self.handlers.borrow_mut();
        handlers
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(handler);
    }

    pub fn emit(&self, event: Event) {
        let handlers = self.handlers.borrow();
        if let Some(handlers) = handlers.get(&event.event_type) {
            for handler in handlers {
                handler(&event);
            }
        }
    }

    pub fn off(&self, event_type: &str) {
        let mut handlers = self.handlers.borrow_mut();
        handlers.remove(event_type);
    }
}

// Usage
fn main() {
    let emitter = Rc::new(EventEmitter::new());

    emitter.on("user_login", |event| {
        println!("User logged in: {}", event.data);
    });

    emitter.on("user_logout", |event| {
        println!("User logged out: {}", event.data);
    });

    emitter.emit(Event {
        event_type: "user_login".to_string(),
        data: "alice".to_string(),
    });

    emitter.emit(Event {
        event_type: "user_logout".to_string(),
        data: "bob".to_string(),
    });
}
```

### Button Click Observer

```rust
use std::rc::Rc;
use std::cell::RefCell;

pub trait ClickListener {
    fn on_click(&self, button_id: u32);
}

pub struct Button {
    id: u32,
    listeners: RefCell<Vec<Rc<dyn ClickListener>>>,
}

impl Button {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            listeners: RefCell::new(Vec::new()),
        }
    }

    pub fn add_listener(&self, listener: Rc<dyn ClickListener>) {
        self.listeners.borrow_mut().push(listener);
    }

    pub fn click(&self) {
        println!("Button {} clicked", self.id);
        let listeners = self.listeners.borrow();
        for listener in listeners.iter() {
            listener.on_click(self.id);
        }
    }
}

pub struct Dialog;

impl ClickListener for Dialog {
    fn on_click(&self, button_id: u32) {
        println!("Dialog handling click from button {}", button_id);
    }
}

pub struct Logger;

impl ClickListener for Logger {
    fn on_click(&self, button_id: u32) {
        println!("Logging click event for button {}", button_id);
    }
}

// Usage
fn main() {
    let button = Rc::new(Button::new(1));

    button.add_listener(Rc::new(Dialog));
    button.add_listener(Rc::new(Logger));

    button.click();
}
```

## Benefits

1. **Loose Coupling** - Subject and observers are loosely coupled
2. **Dynamic Relationships** - Add/remove observers at runtime
3. **Broadcast Communication** - One-to-many notification
4. **Separation of Concerns** - Subject doesn't need to know observer details
5. **Reactive Systems** - Natural fit for event-driven architecture

## Common Patterns

### Pattern 1: Simple Observer
```rust
pub trait Observer {
    fn update(&self, data: &Data);
}

pub impl Subject {
    pub fn notify(&self) {
        for observer in &self.observers {
            observer.update(&self.data);
        }
    }
}
```

### Pattern 2: Generic Observer
```rust
pub trait Observer<T> {
    fn update(&self, data: &T);
}
```

### Pattern 3: Event-Based Observer
```rust
pub struct EventSystem {
    handlers: HashMap<EventType, Vec<Box<dyn Handler>>>,
}
```

### Pattern 4: Subscription Observer
```rust
pub fn subscribe(&mut self, observer: Rc<dyn Observer>) -> Subscription {
    let id = self.next_id();
    self.observers.insert(id, observer);
    Subscription { id, subject: self.clone() }
}
```

### Pattern 5: Typed Observer
```rust
pub struct TypedObserver<T> {
    handler: Box<dyn Fn(&T)>,
}
```

## Real-World Examples

### UI State Observer
```rust
pub struct WindowState {
    width: u32,
    height: u32,
    listeners: RefCell<Vec<Rc<dyn StateListener>>>,
}

impl WindowState {
    pub fn resize(&self, width: u32, height: u32) {
        let listeners = self.listeners.borrow();
        for listener in listeners.iter() {
            listener.on_resize(width, height);
        }
    }
}

pub trait StateListener {
    fn on_resize(&self, width: u32, height: u32);
}
```

### Model-View Observer
```rust
pub struct DataModel {
    data: RefCell<String>,
    views: RefCell<Vec<Rc<dyn View>>>,
}

pub trait View {
    fn on_data_changed(&self, data: &str);
}

impl DataModel {
    pub fn set_data(&self, data: String) {
        *self.data.borrow_mut() = data.clone();
        let views = self.views.borrow();
        for view in views.iter() {
            view.on_data_changed(&data);
        }
    }
}
```

### Price Change Observer
```rust
pub struct Stock {
    symbol: String,
    price: RefCell<f64>,
    observers: RefCell<Vec<Rc<dyn PriceObserver>>>,
}

pub trait PriceObserver {
    fn on_price_changed(&self, symbol: &str, old: f64, new: f64);
}

impl Stock {
    pub fn set_price(&self, price: f64) {
        let old_price = *self.price.borrow();
        *self.price.borrow_mut() = price;
        let observers = self.observers.borrow();
        for observer in observers.iter() {
            observer.on_price_changed(&self.symbol, old_price, price);
        }
    }
}
```

## Anti-Patterns to Avoid

- **Observer Memory Leaks** - Proper cleanup of circular references
- **Notification Order Issues** - Don't rely on specific observer order
- **Observer Side Effects** - Observers shouldn't modify subject
- **Infinite Loops** - Observer updating subject creating new notifications

## Key Takeaways

- ✓ Observer pattern creates loosely coupled event systems
- ✓ Subject notifies all observers of state changes
- ✓ Observers can be added/removed dynamically
- ✓ Use Rc<RefCell<>> for shared mutable state in Rust
- ✓ Consider event-based systems for complex interactions
- ✓ Be aware of memory management with reference cycles
- ✓ Can be combined with channels for async notifications
