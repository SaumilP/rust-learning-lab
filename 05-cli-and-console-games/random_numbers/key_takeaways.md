# Random Numbers - Key Takeaways

## Basic Usage

```rust
use rand::Rng;

let mut rng = rand::thread_rng();

// Random i32
let num: i32 = rng.gen();

// Random in range (1-6 for dice)
let dice = rng.gen_range(1..=6);

// Random float (0.0-1.0)
let float: f64 = rng.gen();
```

## Range Syntax

```rust
// Exclusive end (1-9)
rng.gen_range(1..10)

// Inclusive end (1-10) - PREFERRED for dice/counts
rng.gen_range(1..=10)

// Both work, but inclusive is clearer for bounds
```

## Common Operations

| Operation | Code |
|-----------|------|
| Dice roll (1-6) | `rng.gen_range(1..=6)` |
| Coin flip | `rng.gen_bool(0.5)` |
| Percentage roll | `rng.gen_range(1..=100)` |
| Random element | `vec.choose(&mut rng)` |
| Shuffle | `vec.shuffle(&mut rng)` |
| Seeded RNG | `StdRng::seed_from_u64(42)` |

## Weighted Probabilities

```rust
let roll = rng.gen_range(1..=100);

let outcome = match roll {
    1..=50 => "common",     // 50%
    51..=85 => "uncommon",  // 35%
    86..=99 => "rare",      // 14%
    _ => "legendary",       // 1%
};
```

## Seeding for Reproducibility

```rust
use rand::SeedableRng;
use rand::rngs::StdRng;

// Same seed = same sequence
let mut rng = StdRng::seed_from_u64(12345);

// Used for testing and procedural generation
```

## Important Patterns

| Pattern | Use |
|---------|-----|
| `gen_range()` | Most common - bounded random |
| `choose()` | Random element from collection |
| `shuffle()` | Randomize order of items |
| `gen::<T>()` | Random value of type T |
| Seeded RNG | Testing, reproducible generation |

## Performance Tips

✓ Create RNG once, reuse it
✓ Seeded RNG faster for large batches
✓ `thread_rng()` is standard choice
✓ `gen_range()` is generally faster than modulo
✓ Cache choice lists if used frequently

## Common Uses in Games

```rust
// Dice roll
let result = rng.gen_range(1..=20);

// Combat hit chance
if rng.gen_bool(0.75) { /* hit */ }

// Loot drops
let rarity = match rng.gen_range(1..=100) {
    1..=60 => "common",
    61..=85 => "uncommon",
    _ => "rare",
};

// Random enemy
let enemy = enemies.choose(&mut rng);

// Level generation
let wall_x = rng.gen_range(0..map_width);
```

## Important Notes

✓ Always use `1..=n` for natural bounds
✓ `thread_rng()` is thread-safe and efficient
✓ Seed with `u64` for reproducible sequences
✓ `choose()` returns Option (can be None)
✓ `shuffle()` modifies in place

## Quick Reference

```rust
use rand::Rng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;

// Thread-local RNG
let mut rng = rand::thread_rng();

// Range
let n = rng.gen_range(1..=100);

// Boolean
let coin = rng.gen_bool(0.5);

// Collection choice
let item = vec.choose(&mut rng);

// Shuffle
vec.shuffle(&mut rng);

// Seeded (reproducible)
let mut rng = StdRng::seed_from_u64(42);
```

## Common Mistakes to Avoid

1. ❌ Creating RNG inside loop
2. ❌ Using 1..10 instead of 1..=10
3. ❌ Not seeding tests
4. ❌ Modulo bias
5. ❌ Assuming non-uniform distribution

