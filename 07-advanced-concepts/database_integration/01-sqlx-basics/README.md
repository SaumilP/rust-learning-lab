# SQLx Basics - Compile-Time Verified Queries

Demonstrates SQLx's compile-time query verification, ensuring SQL correctness before runtime.

## Features

- Compile-time SQL verification
- Type-safe query results
- Async/await support
- Connection pooling
- Migrations

## Setup

```bash
# Start PostgreSQL
docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=password postgres:15

# Create .env file
echo "DATABASE_URL=postgres://postgres:password@localhost/testdb" > .env

# Create database
sqlx database create

# Run migrations
sqlx migrate run
```

## Building

```bash
# Requires DATABASE_URL environment variable
cargo build

# For offline mode (no database required)
cargo sqlx prepare
cargo build --offline
```

## Key Concepts

### Compile-Time Query Verification

SQLx verifies queries at compile time by connecting to your database:

```rust
// This fails to compile if:
// - Table doesn't exist
// - Column names are wrong
// - Types don't match
let user = sqlx::query_as!(
    User,
    "SELECT id, name, email FROM users WHERE id = $1",
    user_id
)
.fetch_one(&pool)
.await?;
```

### Type Safety

Rust types are automatically derived from database schema:

```rust
#[derive(sqlx::FromRow)]
struct User {
    id: i32,          // INTEGER
    name: String,     // VARCHAR
    email: String,    // VARCHAR
    created_at: DateTime<Utc>,  // TIMESTAMP
}
```

## Advantages over ORM

✅ Write raw SQL (full database features)
✅ Compile-time verification
✅ No runtime query parsing
✅ Better performance
✅ Simpler debugging

## Next Steps

- Add more complex queries
- Implement transactions
- Try different databases (MySQL, SQLite)
- Add full-text search
