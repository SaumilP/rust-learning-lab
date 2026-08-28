# Error Handling

These examples progress from recoverable failures to programmer invariants.
Each file is a standalone program and uses only Rust's standard library.

| Folder | Main idea | Run command |
|---|---|---|
| `custom_errors` | Design typed, descriptive domain errors | `make run EXAMPLE=error_handling/custom_errors/examples/registration_error.rs` |
| `error_propagation` | Pass failures upward with `?` and add context | `make run EXAMPLE=error_handling/error_propagation/examples/propagate_errors.rs` |
| `libraries` | Recognize and handle common standard-library errors | `make run EXAMPLE=error_handling/libraries/examples/standard_errors.rs` |
| `panic_vs_result` | Choose between `Option`, `Result`, and panic | `make run EXAMPLE=error_handling/panic_vs_result/examples/choosing_failure_types.rs` |

From `01-core-fundamentals/`, run `make check` to compile, test, and document
all module examples. Generated files are disposable; remove them with
`make clean` when finished.
