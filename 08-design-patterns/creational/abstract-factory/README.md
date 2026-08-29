# Abstract Factory Pattern

Abstract Factory creates a family of related products without exposing their
concrete types. The example creates matching buttons and checkboxes for light
and dark themes through the `UiFactory` trait.

Use it when products must be selected as a compatible family. If there is only
one product type, a regular factory is simpler.

```bash
make run
make check
```

See [key takeaways](key_takeaways.md) and the
[design notes with embedded C4-PlantUML](DESIGN.md).
