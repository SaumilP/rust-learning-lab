# Interview Questions: Algorithm & Code Optimization

## Question 1: Optimize a Slow Query

**Difficulty**: Medium
**Concepts**: Algorithm analysis, optimization techniques
**Time**: 25-35 minutes

### Problem Statement

Given a slow implementation of a common problem, analyze it and provide optimized solutions.

### Scenario: Finding Duplicates in Large Array

**Slow Implementation (O(n²)):**
```rust
pub fn find_duplicates_slow(arr: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..arr.len() {
        for j in (i+1)..arr.len() {
            if arr[i] == arr[j] && !result.contains(&arr[i]) {
                result.push(arr[i]);
            }
        }
    }
    result
}
```

### Key Questions

1. **Bottleneck Analysis**
   - What's the time complexity?
   - Why is it slow for large arrays?
   - Where is time spent?

2. **Optimization Strategies**
   - HashSet approach (O(n) space, O(n) time)
   - Sorting approach (O(n log n) time, O(1) extra space)
   - Bitmap approach (O(n) time, O(1) space if small range)

3. **Implementation Trade-offs**
   - Memory vs time trade-offs?
   - Maintaining order of results?
   - Handling negative numbers?

4. **Code Quality**
   - Readability of optimized version?
   - Testing strategy?
   - Generalization to other problems?

5. **Benchmarking**
   - How to measure improvement?
   - Different input sizes?
   - Profile the code?

### Solutions to Discuss

**HashSet Optimization (O(n) time):**
```rust
pub fn find_duplicates_hashset(arr: &[i32]) -> Vec<i32> {
    let mut seen = std::collections::HashSet::new();
    let mut duplicates = std::collections::HashSet::new();
    for &num in arr {
        if !seen.insert(num) {
            duplicates.insert(num);
        }
    }
    duplicates.into_iter().collect()
}
```

**Sort Optimization (O(n log n) time):**
```rust
pub fn find_duplicates_sort(arr: &[i32]) -> Vec<i32> {
    let mut sorted = arr.to_vec();
    sorted.sort_unstable();
    let mut result = Vec::new();
    for i in 0..(sorted.len()-1) {
        if sorted[i] == sorted[i+1] && (result.is_empty() || result.last() != Some(&sorted[i])) {
            result.push(sorted[i]);
        }
    }
    result
}
```

### Discussion Points

- When to use each approach
- Profiling tools available
- Real-world scenarios
- Memory constraints

---

## Question 2: Fix Memory Leak

**Difficulty**: Medium
**Concepts**: Memory management, lifetimes, ownership
**Time**: 25-35 minutes

### Problem Statement

Analyze code that might have memory issues and explain Rust's solutions.

### Scenario: Circular References in Collections

**Problematic Pattern** (in other languages):
```rust
// This is prevented by Rust's type system, but worth understanding
// Hypothetical in C++: circular references causing memory leak
// In Rust: won't compile without Rc/RefCell
```

### Rust-Specific Memory Safety

Since Rust prevents traditional memory leaks at compile time, discuss:

1. **Ownership Model**
   - Move semantics
   - Borrowing rules
   - Why memory leaks are prevented

2. **Lifetimes**
   - Lifetime annotations
   - Dangling references prevention
   - Lifetime elision rules

3. **Reference Counting**
   - Rc<T> for shared ownership
   - Arc<T> for thread-safe sharing
   - RefCell<T> for interior mutability

4. **Common Patterns**
   - Tree structures with parent pointers (use Rc + RefCell)
   - Circular data structures (WeakRef to break cycles)
   - Global state management

### Example: Tree with Parent Pointers

**Problem**: How to store parent reference in tree node?

**Wrong Approach**:
```rust
// Won't compile: circular references
pub struct Node {
    value: i32,
    parent: Option<&'a Node>,  // Dangling reference
    children: Vec<Node>,
}
```

**Correct Approach**:
```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Node {
    value: i32,
    parent: RefCell<Option<Weak<RefCell<Node>>>>,
    children: RefCell<Vec<Rc<RefCell<Node>>>>,
}
```

### Questions to Answer

1. **Design Patterns**
   - Parent-child relationships?
   - Graph structures?
   - Callback/observer patterns?

2. **Trade-offs**
   - Reference counting overhead?
   - Runtime borrow checking with RefCell?
   - Compilation vs runtime safety?

3. **Testing**
   - Verify no actual leaks
   - Performance testing
   - Drop semantics

### Discussion Points

- When to use Rc vs Arc
- WeakRef breaking cycles
- Runtime borrow panic handling
- Alternative designs

---

## Question 3: Handle Concurrency Bug

**Difficulty**: Medium-Hard
**Concepts**: Concurrency, synchronization, data races
**Time**: 30-40 minutes

### Problem Statement

Identify and fix concurrency issues in multi-threaded code.

### Scenario: Race Condition in Counter

**Problem Code**:
```rust
// Simplified: Rust prevents this, but understanding is important
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

let counter = Arc::new(Mutex::new(0));

// If threads try to increment without proper locking
// Or if synchronization is wrong
```

### Common Concurrency Issues

1. **Data Races**
   - Multiple threads accessing shared data
   - Rust type system prevents at compile time
   - Understanding: mutable shared state needs Mutex

2. **Deadlocks**
   - Threads waiting for locks in wrong order
   - Circular wait conditions
   - Rust doesn't prevent, but easier to analyze

