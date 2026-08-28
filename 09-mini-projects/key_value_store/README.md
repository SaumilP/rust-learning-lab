# Persistent Key-Value Store

An interactive key-value store demonstrating `HashMap`, JSON serialization,
file persistence, and a simple read-evaluate-print loop.

```bash
cargo run -p key_value_store
```

Try `set language rust`, `get language`, `list`, and `quit`. Data is stored in
`kvstore.json` in the process working directory. This project favors readable
control flow over database-level durability or concurrent access.
