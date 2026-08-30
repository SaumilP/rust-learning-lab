# Contributing to Rust Learning Lab

Contributions that improve correctness, clarity, or the learning sequence are
welcome. A small correction with a clear explanation is just as useful as a new
example.

By participating, you agree to follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## Before starting

Check existing issues and pull requests to avoid duplicating active work. For a
larger topic or structural change, open an issue first and describe:

- the learner problem you want to solve;
- where the material belongs in the learning path;
- the examples or exercises you expect to add or change.

## Educational guidelines

Content should be technically correct on stable Rust and understandable at the
level where it appears. Prefer a direct implementation over a clever one when
the direct version makes the concept easier to see.

When adding a topic:

- explain what problem it solves before introducing syntax;
- keep each example focused on one main idea;
- include the exact command needed to run it;
- explain important ownership, error-handling, or performance trade-offs;
- use comments to explain reasoning, not to repeat the code;
- distinguish deliberately broken exercises from runnable examples.

Avoid unnecessary dependencies. If a crate is needed, explain what it provides
and why the standard library is not enough for that example.

## Repository structure

Most concept topics use this layout:

```text
topic/
├── README.md
├── key_takeaways.md
├── examples/
└── exercises/        # when appropriate
```

Larger applications use Cargo packages under `src/`. Follow the surrounding
module's layout rather than moving files solely for consistency.

## Local validation

For standalone examples in modules with a Makefile:

```bash
cd 01-core-fundamentals
make check
make clean
```

For a Cargo package or workspace:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
```

Run Cargo commands from the directory containing the relevant `Cargo.toml`.
The repository intentionally has no root Cargo workspace. Do not commit
generated `build/`, `target/`, or rustdoc files.

CI checks local links in every Markdown file with lychee in offline mode. If lychee is installed locally, run `lychee --offline --root-dir . --no-progress './**/*.md'` from the repository root before submitting documentation changes.

Topics that contain `metadata.json` must follow the [content metadata contract](docs/content-metadata.md). Run `cargo run --manifest-path tools/contentctl/Cargo.toml -- validate <path>` against a metadata file or directory before submitting it.

Use the [topic authoring guide](docs/authoring/topic-guide.md) when adding or substantially revising learning material. It covers topic boundaries, status evidence, validation, and the repository-to-website publishing boundary.

Exercises containing `broken_code.rs` are allowed to fail before the learner
repairs them and are excluded from normal build checks.

## Pull requests

Create a focused branch and use a descriptive commit message. A pull request
should explain what changed, why it helps learners, how it was verified, and any
known follow-up work. Include terminal output only when it helps reviewers
understand a failure or behaviour change.

Reviewers will primarily consider:

- technical correctness;
- fit with the learning sequence;
- clarity for the intended audience;
- idiomatic Rust without unexplained complexity;
- reproducible build and test instructions.

## Reporting problems

Use the issue template for incorrect explanations, build failures, learning-flow
problems, or content proposals. Security concerns should not be posted in a
public issue; follow [SECURITY.md](SECURITY.md) instead.

When proposing a content change, check the [content status page](CONTENT_STATUS.md) for the area's known gaps and the evidence required to change its status.
