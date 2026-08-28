# Exercise 4: Borrow Checker and Lifetimes

Repair `choose_label` so it returns one of the caller-owned string slices rather
than a reference to temporary local data. Add the lifetime relationship needed
by the return type.

Do not use `String`, cloning, leaking, or `'static` to bypass the exercise.
