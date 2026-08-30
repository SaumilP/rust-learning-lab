# SQLx basics

This example uses PostgreSQL, SQLx connection pooling, typed row mapping, and parameter binding. It deliberately uses `query` and `query_as` rather than SQLx's compile-time query macros, so formatting, linting, tests, and compilation do not require a running database.

## Build checks

The default checks need Rust and the downloaded Cargo dependencies, but no database service:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
```

## Run the example

Start PostgreSQL and create a database before running the program:

```bash
docker run --name rust-lab-postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=testdb -p 5432:5432 -d postgres:15
export DATABASE_URL=postgres://postgres:password@localhost/testdb
cargo run
```

The program creates its `users` table on startup. Reusing the same database will preserve rows from earlier runs, so use a disposable database when you want repeatable output.

## Why the queries are checked at runtime

SQLx's `query!` and `query_as!` macros validate SQL against a live schema during compilation, or against metadata produced by `cargo sqlx prepare`. That is useful in an application with migrations and a committed offline cache. This standalone lesson has neither, so requiring those macros made an ordinary `cargo check` depend on an unconfigured PostgreSQL service.

The example keeps typed result mapping through `#[derive(sqlx::FromRow)]` and `query_as::<_, User>()`. SQL syntax and schema compatibility are checked when the query runs.

## Exercises

- Return the inserted user's creation timestamp.
- Wrap a group of updates in a transaction.
- Add a migration directory and compare runtime queries with SQLx's offline compile-time verification.
