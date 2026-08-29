# Iterator Pattern

Iterator provides sequential access without exposing a collection's storage.
Rust standardizes the pattern with the `Iterator` trait. The example implements
a small countdown and then uses the normal `collect` adapter.

Implement `Iterator` when a type naturally produces a sequence. Consumers then
gain `map`, `filter`, `take`, `fold`, and other standard adapters.

```bash
make run
make check
```

See [key takeaways](key_takeaways.md) and the
[design notes with embedded C4-PlantUML](DESIGN.md).
