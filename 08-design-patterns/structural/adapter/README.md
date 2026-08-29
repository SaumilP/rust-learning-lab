# Adapter Pattern

An adapter makes an existing type usable through the interface expected by new
code. It is useful when the existing type belongs to a legacy system or an
external crate and cannot be changed directly.

## Example

The example application expects a `TemperatureSource` that returns Celsius. A
legacy thermometer returns Fahrenheit. `CelsiusAdapter` owns the thermometer,
converts its value, and implements the new trait.

```text
display_temperature -> TemperatureSource -> CelsiusAdapter -> LegacyThermometer
```

Run it:

```bash
make run
```

Validate it:

```bash
make check
```

## When to use it

- A useful type has the wrong method names, inputs, or outputs.
- You are isolating third-party code from the rest of an application.
- You need a gradual migration from an old interface to a new one.

Do not add an adapter when you control both interfaces and a direct refactor is
smaller. See [DESIGN.md](DESIGN.md) for the C4 component view and
[key_takeaways.md](key_takeaways.md) for a quick review.
