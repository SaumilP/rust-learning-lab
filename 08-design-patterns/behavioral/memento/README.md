# Memento Pattern

Memento captures an object's state so it can be restored later without exposing
the object's representation to a caretaker. The example saves and restores a
text editor snapshot.

Use it for undo, checkpoints, and rollback. Large snapshots may require command
logs, diffs, or persistent storage instead of cloning the full state.

```bash
make run
make check
```

See [key takeaways](key_takeaways.md) and the
[design notes with embedded C4-PlantUML](DESIGN.md).
