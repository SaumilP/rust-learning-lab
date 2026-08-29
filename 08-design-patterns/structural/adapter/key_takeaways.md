# Adapter: Key Takeaways

- The client depends on the target trait, not the legacy type.
- The adapter owns or borrows the adapted value.
- Conversion logic belongs in the adapter rather than in every client.
- Rust traits express the interface the client expects.
- Composition is usually clearer than trying to mimic class inheritance.

In this example, the only responsibility of `CelsiusAdapter` is converting a
Fahrenheit reading into Celsius.
