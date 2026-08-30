# Building and maintaining programs concept index

This index connects language knowledge to the routine work of building a useful program. Follow the module sequence for a full learning experience; use these links when a project exposes a gap in your testing, diagnostics, I/O, or concurrency knowledge.

| Task or concept | Start here | Useful next step |
| --- | --- | --- |
| Run checks, format code, and understand the development toolchain | [Tooling and quality](../../03-tooling-and-quality/README.md) | [Testing](../../03-tooling-and-quality/testing/README.md) |
| Write and run unit, integration, or documentation tests | [Testing](../../03-tooling-and-quality/testing/README.md) | [Documentation](../../03-tooling-and-quality/documentation/README.md) |
| Investigate a failing program and use compiler feedback | [Debugging](../../03-tooling-and-quality/debugging/README.md) | [Compiler-error labs](../../labs/compiler-errors/README.md) |
| Explain a public API or maintain project documentation | [Documentation](../../03-tooling-and-quality/documentation/README.md) | [Project organization](../../10-real-world-rust/project_organization/README.md) |
| Read command-line arguments and build a small CLI | [CLI arguments](../../04-simple-programs/cli_arguments/README.md) | [Todo CLI project](../../09-mini-projects/todo_cli/README.md) |
| Read, write, and transform files | [File I/O basics](../../04-simple-programs/file_io_basics/README.md) | [Text processing](../../04-simple-programs/text_processing/README.md) |
| Organize code as an application with multiple responsibilities | [Mini projects](../../09-mini-projects/README.md) | [Project organization](../../10-real-world-rust/project_organization/README.md) |
| Coordinate work across threads and understand shared-state constraints | [Concurrency introduction](../../10-real-world-rust/concurrency_intro/README.md) | [Advanced async](../../07-advanced-concepts/advanced_async/README.md) |
| Practice explaining a design or reviewing a small change | [Code-review challenges](../../challenges/code-review/README.md) | [Interview challenges](../../challenges/interview/README.md) |

## Use the compiler as part of the workflow

Run a small check after each focused change, not only at the end of a project. When an error is unfamiliar, reduce the program to the smallest example that still fails, predict what ownership or type rule applies, and then consult the linked lesson. The compiler-error labs are deliberately broken examples designed for that kind of practice.
