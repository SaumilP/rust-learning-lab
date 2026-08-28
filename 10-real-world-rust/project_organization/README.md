# Project Organization

Small Rust programs often begin in one file. As behavior grows, split code by
responsibility rather than by type: configuration, domain logic, persistence,
and interfaces are useful boundaries.

## Practical rules

- Keep `main.rs` focused on wiring dependencies and reporting errors.
- Put reusable behavior in `lib.rs` so integration tests can import it.
- Make items private by default and expose the smallest useful API.
- Use a Cargo workspace only when packages need separate dependency graphs or
  release cycles.
- Keep features additive; a feature should not silently remove core behavior.

```text
src/
├── main.rs       # application entry point
├── lib.rs        # public library surface
├── config.rs     # input and configuration
└── service.rs    # domain behavior
```

Start simple and extract modules when a stable boundary becomes visible.
