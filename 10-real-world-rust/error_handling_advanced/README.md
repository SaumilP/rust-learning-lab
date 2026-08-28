# Advanced Error Handling

Errors are part of an API. Preserve useful context, separate recoverable errors
from programming defects, and decide where an error becomes a user-facing
message.

## Library guidance

- Return typed errors callers can inspect.
- Use `source()` to retain the underlying cause.
- Avoid logging and returning the same error at every layer.
- Document failure conditions for public operations.

## Application guidance

- Add context at subsystem boundaries such as file paths or request IDs.
- Convert internal details into concise messages at the outermost boundary.
- Use nonzero exit codes for failed commands.
- Reserve `panic!` for violated invariants and unrecoverable programmer errors.

The `?` operator propagates an error; it does not remove the need to design the
error type or add context.
