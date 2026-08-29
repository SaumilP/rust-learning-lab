# Strategy Pattern

> Design reference: [C4 component explanation and embedded PlantUML](DESIGN.md).

## Overview

The Strategy pattern defines a family of algorithms, encapsulates each one, and makes them interchangeable. It lets the algorithm vary independently from clients that use it. This is perfect for selecting different algorithms at runtime.

## Problem It Solves

- Eliminating conditional statements for algorithm selection
- Supporting multiple algorithms for a task
- Encapsulating algorithm details
- Allowing runtime switching between algorithms
- Avoiding code duplication across similar algorithms

## Implementation in Rust

### Basic Strategy Pattern

```rust
// Strategy trait
pub trait PaymentStrategy {
    fn pay(&self, amount: f64) -> bool;
}

// Concrete strategies
pub struct CreditCardPayment {
    card_number: String,
}

impl PaymentStrategy for CreditCardPayment {
    fn pay(&self, amount: f64) -> bool {
        println!("Paying ${} with credit card {}", amount, self.card_number);
        true
    }
}

pub struct PayPalPayment {
    email: String,
}

impl PaymentStrategy for PayPalPayment {
    fn pay(&self, amount: f64) -> bool {
        println!("Paying ${} with PayPal account {}", amount, self.email);
        true
    }
}

// Context that uses strategy
pub struct PaymentProcessor {
    strategy: Box<dyn PaymentStrategy>,
}

impl PaymentProcessor {
    pub fn new(strategy: Box<dyn PaymentStrategy>) -> Self {
        Self { strategy }
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn PaymentStrategy>) {
        self.strategy = strategy;
    }

    pub fn process_payment(&self, amount: f64) -> Result<(), String> {
        if self.strategy.pay(amount) {
            Ok(())
        } else {
            Err("Payment failed".to_string())
        }
    }
}
```

## Benefits

1. **Flexibility** - Switch algorithms at runtime
2. **Elimination of Conditionals** - No more long if/else chains
3. **Encapsulation** - Each algorithm in its own class
4. **Open/Closed Principle** - Easy to add new strategies
5. **Single Responsibility** - Each strategy handles one algorithm

## Common Patterns

### Pattern 1: Simple Strategy
```rust
pub trait Strategy {
    fn execute(&self) -> Result<String, Error>;
}

pub struct Context {
    strategy: Box<dyn Strategy>,
}
```

### Pattern 2: Generic Strategy
```rust
pub trait Strategy<T, U> {
    fn execute(&self, input: T) -> Result<U, Error>;
}
```

### Pattern 3: Strategy Factory
```rust
pub fn create_strategy(strategy_type: &str) -> Box<dyn Strategy> {
    match strategy_type {
        "a" => Box::new(StrategyA),
        "b" => Box::new(StrategyB),
        _ => Box::new(StrategyA),
    }
}
```

### Pattern 4: Chainable Strategies
```rust
pub struct StrategyChain {
    strategies: Vec<Box<dyn Strategy>>,
}

impl StrategyChain {
    pub fn execute(&self) -> Result<String, Error> {
        for strategy in &self.strategies {
            strategy.execute()?;
        }
        Ok("Done".to_string())
    }
}
```

### Pattern 5: Conditional Strategy
```rust
pub fn select_strategy(condition: bool) -> Box<dyn Strategy> {
    if condition {
        Box::new(StrategyA)
    } else {
        Box::new(StrategyB)
    }
}
```

## Real-World Examples

### Routing Strategy
```rust
pub trait RouteStrategy {
    fn calculate_route(&self, from: &str, to: &str) -> Vec<String>;
}

pub struct FastestRoute;
pub struct ShortestRoute;

impl RouteStrategy for FastestRoute {
    fn calculate_route(&self, from: &str, to: &str) -> Vec<String> {
        vec![from.to_string(), to.to_string()]
    }
}
```

### Search Strategy
```rust
pub trait SearchStrategy {
    fn search(&self, data: &[i32], target: i32) -> Option<usize>;
}

pub struct BinarySearch;
pub struct LinearSearch;
```

### Validation Strategy
```rust
pub trait ValidationStrategy {
    fn validate(&self, input: &str) -> bool;
}

pub struct EmailValidator;
pub struct PhoneValidator;

impl ValidationStrategy for EmailValidator {
    fn validate(&self, input: &str) -> bool {
        input.contains('@')
    }
}
```

## Anti-Patterns to Avoid

- **Over-Stratification** - Creating strategies for simple choices
- **Shared State** - Strategies modifying shared mutable state
- **Complex Strategy Selection** - Intricate logic for choosing strategy
- **Strategy-Per-Use** - Creating new strategy for each operation

## Key Takeaways

- ✓ Strategy pattern encapsulates interchangeable algorithms
- ✓ Eliminates conditional logic for algorithm selection
- ✓ Allows runtime switching between strategies
- ✓ Each strategy should be independent and testable
- ✓ Use trait objects for dynamic dispatch
- ✓ Consider factory functions for strategy creation
- ✓ Combine with dependency injection for flexibility
