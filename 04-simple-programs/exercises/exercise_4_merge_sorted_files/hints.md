# Hints for Exercise 4: Merge Sorted Files

## Stuck? Here are some hints:

### About the bugs:

**Bug 1: No Argument Validation**
- The code directly accesses: `args[1]`, `args[2]`, `args[3]` without checking
- Will panic if insufficient arguments provided
- Solution: Validate arguments first:
  ```rust
  if args.len() < 4 {
      eprintln!("Usage: {} <file1> <file2> <output> [options]", args[0]);
      std::process::exit(1);
  }

  let file1 = &args[1];
  let file2 = &args[2];
  let output = &args[3];
  ```

**Bug 2 & 3: Merge Algorithm Issues**
- Missing increment statements in while loops
- The loops will infinite-loop because i and j never increment in remainder loops
- Correct merge algorithm requires three phases:

Phase 1: Compare and merge while both have elements
```rust
while i < lines1.len() && j < lines2.len() {
    if lines1[i] <= lines2[j] {
        merged.push(lines1[i].to_string());
        i += 1;
    } else {
        merged.push(lines2[j].to_string());
        j += 1;
    }
}
```

Phase 2: Add remaining from array1
```rust
while i < lines1.len() {
    merged.push(lines1[i].to_string());
    i += 1;  // IMPORTANT: Must increment!
}
```

Phase 3: Add remaining from array2
```rust
while j < lines2.len() {
    merged.push(lines2[j].to_string());
    j += 1;  // IMPORTANT: Must increment!
}
```

### Complete Solution Pattern:

```rust
use std::env;
use std::fs;
use std::io::Write;

fn main() {
    // 1. Validate arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <file1> <file2> <output>", args[0]);
        std::process::exit(1);
    }

    let file1 = &args[1];
    let file2 = &args[2];
    let output = &args[3];

    println!("Input 1: {}", file1);
    println!("Input 2: {}", file2);
    println!("Output: {}", output);
    println!();

    // 2. Read both files
    match (fs::read_to_string(file1), fs::read_to_string(file2)) {
        (Ok(content1), Ok(content2)) => {
            let lines1: Vec<&str> = content1.lines().collect();
            let lines2: Vec<&str> = content2.lines().collect();

            println!("Processing:");
            println!("  File 1 lines: {}", lines1.len());
            println!("  File 2 lines: {}", lines2.len());
            println!("  Total lines: {}", lines1.len() + lines2.len());
            println!();

            // 3. Merge using two-pointer technique
            let mut merged = Vec::new();
            let mut i = 0;
            let mut j = 0;

            // Phase 1: Merge while both have elements
            while i < lines1.len() && j < lines2.len() {
                if lines1[i] <= lines2[j] {
                    merged.push(lines1[i].to_string());
                    i += 1;
                } else {
                    merged.push(lines2[j].to_string());
                    j += 1;
                }
            }

            // Phase 2: Add remaining from file1
            while i < lines1.len() {
                merged.push(lines1[i].to_string());
                i += 1;  // CRITICAL: Increment!
            }

            // Phase 3: Add remaining from file2
            while j < lines2.len() {
                merged.push(lines2[j].to_string());
                j += 1;  // CRITICAL: Increment!
            }

            // 4. Write output
            match fs::File::create(output) {
                Ok(mut file) => {
                    for line in &merged {
                        let _ = writeln!(file, "{}", line);
                    }
                    println!("Merge complete!");
                    println!("Output written to: {}", output);
                }
                Err(e) => {
                    eprintln!("Error writing file: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Error reading input files");
            std::process::exit(1);
        }
    }
}
```

### Testing your fix:

Create test files:
```bash
# Integer merge
echo -e "1\n3\n5\n7\n9" > int1.txt
echo -e "2\n4\n6\n8\n10" > int2.txt

# String merge
echo -e "apple\ncherry\nelephant" > str1.txt
echo -e "banana\ndog\nfig" > str2.txt
```

Test merges:
```bash
cargo run -- int1.txt int2.txt out1.txt
cargo run -- str1.txt str2.txt out2.txt
```

Verify output files are sorted and contain all elements from both inputs.

### Understanding the Two-Pointer Merge:

The merge algorithm compares elements from two sorted sequences:
1. **Compare**: Which element is smaller?
2. **Add**: Add smaller element to result
3. **Advance**: Move pointer of the sequence we took from
4. **Repeat**: Until one sequence is exhausted
5. **Cleanup**: Add all remaining elements from non-empty sequence

This is O(n + m) time complexity - very efficient!

### Debugging tips:

1. **Infinite loop**: Check if you're incrementing i and j in both remainder loops
2. **Missing elements**: Ensure all three phases are present (merge, remainder1, remainder2)
3. **Unsorted output**: Verify input files are actually sorted
4. **Wrong output**: Trace through merge manually with small examples
5. **File issues**: Verify output file is created and contains data

### Key Concepts:

- **Two-pointer technique**: Efficient for processing two sorted sequences
- **Three phases**: Merge phase, then two cleanup phases
- **Loop invariants**: i and j must always advance
- **Edge cases**: One file empty, duplicates at boundaries, etc.

### Common Mistakes:

1. ❌ Forgetting to increment i/j in remainder loops → infinite loop
2. ❌ Skipping remainder loops → losing elements
3. ❌ Wrong comparison operator → incorrect sort order
4. ❌ No argument validation → panic on startup

### If still stuck:

1. **Panic on startup**: Add argument count validation
2. **Infinite loop**: Add `i += 1` and `j += 1` to remainder loops
3. **Wrong order**: Check if comparison is `<=` not `>=`
4. **Missing elements**: Ensure all three merge phases present
5. **Type issues**: For integers, parse first then compare

The fixes are usually 8-12 line changes total!

