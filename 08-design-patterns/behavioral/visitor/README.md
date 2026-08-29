# Visitor Pattern

Visitor adds operations across a stable family of element types. The example's
shapes implement `accept`, while `AreaVisitor` contains the operation for each
shape.

Use it when element types change rarely but new operations are added often. For
a closed set of simple Rust variants, matching on an enum is usually smaller.

```bash
make run
make check
```

See [key takeaways](key_takeaways.md) and the
[design notes with embedded C4-PlantUML](DESIGN.md).
