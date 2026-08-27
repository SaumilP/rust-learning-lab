# Exercise 4: Memory Sequence Game

## Difficulty: Hard
## Concepts Tested: Game Loop, Random Numbers, Console I/O, Game Logic
## Prerequisites: Module 05 concepts

## Problem Statement

Create a memory game where:

1. Computer generates a sequence of random numbers
2. Display sequence to player (briefly)
3. Player must enter the sequence from memory
4. Sequence grows longer each round
5. Game ends when player makes a mistake
6. Display score (level reached)

## Requirements

- Generate random sequence of 1-4 (4 options)
- Display sequence with timing (0.5s per number)
- Clear screen between display and input
- Validate player input
- Track which level player reached
- Show encouraging/discouraging messages

## Expected Output

```
╔════════════════════════════════╗
║ MEMORY SEQUENCE GAME           ║
║ Level 1                        ║
╚════════════════════════════════╝

Sequence: 3 2

Memorize the sequence...
(clearing screen)

Enter the sequence (1-4): > 3
Enter the sequence (1-4): > 2

✓ Correct! Level Complete!

╔════════════════════════════════╗
║ MEMORY SEQUENCE GAME           ║
║ Level 2                        ║
╚════════════════════════════════╝

Sequence: 3 2 1

(process repeats)

Sequence was: 3 2 1 4
You entered:  3 2 4
✗ Wrong at position 3!

╔════════════════════════════════╗
║ GAME OVER                      ║
║ Final Level: 3                 ║
║ Score: 30 points               ║
╚════════════════════════════════╝
```

## Hints

1. **Sequence Storage**: Use Vec<u32> to store the sequence
2. **Random Generation**: Generate numbers 1-4 (4 options)
3. **Display Timing**: Display each number for 0.5s with 0.3s pause
4. **Input Collection**: Loop to get each guess from player
5. **Validation**: Compare player sequence with game sequence

## Solution Concepts

- Game state and loop management
- Collections (Vec for sequence)
- Input validation and comparison
- Timing and display control

