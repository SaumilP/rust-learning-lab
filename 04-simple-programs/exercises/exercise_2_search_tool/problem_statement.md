# Exercise 2: File Search Tool

## Difficulty: Medium
## Concepts Tested: CLI Arguments, File I/O, Text Processing, Algorithms
## Prerequisites: Module 04 - All Concepts

---

## Problem Statement

Write a Rust program that searches for a pattern in a file. The program should:

1. Accept search pattern and filename as CLI arguments
2. Read the file and find matching lines
3. Display line numbers with matching content
4. Count total matches
5. Support optional flags (case-insensitive, whole-word)

## Requirements

- Accept pattern and filename from CLI arguments
- Read file and search for pattern
- Display matching lines with line numbers
- Show count of matches
- Support `--ignore-case` flag for case-insensitive search
- Support `--whole-word` flag to match complete words only
- Handle errors gracefully
- Clear output formatting

## Expected Input/Output

### Test Case 1: Basic Search
```
Input file (text.txt):
The quick brown fox
jumps over the lazy dog
the fox runs away

Running: cargo run -- "fox" text.txt

Output:
=== Search Results ===
Pattern: "fox"
File: text.txt

Matches found: 2
1: The quick brown fox
3: the fox runs away
```

### Test Case 2: Case-Insensitive Search
```
Running: cargo run -- "fox" text.txt --ignore-case

Output:
=== Search Results ===
Pattern: "fox"
File: text.txt (case-insensitive)

Matches found: 2
1: The quick brown fox
3: the fox runs away
```

### Test Case 3: Whole Word Match
```
Running: cargo run -- "the" text.txt --whole-word

Output:
=== Search Results ===
Pattern: "the"
File: text.txt (whole-word)

Matches found: 1
2: jumps over the lazy dog
```

## Hints

1. **Hint 1**: Use `env::args().collect()` to get all arguments
2. **Hint 2**: Parse arguments: first is pattern, second is filename, rest are flags
3. **Hint 3**: Check for flags using `contains()` or `iter().any()`
4. **Hint 4**: Use `lines()` to process file line by line
5. **Hint 5**: Use `.enumerate()` to get line numbers
6. **Hint 6**: The broken code has bugs in argument parsing, flag handling, or search logic

## Testing

Create test file:
```bash
cat > search_test.txt << 'EOF'
The quick brown fox
jumps over the lazy dog
the fox runs away
EOF
```

Run searches:
```bash
cargo run -- "fox" search_test.txt
cargo run -- "the" search_test.txt --ignore-case
```

## Learning Objectives

After completing this exercise, you should understand:
- Combining multiple CLI features
- Processing command-line flags
- Efficient file searching
- Pattern matching in text
- Building practical command-line tools
- Handling multiple optional parameters
- String comparison strategies

