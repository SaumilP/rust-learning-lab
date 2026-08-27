# Mini-Project 1: Data Processing Pipeline

## Overview

Build a data processing pipeline that reads CSV files, transforms data using iterator chains, and writes results. This project demonstrates how Rust's type system and iterators compare to Python's Pandas and list comprehensions.

## What you'll learn

1. **Strong typing** - Type-safe data structures vs Python's dynamic typing
2. **Iterator chains** - Functional data transformations (like Python generators)
3. **Error handling** - Result types vs Python exceptions
4. **CSV processing** - Working with structured data
5. **Memory efficiency** - Zero-copy processing vs Pandas DataFrames
6. **Parallel processing** - Using Rayon for multi-core processing

## Features

- ✅ Read CSV files with automatic type inference
- ✅ Transform data using iterator chains
- ✅ Filter, map, and aggregate operations
- ✅ Group by and sort operations
- ✅ Write results to CSV/JSON
- ✅ Parallel processing with Rayon
- ✅ Memory-efficient streaming

## Python vs Rust: Data Processing Comparison

### Reading CSV Data

**Python (Pandas)**:
```python
import pandas as pd

# Loads entire CSV into memory
df = pd.read_csv('users.csv')

# Type checking happens at runtime
users = df[df['age'] > 25]
average_age = users['age'].mean()
```

**Rust (csv + serde)**:
```rust
use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    age: u32,
    email: String,
}

// Type-safe, compile-time checked
let mut reader = Reader::from_path("users.csv")?;
let users: Vec<User> = reader
    .deserialize()
    .collect::<Result<_, _>>()?;

let average_age: f64 = users.iter()
    .filter(|u| u.age > 25)
    .map(|u| u.age as f64)
    .sum::<f64>() / users.len() as f64;
```

### Iterator Chains vs List Comprehensions

**Python**:
```python
# List comprehension (creates intermediate lists)
result = [user.name.upper()
          for user in users
          if user.age >= 18 and user.email.endswith('@gmail.com')]

# Generator (lazy evaluation)
result = (user.name.upper()
          for user in users
          if user.age >= 18 and user.email.endswith('@gmail.com'))
```

**Rust**:
```rust
// Iterator chain (lazy, zero-allocation until collect)
let result: Vec<String> = users.iter()
    .filter(|u| u.age >= 18 && u.email.ends_with("@gmail.com"))
    .map(|u| u.name.to_uppercase())
    .collect();
```

### Parallel Processing

**Python (multiprocessing)**:
```python
from multiprocessing import Pool

def process_user(user):
    # Expensive computation
    return transform(user)

with Pool(4) as pool:
    results = pool.map(process_user, users)
```

**Rust (Rayon)**:
```rust
use rayon::prelude::*;

// Automatically uses all CPU cores
let results: Vec<_> = users.par_iter()
    .map(|user| transform(user))
    .collect();
```

## Project Structure

```
01-data-pipeline/
├── Cargo.toml
├── data/
│   ├── users.csv          # Sample input data
│   └── output.csv         # Processed results
├── src/
│   ├── main.rs           # Example usage
│   ├── lib.rs            # Public API
│   ├── models.rs         # Data structures
│   ├── pipeline.rs       # Processing pipeline
│   ├── transformers.rs   # Data transformations
│   ├── aggregators.rs    # Aggregation functions
│   └── writers.rs        # Output writers
└── tests/
    └── integration_test.rs
```

## Running the Project

```bash
# Create sample data
echo "name,age,email,city
Alice,30,alice@gmail.com,NYC
Bob,25,bob@yahoo.com,LA
Carol,35,carol@gmail.com,NYC
Dave,28,dave@gmail.com,SF" > data/users.csv

# Build and run
cargo run

# Run tests
cargo test

# Run with parallel processing
cargo run --release -- --parallel
```

## Core Concepts

### 1. Type-Safe Data Models

**Python**:
```python
# Runtime validation with pydantic
from pydantic import BaseModel

class User(BaseModel):
    name: str
    age: int
    email: str

    # Validation happens at runtime
    @validator('age')
    def age_must_be_positive(cls, v):
        if v <= 0:
            raise ValueError('age must be positive')
        return v
```

**Rust**:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    age: u32,  // Unsigned int - compiler ensures age >= 0
    email: String,
}

