# Proxy Pattern

A proxy implements the same interface as another object and controls access to
it. The example wraps a database-like store and caches successful user lookups.

Other proxies may enforce authorization, perform lazy initialization, record
metrics, or represent a remote service. Keep proxy behaviour visible because it
can change latency and failure characteristics.

```bash
make run
make check
```

See [key takeaways](key_takeaways.md) and the
[design notes with embedded C4-PlantUML](DESIGN.md).
