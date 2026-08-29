# Rust Learning Lab Roadmap

This roadmap separates material that is ready to study from work that still
needs attention. It is a repository status document, not a promise that every
directory is complete.

## Current learning path

The main sequence is:

1. `00-setup-and-basics` through `03-tooling-and-quality` for language and tool foundations.
2. `04-simple-programs` and `05-cli-and-console-games` for applied practice.
3. `06-intermediate-rust` for modules, concurrency, async, macros, and deeper type-system topics.
4. `07-advanced-concepts` and `08-design-patterns` for advanced and architectural material.
5. `09-mini-projects` and `10-real-world-rust` for larger programs and production concerns.
6. `11-language-specific-tracks` when translating experience from another language.

See [LEARNING_PATH.md](LEARNING_PATH.md) for the recommended study routine.

## Completed foundations

- Repository structures and topic documentation for modules 01–03
- Runnable examples and exercises for core fundamentals, the standard library,
  and tooling
- Make-based compilation and rustdoc generation for standalone examples in
  modules 01, 02, 03, and 06
- Cargo applications in the simple-program, console-game, design-pattern, and
  mini-project sections
- Language transition guides for Java, Python, Go, and C++ developers
- A public repository baseline: project overview, contribution guidance,
  conduct and security policies, license, and CI coverage

## Outstanding content work

### Consolidate overlapping topic folders

Some early scaffolding remains beside the completed material. Review and either
merge, redirect, or complete these folders so learners do not have to guess
which version is authoritative:

- `01-core-fundamentals/pattern_matching`
- `01-core-fundamentals/structs_and_enums`
- `02-standard-library/generics`
- `02-standard-library/iterators`
- `02-standard-library/smart_pointers`
- `02-standard-library/strings`
- `02-standard-library/traits`
- `03-tooling-and-quality/cargo`
- `03-tooling-and-quality/clippy-and-fmt`
- `03-tooling-and-quality/logging`

### Expand application modules

- Add complete implementations for planned game folders such as Minesweeper,
  Snake, and the ASCII roguelike, or mark them explicitly as design exercises.
- Complete the empty or partial program scaffolds in modules 04 and 09.
- Add tests around input parsing, state transitions, and error paths in the
  interactive applications.

### Improve exercises

- Add more setup-and-basics exercises in module 00.
- Review exercise difficulty and prerequisites across all modules.
- Add optional solution notes that explain trade-offs without making the
  solution the first thing learners see.
- Remove duplicate challenge naming schemes after preserving any unique tasks.

### Strengthen advanced material

- Turn the advanced concept notes into small, reproducible projects where that
  improves understanding.
- Add measurement-driven examples for benchmarking and profiling.
- Add explicit safety contracts to every unsafe Rust and FFI example.
- Clarify the relationship between `06-advanced-functions` and
  `06-intermediate-rust`.

### Curate resources and maintenance guidance

- Populate the resource pages with a small, maintained list of books, videos,
  tools, crates, and articles.
- Add a documented release and maintenance process.
- Add automated checks for internal Markdown links and duplicate scaffolding.

## Completion criteria for a topic

A topic is considered ready when it has:

- A short explanation of what the concept solves and when to use it
- At least one focused example that builds on stable Rust
- A runnable command in its documentation
- Common mistakes or trade-offs where they matter
- An exercise or suggested experiment when practical
- Clean formatting and a warning-free validation command

Progress is best tracked through focused issues and pull requests rather than by
marking broad subject names complete without checking their examples.
