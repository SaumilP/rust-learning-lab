# Hints for Exercise 1: Word Counter CLI Tool

## Stuck? Here are some hints:

### About the bugs:

**Bug 1: No Argument Count Check**
- The code: `let filename = &env::args().collect::<Vec<_>>()[1];`
- This directly accesses index 1 without checking if it exists
- If no arguments provided, this will panic with "index out of bounds"
- Solution: Check argument count first:
  ```rust
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
      eprintln!("Usage: {} <filename>", args[0]);
      std::process::exit(1);
  }

  let filename = &args[1];
  ```
- Provides clear error message to user

**Bug 2: Punctuation Not Removed**
- The code: `let word_lower = word.to_lowercase();`
- This keeps punctuation attached to words
- "fox," and "fox" are treated as different words
- Solution: Remove punctuation before counting:
  ```rust
  let word_clean = word
      .to_lowercase()
      .chars()
      .filter(|c| c.is_alphabetic())
      .collect::<String>();

  if !word_clean.is_empty() {
      *frequencies.entry(word_clean).or_insert(0) += 1;
  }
  ```
- Or use a helper function:
  ```rust
  fn clean_word(word: &str) -> String {
      word.chars()
          .filter(|c| c.is_alphabetic())
          .collect::<String>()
          .to_lowercase()
  }
  ```

**Bug 3: Not Sorting by Frequency**
- The code iterates HashMap directly without sorting
- HashMap iteration order is not guaranteed
- Top words won't be in order by frequency
- Solution: Sort before displaying:
  ```rust
  // Convert to vector of tuples
  let mut freq_vec: Vec<_> = frequencies.iter().collect();

  // Sort by frequency (descending)
  freq_vec.sort_by(|a, b| b.1.cmp(a.1));

  // Display top 5
  println!("\nTop 5 most common words:");
  for (i, (word, freq)) in freq_vec.iter().take(5).enumerate() {
      println!("{}. {} ({} occurrences)", i + 1, word, freq);
  }
  ```
- The `.take(5)` ensures we only show top 5

### Testing your fix:

Create test file:
```bash
echo "the quick brown fox jumps over the lazy dog" > test.txt
echo "the fox runs in the forest" >> test.txt
```

Run:
```bash
cargo run -- test.txt
```

Verify output shows:
- "the" appears 3 times (most common)
- "fox" appears 2 times (second most)
- Other words appear once each

### Complete Solution Pattern:

```rust
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    // 1. Check arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    // 2. Read file
    match fs::read_to_string(filename) {
        Ok(contents) => {
            // 3. Count words with cleaning
            let mut frequencies = HashMap::new();
            for word in contents.split_whitespace() {
                let clean = word
                    .to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .collect::<String>();

                if !clean.is_empty() {
                    *frequencies.entry(clean).or_insert(0) += 1;
                }
            }

            // 4. Sort and display
            let mut freq_vec: Vec<_> = frequencies.iter().collect();
            freq_vec.sort_by(|a, b| b.1.cmp(a.1));

            println!("File: {}", filename);
            println!("Total words: {}", contents.split_whitespace().count());
            println!("Unique words: {}", frequencies.len());
            println!("\nTop 5 most common words:");

            for (i, (word, freq)) in freq_vec.iter().take(5).enumerate() {
                println!("{}. {} ({} occurrences)", i + 1, word, freq);
            }
        }
        Err(e) => {
            eprintln!("Error: Cannot read file '{}': {}", filename, e);
            std::process::exit(1);
        }
    }
}
```

### Debugging tips:

1. Test with no arguments - should show usage message
2. Test with nonexistent file - should show error message
3. Test with file containing punctuation - verify it's removed
4. Count manually and verify top words are correct
5. Add temporary println! statements to debug word cleaning

### Key Concepts:

- **Argument validation**: Always check before accessing
- **Text cleaning**: Remove punctuation for accurate counting
- **Sorting data**: Use `.sort_by()` with custom comparators
- **Error handling**: Use Result and pattern matching
- **HashMap iteration**: Convert to Vec to sort

### If still stuck:

1. **Panic on missing args**: Add argument count check
2. **Wrong word counts**: Remove punctuation with `.filter(|c| c.is_alphabetic())`
3. **Words in wrong order**: Sort by frequency before displaying
4. **Empty word count**: Skip empty strings with `if !clean.is_empty()`

The fixes are usually 8-10 line changes total!

