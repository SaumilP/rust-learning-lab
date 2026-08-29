# Prototype Pattern

Prototype creates a new value by copying an existing configured value. Rust's
`Clone` trait is the direct tool for this pattern. The example clones a report
template and changes only the new report's title.

Use it when building a baseline value is expensive or verbose and copies should
start with the same configuration.

```bash
make run
make check
```

See [key takeaways](key_takeaways.md) and the
[design notes with embedded C4-PlantUML](DESIGN.md).
