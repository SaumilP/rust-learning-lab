# Exercise 1: Word Counter CLI Tool

## Difficulty: Easy to Medium
## Concepts Tested: CLI Arguments, File I/O, Text Processing
## Prerequisites: Module 04 - CLI Arguments, File I/O Basics, Text Processing

---

## Problem Statement

Write a Rust program that counts words in a file provided via command-line argument. The program should:

1. Accept a filename as command-line argument
2. Read the file and count total words
3. Find most common words
4. Display statistics
5. Handle errors gracefully

## Requirements

- Accept filename from CLI arguments
- Read file and process text
- Count total words and unique words
- Display top 5 most common words
- Handle missing file gracefully
- Display clear error messages
- Show processing statistics

## Expected Input/Output

### Test Case 1: Count Words
```
Input file (sample.txt):
the quick brown fox jumps over the lazy dog
the fox runs in the forest

Running: cargo run -- sample.txt

Output:
=== Word Counter ===
File: sample.txt
Total words: 16
Unique words: 10

Top 5 most common words:
1. the (3 occurrences)
2. fox (2 occurrences)
3. quick (1 occurrence)
4. brown (1 occurrence)
5. jumps (1 occurrence)
```

### Test Case 2: Missing File
```
Running: cargo run -- nonexistent.txt

Output:
Error: Cannot read file 'nonexistent.txt': No such file or directory
```

### Test Case 3: Empty File
```
Running: cargo run -- empty.txt

Output:
=== Word Counter ===
File: empty.txt
Total words: 0
Unique words: 0

No words to display.
```

## Hints

1. **Hint 1**: Use `std::env::args()` to get filename from CLI
2. **Hint 2**: Check argument count before accessing
3. **Hint 3**: Use `fs::read_to_string()` to read file
4. **Hint 4**: Use `.split_whitespace()` to split into words
5. **Hint 5**: Use HashMap to count word frequencies
6. **Hint 6**: The broken code has bugs in argument handling, file reading, or word counting

## Testing

Create a test file first:
```bash
echo "the quick brown fox jumps over the lazy dog" > test.txt
echo "the fox runs in the forest" >> test.txt
```

Then run:
```bash
cargo run -- test.txt
```

## Learning Objectives

After completing this exercise, you should understand:
- Combining CLI arguments with file I/O
- Text processing with word splitting
- HashMap usage for frequency counting
- Error handling in practical programs
- Building command-line tools
- Displaying formatted output
- Processing real data from files

