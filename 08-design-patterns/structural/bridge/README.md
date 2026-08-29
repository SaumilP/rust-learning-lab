# Bridge Pattern

Bridge separates an abstraction from the implementation it controls so both can
vary independently. The example's `Remote` abstraction controls either a
television or a radio through the `Device` trait.

Use it when two dimensions of a design would otherwise produce many combined
types. Generics provide static dispatch; trait objects can select devices at
runtime.

```bash
make run
make check
```

See [key takeaways](key_takeaways.md) and the
[design notes with embedded C4-PlantUML](DESIGN.md).
