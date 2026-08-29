# Mediator Pattern

Mediator centralizes communication between components that should not depend on
one another directly. The example's chat room knows the member list and routes a
message without the sender knowing every recipient.

Use it when pairwise dependencies are growing faster than the collaboration
rules. Avoid turning the mediator into a large object containing unrelated
business logic.

```bash
make run
make check
```

See [key takeaways](key_takeaways.md) and the
[design notes with embedded C4-PlantUML](DESIGN.md).
