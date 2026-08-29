# Iterator: Key Takeaways

- `next` returns `Some(item)` until the sequence is exhausted, then `None`.
- Iterators are lazy unless consumed.
- Standard adapters compose without exposing storage details.
- Implement `IntoIterator` when users should iterate directly over a type.
