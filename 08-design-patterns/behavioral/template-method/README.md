# Template Method Pattern

Template Method defines an algorithm's fixed sequence while allowing selected
steps to vary. The example's `DataPipeline::run` always parses and validates,
while each implementation supplies its parsing step.

Rust uses trait default methods instead of an abstract base class. Prefer plain
function composition when the steps do not need a shared trait.

```bash
make run
make check
```

See [key takeaways](key_takeaways.md) and the
[design notes with embedded C4-PlantUML](DESIGN.md).
