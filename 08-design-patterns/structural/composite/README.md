# Composite Pattern

Composite represents individual objects and groups with the same operation. The
example models both files and directories as `FileNode`; asking either for its
size uses the same method.

Rust enums are a natural fit for closed tree structures. Use trait objects when
new node types must be added outside the defining crate.

```bash
make run
make check
```

See [key takeaways](key_takeaways.md) and the
[design notes with embedded C4-PlantUML](DESIGN.md).
