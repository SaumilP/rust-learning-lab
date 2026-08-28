# Enums and Pattern Matching: Key Takeaways

- Variants can store no data, tuple-like data, or named fields.
- `match` is exhaustive and can return a value.
- Prefer meaningful variants over loosely related booleans.
- `Option` and `Result` are standard-library enums.
- A catch-all pattern should not hide variants that need distinct behavior.
