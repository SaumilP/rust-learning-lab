# Flyweight Pattern

Flyweight shares immutable data that would otherwise be repeated across many
small objects. The example stores each glyph's character and position directly
while sharing one `TextStyle` through `Rc`.

Use it only after measurement shows repeated intrinsic data has a meaningful
memory cost. Use `Arc` rather than `Rc` when values cross thread boundaries.

```bash
make run
make check
```

See [key takeaways](key_takeaways.md) and the
[design notes with embedded C4-PlantUML](DESIGN.md).
