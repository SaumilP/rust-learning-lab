# Hints for Exercise 1: String Processor

## Bug 1: Wrong Implementation

**Location**: `ReverseWords` implementation

**Issue**: Code reverses characters instead of reversing word order

**Hint**:
```rust
// Wrong: reverses characters
text.chars().rev().collect()

// Right: reverses words
text.split_whitespace()
    .rev()
    .collect::<Vec<_>>()
    .join(" ")
```

## Bug 2: Unnecessary Clones

**Location**: All calls to `process()`

**Issue**: Code calls `.clone()` but references work fine since text is already borrowed

**Hint**:
```rust
// Wrong: clones unnecessarily
let upper = Uppercase.process(&text.clone());

// Right: use reference directly
let upper = Uppercase.process(text);  // or &text
```

## Bug 3: Statistics Calculation

**Location**: Stats section

**Issue**: Doesn't properly compare before/after metrics

**Hint**:
```rust
// Calculate statistics
let words_before = text.split_whitespace().count();
let words_after = upper.split_whitespace().count();

println!("Words: {} -> {} (unchanged)", words_before, words_after);
println!("Chars: {} -> {}", text.len(), upper.len());
```

## Testing

- Process same text with different processors
- Check that original text is unchanged
- Verify reverse words actually reverses
- Count characters correctly

