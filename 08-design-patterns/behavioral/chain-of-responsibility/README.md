# Chain of Responsibility Pattern

Chain of Responsibility sends a request through ordered handlers until one
produces a result. The example applies authentication, rate-limit, and
application handlers to a request.

Use it for ordered processing stages whose membership may change. Be explicit
about whether a handler stops the chain or allows processing to continue.

```bash
make run
make check
```

See [key takeaways](key_takeaways.md) and the
[design notes with embedded C4-PlantUML](DESIGN.md).
