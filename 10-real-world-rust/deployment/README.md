# Deployment

A Rust deployment normally ships a release binary plus configuration and any
runtime assets. Build in a reproducible environment and test the artifact, not
only the source checkout.

## Release checklist

- Pin dependencies with `Cargo.lock` for applications.
- Run formatting, Clippy, tests, and documentation checks in CI.
- Build with `cargo build --locked --release`.
- Avoid embedding credentials; inject secrets at runtime.
- Log actionable errors to standard error and return meaningful exit codes.
- Verify the target architecture and required system libraries.
- Produce checksums or signatures for distributed binaries.

Cross-compilation and containers are delivery choices, not substitutes for
testing the final artifact on its target platform.
