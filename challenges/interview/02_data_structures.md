# Interview Questions: Data Structures & Algorithms

## Question 1: Implement Custom HashMap

**Difficulty**: Hard
**Concepts**: Hash functions, collision handling, dynamic sizing
**Time**: 35-45 minutes

### Problem Statement

Implement a HashMap from scratch with hash function, collision resolution, and dynamic resizing.

### Requirements

**Core Features**:
- Insert key-value pairs
- Retrieve values by key
- Delete entries
- Handle hash collisions
- Dynamic resizing when load factor exceeded

**Design Decisions**:
- Hash function choice
- Collision resolution (chaining, linear probing, quadratic probing?)
- Load factor threshold
- Initial capacity and resize strategy

### Key Questions

1. **Hash Function**
   - What makes a good hash function?
   - How to hash different key types?
   - Distribution properties needed?

2. **Collision Handling**
   - Chaining vs open addressing trade-offs?
   - Memory overhead of each approach?
   - Performance characteristics?

3. **Resizing**
   - When to resize (load factor)?
   - New capacity strategy (2x, golden ratio)?
   - Rehashing all entries cost?

4. **API Design**
   - `insert(K, V) -> Option<V>` (return old value)?
   - `get(K) -> Option<&V>` or `Option<V>`?
   - Iterator support?

5. **Performance**
   - Average and worst-case complexity?
   - Memory usage?
   - Cache locality?

### Rust-Specific Challenges

- Trait bounds for hashable keys
- Generic types for K and V
- Handling drop of stored values
- Thread safety considerations

### Discussion Points

- Linear probing vs separate chaining
- Prime vs power-of-2 capacity
- String vs custom key types
- Memory layout optimization

---

## Question 2: Implement Custom Vec

**Difficulty**: Hard
**Concepts**: Memory management, allocations, capacity
**Time**: 35-45 minutes

### Problem Statement

Implement a dynamic vector (array) from scratch with automatic growing and memory management.

### Requirements

**Core Features**:
- Store elements of generic type
- Dynamic resizing when full
- Random access by index
- Push and pop operations
- Memory efficiency

**Design Decisions**:
- Initial capacity
- Growth strategy (2x, 1.5x, golden ratio?)
- When to shrink?
- Memory layout

### Key Questions

1. **Memory Management**
   - Allocate from heap
   - Track capacity vs length
   - Deallocation strategy
   - Alignment considerations?

2. **Growth Strategy**
   - Amortized complexity analysis
   - Different growth factors
   - Shrinking (when and how)?

3. **Index Bounds**
   - Panic vs Result for out-of-bounds?
   - Performance of bounds checking?

4. **Iterator Implementation**
   - Ownership and borrowing?
   - Mutable iteration?
   - Double-ended iteration?

5. **Edge Cases**
   - Zero-sized types?
   - Drop semantics?
   - Panic during reallocation?

### Rust-Specific Challenges

- Use unsafe for raw allocations (or std::alloc)
- Drop trait for cleanup
- Correct use of std::ptr operations
- Lifetime and borrowing rules

### Discussion Points

- Growth factor trade-offs
- Drop implementations for stored types
- Optimization for zero-sized types
- Memory fragmentation

---

## Question 3: Implement LRU Cache

**Difficulty**: Medium-Hard
**Concepts**: Data structure combination, eviction policy
**Time**: 30-40 minutes

### Problem Statement

Implement an LRU (Least Recently Used) cache with fixed capacity and O(1) get/set operations.

### Requirements

**Core Features**:
- Fixed capacity (configurable)
- Get/Put operations
- LRU eviction when full
- Track access order
- Return evicted values

**Design Challenges**:
- Achieve O(1) time for all operations
- Combine HashMap with doubly-linked list

### Key Questions

1. **Data Structure**
   - HashMap for fast lookups
   - Doubly-linked list for order tracking
   - Why both needed?
   - Alternative designs?

2. **Implementation Details**
   - Store key-value in linked list
   - Store key -> node pointer in map
   - Handle moves within list
   - Deletion complexity

