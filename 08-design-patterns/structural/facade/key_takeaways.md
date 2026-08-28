# Facade: Key Takeaways

- The facade represents a use case, not another copy of every subsystem method.
- It coordinates ordering, validation, and error handling.
- Callers become less coupled to subsystem types.
- Keep domain rules in the appropriate subsystem; the facade orchestrates them.
- Split a facade when it grows into an unrelated collection of operations.
