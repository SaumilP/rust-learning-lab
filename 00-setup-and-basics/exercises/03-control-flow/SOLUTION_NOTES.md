# Solution notes: classify a number

An `if` in Rust is an expression, so each branch can produce the string returned by `classify`. The branches are ordered from negative to zero to the remaining case; after the first two checks fail, the number must be positive.

The return type is `&'static str` because every branch returns a string literal. Those literals are embedded in the program and remain available for the whole execution, so the function can return a reference to one without allocating a new `String`.

This is also a useful place to distinguish `=` from `==`: `number == 0` compares two values, while `=` would assign a value and is not the condition this function needs.
