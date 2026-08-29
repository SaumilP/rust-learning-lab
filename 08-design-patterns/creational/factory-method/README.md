# Factory Method Pattern

Factory Method defines shared workflow logic while allowing each creator to
choose the concrete helper it uses. In the example, importers share `import`
logic but create different parsers.

Use it when subclasses in an inheritance-based design would vary one creation
step. Rust expresses the creator and product interfaces with traits.

```bash
make run
make check
```

See [key takeaways](key_takeaways.md) and the
[design notes with embedded C4-PlantUML](DESIGN.md).
