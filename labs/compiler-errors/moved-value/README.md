# Moved value

The program wants to retain a message and also create a second owned value with the same text. Predict why `broken.rs` fails before compiling it.

Hint: assigning a `String` does not copy its heap allocation by default.

`broken.rs` moves ownership from `message` to `stored`, then tries to use the moved binding. The compiler reports E0382. `fixed.rs` uses `clone` to make a deliberate second owned string, so both values can be read afterward.

An alternative is to borrow the string when a second owner is unnecessary. Choose borrowing over cloning when the other code only needs temporary read access.
