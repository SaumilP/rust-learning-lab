# Visitor: Key Takeaways

- Elements dispatch to the matching visitor method.
- New visitors add operations without changing element behaviour.
- Adding a new element type requires updating every visitor.
- Prefer exhaustive enum matching for small closed type families.
