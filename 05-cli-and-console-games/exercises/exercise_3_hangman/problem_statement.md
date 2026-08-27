# Exercise 3: Hangman Word Game

## Difficulty: Medium
## Concepts Tested: Game Loop, Console I/O, Game Logic, String Operations
## Prerequisites: Module 05 concepts

## Problem Statement

Create a Hangman game where:

1. Computer picks a word from a list
2. Player guesses letters one at a time
3. Display progress of guessed letters
4. Track remaining guesses (typically 6)
5. Win if all letters are guessed
6. Lose if guesses run out
7. Display final score

## Requirements

- Have a list of words to choose from
- Display blanks for unguessed letters and guessed letters
- Track wrong guesses
- Prevent duplicate guesses
- Clear game-over messages
- Calculate score based on remaining guesses

## Expected Output

```
╔════════════════════════════════╗
║ HANGMAN GAME                   ║
╚════════════════════════════════╝

Word: _ _ _ _ _ _ _
Guessed letters:
Wrong guesses: 0/6

Guess a letter: > e
Word: _ _ _ _ _ _ e
Guessed letters: e
Wrong guesses: 0/6

Guess a letter: > a
Word: _ a _ _ a _ e
Guessed letters: a e
Wrong guesses: 0/6

...more guesses...

Word: e x a m p l e
Guessed letters: a e l m p x
Wrong guesses: 1/6

╔════════════════════════════════╗
║ YOU WIN!                       ║
║ Word: example                  ║
║ Score: 50 points               ║
╚════════════════════════════════╝
```

## Hints

1. **Word Selection**: Use a predefined list of words (sample provided)
2. **Letter Tracking**: Use String/Vec to track guessed and wrong letters
3. **Display Progress**: Show blanks with spaces between
4. **Duplicate Check**: Check if letter already guessed
5. **Win/Lose Condition**: Check if all letters guessed or guesses = 0

## Solution Concepts

- Game loop management
- String operations (chars, contains)
- State tracking (guessed letters, wrong count)
- User input validation

