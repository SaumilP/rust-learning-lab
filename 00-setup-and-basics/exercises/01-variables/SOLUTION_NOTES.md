# Solution notes: mutable running total

The first two bindings do not need `mut`: their values are assigned once and then only read. The total changes after it is created, so `let mut total` gives that specific binding permission to change.

`first + second` is evaluated before the result is stored in `total`. The `+= 1` shorthand then updates that same binding. The final `println!` uses Rust's captured-identifier formatting, so `{total}` means “format the value named `total`.”

If you reached `8` with a different expression, compare whether your version still makes the changing value explicit. Keeping the immutable inputs separate from the mutable result is usually easier to read and reason about.
