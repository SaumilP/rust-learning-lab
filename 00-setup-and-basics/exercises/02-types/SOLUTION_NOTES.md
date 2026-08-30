# Solution notes: convert a temperature

The formula is `(fahrenheit - 32) * 5 / 9`. This solution writes the constants as `32.0`, `5.0`, and `9.0` so the whole calculation uses `f64` floating-point values rather than integer arithmetic.

The function signature documents both sides of the conversion: it accepts an `f64` and returns an `f64`. That lets the caller reuse the conversion instead of tying it to one printed example.

`println!("{:.1}", ...)` displays one decimal place, which makes the expected output `20.0`. Formatting affects how the result is shown; it does not change the value returned by `to_celsius`.
