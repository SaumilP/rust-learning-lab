# Exercise 4: Merge Sorted Files

## Difficulty: Medium to Hard
## Concepts Tested: CLI Arguments, File I/O, Algorithms, Data Processing
## Prerequisites: Module 04 - All Concepts

---

## Problem Statement

Write a Rust program that merges two sorted files into a single sorted output file. The program should:

1. Accept two input filenames and output filename from CLI arguments
2. Read both input files (assumed to be sorted)
3. Merge them while maintaining sorted order
4. Write merged result to output file
5. Handle different data types (integers, floats, strings)
6. Display merge statistics

## Requirements

- Accept two input files and output file from CLI arguments
- Support optional `--type` flag for data type (int, float, string)
- Read and parse input files based on type
- Implement merge algorithm (efficient merge of two sorted sequences)
- Write merged results to output file
- Display statistics (lines from each file, total lines, time taken)
- Handle invalid data gracefully
- Detect and skip duplicates (optional)
- Maintain sorted order

## Expected Input/Output

### Test Case 1: Merge Integer Files
```
File 1 (numbers1.txt):
1
3
5
7
9

File 2 (numbers2.txt):
2
4
6
8
10

Running: cargo run -- numbers1.txt numbers2.txt output.txt --type int

Output:
=== Merge Sorted Files ===
Input 1: numbers1.txt
Input 2: numbers2.txt
Output: output.txt
Type: integer

Processing:
  File 1 lines: 5
  File 2 lines: 5
  Total lines: 10

Output file contents:
1
2
3
4
5
6
7
8
9
10
```

### Test Case 2: Merge String Files
```
File 1 (words1.txt):
apple
cherry
elephant

File 2 (words2.txt):
banana
dog
fig

Running: cargo run -- words1.txt words2.txt output.txt --type string

Output:
=== Merge Sorted Files ===
Input 1: words1.txt
Input 2: words2.txt
Output: output.txt
Type: string

Processing:
  File 1 lines: 3
  File 2 lines: 3
  Total lines: 6

Output file contents:
apple
banana
cherry
dog
elephant
fig
```

### Test Case 3: Merge with Duplicates
```
Running: cargo run -- dup1.txt dup2.txt output.txt --no-duplicates

Output:
=== Merge Sorted Files ===
Input 1: dup1.txt
Input 2: dup2.txt
Output: output.txt
Duplicates: removed

Processing:
  File 1 lines: 5
  File 2 lines: 5
  Total lines: 10
  After dedup: 7
```

## Hints

1. **Hint 1**: Use `env::args().collect()` to get filename arguments
2. **Hint 2**: Read both files and split into lines/values
3. **Hint 3**: Implement two-pointer merge algorithm (like merge sort)
4. **Hint 4**: Parse values based on type: `parse::<i32>()`, `parse::<f64>()`, or use as String
5. **Hint 5**: Compare values and add smaller one to result, advance pointer
6. **Hint 6**: The broken code has bugs in file reading, merge algorithm, or output handling

## Testing

Create test files:
```bash
# Integer test
echo -e "1\n3\n5\n7\n9" > int1.txt
echo -e "2\n4\n6\n8\n10" > int2.txt

# String test
echo -e "apple\ncherry\nelephant" > str1.txt
echo -e "banana\ndog\nfig" > str2.txt
```

Run merges:
```bash
cargo run -- int1.txt int2.txt out_int.txt --type int
cargo run -- str1.txt str2.txt out_str.txt --type string
```

## Learning Objectives

After completing this exercise, you should understand:
- Implementing efficient algorithms (merge from merge sort)
- Processing multiple files
- Parsing different data types
- Working with sorted data
- Implementing two-pointer technique
- Building data processing tools
- Handling edge cases in merging
- Performance implications of merging algorithms

