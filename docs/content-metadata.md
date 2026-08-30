# Content metadata

Topic metadata gives repository tools and the future website a shared description of each learning topic without copying lesson bodies. Version 1 uses JSON so editors, Rust tooling, CI, and Astro can consume the same source without a format conversion layer.

The [topic authoring guide](authoring/topic-guide.md) explains when to create a topic, how to choose its boundary, and how metadata evidence relates to the learning material. The [content rubric](authoring/content-rubric.md) defines the human review that is required before a topic can become Stable.

The formal contract is [topic-metadata-v1.schema.json](../schemas/topic-metadata-v1.schema.json). A complete Review-status example is available in [topic-metadata.json](examples/topic-metadata.json). Metadata migration is staged: migrate a canonical topic when its learner boundary and currently supported evidence can be described truthfully.

## File placement

Each migrated topic stores its metadata in `metadata.json` beside its learning material. The topic ID is independent of that directory path, so a later repository normalization can move a topic without changing prerequisite links, track membership, or website URLs.

## Required fields

- `schema_version` selects the contract version. Version 1 is the only accepted value.
- `id` is a unique lowercase kebab-case identifier that must remain stable when files move.
- `title` is the learner-facing topic name.
- `status` is one of `planned`, `draft`, `review`, `stable`, or `deprecated` and follows [CONTENT_STATUS.md](../CONTENT_STATUS.md).
- `level` is one of `beginner`, `intermediate`, `advanced`, or `expert`.
- `estimated_minutes` is a positive estimate for completing the topic.
- `prerequisites` contains stable topic IDs. The list may be empty, but a topic cannot require itself. During staged migration, an unpublished topic may reference an intended ID whose metadata has not yet been added; the validator reports that reference as a warning. A published topic must reference metadata that exists within the validated repository scope.
- `concepts` contains one or more lowercase kebab-case concept tags.
- `tracks` contains one or more of `core`, `java`, `python`, `cpp`, or `go`.
- `learning_objectives` contains one or more concrete outcomes stated from the learner's perspective.
- `website` controls publication and ordering without moving or duplicating the source material.
- `validation` records evidence for the content rubric, examples, and exercises as `pending`, `passed`, or `not_applicable`.

The optional `$schema` property lets an editor associate a metadata file with the JSON Schema. It is an editing aid and is omitted from exported topic records.

## Stability and publication rules

A topic cannot use `stable` until its content rubric is `passed` and its example and exercise evidence is either `passed` or `not_applicable`. A topic cannot set `website.published` to `true` unless it is Stable, and every published topic requires a numeric website order. These rules are enforced by the Rust validator as well as represented in the JSON Schema.

Passing metadata validation does not promote a topic. Status changes still require the evidence described in `CONTENT_STATUS.md` and the applicable criteria in the content rubric, and must be reviewed with the content change. The validator checks duplicate IDs across every parsed metadata file, even when one of those files has other semantic errors, so an invalid file cannot hide an identifier collision.

## Validator and export commands

Run the validator against one metadata file while authoring it:

```bash
cargo run --manifest-path tools/contentctl/Cargo.toml -- validate docs/examples/topic-metadata.json
```

After topic metadata exists in the curriculum, validate a directory to find every nested `metadata.json` file:

```bash
cargo run --manifest-path tools/contentctl/Cargo.toml -- validate .
```

The export command validates the same input, sorts topics by stable ID, records each source path, and writes deterministic JSON to standard output:

```bash
cargo run --manifest-path tools/contentctl/Cargo.toml -- export .
```

The website will consume this export boundary in a later phase. Generated exports are build artifacts and should not replace canonical metadata or lesson files in this repository.
