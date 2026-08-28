# Facade Pattern

A facade offers a small, task-focused interface over several lower-level
components. Callers use the workflow they need without learning how every
subsystem is coordinated.

## Example

`CheckoutFacade::checkout` reserves inventory, charges a payment, and creates a
shipping label. The caller performs one operation and receives one `Result`.

```text
Client -> CheckoutFacade -> Inventory
                         -> PaymentGateway
                         -> Shipping
```

Run and validate it:

```bash
make run
make check
```

## When to use it

- A common operation always coordinates several services.
- Callers should not depend on subsystem details.
- You want one boundary for validation and error translation.

A facade does not prevent advanced callers from using lower-level APIs when
those APIs must remain public. See [DESIGN.md](DESIGN.md) and
[key_takeaways.md](key_takeaways.md).
