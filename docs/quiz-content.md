# Quiz content

Quiz items are small reasoning exercises, not trivia cards. They live in `challenges/quiz-items/` as one JSON object per file and follow [quiz-item-v1.schema.json](../schemas/quiz-item-v1.schema.json).

Use `cargo run --manifest-path tools/contentctl/Cargo.toml -- validate-quiz challenges/quiz-items` to validate every item. The validator checks the JSON shape, stable IDs, duplicate IDs, required explanations and references, and the answer format required by each question type.

## Item contract

Every item has a stable `id`, a canonical-topic `topic`, a difficulty, concept tags, the learner-facing question, a correct answer, an explanation, and at least one repository reference. `code` is required for `will_it_compile`, `predict_output`, `code_review`, and `fix_error` questions. Multiple-choice and design-choice questions need at least two choices, and their answer must name one of those choices.

The six supported question types are `multiple_choice`, `will_it_compile`, `predict_output`, `code_review`, `fix_error`, and `design_choice`. A question should reveal a useful misconception or decision point, then explain the reasoning in language a learner can act on.

## Authoring guidance

Keep the prompt self-contained. Show only the code needed to reason about the question, avoid unstated assumptions, and use a reference that lets the learner continue studying the concept. Do not publish a question merely because its JSON validates: the explanation must accurately describe Rust’s behaviour.

For code that is intended to compile or fail, verify the claim with the Rust toolchain whenever practical. The initial foundation set contains both compile-behaviour questions and conceptual questions; it does not make claims about a remote runner or a public quiz interface.
