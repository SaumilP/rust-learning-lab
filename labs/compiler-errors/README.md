# Compiler error lab

Each lab contains deliberately broken Rust, a small fixed version, and a learner guide. Read `broken.rs`, predict the primary compiler error and its cause, then compare your reasoning with the guide before opening `fixed.rs`.

Run `make check` in this directory to verify that every broken example fails with its recorded Rust error code and every fixed example type-checks. The check uses the local compiler, so diagnostics remain tied to the Rust version used to run it; the lesson explanations describe language rules rather than relying on diagnostic wording.

The initial labs cover a moved value, overlapping mutable borrows, a returned reference to a local value, an unsatisfied trait bound, and sending a non-thread-safe type across a thread boundary.
