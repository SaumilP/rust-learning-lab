# Memento: Key Takeaways

- The originator creates and restores its own snapshots.
- Keep snapshot internals private from the caretaker.
- `Clone` is sufficient for small in-memory state.
- Set limits or use incremental history for large state.
