# Level 4: Intermediate/Medium (6 kyu) Challenges

Scope: 100-200 lines of code, 5+ concepts per problem, complex algorithms

## 1. Advanced Algorithms (4 problems)

### 1.1 QuickSort & MergeSort
**Difficulty**: 6 kyu
**Concepts**: sorting, recursion, algorithms, performance
**Expected Lines**: 100-150

Problem: Implement and compare sorting algorithms
- Implement QuickSort with pivot strategies
- Implement MergeSort with proper merging
- Compare performance characteristics
- Handle edge cases (duplicates, reversed, nearly sorted)

### 1.2 Topological Sort
**Difficulty**: 6 kyu
**Concepts**: graphs, algorithms, depth-first search
**Expected Lines**: 100-140

Problem: Sort vertices in directed acyclic graph
- Use DFS-based approach
- Detect cycles
- Order tasks by dependencies
- Real-world: build systems, course prerequisites

### 1.3 Dijkstra's Algorithm
**Difficulty**: 6 kyu
**Concepts**: graphs, heaps, algorithms, optimization
**Expected Lines**: 120-180

Problem: Find shortest paths in weighted graph
- Implement with priority queue
- Handle all edge weights
- Track path reconstruction
- Real-world: GPS, network routing

### 1.4 Knapsack Problem (DP)
**Difficulty**: 6 kyu
**Concepts**: dynamic programming, algorithms, optimization
**Expected Lines**: 100-150

Problem: Optimize items to fit in knapsack
- 0/1 knapsack variant
- Track items selected
- Optimize space usage
- Variants: bounded, unbounded

---

## 2. Data Structure Implementation (4 problems)

### 2.1 AVL Tree Basics
**Difficulty**: 6 kyu
**Concepts**: trees, balance, recursion, algorithms
**Expected Lines**: 150-200

Problem: Implement self-balancing binary search tree
- Insert with rebalancing
- Rotation operations
- Height maintenance
- Search and delete

### 2.2 Red-Black Tree Basics
**Difficulty**: 6 kyu
**Concepts**: trees, color properties, algorithms
**Expected Lines**: 150-200

Problem: Implement red-black tree properties
- Color constraints
- Insertion with rebalancing
- Deletion with fixup
- Maintain balance invariants

### 2.3 Trie (Prefix Tree)
**Difficulty**: 6 kyu
**Concepts**: trees, strings, algorithms, data structures
**Expected Lines**: 100-150

Problem: Build and search in trie
- Insert words
- Search with prefix
- Auto-complete functionality
- Word frequency tracking

### 2.4 Graph Representation & Algorithms
**Difficulty**: 6 kyu
**Concepts**: graphs, collections, algorithms
**Expected Lines**: 120-180

Problem: Multiple graph representations and queries
- Adjacency list vs matrix
- Connected components
- Cycle detection
- Strongly connected components

---

## 3. Concurrent & Async (4 problems)

### 3.1 Mutex & Shared State
**Difficulty**: 6 kyu
**Concepts**: concurrency, Mutex, Arc, threads
**Expected Lines**: 100-150

Problem: Safely share mutable state
- Multiple threads accessing shared counter
- Proper locking patterns
- Avoid deadlocks
- Performance under contention

### 3.2 Channel Operations
**Difficulty**: 6 kyu
**Concepts**: channels, message passing, threads
**Expected Lines**: 100-140

Problem: Implement producer-consumer pattern
- Multi-producer, single consumer
- Handle disconnections
- Bounded vs unbounded channels
- Blocking operations

### 3.3 Thread Safety Challenges
**Difficulty**: 6 kyu
**Concepts**: concurrency, Send/Sync, thread safety
**Expected Lines**: 100-150

Problem: Identify and fix concurrency issues
- Race conditions
- Deadlock scenarios
- Memory safety violations
- Proper synchronization

### 3.4 Arc & Rc Patterns
**Difficulty**: 6 kyu
**Concepts**: reference counting, borrowing, ownership
**Expected Lines**: 100-140

Problem: Manage shared ownership
- Arc for thread-safe sharing
- Rc for single-threaded sharing
- Circular reference handling
- Weak pointers

---

## 4. System Design (4 problems)

### 4.1 LRU Cache Implementation
**Difficulty**: 6 kyu
**Concepts**: data structures, algorithms, design patterns
**Expected Lines**: 120-180

Problem: Implement Least Recently Used cache
- O(1) get and put operations
- Eviction policy
- Generic over key/value types
- Handle capacity limits

### 4.2 Rate Limiter
**Difficulty**: 6 kyu
**Concepts**: algorithms, state management, design
**Expected Lines**: 100-150

Problem: Implement token bucket rate limiter
- Allow N requests per time window
- Thread-safe for concurrent access
- Configurable rate
- Multiple strategies (fixed window, sliding window)

### 4.3 Observer Pattern
**Difficulty**: 6 kyu
**Concepts**: design patterns, generics, Rc/RefCell
**Expected Lines**: 100-150

Problem: Implement publish-subscribe system
- Multiple observers
- Event notification
- Registration/deregistration
- Type safety

### 4.4 Event System
**Difficulty**: 6 kyu
**Concepts**: design patterns, trait objects, collections
**Expected Lines**: 120-180

Problem: Build event dispatcher
- Event types and handlers
- Register/unregister listeners
- Dispatch events
- Error handling and recovery

---

## Problem Categories

### Performance Focus
- QuickSort/MergeSort (algorithm selection)
- Dijkstra's (optimization)
- Trie (space/time tradeoff)

### Correctness Focus
- AVL/Red-Black Trees (invariant maintenance)
- Thread safety (race condition prevention)
- LRU Cache (correctness under pressure)

### Design Focus
- Observer pattern (architecture)
- Event system (extensibility)
- Rate limiter (requirements to code)

---

## Learning Path

1. **Week 1**: Advanced Algorithms (QuickSort, MergeSort, Topological)
2. **Week 2**: Data Structures (AVL, Trie, Graphs)
3. **Week 3**: Concurrency (Mutex, Channels, Thread safety)
4. **Week 4**: System Design (LRU, Rate Limiter, Patterns)

## Prerequisites

Before Level 4, ensure mastery of:
- ✓ Level 1-3 challenges completed
- ✓ Recursion and recursive thinking
- ✓ Generics and trait bounds
- ✓ Ownership and borrowing
- ✓ Basic threading concepts
- ✓ HashMap and collections

## Success Metrics

| Concept | Proficiency |
|---------|-------------|
| Sorting algorithms | Can implement and explain |
| Graph algorithms | DFS/BFS/Dijkstra fluency |
| Trees | Balance and rotation mastery |
| Concurrency | Mutex/channel patterns |
| System design | Real-world design thinking |

## Tips & Strategies

- **Understand deeply**: Know why each algorithm works
- **Test thoroughly**: Large inputs, edge cases, stress
- **Optimize iteratively**: Correct first, optimize second
- **Handle concurrency**: Test with multiple threads
- **Think about tradeoffs**: Space vs time, correctness vs simplicity

---

**Level 4 Total**: 16 problems
**Recommended Time**: 30-40 hours
**Difficulty**: 6 kyu (intermediate/medium)
**Focus**: Complex algorithms and concurrent systems

