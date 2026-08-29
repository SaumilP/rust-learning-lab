# Multiple mutable borrows

The function wants to append two values to one vector. Predict why `broken.rs` fails before compiling it.

Hint: Rust allows one mutable reference or many immutable references to a value at one time.

`broken.rs` creates `second` while `first` is still used later, so the two mutable borrows overlap. The compiler reports E0499. `fixed.rs` limits the first borrow to an inner scope before it creates the second borrow.

An alternative is to use one mutable reference for both pushes. The important part is not the braces themselves; it is that the first exclusive borrow is no longer active.
