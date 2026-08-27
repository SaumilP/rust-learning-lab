# Hints for Exercise 4: Memory Game

## Bug 1: No Range Validation

**Location**: Input parsing in `get_player_sequence()`

**Issue**: Code accepts any parsed number, should validate it's 1-4

**Hint**:
```rust
if let Ok(num) = input.trim().parse::<u32>() {
    if num >= 1 && num <= 4 {
        player_seq.push(num);
    } else {
        println!("Please enter a number between 1 and 4");
        // Ask again (loop or decrement counter)
    }
}
```

## Bug 2: Invalid Input Skips Position

**Location**: Error handling in input loop

**Issue**: When parse fails, code continues without asking again

**Hint**: Use a loop to keep asking until valid input:
```rust
for i in 0..expected_len {
    loop {
        print!("Number {}: > ", i + 1);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(num) = input.trim().parse::<u32>() {
            if num >= 1 && num <= 4 {
                player_seq.push(num);
                break;  // Exit inner loop, continue outer
            }
        }
        println!("Invalid! Enter 1-4");
    }
}
```

## Bug 3: Only Checks Length, Not Contents

**Location**: Comparison after getting player sequence

**Issue**: Code compares `len()` but should compare actual values

**Hint**:
```rust
// Wrong: only length check
if player_seq.len() != sequence.len() { }

// Right: compare sequences element by element
if player_seq != sequence {
    // Find which position was wrong
    for (i, (&player, &expected)) in
        player_seq.iter().zip(sequence.iter()).enumerate() {
        if player != expected {
            println!("✗ Wrong at position {}!", i + 1);
            break;
        }
    }
}
```

## Testing

- Enter correct sequence
- Enter wrong number at position 1
- Enter wrong number at end
- Enter non-numeric input
- Enter numbers outside 1-4 range