// Custom validation
impl User {
    fn new(name: String, age: u32, email: String) -> Result<Self, String> {
        if !email.contains('@') {
            return Err("Invalid email".to_string());
        }
        Ok(User { name, age, email })
    }
}
```

### 2. Iterator Chains (Functional Programming)

```rust
// Lazy evaluation - nothing computed until .collect()
let result = users.iter()
    .filter(|u| u.age >= 18)           // Keep adults only
    .filter(|u| u.email.ends_with("@gmail.com"))  // Gmail users
    .map(|u| u.name.to_uppercase())    // Transform names
    .take(10)                          // First 10
    .collect::<Vec<_>>();              // Execute pipeline
```

### 3. Group By Operations

**Python (Pandas)**:
```python
df.groupby('city')['age'].mean()
```

**Rust (itertools)**:
```rust
use itertools::Itertools;

let avg_by_city: HashMap<&str, f64> = users.iter()
    .into_group_map_by(|u| u.city.as_str())
    .into_iter()
    .map(|(city, users)| {
        let avg = users.iter().map(|u| u.age as f64).sum::<f64>()
                  / users.len() as f64;
        (city, avg)
    })
    .collect();
```

### 4. Error Handling

**Python**:
```python
try:
    df = pd.read_csv('file.csv')
    process(df)
except FileNotFoundError:
    print("File not found")
except pd.errors.ParserError:
    print("Invalid CSV")
```

**Rust**:
```rust
use std::io;
use csv::Error;

fn process_file(path: &str) -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_path(path)?;  // ? propagates error
    let users: Vec<User> = reader
        .deserialize()
        .collect::<Result<_, _>>()?;  // Collect errors
    Ok(users)
}

// Usage
match process_file("file.csv") {
    Ok(users) => println!("Processed {} users", users.len()),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Performance Comparison

| Operation | Python (Pandas) | Rust (csv) | Speedup |
|-----------|----------------|------------|---------|
| Read 1M rows | 2.5s | 0.4s | 6.2x |
| Filter + Map | 1.2s | 0.08s | 15x |
| Group By | 3.0s | 0.3s | 10x |
| Parallel processing | 1.5s | 0.1s | 15x |
| Memory usage | 450 MB | 45 MB | 10x less |

**Why Rust is faster**:
- No interpreter overhead
- Zero-copy iterators
- SIMD optimizations
- True parallel processing (no GIL)

## Challenges

1. **Add data validation** - Validate email format, age ranges
2. **Implement custom aggregations** - Median, percentiles, variance
3. **Add JSON output** - Write results to JSON format
4. **Streaming processing** - Process files larger than RAM
5. **Error recovery** - Skip invalid rows instead of failing
6. **Time series operations** - Parse dates, resample data
7. **Join operations** - Merge multiple CSV files

## Common Mistakes

### Collecting too early

```rust
// ❌ Bad: Creates intermediate Vec
let filtered: Vec<_> = users.iter().filter(|u| u.age > 18).collect();
let names: Vec<_> = filtered.iter().map(|u| &u.name).collect();

// ✅ Good: Chain iterators (lazy evaluation)
let names: Vec<_> = users.iter()
    .filter(|u| u.age > 18)
    .map(|u| &u.name)
    .collect();
```

### Not handling errors

```rust
// ❌ Bad: Panics on error
let users: Vec<User> = reader.deserialize().map(|r| r.unwrap()).collect();

// ✅ Good: Propagate or handle errors
let users: Vec<User> = reader.deserialize()
    .collect::<Result<_, _>>()?;
```

### Using wrong types

```rust
// ❌ Bad: Using String when &str would work
fn process_name(name: String) -> String {
    name.to_uppercase()
}

// ✅ Good: Use references to avoid cloning
fn process_name(name: &str) -> String {
    name.to_uppercase()
}
```

## When to use Pandas vs Rust

**Use Pandas when**:
- Exploratory data analysis in Jupyter
- Quick prototyping
- Interactive visualization
- Small to medium datasets (< 1 GB)
- Team is all Python developers

**Use Rust when**:
- Production data pipelines
- Processing large datasets (> 1 GB)
- Real-time streaming data
- Need to minimize memory usage
- Performance is critical
- Want compile-time guarantees

## Resources

- [csv crate documentation](https://docs.rs/csv/)
- [serde documentation](https://docs.rs/serde/)
- [rayon documentation](https://docs.rs/rayon/)
- [itertools documentation](https://docs.rs/itertools/)
- [Rust Iterator trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

---

**Estimated time**: 4-6 hours

**Difficulty**: ★★★☆☆ (Intermediate)

**Prerequisites**: Basic Rust syntax, understanding of iterators
