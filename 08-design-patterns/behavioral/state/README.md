# State Pattern

## Overview

The State pattern allows an object to alter its behavior when its internal state changes. The object will appear to change its class. This pattern is particularly useful for state machines and workflow systems.

## Problem It Solves

- Objects with behavior dependent on state
- Large conditional statements based on state
- State transitions and validation
- Encapsulating state-specific behavior
- Making state changes explicit and safe

## Implementation in Rust

### Basic State Pattern

```rust
pub trait State {
    fn handle_request(&self, context: &mut Context);
}

pub struct Context {
    state: Box<dyn State>,
    data: String,
}

impl Context {
    pub fn new() -> Self {
        Self {
            state: Box::new(ConcreteStateA),
            data: String::new(),
        }
    }

    pub fn request(&mut self) {
        self.state.handle_request(self);
    }

    pub fn set_state(&mut self, state: Box<dyn State>) {
        self.state = state;
    }
}

pub struct ConcreteStateA;

impl State for ConcreteStateA {
    fn handle_request(&self, context: &mut Context) {
        println!("In state A");
        context.set_state(Box::new(ConcreteStateB));
    }
}

pub struct ConcreteStateB;

impl State for ConcreteStateB {
    fn handle_request(&self, context: &mut Context) {
        println!("In state B");
        context.set_state(Box::new(ConcreteStateA));
    }
}
```

### Order State Machine

```rust
pub trait OrderState {
    fn confirm(&self) -> Box<dyn OrderState>;
    fn ship(&self) -> Box<dyn OrderState>;
    fn deliver(&self) -> Box<dyn OrderState>;
    fn cancel(&self) -> Box<dyn OrderState>;
    fn status(&self) -> &'static str;
}

pub struct PendingOrder;

impl OrderState for PendingOrder {
    fn confirm(&self) -> Box<dyn OrderState> {
        println!("Order confirmed");
        Box::new(ConfirmedOrder)
    }

    fn ship(&self) -> Box<dyn OrderState> {
        panic!("Cannot ship pending order");
    }

    fn deliver(&self) -> Box<dyn OrderState> {
        panic!("Cannot deliver pending order");
    }

    fn cancel(&self) -> Box<dyn OrderState> {
        println!("Order cancelled");
        Box::new(CancelledOrder)
    }

    fn status(&self) -> &'static str {
        "Pending"
    }
}

pub struct ConfirmedOrder;

impl OrderState for ConfirmedOrder {
    fn confirm(&self) -> Box<dyn OrderState> {
        panic!("Already confirmed");
    }

    fn ship(&self) -> Box<dyn OrderState> {
        println!("Order shipped");
        Box::new(ShippedOrder)
    }

    fn deliver(&self) -> Box<dyn OrderState> {
        panic!("Not shipped yet");
    }

    fn cancel(&self) -> Box<dyn OrderState> {
        println!("Order cancelled");
        Box::new(CancelledOrder)
    }

    fn status(&self) -> &'static str {
        "Confirmed"
    }
}

pub struct ShippedOrder;

impl OrderState for ShippedOrder {
    fn confirm(&self) -> Box<dyn OrderState> {
        panic!("Already confirmed");
    }

    fn ship(&self) -> Box<dyn OrderState> {
        panic!("Already shipped");
    }

    fn deliver(&self) -> Box<dyn OrderState> {
        println!("Order delivered");
        Box::new(DeliveredOrder)
    }

    fn cancel(&self) -> Box<dyn OrderState> {
        panic!("Cannot cancel shipped order");
    }

    fn status(&self) -> &'static str {
        "Shipped"
    }
}

pub struct DeliveredOrder;

impl OrderState for DeliveredOrder {
    fn confirm(&self) -> Box<dyn OrderState> {
        panic!("Already delivered");
    }

    fn ship(&self) -> Box<dyn OrderState> {
        panic!("Already delivered");
    }

    fn deliver(&self) -> Box<dyn OrderState> {
        panic!("Already delivered");
    }

    fn cancel(&self) -> Box<dyn OrderState> {
        panic!("Cannot cancel delivered order");
    }

    fn status(&self) -> &'static str {
        "Delivered"
    }
}

pub struct CancelledOrder;

impl OrderState for CancelledOrder {
    fn confirm(&self) -> Box<dyn OrderState> {
        panic!("Cancelled order");
    }

    fn ship(&self) -> Box<dyn OrderState> {
        panic!("Cancelled order");
    }

    fn deliver(&self) -> Box<dyn OrderState> {
        panic!("Cancelled order");
    }

    fn cancel(&self) -> Box<dyn OrderState> {
        panic!("Already cancelled");
    }

    fn status(&self) -> &'static str {
        "Cancelled"
    }
}

pub struct Order {
    state: Box<dyn OrderState>,
}

impl Order {
    pub fn new() -> Self {
        Self {
            state: Box::new(PendingOrder),
        }
    }

    pub fn confirm(&mut self) {
        self.state = self.state.confirm();
    }

    pub fn ship(&mut self) {
        self.state = self.state.ship();
    }

    pub fn deliver(&mut self) {
        self.state = self.state.deliver();
    }

    pub fn cancel(&mut self) {
        self.state = self.state.cancel();
    }

    pub fn status(&self) -> &'static str {
        self.state.status()
    }
}
```

