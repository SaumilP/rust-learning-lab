# Random Numbers

## Overview

Random numbers are essential for games and simulations, enabling unpredictable events, NPC behavior, and procedural generation. Rust's standard library provides basic random functionality through `rand`, one of the most popular crates for random number generation. Understanding how to generate and use random numbers properly is critical for building engaging games.

## Theory

### Random vs Pseudo-Random

- **True Random** - Unpredictable, non-repeatable
- **Pseudo-Random** - Deterministic, repeatable with seed
- **Why Pseudo-Random** - Predictable for testing, reproducible for debugging

### Common Uses in Games

1. **Probability** - Dice rolls, hit chance, loot drops
2. **Positioning** - Random enemy placement
3. **Events** - Random encounters, loot tables
4. **Procedural Generation** - Random level generation

### Random Distributions

- **Uniform** - Equal probability for all values
- **Normal** - Bell curve distribution
- **Range** - Random within bounds
- **Weighted** - Custom probability distributions

## Syntax

### Basic Random Number Generation

```rust
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    // Random i32
    let num: i32 = rng.gen();

    // Random in range
    let dice = rng.gen_range(1..=6);

    // Random f64 (0.0 to 1.0)
    let float = rng.gen::<f64>();
}
```

### Random with Seeding

```rust
use rand::SeedableRng;
use rand::rngs::StdRng;

fn main() {
    // Reproducible randomness
    let seed = 42;
    let mut rng = StdRng::seed_from_u64(seed);

    // Same seed always produces same sequence
    let num1 = rng.gen_range(1..=100);
    let num2 = rng.gen_range(1..=100);
}
```

### Random Collection Selection

```rust
use rand::seq::SliceRandom;

fn main() {
    let items = vec!["sword", "shield", "potion", "gold"];
    let mut rng = rand::thread_rng();

    // Random element
    if let Some(&item) = items.choose(&mut rng) {
        println!("You got: {}", item);
    }

    // Shuffle
    let mut deck = vec![1, 2, 3, 4, 5];
    deck.shuffle(&mut rng);
}
```

## Common Patterns

### Pattern 1: Dice Roll Simulator

```rust
use rand::Rng;

fn roll_dice(sides: u32, count: u32) -> u32 {
    let mut rng = rand::thread_rng();
    (0..count)
        .map(|_| rng.gen_range(1..=sides))
        .sum()
}

fn main() {
    println!("2d6: {}", roll_dice(6, 2));
    println!("3d20: {}", roll_dice(20, 3));
}
```

### Pattern 2: Weighted Probability

```rust
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    // Weighted choice
    let roll = rng.gen_range(1..=100);
    let outcome = if roll <= 50 {
        "common"
    } else if roll <= 85 {
        "uncommon"
    } else if roll <= 99 {
        "rare"
    } else {
        "legendary"
    };

    println!("Loot rarity: {}", outcome);
}
```

### Pattern 3: Random Position Generation

```rust
use rand::Rng;

fn spawn_enemy(width: u32, height: u32) -> (u32, u32) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..width);
    let y = rng.gen_range(0..height);
    (x, y)
}

fn main() {
    let pos = spawn_enemy(100, 100);
    println!("Enemy spawned at: {:?}", pos);
}
```

### Pattern 4: Random Selection from Vec

```rust
use rand::seq::SliceRandom;

fn random_encounter(possible_enemies: &[&str]) -> &str {
    let mut rng = rand::thread_rng();
    possible_enemies.choose(&mut rng).copied().unwrap_or("goblin")
}

fn main() {
    let enemies = vec!["goblin", "orc", "dragon"];
    for _ in 0..5 {
        println!("Encountered: {}", random_encounter(&enemies));
    }
}
```

### Pattern 5: Seeded RNG for Testing

```rust
use rand::SeedableRng;
use rand::rngs::StdRng;

fn generate_level(seed: u64) -> Vec<i32> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..10).map(|_| rng.gen_range(1..=100)).collect()
}

fn main() {
    // Same seed always produces same level
    let level1 = generate_level(12345);
    let level2 = generate_level(12345);
    assert_eq!(level1, level2);

    println!("Reproducible procedural generation!");
}
```

## Common Mistakes

### Mistake 1: Creating RNG Every Time

```rust
// ❌ WRONG - Inefficient, poor randomness quality
for _ in 0..100 {
    let rng = rand::thread_rng();
    let num = rng.gen_range(1..=6);
}

// ✅ CORRECT - Reuse RNG
let mut rng = rand::thread_rng();
for _ in 0..100 {
    let num = rng.gen_range(1..=6);
}
```

