# Challenge scoring

Challenge scores are feedback, not a measure of someone’s potential as a Rust developer. They show whether the reasoning needed for a particular prompt is present and help a learner decide what to revisit next.

## Shared rules

Give credit for a correct conclusion and for the reasoning that supports it. A lucky answer without an explanation is weaker evidence than a well-reasoned answer with a small mistake. Do not deduct points for using different, idiomatic wording when the underlying Rust rule is correct.

For any code-based challenge, judge the current toolchain behaviour described by the item or lab. If a diagnostic changes wording across Rust releases, score the learner’s explanation of the language rule rather than exact diagnostic text.

## Short structured questions

| Challenge type | Full-credit evidence | Score |
| --- | --- | --- |
| Multiple choice or design choice | Correct choice and a reason that identifies the relevant trade-off | 2 points |
| Will It Compile? | Correct prediction and the ownership, borrowing, type, trait, or lifetime rule that decides it | 2 points |
| Predict output | Correct output and an explanation of the evaluated expressions or control flow | 2 points |
| Fix error | A compiling, proportionate fix and an explanation of the original rule violation | 2 points |

Award one point when the conclusion is correct but the explanation is missing or materially incomplete. Award zero when the conclusion is incorrect or based on a contradictory explanation.

## Code-review challenges

Score each review out of four points. Award one point each for identifying the main issue, explaining the concrete consequence, proposing a proportionate improvement, and naming a relevant trade-off or boundary. A review does not need to propose the same fix as the stored answer when its alternative is correct and fits the code’s constraints.

## Compiler-error labs

Score each lab out of four points. Award one point each for predicting that the code fails, naming the primary rule involved, describing why the code violates it, and producing or recognizing a sound fixed version. Error codes are useful reference points but are not required for full credit unless the prompt specifically asks for one.

## Interview prompts

Interview prompts are evaluated on reasoning, not memorized vocabulary. Use this four-point scale for each prompt.

| Score | Evidence |
| --- | --- |
| 0 | The answer does not address the question or contains a fundamental misconception. |
| 1 | The answer identifies a relevant concept but cannot apply it to the situation. |
| 2 | The answer gives a correct baseline explanation and one relevant trade-off. |
| 3 | The answer explains a sound approach, trade-offs, and failure or edge cases. |
| 4 | The answer is technically precise, surfaces assumptions, compares viable alternatives, and justifies a decision for the stated context. |

Use the question’s `evaluation_criteria` as the checklist for a score of 2 or higher. A senior-level prompt should reward clear assumptions, constraints, and trade-offs; it should not demand a single predetermined implementation.

## Using results

Treat a low score as a study signal, not a stop sign. Follow the question’s canonical reference, restate the rule in your own words, and retry a related challenge after making a small working example. Website progress features may present these scores later, but this repository does not collect or transmit learner results.
