# Topic authoring guide

This guide describes how to add or revise a learning topic without weakening the curriculum structure. A topic should teach one coherent idea, provide enough evidence for its public status, and remain usable from both the repository and the future website.

## Choose the topic boundary first

A topic needs a clear learner outcome that can be completed and reviewed as one unit. “Use `Result` to represent and propagate recoverable errors” is a topic; “error handling” may be a module containing several topics. Split material when its prerequisites, level, exercises, or validation requirements differ enough that learners should encounter it separately.

Before creating a directory, search the repository for the same concept. Extend the canonical topic when it already exists. Translation tracks should link to canonical Rust explanations and add language-transfer context rather than copying the lesson.

Do not move or rename an existing topic merely to make its current path match a preferred structure. Stable metadata IDs are independent of paths, and structural normalization is handled as a separate reviewed change.

## Recommended topic contents

A complete topic normally contains a `README.md`, focused examples, practice work, and `metadata.json`. Larger topics may add a mental model, key takeaways, or separate concept notes. Include only files that serve the learner; empty scaffolds do not count as completed material.

Runnable examples should be small enough to expose the idea being taught. Add tests when behavior matters, and use compile-fail validation for intentionally broken examples once that harness is available. Do not silence warnings just to pass checks unless the warning itself is part of the lesson.

## Write the metadata

Use the [version 1 metadata contract](../content-metadata.md) and start from the [documented example](../examples/topic-metadata.json). Place the finished file at `metadata.json` in the topic directory.

- Give the topic a lowercase kebab-case `id` that describes the concept rather than its current directory. Treat that ID as permanent once prerequisite links or exports depend on it.
- Choose the public `status` from [CONTENT_STATUS.md](../../CONTENT_STATUS.md) and record only evidence that exists in the repository today.
- Use `prerequisites` for stable topic IDs, not paths or display titles. A prerequisite may be pending migration during the pilot, but it must still use its intended stable ID.
- Keep `concepts` specific enough to support navigation and search. Prefer `move-semantics` over broad tags such as `programming`.
- List every applicable learner `track`, using `core` for the canonical curriculum and language names only when the same topic is intentionally part of those journeys.
- Write concrete `learning_objectives` that describe what a learner can explain, predict, implement, or debug after completing the topic.
- Base `estimated_minutes` on reading, running examples, and completing applicable practice rather than reading time alone.
- Keep `website.published` false until the topic is Stable. Website order controls presentation later and does not determine curriculum correctness.
- Mark rubric, example, and exercise evidence as `passed` only when the corresponding review or automated check has actually passed. Use `pending` for unfinished evidence and `not_applicable` only when the topic deliberately does not need that form of evidence.

Exercise evidence may come from a module-level exercise that is clearly mapped to the topic; it does not require a topic-local `exercises` directory. The absence of that directory is not enough to claim `not_applicable`. Use `pending` when practice is promised, exists without an automated check, or has not yet been mapped and validated.

## Apply status honestly

Planned means the intended topic is visible but substantive learning material or implementation is missing. Draft means useful material exists but known gaps remain. Review means the material is usable and all currently applicable repository checks pass, while the formal rubric may still be pending.

Stable is deliberately difficult to reach. The content rubric must pass, examples and exercises must pass their applicable automated validation or be explicitly not applicable, and the metadata validator must accept the topic. Passing the metadata schema alone never promotes content.

When a status changes, update `CONTENT_STATUS.md` in the same change and state the supporting commands. A failed correctness check or newly discovered teaching problem moves content back to Draft until it is resolved.

## Validate the topic

Validate one metadata file while editing it:

```bash
cargo run --manifest-path tools/contentctl/Cargo.toml -- validate path/to/topic/metadata.json
```

Validate all migrated topics and inspect their deterministic export from the repository root:

```bash
cargo run --manifest-path tools/contentctl/Cargo.toml -- validate .
cargo run --manifest-path tools/contentctl/Cargo.toml -- export .
```

Run the topic's own formatting, lint, compilation, and test commands as well. Use `make check` for modules that provide it and the Cargo commands documented in `CONTRIBUTING.md` for standalone packages. Target-specific, database-backed, and hardware examples must use their documented gates instead of pretending a host-only check proves runtime behavior.

## Keep repository and website responsibilities separate

The learning repository remains the canonical source for topic metadata and lesson bodies. The website consumes a deterministic export and renders that source; it must not maintain a second hand-edited copy of the curriculum. Do not add Astro files, Vercel configuration, generated website records, or private coordination material to a topic directory.

Generated exports are disposable build artifacts. Review changes to source metadata and lessons, then regenerate downstream data when needed.

## Write for people

Use direct explanations, concrete examples, and terminology that matches the learner's level. Remove generic introductions, repetitive summaries, inflated claims, and narration about how the material was produced.

Do not hard-wrap Markdown prose at a fixed column. Keep each paragraph and list item on one physical source line and let the renderer wrap it. Preserve deliberate boundaries in headings, tables, quotations, and code blocks.

Before requesting review, read the rendered page, run the documented commands, confirm every link and prerequisite, and compare the metadata claims with the files that actually exist.
