# State: Key Takeaways

- Give each state a meaningful name.
- Keep transition rules close to the state representation.
- Return errors for invalid runtime transitions instead of panicking.
- Prefer an enum for a closed, small state set.
- Consider trait objects for complex state-specific behaviour and typestate for compile-time transition guarantees.
