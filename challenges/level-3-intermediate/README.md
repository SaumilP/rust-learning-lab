# Level 3: Intermediate/Easy (5-6 kyu) Challenges

Scope: 50-100 lines of code, 3-4 concepts per problem

## 1. String Processing (4 problems)

### 1.1 Regex-Like Pattern Matching
**Difficulty**: 5 kyu
**Concepts**: strings, iterators, pattern matching
**Expected Lines**: 60-80

Problem: Find all occurrences of a pattern in text (simple wildcard matching: . for any char, * for zero or more)
- Input: text string, pattern string
- Output: Vec of match positions
- Constraints: Handle overlapping matches
- Test cases: 5+

### 1.2 Run-Length Encoding (RLE) Compression
**Difficulty**: 5 kyu
**Concepts**: strings, iterators, collections
**Expected Lines**: 50-70

Problem: Compress string using run-length encoding
- Input: string
- Output: compressed string (e.g., "aaa" -> "3a")
- Variants: compression and decompression
- Test cases: 6+

### 1.3 Balanced Bracket Checker
**Difficulty**: 5 kyu
**Concepts**: strings, stacks, pattern matching
**Expected Lines**: 50-70

Problem: Validate bracket sequences with nesting
- Input: string with brackets
- Output: boolean (valid/invalid)
- Support: (), {}, []
- Test cases: 8+

### 1.4 Levenshtein Distance
**Difficulty**: 6 kyu
**Concepts**: strings, algorithms, DP
**Expected Lines**: 60-90

Problem: Calculate edit distance between two strings
- Input: two strings
- Output: minimum edits needed
- Operations: insert, delete, replace
- Test cases: 6+

---

## 2. Array Manipulation (4 problems)

### 2.1 Merge Sorted Arrays
**Difficulty**: 5 kyu
**Concepts**: arrays, iterators, merging
**Expected Lines**: 50-70

Problem: Merge two sorted arrays efficiently
- Input: two sorted arrays
- Output: single merged sorted array
- Constraints: O(n+m) time
- Test cases: 6+

### 2.2 Sliding Window Maximum
**Difficulty**: 5 kyu
**Concepts**: arrays, sliding window, collections
**Expected Lines**: 60-80

Problem: Find maximum in each window of size k
- Input: array, window size k
- Output: Vec of max values per window
- Constraints: Efficient approach
- Test cases: 6+

### 2.3 Array Partition
**Difficulty**: 5 kyu
**Concepts**: arrays, mutable refs, algorithms
**Expected Lines**: 50-70

Problem: Partition array by predicate (in-place)
- Input: array, predicate function
- Output: partition point index
- Variants: move vs partition strategy
- Test cases: 6+

### 2.4 Longest Increasing Subsequence
**Difficulty**: 6 kyu
**Concepts**: arrays, algorithms, DP
**Expected Lines**: 70-100

Problem: Find longest increasing subsequence
- Input: array of integers
- Output: length of LIS (or the sequence itself)
- Variants: strictly increasing or non-decreasing
- Test cases: 6+

---

## 3. Sorting & Searching (4 problems)

### 3.1 Custom Sort Comparator
**Difficulty**: 5 kyu
**Concepts**: sorting, closures, trait impl
**Expected Lines**: 60-80

Problem: Sort structs by multiple fields
- Input: Vec of structs, sort criteria
- Output: sorted Vec
- Example: sort by name then age
- Test cases: 6+

### 3.2 Binary Search Implementation
**Difficulty**: 5 kyu
**Concepts**: algorithms, loops/recursion, math
**Expected Lines**: 50-70

Problem: Implement binary search
- Input: sorted array, target value
- Output: Option with index
- Variants: find first/last, approximate matches
- Test cases: 8+

### 3.3 K-Way Merge
**Difficulty**: 6 kyu
**Concepts**: collections, algorithms, iterators
**Expected Lines**: 70-100

Problem: Merge k sorted lists efficiently
- Input: Vec of sorted lists
- Output: single sorted merged list
- Constraints: Use heap for efficiency
- Test cases: 6+

### 3.4 Radix Sort Implementation
**Difficulty**: 6 kyu
**Concepts**: sorting, bit operations, algorithms
**Expected Lines**: 80-120

Problem: Implement radix sort for integers
- Input: array of positive integers
- Output: sorted array
- Approach: sort by digits
- Test cases: 6+

---

## 4. Problem Solving (4 problems)

### 4.1 Calendar Date Problems
**Difficulty**: 5 kyu
**Concepts**: math, algorithms, date logic
**Expected Lines**: 60-80

Problem: Calculate days between dates, day of week
- Input: two dates (year, month, day)
- Output: days difference, day name
- Constraints: Handle leap years, month lengths
- Test cases: 8+

### 4.2 Generate Permutations
**Difficulty**: 6 kyu
**Concepts**: recursion, collections, algorithms
**Expected Lines**: 70-100

Problem: Generate all permutations of elements
- Input: array or string
- Output: Vec of all permutations
- Constraints: Efficient generation
- Test cases: 6+

### 4.3 Graph Traversal (DFS/BFS)
**Difficulty**: 6 kyu
**Concepts**: graphs, recursion, collections
**Expected Lines**: 80-120

Problem: Implement DFS and BFS
- Input: graph (adjacency list), start node
- Output: traversal order
- Variants: find paths, detect cycles
- Test cases: 8+

### 4.4 Tree Traversal (In/Pre/Post Order)
**Difficulty**: 6 kyu
**Concepts**: trees, recursion, enums
**Expected Lines**: 70-100

Problem: Implement tree traversals
- Input: binary tree structure
- Output: ordered traversal results
- Variants: in-order, pre-order, post-order, level-order
- Test cases: 8+

---

## Learning Path

Recommended order:
1. Start with String Processing (easier to visualize)
2. Move to Array Manipulation (build intuition)
3. Study Sorting & Searching (classic algorithms)
4. Challenge with Problem Solving (integration)

## Tips for Success

- **Start simple**: Begin with smaller examples
- **Build incrementally**: Solve variants progressively
- **Understand before coding**: Know algorithm first
- **Test edge cases**: Empty, single element, large data
- **Optimize iteratively**: Simple first, then efficient

## Common Patterns

| Pattern | Used In | Solution |
|---------|---------|----------|
| Two pointers | Merge, Partition | Track both ends |
| Sliding window | Max in window | Move window efficiently |
| DFS/BFS | Graphs, Trees | Recursion or stack/queue |
| Sorting | Custom sort | Closures as comparators |
| DP | LIS, Edit distance | Memoization table |

## Next Steps

After Level 3:
- Proceed to Level 4 (Advanced Algorithms)
- Study data structures deeply
- Practice system design
- Prepare for interviews

---

**Level 3 Total**: 16 problems
**Recommended Time**: 20-30 hours
**Difficulty**: 5-6 kyu equivalent
**Focus**: Algorithms and complex problem solving

