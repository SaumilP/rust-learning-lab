# Solution notes: return the larger value

The `if` expression is the final expression in `larger`, so its selected branch becomes the function's return value. No `return` keyword or trailing semicolon is needed here.

When `left` is greater than `right`, the first branch produces `left`; otherwise the second branch produces `right`. For equal inputs, this implementation returns `right`, but the numeric result is the same either way.

Keeping `println!` in `main` separates calculation from presentation. That makes `larger` easier to test and reuse in code that needs the value without writing to the terminal.
