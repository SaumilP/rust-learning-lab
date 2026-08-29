# State Pattern

The State pattern keeps behaviour and valid transitions tied to an object's
current state. It replaces scattered state checks with one explicit transition
model.

## Example

An order moves from `Draft` to `Paid` to `Shipped`. `pay` and `ship` reject
invalid transitions with a `Result`. The example uses an enum because the state
set is small and known at compile time—often the clearest Rust representation.

```text
Draft --pay--> Paid --ship--> Shipped
```

Run and validate it:

```bash
make run
make check
```

## When to use it

- Behaviour depends on a small number of named states.
- Only certain transitions are valid.
- Repeated `if current_state == ...` checks are spreading through the code.

Use trait-backed state objects when each state has substantial independent
behaviour or the state set must be extensible. Consider typestate when invalid
transitions should be impossible to compile. See [DESIGN.md](DESIGN.md) and
[key_takeaways.md](key_takeaways.md).
