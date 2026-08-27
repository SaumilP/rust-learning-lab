# Exercise 3: Text Filter with CLI Arguments

## Difficulty: Medium
## Concepts Tested: CLI Arguments, File I/O, Text Processing, Algorithms
## Prerequisites: Module 04 - All Concepts

---

## Problem Statement

Write a Rust program that filters text based on criteria specified via CLI arguments. The program should:

1. Accept input file, output file, and filter criteria from arguments
2. Read input file line by line
3. Apply filtering based on criteria (length, pattern, content)
4. Write filtered results to output file
5. Display statistics about filtering

## Requirements

- Accept input file, output file, and filter type from CLI arguments
- Support multiple filter types: `--min-length N`, `--max-length N`, `--contains PATTERN`, `--exclude PATTERN`
- Read and process input file line by line
- Write matching lines to output file
- Display filtering statistics (total lines, filtered lines, removed lines)
- Handle errors gracefully
- Support combining multiple filters
- Show clear progress/summary

## Expected Input/Output

### Test Case 1: Filter by Minimum Length
```
Input file (lines.txt):
a
hello
the quick brown fox
rust
programming language

Running: cargo run -- lines.txt output.txt --min-length 5

Output:
=== Text Filter ===
Input: lines.txt
Output: output.txt
Filter: Minimum length 5

Processing complete:
  Total lines: 5
  Filtered lines: 4
  Removed lines: 1

Output written to: output.txt

Output file contents:
hello
the quick brown fox
programming language
```

### Test Case 2: Filter by Pattern
```
Running: cargo run -- lines.txt output.txt --contains "fox"

Output:
=== Text Filter ===
Input: lines.txt
Output: output.txt
Filter: Contains "fox"

Processing complete:
  Total lines: 5
  Filtered lines: 1
  Removed lines: 4

Output written to: output.txt

Output file contents:
the quick brown fox
```

### Test Case 3: Multiple Filters
```
Running: cargo run -- lines.txt output.txt --min-length 5 --contains "g"

Output:
=== Text Filter ===
Input: lines.txt
Output: output.txt
Filter: Minimum length 5, Contains "g"

Processing complete:
  Total lines: 5
  Filtered lines: 2
  Removed lines: 3

Output written to: output.txt

Output file contents:
programming language
```

## Hints

1. **Hint 1**: Use `env::args().collect()` to get all arguments
2. **Hint 2**: Parse arguments starting from index 1 (skip program name)
3. **Hint 3**: Check for filter flags: `--min-length`, `--max-length`, `--contains`, `--exclude`
4. **Hint 4**: Apply each filter condition with boolean logic (AND)
5. **Hint 5**: Use `lines()` for line-by-line processing
6. **Hint 6**: The broken code has bugs in argument parsing, filter logic, or output writing

## Testing

Create test file:
```bash
cat > filter_test.txt << 'EOF'
a
hello
the quick brown fox
rust
programming language
EOF
```

Run filters:
```bash
cargo run -- filter_test.txt out1.txt --min-length 5
cargo run -- filter_test.txt out2.txt --contains "fox"
cargo run -- filter_test.txt out3.txt --min-length 5 --contains "g"
```

## Learning Objectives

After completing this exercise, you should understand:
- Parsing multiple CLI arguments and flags
- Combining multiple filter conditions
- Processing text with complex criteria
- Writing filtered results to files
- Building text manipulation tools
- Handling variable argument counts
- Reporting statistics on processed data

