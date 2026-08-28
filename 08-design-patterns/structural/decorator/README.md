# Decorator Pattern

A decorator wraps a value that implements an interface and implements the same
interface itself. Each wrapper adds one behaviour, and wrappers can be stacked
without changing the original component.

## Example

`PlainNotifier` returns a message unchanged. `Urgent` adds a prefix and
`WithSignature` adds a sender. Both decorators implement `Notifier`, so the
client treats the final stack as one notifier.

```text
WithSignature -> Urgent -> PlainNotifier
```

Run and validate it:

```bash
make run
make check
```

## When to use it

- Features need to be combined in several arrangements.
- Subclass-style inheritance would create many combinations.
- Each added behaviour can remain small and independent.

Middleware and I/O wrappers often use this shape in Rust. If the set of
combinations is fixed and small, an enum or regular function may be simpler.
See [DESIGN.md](DESIGN.md) and [key_takeaways.md](key_takeaways.md).
