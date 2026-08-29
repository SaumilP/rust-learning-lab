# Rust interview taxonomy

The interview track is organized by competency, not as a random question list. Its 11 categories define the progression for future interview prompts, from language fundamentals through systems design and code review.

Category data lives in [taxonomy/](./taxonomy/) as validated JSON. Each category records its learning focus, suitable audience levels, and canonical lessons. RLL-066 will add individual questions to this structure; this taxonomy does not pretend those questions already exist.

Validate the category files with `cargo run --manifest-path tools/contentctl/Cargo.toml -- validate-interview challenges/interview/taxonomy`.
