# Exercise 2: Dice Roller with Statistics

## Difficulty: Medium
## Concepts Tested: Random Numbers, Console I/O, Game Logic
## Prerequisites: Module 05 concepts

## Problem Statement

Create a dice rolling simulator that:

1. Allows user to specify number of sides and number of rolls
2. Performs the rolls and collects statistics
3. Displays frequency distribution of results
4. Shows summary statistics (min, max, average)
5. Compares actual average to expected theoretical average

## Requirements

- Validate user input (positive integers for sides and rolls)
- Track all roll results
- Calculate frequency for each result
- Display results in clear format
- Calculate min, max, average
- Show theoretical expected value for comparison

## Expected Output

```
How many sides? (default: 6)
> 6

How many rolls? (default: 100)
> 100

Rolling 100 dice with 6 sides...

╔════════════════════════════════╗
║ DICE STATISTICS                ║
║ Total Rolls: 100               ║
╠════════════════════════════════╣
║  1: [████░░░░░░░░░░░░░] 12 (12.0%) ║
║  2: [██████░░░░░░░░░░░░] 18 (18.0%) ║
║  3: [█████░░░░░░░░░░░░░] 15 (15.0%) ║
║  4: [████████░░░░░░░░░░] 24 (24.0%) ║
║  5: [████░░░░░░░░░░░░░░] 12 (12.0%) ║
║  6: [█████████░░░░░░░░░] 19 (19.0%) ║
╚════════════════════════════════╝

╔════════════════════════════════╗
║ STATISTICS SUMMARY             ║
║ Actual Average:     3.57        ║
║ Expected Average:   3.50        ║
║ Min Roll:           1           ║
║ Max Roll:           6           ║
╚════════════════════════════════╝
```

## Hints

1. **Data Structure**: Use a HashMap to track frequency of each result
2. **Statistics Calculation**: Sum all results and divide by count for average
3. **Expected Value**: For a d-sided die, expected value is (sides + 1) / 2
4. **Visual Bar**: Use character repetition to show distribution
5. **Input Validation**: Ensure sides >= 2 and rolls >= 1

## Solution Concepts

- Random number generation
- Collection operations (HashMap)
- Statistics calculation
- Formatted output

