# Hints for Exercise 3: Hangman

## Bug 1: Always First Word

**Location**: Word selection line

**Issue**: Code uses `words[0]` which always picks the first word

**Hint**: Use random selection:
```rust
use std::time::SystemTime;

let seed = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_nanos() as usize;

let word = words[seed % words.len()];
```

## Bug 2: Duplicate Guesses Allowed

**Location**: After getting user input

**Issue**: Code doesn't check if letter was already guessed

**Hint**: Add check before adding to guessed_letters:
```rust
if guessed_letters.contains(guess) {
    println!("You already guessed that letter!");
    continue;  // Skip this turn
}

guessed_letters.push(guess);
```

## Bug 3: Wrong Letters Shown as Correct

**Location**: Logic for wrong vs correct guesses

**Issue**: Wrong guesses are added to `guessed_letters`, so they appear as guessed correctly

**Hint**: Separate tracking for wrong guesses:
```rust
let mut guessed_letters = String::new();
let mut wrong_letters = String::new();

// When guess is wrong:
if word.contains(guess) {
    guessed_letters.push(guess);  // Right guess
} else {
    wrong_letters.push(guess);     // Wrong guess
    wrong_guesses += 1;
}

// Display only correct guesses
println!("Guessed letters: {}", guessed_letters);
```

## Testing

- Guess correct letters progressively
- Try guessing same letter twice
- Enter invalid input (non-letter)
- Win by guessing all letters
- Lose by running out of guesses

