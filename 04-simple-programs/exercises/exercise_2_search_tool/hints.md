# Hints for Exercise 2: File Search Tool

## Stuck? Here are some hints:

### About the bugs:

**Bug 1: No Argument Validation**
- The code directly accesses: `args[1]`, `args[2]`, `args[3]`, `args[4]`
- Will panic if not enough arguments provided
- Solution: Check argument count first:
  ```rust
  if args.len() < 3 {
      eprintln!("Usage: {} <pattern> <filename> [options]", args[0]);
      eprintln!("Options: --ignore-case, --whole-word");
      std::process::exit(1);
  }

  let pattern = &args[1];
  let filename = &args[2];
  ```

**Bug 2: Flags at Fixed Positions**
- Code assumes flags are at args[3] and args[4]
- Users might provide flags in any position
- Solution: Check all arguments for flags:
  ```rust
  let ignore_case = args.contains(&"--ignore-case".to_string());
  let whole_word = args.contains(&"--whole-word".to_string());
  ```
- This works regardless of flag position
- Much more robust approach

**Bug 3: Wrong Case-Insensitive Logic**
- Code: `line.contains(&pattern.to_lowercase())`
- Only converts pattern to lowercase, not the line
- Comparison fails: "Fox" doesn't contain "fox"
- Solution: Convert both to lowercase:
  ```rust
  let matches = if ignore_case {
      line.to_lowercase().contains(&pattern.to_lowercase())
  } else {
      line.contains(pattern)
  };
  ```
- Now both sides are lowercase for comparison

### Testing your fix:

Create test file:
```bash
cat > test.txt << 'EOF'
The quick brown fox
jumps over the lazy dog
the fox runs away
EOF
```

Test cases:
```bash
cargo run -- "fox" test.txt
# Should find: lines 1 and 3

cargo run -- "fox" test.txt --ignore-case
# Should find: lines 1 and 3 (same as above)

cargo run -- "the" test.txt --ignore-case
# Should find: lines 1, 2, 3

cargo run -- "the" test.txt --ignore-case --whole-word
# Should find: line 2 only (only line with "the" as whole word)
```

### Complete Solution Pattern:

```rust
use std::env;
use std::fs;

fn main() {
    // 1. Validate arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <pattern> <filename> [options]", args[0]);
        std::process::exit(1);
    }

    let pattern = &args[1];
    let filename = &args[2];

    // 2. Check for flags
    let ignore_case = args.contains(&"--ignore-case".to_string());
    let whole_word = args.contains(&"--whole-word".to_string());

    // 3. Display header
    print!("Pattern: \"{}\"", pattern);
    print!("\nFile: {}", filename);
    if ignore_case {
        print!(" (case-insensitive)");
    }
    if whole_word {
        print!(" (whole-word)");
    }
    println!("\n");

    // 4. Read and search
    match fs::read_to_string(filename) {
        Ok(contents) => {
            let mut match_count = 0;

            for (line_num, line) in contents.lines().enumerate() {
                // Check if line matches pattern
                let matches = if ignore_case {
                    line.to_lowercase().contains(&pattern.to_lowercase())
                } else {
                    line.contains(pattern)
                };

                if matches {
                    // Apply whole-word filter if needed
                    let pattern_to_check = if ignore_case {
                        pattern.to_lowercase()
                    } else {
                        pattern.to_string()
                    };

                    let should_print = if whole_word {
                        line.split_whitespace()
                            .any(|word| {
                                if ignore_case {
                                    word.to_lowercase() == pattern_to_check
                                } else {
                                    word == pattern
                                }
                            })
                    } else {
                        true
                    };

                    if should_print {
                        println!("{}: {}", line_num + 1, line);
                        match_count += 1;
                    }
                }
            }

            println!("\nMatches found: {}", match_count);
        }
        Err(e) => {
            eprintln!("Error: Cannot read file '{}': {}", filename, e);
            std::process::exit(1);
        }
    }
}
```

### Debugging tips:

1. Always check: `args.len()` before accessing indices
2. Test without any flags first - should work
3. Test with flags - verify order doesn't matter
4. Test case-insensitive search with mixed case text
5. Test whole-word with pattern that's substring of other words

### Key Concepts:

- **Vector contains**: `args.contains()` checks anywhere in vector
- **Case conversion**: Convert both strings for comparison
- **String splitting**: `.split_whitespace()` gives individual words
- **Any/all predicates**: `.any(|word| ...)` for conditional matching

### If still stuck:

1. **Panic on startup**: Add argument count validation
2. **Flags not working**: Use `.contains()` instead of checking positions
3. **Case-insensitive not working**: Convert both sides: `line.to_lowercase()`
4. **Whole-word not working**: Compare individual words from `.split_whitespace()`

The fixes are usually 12-15 line changes total!