3. **API Design**
   - `get(key) -> Option<V>`
   - `put(key, value) -> Option<V>` (return evicted?)
   - Capacity query?

4. **Edge Cases**
   - Capacity of 1?
   - Put existing key (move to end)?
   - Concurrent access?

5. **Performance Verification**
   - Test that all ops are O(1)
   - Memory overhead?
   - Cache hit rate measurement?

### Rust-Specific Challenges

- Ownership and mutable references in linked list
- Interior mutability (RefCell)?
- Avoiding cycles/memory leaks
- Generic over K and V

### Discussion Points

- HashMap + LinkedList complexity
- Interior mutability needs
- Alternative to linked list
- Concurrency support

---

## Question 4: Implement Binary Search Tree

**Difficulty**: Hard
**Concepts**: Tree structures, recursion, balancing
**Time**: 40-50 minutes

### Problem Statement

Implement a binary search tree with insert, search, delete, and traversal operations.

### Requirements

**Core Features**:
- Insert elements maintaining BST property
- Search for values
- In-order/pre-order/post-order traversal
- Delete nodes (tricky: 0, 1, or 2 children)
- Height tracking (optional)

**Design Decisions**:
- Recursive vs iterative?
- Self-balancing (AVL) or simple BST?
- Node representation

### Key Questions

1. **Node Structure**
   - Value, left child, right child pointers
   - Parent pointers needed?
   - Height/balance factor for AVL?

2. **Insertion**
   - Recursive algorithm
   - Handling duplicates?
   - Rebalancing (if AVL)?

3. **Deletion**
   - Leaf node (easy)
   - One child (replace)
   - Two children (in-order successor/predecessor?)
   - Rebalancing?

4. **Traversal**
   - Recursive implementations
   - Iterative with stack
   - Return order of values

5. **Search**
   - Binary search property usage
   - Complexity analysis
   - Return Option<&V>?

### Rust-Specific Challenges

- Ownership in recursive structures
- Mutable references and borrowing
- Box<T> for heap allocation
- Option<Box<Node>> for child pointers
- Interior mutability if needed

### Discussion Points

- Recursive vs iterative trade-offs
- AVL vs simple BST
- Memory layout optimization
- Handling of equal values

---

## Question 5: Implement Trie (Prefix Tree)

**Difficulty**: Medium-Hard
**Concepts**: Tree structures, character-based indexing
**Time**: 30-40 minutes

### Problem Statement

Implement a Trie data structure for efficient string storage and prefix searching.

### Requirements

**Core Features**:
- Insert strings
- Search for exact strings
- Search by prefix
- Autocomplete suggestions (optional)
- Delete strings (optional)

**Design Decisions**:
- Node per character
- HashMap vs array for child nodes
- Mark end of word vs implicit
- Space vs time trade-offs

### Key Questions

1. **Node Structure**
   - HashMap of character -> child node
   - Boolean for "end of word"
   - Optional values associated with strings

2. **Insertion**
   - Create nodes as needed
   - Mark word endings
   - Reuse common prefixes

3. **Search Operations**
   - Exact match search
   - Prefix search (returns all words)
   - Autocomplete (prefix + suggestions)

4. **Space Optimization**
   - Empty nodes
   - Shared prefixes benefit
   - Memory overhead

5. **Traversal**
   - DFS for finding all strings
   - Collecting matches
   - Ordering results

### Rust-Specific Challenges

- Generic over key type (usually char)
- HashMap vs array for children
- Collecting results with lifetime issues
- Recursive collection patterns

### Discussion Points

- HashMap vs array for character indexing
- Space complexity for sparse tries
- Suffix array alternatives
- Unicode handling

---

## Summary of Data Structure Questions

These questions practice:
- Implementing core data structures
- Memory management
- Performance analysis
- Algorithm design
- Rust-specific ownership challenges

**Preparation**:
- Understand trade-offs of each structure
- Practice implementation in language of choice
- Discuss complexity analysis
- Consider edge cases

**Tips**:
- Start with simple version, then optimize
- Draw pictures of structures
- Implement basic case first
- Handle edge cases systematically
