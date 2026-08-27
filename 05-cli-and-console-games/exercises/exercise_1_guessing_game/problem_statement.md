# Exercise 1: Higher/Lower Guessing Game

## Difficulty: Easy
## Concepts Tested: Game Loop Basics, Random Numbers, Console I/O, Simple Game Logic
## Prerequisites: Module 05 concepts

## Problem Statement

Create a number guessing game where:

1. The game picks a random number between 1 and 100
2. The player makes guesses and receives feedback ("higher", "lower", or "correct")
3. The game tracks the number of guesses
4. The game continues until the player guesses correctly
5. After winning, display the final score (based on number of guesses)

## Requirements

- Use a game loop structure (input → update → render cycle)
- Generate a random number using appropriate seeding (or simple random)
- Validate user input (must be a number between 1-100)
- Display clear feedback after each guess
- Calculate and display final score
- Handle invalid input gracefully

## Expected Output

```
╔════════════════════════════════╗
║ NUMBER GUESSING GAME           ║
║ Guess a number (1-100)         ║
╚════════════════════════════════╝

Guesses: 0
> 50
Too low! Try higher.

Guesses: 1
> 75
Too high! Try lower.

Guesses: 2
> 62
Too low! Try higher.

Guesses: 3
> 68
Too high! Try lower.

Guesses: 4
> 65
Correct! You found it!

╔════════════════════════════════╗
║ GAME OVER                      ║
║ Final Score: 96 points         ║
║ Guesses: 4                     ║
║ Efficiency: 96%                ║
╚════════════════════════════════╝
```

## Hints

1. **Random Number**: Use a simple method to generate random numbers. The `rand` crate is helpful, but you can also use `SystemTime` for a simple approach.

2. **Game Loop**: Structure your game with:
   - Input phase: Read user guess
   - Update phase: Check if guess is correct
   - Render phase: Display feedback

3. **Scoring**: Calculate score as `100 - (guesses * 2)` with a minimum of 10 points

4. **Input Validation**: Check that input is a valid number and in the range 1-100. Display error message if not.

5. **State Management**: Track player's number of guesses and whether game is still running using variables or a struct

## Testing

Test your solution with:
- Guessing the number correctly first try (max score)
- Taking many guesses (lower score)
- Entering invalid input (should be handled gracefully)
- Entering numbers outside range (should be rejected)

## Solution Concepts

This exercise combines:
- **Game Loop**: Main loop that continues until game ends
- **Random Numbers**: Generating the secret number
- **Console I/O**: Reading input, displaying output
- **Game Logic**: Comparing guesses, tracking state