### Mistake 2: Forgetting Range Bounds

```rust
// ❌ WRONG - May panic or give unexpected results
let num = rng.gen_range(1..10);  // 1-9, not 1-10

// ✅ CORRECT - Use inclusive range for dice/indices
let num = rng.gen_range(1..=10);  // 1-10 inclusive
```

### Mistake 3: Not Seeding for Testing

```rust
// ❌ WRONG - Tests are non-deterministic
#[test]
fn test_loot_generation() {
    let loot = generate_loot();
    assert!(!loot.is_empty());
}

// ✅ CORRECT - Use seed for reproducible tests
#[test]
fn test_loot_generation() {
    let seed = 12345;
    let loot = generate_loot_seeded(seed);
    assert_eq!(loot.len(), EXPECTED_COUNT);
}
```

### Mistake 4: Non-Uniform Modulo Bias

```rust
// ❌ WRONG - Biased if range doesn't divide evenly
let biased = rng.gen::<u32>() % 20;

// ✅ CORRECT - Use gen_range for uniform distribution
let uniform = rng.gen_range(0..20);
```

### Mistake 5: Predictable "Random" Sequences

```rust
// ❌ WRONG - Not actually random
let mut seed = 1;
fn next_random() -> u32 {
    seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
    seed / 65536 % 32768
}

// ✅ CORRECT - Use established library
use rand::Rng;
let mut rng = rand::thread_rng();
let num = rng.gen_range(1..=100);
```

## Real-World Examples

### Example 1: Simple Guessing Game

```rust
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let secret = rng.gen_range(1..=100);
    let mut guesses = 0;

    loop {
        println!("Guess the number (1-100):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let guess: u32 = input.trim().parse().unwrap_or(0);

        guesses += 1;

        if guess < secret {
            println!("Too low!");
        } else if guess > secret {
            println!("Too high!");
        } else {
            println!("Correct! Took {} guesses", guesses);
            break;
        }
    }
}
```

### Example 2: Loot Drop System

```rust
use rand::Rng;
use std::collections::HashMap;

fn get_loot_rarity() -> &'static str {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1..=100);

    match roll {
        1..=60 => "common",
        61..=85 => "uncommon",
        86..=99 => "rare",
        _ => "legendary",
    }
}

fn main() {
    let mut loot_counts = HashMap::new();

    for _ in 0..1000 {
        let rarity = get_loot_rarity();
        *loot_counts.entry(rarity).or_insert(0) += 1;
    }

    for (rarity, count) in loot_counts {
        println!("{}: {} ({}%)", rarity, count, count / 10);
    }
}
```

### Example 3: Procedural Level Layout

```rust
use rand::Rng;

fn generate_room_layout(width: usize, height: usize) -> Vec<Vec<char>> {
    let mut rng = rand::thread_rng();
    let mut layout = vec![vec!['.'; width]; height];

    // Place random walls
    for _ in 0..20 {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        layout[y][x] = '#';
    }

    layout
}

fn main() {
    let layout = generate_room_layout(20, 10);
    for row in layout {
        println!("{}", row.iter().collect::<String>());
    }
}
```

## Related Concepts

### Prerequisites
- Module 04: Basic Rust (variables, loops)
- Module 05: Game Loop Basics
- Understanding of probability basics

### Follow-ups
- Console I/O (displaying random results)
- Simple Game Logic (probability-based decisions)
- Advanced probabilistic systems
- External crate: `rand` library

## Best Practices

1. **Reuse RNG** - Create once, use many times
2. **Use Correct Range** - `gen_range(1..=6)` not `1..6`
3. **Seed for Testing** - Reproducible pseudo-random
4. **Understand Distribution** - Uniform vs weighted
5. **Performance** - Seeded RNG faster for large batches
6. **Quality** - Use established crates, not custom implementations
7. **Document Probability** - Make drop rates clear in comments

## Summary

Random numbers are essential for game development, enabling unpredictable gameplay and procedural generation. The `rand` crate provides reliable, efficient random number generation. Understanding seeding enables reproducible testing while maintaining fun, unpredictable gameplay in production.

## Practice Exercise Ideas

1. Create a dice roller simulator with configurable sides
2. Build a simple slot machine with weighted probabilities
3. Implement treasure chest with random loot drops
4. Create procedural dungeon generator
5. Build a card shuffler and dealer for card games

