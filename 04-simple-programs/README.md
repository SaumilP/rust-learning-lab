# Simple Programs

This module turns the earlier language concepts into small command-line
programs. The examples focus on input parsing, text processing, file I/O,
algorithms, and recoverable errors.

## Contents

| Topic or project | Main idea | How to run |
|---|---|---|
| `cli_arguments` | Read and validate command-line arguments | Compile the files in `examples/` with `rustc` |
| `file_io_basics` | Read and write files with `std::fs` | Compile the files in `examples/` with `rustc` |
| `simple_algorithms` | Implement searching and sorting directly | Compile the files in `examples/` with `rustc` |
| `text_processing` | Transform and inspect strings | Compile the files in `examples/` with `rustc` |
| `calculator` | Parse operands and dispatch operations | `cargo run -p calculator` |
| `rfind` | Search text from the command line | `cargo run -p rfind -- --help` |
| `todo_app` | Persist simple task data | `cargo run -p todo_app` |
| `exercises` | Repair and extend small programs | Follow each exercise README |

The Cargo commands above should be run from this directory.

## Validate the module

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The standalone examples are also checked by the repository's validation script.
Generated Cargo artifacts are written to `target/` and can be removed with
`cargo clean`.

## Suggested order

Start with command-line arguments and text processing, then work through file
I/O and algorithms. Use the calculator before the todo application; the todo
application combines parsing, collections, file persistence, and error
handling.