3. **Lock Contention**
   - Too many locks
   - Large critical sections
   - Performance degradation

4. **Memory Ordering**
   - AtomicTypes and memory ordering
   - When sequentially consistent isn't needed
   - Performance optimization with Relaxed

### Design Questions

1. **Synchronization Primitive**
   - Mutex for complex data?
   - RwLock for read-heavy workloads?
   - Atomic operations for simple counters?
   - Channels for message passing?

2. **Lock Scope**
   - Minimize time holding locks
   - What to protect together?
   - Granularity of locking?

3. **Thread Coordination**
   - Barrier for synchronization points?
   - Condition variables?
   - Channels for signaling?

4. **Performance**
   - Lock-free alternatives?
   - Concurrent data structures?
   - Batch processing?

### Example: Counter Implementation

**Naive** (Mutex for everything):
```rust
let counter = Arc::new(Mutex::new(0));
// High contention
```

**Better** (Atomic for counter):
```rust
let counter = Arc::new(AtomicUsize::new(0));
// Lock-free operations
```

**Thread Pool** (Reduce contention):
```rust
// Each thread maintains local count
// Merge at end
```

### Discussion Points

- Rust type system guarantees
- When compile-time safety isn't enough
- Deadlock detection strategies
- Profiling concurrent code

---

## Question 4: Design Error Handling

**Difficulty**: Medium
**Concepts**: Error types, recovery, user experience
**Time**: 25-35 minutes

### Problem Statement

Design comprehensive error handling for an application with multiple error sources.

### Scenario: Web Server Error Handling

**Sources of Errors**:
- Network errors (connection failures)
- File I/O errors (file not found)
- Parsing errors (invalid data)
- Authentication errors (permission denied)
- Database errors (transaction failed)

### Design Questions

1. **Error Types**
   - Single error type vs multiple?
   - Enum vs trait objects?
   - Error codes vs messages?

2. **Error Propagation**
   - Use `?` operator
   - Custom conversion From impl
   - Error context and cause chains

3. **User Feedback**
   - Which errors to show to users?
   - How much detail?
   - Localization needs?

4. **Recovery**
   - Recoverable vs fatal errors?
   - Retry logic?
   - Fallback strategies?

5. **Logging**
   - What to log?
   - Log levels for different errors?
   - Sensitive data handling?

### Implementation Approach

**Custom Error Type**:
```rust
use std::fmt;

#[derive(Debug)]
pub enum ServerError {
    Io(std::io::Error),
    Parse(String),
    Database(String),
    NotFound,
    Unauthorized,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerError::Io(e) => write!(f, "IO error: {}", e),
            ServerError::Parse(e) => write!(f, "Parse error: {}", e),
            ServerError::Database(e) => write!(f, "Database error: {}", e),
            ServerError::NotFound => write!(f, "Resource not found"),
            ServerError::Unauthorized => write!(f, "Unauthorized access"),
        }
    }
}

impl std::error::Error for ServerError {}
```

### Discussion Points

- Using error-handling crates (anyhow, thiserror)
- HTTP status codes mapping
- Structured error information
- Context preservation

---

## Question 5: Implement Proper Testing

**Difficulty**: Medium
**Concepts**: Test design, coverage, mocking
**Time**: 25-35 minutes

### Problem Statement

Design a comprehensive testing strategy for a non-trivial module.

### Scenario: Testing a File Processing Module

**Module Responsibilities**:
- Read file from disk
- Parse CSV format
- Validate data
- Transform data
- Write results

### Testing Strategy

1. **Unit Tests**
   - Test each function independently
   - Happy path and error cases
   - Edge cases (empty files, single line, etc.)

2. **Integration Tests**
   - Test multiple functions together
   - File I/O with actual files
   - End-to-end workflows

3. **Test Data**
   - Minimal valid data
   - Invalid data variants
   - Large datasets for performance
   - Edge cases

4. **Mocking**
   - Mock file system for unit tests
   - Dependency injection for testability
   - Trait-based design for swappable components

5. **Coverage**
   - Line coverage targets
   - Branch coverage for conditionals
   - Error path testing

### Testing Patterns

**Property-Based Testing**:
```rust
#[cfg(test)]
mod tests {
    use quickcheck::{quickcheck, TestResult};

    fn parse_valid_line(line: String) -> TestResult {
        // Only test valid CSV lines
        if line.contains("\"") && !line.contains(",") {
            return TestResult::discard();
        }
        // Property: reparsing should be idempotent
        TestResult::from_bool(true)
    }
}
```

**Parameterized Tests**:
```rust
#[test]
fn test_parsing_variants() {
    let test_cases = vec![
        ("1,2,3", vec!["1", "2", "3"]),
        ("a,b,c", vec!["a", "b", "c"]),
        ("", vec![]),
    ];

    for (input, expected) in test_cases {
        assert_eq!(parse_line(input), expected);
    }
}
```

### Discussion Points

- Test coverage vs implementation cost
- Fast vs slow tests
- Flaky test handling
- Continuous integration strategies

---

## Summary of Algorithm & Optimization Questions

These questions practice:
- Performance analysis and optimization
- Understanding Rust's memory safety
- Concurrency and synchronization
- Error handling design
- Testing strategies

**Preparation**:
- Understand complexity analysis
- Know common optimization techniques
- Practice identifying bottlenecks
- Design error hierarchies
