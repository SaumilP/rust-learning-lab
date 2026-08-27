# Hints for Exercise 3: Text Filter with CLI Arguments

## Stuck? Here are some hints:

### About the bugs:

**Bug 1: No Argument Validation**
- The code directly accesses: `args[1]`, `args[2]` without checking length
- Will panic if not enough arguments provided
- Solution: Validate first:
  ```rust
  if args.len() < 3 {
      eprintln!("Usage: {} <input> <output> [filters]", args[0]);
      eprintln!("Filters: --min-length N, --max-length N, --contains PATTERN, --exclude PATTERN");
      std::process::exit(1);
  }

  let input_file = &args[1];
  let output_file = &args[2];
  ```

**Bug 2: Incorrect Flag Value Parsing**
- The code: `min_length = args[i].parse::<usize>().unwrap_or(0);`
- Trying to parse the flag itself (e.g., "--min-length") as a number
- Should parse the NEXT argument: `args[i+1]`
- Solution: Get the value from next argument:
  ```rust
  for i in 3..args.len() {
      match args[i].as_str() {
          "--min-length" => {
              if i + 1 < args.len() {
                  min_length = args[i + 1].parse::<usize>().unwrap_or(0);
              }
          }
          "--contains" => {
              if i + 1 < args.len() {
                  contains_pattern = args[i + 1].clone();  // Use i+1, not i
              }
          }
          _ => {}
      }
  }
  ```
- Note: You should also skip the value on next iteration

**Bug 3: Wrong Filter Logic - OR Instead of AND**
- The code: `if passes_min_length || passes_contains || passes_exclude`
- Uses OR: line passes if ANY condition is true
- Should use AND: line passes if ALL conditions are true
- When filtering by min-length AND contains, need both true
- Solution: Use AND logic:
  ```rust
  if passes_min_length && passes_contains && passes_exclude {
      filtered_lines.push(line.to_string());
  }
  ```
- Example: min_length=5 AND contains="g" should only match lines with 5+ chars AND "g"

### Complete Solution Pattern:

```rust
use std::env;
use std::fs;
use std::io::Write;

fn main() {
    // 1. Validate arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input> <output> [filters]", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    // 2. Parse filters
    let mut min_length = 0;
    let mut max_length = usize::MAX;
    let mut contains_pattern = String::new();
    let mut exclude_pattern = String::new();

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--min-length" => {
                if i + 1 < args.len() {
                    min_length = args[i + 1].parse().unwrap_or(0);
                    i += 2;  // Skip flag and value
                    continue;
                }
            }
            "--contains" => {
                if i + 1 < args.len() {
                    contains_pattern = args[i + 1].clone();
                    i += 2;
                    continue;
                }
            }
            "--exclude" => {
                if i + 1 < args.len() {
                    exclude_pattern = args[i + 1].clone();
                    i += 2;
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }

    // 3. Display filter info
    println!("Input: {}", input_file);
    println!("Output: {}", output_file);
    print!("Filter: ");
    if min_length > 0 {
        print!("Minimum length {}", min_length);
    }
    if !contains_pattern.is_empty() {
        print!(", Contains \"{}\"", contains_pattern);
    }
    println!("\n");

    // 4. Read and filter
    match fs::read_to_string(input_file) {
        Ok(contents) => {
            let mut filtered_lines = Vec::new();
            let total_lines = contents.lines().count();

            for line in contents.lines() {
                // ALL conditions must be true (AND)
                let passes_min = min_length == 0 || line.len() >= min_length;
                let passes_max = max_length == usize::MAX || line.len() <= max_length;
                let passes_contains = contains_pattern.is_empty() || line.contains(&contains_pattern);
                let passes_exclude = exclude_pattern.is_empty() || !line.contains(&exclude_pattern);

                if passes_min && passes_max && passes_contains && passes_exclude {
                    filtered_lines.push(line.to_string());
                }
            }

            // 5. Write output
            match fs::File::create(output_file) {
                Ok(mut file) => {
                    for line in &filtered_lines {
                        let _ = writeln!(file, "{}", line);
                    }

                    println!("Processing complete:");
                    println!("  Total lines: {}", total_lines);
                    println!("  Filtered lines: {}", filtered_lines.len());
                    println!("  Removed lines: {}", total_lines - filtered_lines.len());
                    println!("\nOutput written to: {}", output_file);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Testing your fix:

Create test file:
```bash
cat > test.txt << 'EOF'
a
hello
the quick brown fox
rust
programming language
EOF
```

Test cases:
```bash
# Min length 5: should get "hello", "the quick brown fox", "programming language"
cargo run -- test.txt out1.txt --min-length 5

# Contains "fox": should get "the quick brown fox"
cargo run -- test.txt out2.txt --contains "fox"

# Min length 5 AND contains "g": should get "programming language"
cargo run -- test.txt out3.txt --min-length 5 --contains "g"
```

### Debugging tips:

1. Print filters before processing to verify parsing
2. Test each filter individually first
3. Verify AND logic: if min=5 and contains="g", only lines with both should pass
4. Check file writing: verify output file contains expected lines
5. Test edge cases: empty patterns, zero length, non-matching filters

### Key Concepts:

- **Flag-value pairs**: Always skip both flag and value when found
- **AND vs OR logic**: Multiple conditions usually combine with AND
- **File writing**: Use `writeln!` macro for clean line output
- **String parsing**: Handle parse errors with `unwrap_or()`

### If still stuck:

1. **Panic on startup**: Add argument count validation with usage message
2. **Wrong flag values**: Access `args[i+1]` not `args[i]` for the value
3. **Wrong filter results**: Change `||` to `&&` in filter condition
4. **Skip increment bug**: Remember to increment i by 2 when consuming flag+value

The fixes are usually 15-20 line changes total!

