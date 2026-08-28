# Ownership and Borrowing: Key Takeaways

- Each value has one owner.
- Assignment moves non-`Copy` values unless they are cloned.
- `&T` is a shared, read-only borrow.
- `&mut T` is an exclusive, mutable borrow.
- Prefer borrowing function parameters when ownership is not required.
- Slices such as `&str` and `&[T]` borrow part or all of another value.
