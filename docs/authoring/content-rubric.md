# Content rubric

This rubric is the human review required before a topic can become Stable. It is deliberately separate from `contentctl`: automation can prove that declared files and metadata are internally consistent, but it cannot decide whether an explanation is accurate, proportionate, or useful to a learner.

## Use the rubric

Review the topic in its rendered form and run every command that applies to its examples and exercises. Record `validation.content_rubric` as `passed` only when every applicable criterion below passes and the reviewer can point to the supporting material in the same change. Until then, keep it `pending`; do not promote a topic because its metadata parses or one example compiles.

## Required criteria

- Scope: The title, objectives, prerequisites, level, and estimated time describe the material the learner actually receives.
- Explanation: The lesson states the mental model before relying on terminology, distinguishes closely related concepts, and does not make claims that its examples contradict.
- Examples: Each example is focused on the stated objective, is runnable or has a documented environment-specific gate, and has been checked with that gate.
- Practice: Applicable practice has a clear learner task and a verifiable result. If practice is deliberately not applicable, the topic explains why; a missing exercise is not a reason to mark it not applicable.
- Navigation: Links resolve, prerequisites are truthful, and canonical material is linked instead of copied into transition tracks.
- Safety and maintenance: Commands do not require undisclosed credentials or destructive setup, and the material avoids version-specific claims unless they are documented.
- Voice: The page uses direct learner-facing language, does not overclaim completeness, and keeps Markdown prose unwrapped in source.

## Automation boundary

`contentctl validate .` checks metadata shape, stable IDs, prerequisite rules, and the prohibition on publishing non-Stable material. Module and package commands supply the executable evidence named by `validation.examples` and `validation.exercises`. A successful run of those checks is necessary evidence, but the rubric review is still required before `status` can be `stable`.

## Evidence record

In a content change, include the commands run and the reviewed files in the pull request description or other project-approved review record. Do not add private agent logs or working notes to this repository. The metadata should report only the resulting evidence state, not a claim that a future reviewer still needs to verify.
