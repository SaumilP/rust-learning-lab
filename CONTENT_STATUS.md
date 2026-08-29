# Content status

This page gives learners and contributors a quick view of what is usable now
and what still needs work. A status describes the evidence available for an
area; it is not a score for the value or difficulty of the material.

## Status definitions

### Planned

The intended topic or project is visible, but its learning material or runnable
implementation has not been written. Empty directories, zero-byte files, and
README-only project outlines belong here.

A Planned item moves to Draft when it contains substantive material that can be
reviewed, even if that material is incomplete.

### Draft

Substantive documentation or code exists, but there are known gaps, build
failures, missing validation, or unresolved structural questions. Draft content
can still be useful, but its limitations should be read before relying on it.

A Draft item moves to Review when its known blocking issues are addressed and
all applicable repository checks pass.

### Review

The material is usable and its applicable checks currently pass. Review does
not mean that every optional exercise exists or that test coverage is complete.
It means there is enough evidence to review the content against the formal
learning rubric when that rubric is available.

A Review item moves to Stable only after it passes both the applicable content
rubric and automated validation. It returns to Draft if a correctness problem
or validation failure is found.

### Stable

The material has passed the content rubric and automated validation. Those two
requirements are still being developed, so no topic is Stable today.

### Deprecated

The material has been superseded and is no longer recommended as part of the
learning path. A replacement and a clear migration note must exist before this
status is used. No area is currently Deprecated.

## Current overview

| Area | Status | Evidence and remaining work |
|---|---|---|
| `00-setup-and-basics` | Review | Four Cargo packages are checked directly in CI. The module still needs more exercises and the formal content review. |
| `01-core-fundamentals` | Review | Ownership, borrowing, and lifetimes have direct Cargo jobs; the module's standalone topics are covered by `make check`. Overlapping early scaffolds still need normalization. |
| `02-standard-library` | Review | The module's examples are covered by `make check`. Duplicate or scaffold topic names remain to be reconciled. |
| `03-tooling-and-quality` | Review | The module's examples are covered by `make check`. Several older scaffold directories still need a canonical mapping. |
| `04-simple-programs` | Review | CI runs the workspace and standalone-example checks. Todo and rfind need stronger behavioral tests, and some physical project directories remain empty. |
| `05-cli-and-console-games` | Review | CI covers the Cargo workspace, the separate randomness package, and standalone examples. Game test depth varies, while Minesweeper, Snake, and the ASCII roguelike remain planned. |
| `06-intermediate-rust` | Review | This is the canonical Module 06 candidate and is covered by `make check`. It still awaits the formal content rubric. |
| `06-advanced-functions` | Draft | This supplementary set overlaps Module 06, is not in CI, and needs a normalization decision before publication as part of the main path. |
| `07-advanced-concepts` | Draft | The module is not in CI. Axum, SQLx, embedded configuration, and procedural-macro examples have confirmed failures; benchmark, SIMD, and WebAssembly checks still need network-enabled validation. |
| `08-design-patterns` | Review | CI runs formatting, lint, workspace tests, standalone compilation, and design-document checks for all 23 patterns. Behavioral test coverage remains thin. |
| `09-mini-projects` | Mixed | Chat, HTTP server, key-value store, and load balancer are Review and covered as one CI workspace. Calculator, todo, note taker, and weather CLI are Planned documentation-only projects. |
| `10-real-world-rust` | Planned | Topic notes and a Makefile exist, but the five planned topic implementations do not. |
| `11-language-specific-tracks` | Mixed | The Java, Python, Go, and C++ overviews are Review. The four existing Java/Python Cargo projects are Draft because they are not in CI; Go and C++ projects remain planned. |
| `challenges` | Draft | Twenty-three Markdown files contain kata, beginner, interview, and level-guide material. There is no automated validation, and duplicate directory naming must be resolved without losing unique questions. |
| `resources` | Planned | The five tracked resource pages exist but are empty. They need a small, maintained set of recommendations. |

## Changing a status

A status change should be made in the same pull request as the evidence that
supports it. Include the relevant build or validation command, describe any
remaining gaps, and update this table if the status of a major area changes.

Passing CI is necessary where checks apply, but it is not enough for Stable.
Stable always requires both the content rubric and automated validation.
