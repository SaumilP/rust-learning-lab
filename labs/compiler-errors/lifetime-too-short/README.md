# Reference to a local value

The function wants to return a greeting by reference. Predict why `broken.rs` fails before compiling it.

Hint: the returned reference must remain valid after the function has returned.

`message` is dropped when `greeting` ends, so a reference to it cannot safely escape. The compiler reports E0515. `fixed.rs` returns a string literal, whose `static` lifetime is sufficient for the returned reference.

An alternative is to return an owned `String` when the greeting must be constructed during the function call. Do not add a lifetime annotation to a reference unless the referenced data actually lives long enough.
