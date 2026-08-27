# Hints for Exercise 4: Nested Loop Pattern Generation

## Stuck? Here are some hints:

### About the bugs:

**Bug 1: Right Triangle Loop Range**
- Look at `print_right_triangle`: `for i in 1..size`
- The range `1..size` is exclusive at the end, so it goes 1, 2, 3, 4 (stops before 5)
- You need 5 rows for size 5, but you only get 4
- Also, it starts at 1, but the first row should have 1 star (correct), second row 2 stars, etc.
- Fix: Use `for i in 1..=size` to make it inclusive
- Alternative: `for i in 0..size` then adjust inner loop to `for j in 0..=i`
- Test: For size 5, you should print exactly 5 rows

**Bug 2: Hollow Square Logic**
- Actually, the condition `if i == 0 || i == size - 1 || j == 0 || j == size - 1` IS correct
- It prints '*' on all border positions and ' ' inside
- The real issue might be in spacing - you need spaces inside, not empty
- The current code should work, but check:
  - Does `i` range from 0 to size-1? Yes ✓
  - Does `j` range from 0 to size-1? Yes ✓
  - Are border conditions correct? Yes ✓
- The hollow square code should actually be correct! But if it fails, verify spacing calculation

**Bug 3: Diamond Spacing Calculation**
- Look at diamond pattern: the top row has 4 spaces + 1 star
- Row 2 has 3 spaces + 3 stars
- Row 3 has 2 spaces + 5 stars
- Row 4 has 1 space + 7 stars
- Row 5 (middle) has 0 spaces + 9 stars
- For size=5, half=2
- Current code: `let spaces = i;` gives 0, 1, 2, 3, 4 spaces
- Should be: `let spaces = half - i` or `let spaces = size / 2 - i`
- Example: i=0 → spaces should be 2 (not 0)
- Example: i=2 → spaces should be 0 (middle row)
- Fix: `let spaces = half - i;` for the upper half
- For lower half, the code already has correct logic

### Testing your fix:

After fixing the bugs, run:
```bash
cargo run
```

You should see:
- Triangle: 5 rows with 1, 2, 3, 4, 5 asterisks
- Square: 5x5 with borders and hollow interior
- Diamond: 9 rows (5 expanding + 4 contracting) with proper spacing

### Debugging tips:

1. Count the output lines - should match expected pattern
2. Count characters per line - verify spacing is correct
3. For triangle: line i should have i asterisks
4. For square: first/last rows all *, middle rows have * at ends only
5. For diamond: top and bottom rows closest together, middle row widest

### Key concepts to remember:

- **Inclusive ranges**: Use `..=` to include the end value
- **Exclusive ranges**: Use `..` to exclude the end value
- **Nested loops**: Outer loop controls major structure, inner controls details
- **Conditions in loops**: Use `if` to selectively print based on position
- **String building**: Use `String::new()` and `push_str()` for dynamic strings
- **Spacing calculations**: Mathematical formulas based on loop index
- **Pattern symmetry**: Diamond mirrors around center

### Loop range checklist:

- Right triangle: Need exactly `size` rows → use `1..=size` or `0..size` with adjustments
- Hollow square: Need `size` rows and `size` columns → `0..size` for both
- Diamond: Need `2*size - 1` rows total → `size` for upper, `size-1` for lower

### Alternative loop ranges to try:

```rust
// Right triangle options:
for i in 0..size { inner 0..=i }  // 0 stars, 1 star, 2 stars, 3 stars, 4 stars (wrong)
for i in 1..=size { inner 0..=i } // 1 star, 2 stars, ... 5 stars (correct!)
for i in 0..size { inner 1..=i+1} // 1 star, 2 stars, ... 5 stars (correct!)
```

### If still stuck:

1. **Triangle**: Use `1..=size` - simplest fix
2. **Square**: Verify the condition produces correct border/space pattern
3. **Diamond**: Calculate spaces from the middle point, not from loop start
4. **General**: Print intermediate values to debug: `println!("i={}, spaces={}", i, spaces);`

The fixes are usually 1-3 line changes per function!

### Related concepts:

- **Off-by-one errors**: Common in loops - verify ranges carefully
- **Accumulating strings**: Building output incrementally in loops
- **Coordinate systems**: Thinking of position (row, column) for 2D patterns
- **Mathematical patterns**: Using formulas to generate shapes