## Benefits

1. **Encapsulation** - Each state encapsulates behavior
2. **Clarity** - State transitions are explicit
3. **Maintainability** - Adding states doesn't affect existing states
4. **Single Responsibility** - Each state class handles one state
5. **Eliminates Conditionals** - No long if/else chains

## Common Patterns

### Pattern 1: Simple State
```rust
pub trait State {
    fn handle(&self, context: &mut Context);
}
```

### Pattern 2: State with Data
```rust
pub trait State {
    fn enter(&mut self, context: &mut Context);
    fn exit(&mut self, context: &mut Context);
    fn handle(&self, context: &mut Context);
}
```

### Pattern 3: Typed State Machine
```rust
pub struct StateMachine<S: State> {
    state: S,
}

impl<S: State> StateMachine<S> {
    pub fn transition<T: State>(self, new_state: T) -> StateMachine<T> {
        StateMachine { state: new_state }
    }
}
```

### Pattern 4: State with Enum
```rust
pub enum State {
    Pending,
    Active,
    Done,
}
```

### Pattern 5: Hierarchical States
```rust
pub struct CompositeState {
    children: Vec<Box<dyn State>>,
    current: usize,
}
```

## Real-World Examples

### Traffic Light State Machine
```rust
pub trait LightState {
    fn change_light(&self) -> Box<dyn LightState>;
    fn color(&self) -> &'static str;
}

pub struct RedLight;
impl LightState for RedLight {
    fn change_light(&self) -> Box<dyn LightState> {
        Box::new(GreenLight)
    }
    fn color(&self) -> &'static str { "Red" }
}

pub struct GreenLight;
impl LightState for GreenLight {
    fn change_light(&self) -> Box<dyn LightState> {
        Box::new(YellowLight)
    }
    fn color(&self) -> &'static str { "Green" }
}
```

### TCP Connection State Machine
```rust
pub trait TcpState {
    fn open(&self) -> Box<dyn TcpState>;
    fn close(&self) -> Box<dyn TcpState>;
    fn send(&self) -> Box<dyn TcpState>;
}
```

### Document Workflow State
```rust
pub trait DocumentState {
    fn publish(&self) -> Box<dyn DocumentState>;
    fn archive(&self) -> Box<dyn DocumentState>;
}
```

## Anti-Patterns to Avoid

- **Too Many States** - State explosion
- **Complex State Transitions** - Unclear state diagram
- **Shared Mutable State** - States modifying shared context
- **Infinite State Loops** - States creating circular transitions

## Key Takeaways

- ✓ State pattern encapsulates state-specific behavior
- ✓ Eliminates large conditional statements
- ✓ Makes state transitions explicit and type-safe
- ✓ Each state object handles its own behavior
- ✓ Context delegates to current state
- ✓ Can combine with trait objects for flexibility
- ✓ Useful for state machines and workflows

