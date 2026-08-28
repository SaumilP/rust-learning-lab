# Calculator

A small command-line calculator used to practise parsing input, matching on an
operation, and reporting invalid input without panicking.

Run it from the `04-simple-programs` directory:

```bash
cargo run -p calculator
```

Read `src/main.rs` once without changing it, then try these extensions:

1. Add exponentiation and remainder operations.
2. Reject division by zero with a useful message.
3. Move parsing and calculation into separate functions.
4. Add unit tests for successful and invalid calculations.

The important design question is where errors should be handled. Parsing
functions should return a useful result, while the command-line boundary should
decide what message to show the user.
