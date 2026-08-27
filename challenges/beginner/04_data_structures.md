# Beginner Level: Data Structure Implementation Problems

## Problem 1: Simple Stack

**Difficulty**: Beginner-Intermediate
**Concepts**: Data structure design, Vec operations
**Prerequisites**: 02-standard-library (Collections, Vec<T>)

### Problem Statement

Implement a simple stack data structure with push, pop, and peek operations.

### Requirements
- Create a Stack struct
- Implement push() - add element to top
- Implement pop() - remove and return top element
- Implement peek() - view top without removing
- Implement is_empty() - check if stack is empty
- Return Option types for pop/peek

### Function Signatures
```rust
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self { /* ... */ }
    pub fn push(&mut self, item: T) { /* ... */ }
    pub fn pop(&mut self) -> Option<T> { /* ... */ }
    pub fn peek(&self) -> Option<&T> { /* ... */ }
    pub fn is_empty(&self) -> bool { /* ... */ }
    pub fn len(&self) -> usize { /* ... */ }
}
```

### Test Cases
```
Creating stack, pushing 1, 2, 3:
- len() == 3
- peek() == Some(&3)
- pop() == Some(3)
- pop() == Some(2)
- len() == 1
- pop() == Some(1)
- pop() == None
- is_empty() == true
```

### Hints
- Use Vec<T> internally
- push(): use .push()
- pop(): use .pop()
- peek(): return reference to last element
- LIFO: Last In, First Out

### Algorithm Approach
1. Store items in Vec
2. Push: add to end
3. Pop: remove from end
4. Peek: return reference to last
5. All operations O(1)

### Related Concepts
- Generic structs
- Ownership and references
- Option types
- Data structure design

---

## Problem 2: Simple Queue

**Difficulty**: Beginner-Intermediate
**Concepts**: Data structure design, VecDeque
**Prerequisites**: 02-standard-library (Collections, VecDeque)

### Problem Statement

Implement a simple queue data structure with enqueue, dequeue, and peek operations.

### Requirements
- Create a Queue struct
- Implement enqueue() - add to back
- Implement dequeue() - remove from front
- Implement peek() - view front without removing
- Implement is_empty() - check if queue is empty
- Return Option types for dequeue/peek

### Function Signatures
```rust
use std::collections::VecDeque;

pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self { /* ... */ }
    pub fn enqueue(&mut self, item: T) { /* ... */ }
    pub fn dequeue(&mut self) -> Option<T> { /* ... */ }
    pub fn peek(&self) -> Option<&T> { /* ... */ }
    pub fn is_empty(&self) -> bool { /* ... */ }
    pub fn len(&self) -> usize { /* ... */ }
}
```

### Test Cases
```
Creating queue, enqueueing 1, 2, 3:
- len() == 3
- peek() == Some(&1)
- dequeue() == Some(1)
- dequeue() == Some(2)
- len() == 1
- dequeue() == Some(3)
- dequeue() == None
- is_empty() == true
```

### Hints
- Use VecDeque<T> for efficient front removal
- enqueue(): use .push_back()
- dequeue(): use .pop_front()
- peek(): return reference to first element
- FIFO: First In, First Out

### Algorithm Approach
1. Store items in VecDeque
2. Enqueue: add to back
3. Dequeue: remove from front
4. Peek: return reference to front
5. All operations O(1)

### Related Concepts
- VecDeque vs Vec
- Generic structs
- FIFO vs LIFO
- Efficiency considerations

---

## Problem 3: HashMap Usage

**Difficulty**: Beginner-Intermediate
**Concepts**: HashMap operations, word frequency
**Prerequisites**: 02-standard-library (Collections, HashMap)

### Problem Statement

Use HashMap to count word frequencies in text (similar to earlier problem but as data structure practice).

### Requirements
- Create a frequency counter using HashMap
- Count occurrences of each word
- Case-insensitive
- Return HashMap with results
- Handle empty input

### Function Signature
```rust
use std::collections::HashMap;

pub fn word_frequency(text: &str) -> HashMap<String, usize> {
    // Your implementation here
}
```

### Test Cases
```
Input: "hello world hello"
Output: {"hello": 2, "world": 1}

Input: "rust rust RUST"
Output: {"rust": 3}

Input: "The quick brown fox"
Output: {"the": 1, "quick": 1, "brown": 1, "fox": 1}

Input: ""
Output: {} (empty)
```

### Hints
- Use .split_whitespace() to get words
- Convert to lowercase for case-insensitive counting
- Use .entry(key).or_insert(0) to handle new/existing keys
- Increment count for each occurrence

### Algorithm Approach
1. Split text into words
2. For each word:
   - Convert to lowercase
   - Get or create entry in HashMap
   - Increment count
3. Return HashMap

### Related Concepts
- HashMap entry API
- Mutable references
- String operations
- Counting/frequency patterns

---

## Summary of Beginner Level: Data Structures

These problems practice:
- Implementing abstract data types
- Generic programming
- Choosing appropriate containers
- Data structure operations
- Ownership and references in structs

**Recommended Order**: Complete problems 1-3 sequentially
**Time Estimate**: 60-90 minutes total
**Difficulty Progression**: 1→2→3 (gradual increase)

---

## Prerequisites Before Starting

Before attempting these problems, ensure you understand:
- Struct definition and methods
- Generics basics
- Vec, VecDeque, HashMap collections
- Option<T> type
- Mutable references (&mut)
- impl blocks
