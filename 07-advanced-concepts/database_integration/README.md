# Section 13: Database Integration

## Overview

Learn to work with databases in Rust using type-safe ORMs, async drivers, and connection pooling. Rust's type system ensures compile-time safety for database queries.

## Database Support

### SQL Databases
- **PostgreSQL** - Most popular, full-featured
- **MySQL/MariaDB** - Widely used, good performance
- **SQLite** - Embedded, serverless
- **Microsoft SQL Server** - Enterprise

### NoSQL Databases
- **MongoDB** - Document database
- **Redis** - Key-value store, caching
- **Cassandra** - Wide-column store
- **DynamoDB** - AWS managed NoSQL

## What You'll Learn

1. **SQLx** - Compile-time verified SQL queries
2. **Diesel** - Type-safe ORM
3. **tokio-postgres** - Async PostgreSQL client
4. **MongoDB Driver** - Async MongoDB operations
5. **Connection Pooling** - Efficient connection management
6. **Migrations** - Database schema versioning
7. **Transactions** - ACID guarantees

## Section Contents

### 01-sqlx-basics/
SQLx with compile-time query verification

### 02-diesel-orm/
Diesel ORM with type-safe queries

### 03-async-postgres/
Async PostgreSQL with tokio

### 04-mongodb/
MongoDB document operations

### 05-redis-cache/
Redis for caching and pub/sub

### 06-connection-pools/
Connection pooling patterns

## Prerequisites

- Rust installed
- Docker (for running databases)
- Basic SQL knowledge
- Understanding of async/await

## Database Setup

```bash
# PostgreSQL
docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=password postgres:15

# MySQL
docker run -d -p 3306:3306 -e MYSQL_ROOT_PASSWORD=password mysql:8

# MongoDB
docker run -d -p 27017:27017 mongo:6

# Redis
docker run -d -p 6379:6379 redis:7
```

## Key Crates

### SQLx (Recommended)

```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }
tokio = { version = "1", features = ["full"] }
```

**Advantages**:
- Compile-time query verification
- Async by default
- No code generation
- Supports multiple databases

### Diesel

```toml
[dependencies]
diesel = { version = "2.1", features = ["postgres"] }
```

**Advantages**:
- Type-safe query builder
- Excellent compile-time guarantees
- Mature ecosystem
- Migration support

## Project Structure

```
13-database-integration/
├── README.md
├── docker-compose.yml    # Start all databases
├── 01-sqlx-basics/
│   ├── Cargo.toml
│   ├── .env             # DATABASE_URL
│   ├── migrations/
│   ├── src/
│   │   ├── main.rs
│   │   └── models.rs
│   └── README.md
├── 02-diesel-orm/
├── 03-async-postgres/
├── 04-mongodb/
├── 05-redis-cache/
└── 06-connection-pools/
```

## Key Concepts

### Compile-Time Query Verification (SQLx)

```rust
use sqlx::postgres::PgPool;

#[derive(sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

// Query is verified at compile time!
let user = sqlx::query_as!(
    User,
    "SELECT id, name, email FROM users WHERE id = $1",
    user_id
)
.fetch_one(&pool)
.await?;
```

### Type-Safe Query Builder (Diesel)

```rust
use diesel::prelude::*;

let results = users::table
    .filter(users::age.gt(18))
    .filter(users::email.like("%@gmail.com"))
    .order(users::created_at.desc())
    .limit(10)
    .load::<User>(&mut conn)?;
```

### Connection Pooling

```rust
use sqlx::postgres::PgPoolOptions;

let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://user:pass@localhost/db")
    .await?;

// Pool automatically manages connections
let row = sqlx::query("SELECT 1")
    .fetch_one(&pool)
    .await?;
```

### Transactions

```rust
let mut tx = pool.begin().await?;

sqlx::query("INSERT INTO accounts (balance) VALUES ($1)")
    .bind(100)
    .execute(&mut *tx)
    .await?;

sqlx::query("UPDATE accounts SET balance = balance - $1 WHERE id = $2")
    .bind(50)
    .bind(1)
    .execute(&mut *tx)
    .await?;

tx.commit().await?;  // Or rollback on error
```

### Migrations

```sql
-- migrations/20240101_create_users.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);
```

```bash
# Run migrations
sqlx migrate run
```

## Performance Patterns

### Batch Insertions

```rust
// Efficient bulk insert
let mut tx = pool.begin().await?;

for user in users.chunks(1000) {
    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind(&user.name)
        .bind(&user.email)
        .execute(&mut *tx)
        .await?;
}

tx.commit().await?;
```

### Prepared Statements

```rust
// Reuse prepared statement
let stmt = pool.prepare("SELECT * FROM users WHERE id = $1").await?;

for id in ids {
    let user = stmt.query_one(&pool, &[&id]).await?;
}
```

### Read Replicas

```rust
// Write to primary
write_pool.execute("INSERT ...").await?;

// Read from replica
read_pool.query("SELECT ...").await?;
```

## MongoDB Pattern

```rust
use mongodb::{Client, options::ClientOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    age: i32,
}

let client = Client::with_uri_str("mongodb://localhost:27017").await?;
let db = client.database("mydb");
let collection = db.collection::<User>("users");

// Insert
collection.insert_one(user, None).await?;

// Find
let filter = doc! { "age": { "$gt": 18 } };
let cursor = collection.find(filter, None).await?;
```

## Redis Caching

```rust
use redis::Commands;

let client = redis::Client::open("redis://localhost")?;
let mut con = client.get_connection()?;

// Set with expiration
con.set_ex("user:1", "John", 3600)?;

// Get
let name: String = con.get("user:1")?;

// Pub/Sub
con.publish("notifications", "New message")?;
```

## Common Patterns

### Repository Pattern

```rust
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub async fn find_by_id(&self, id: i32) -> Result<User> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(Into::into)
    }

    pub async fn create(&self, user: NewUser) -> Result<User> {
        sqlx::query_as!(
            User,
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
            user.name,
            user.email
        )
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }
}
```

### Active Record Pattern

```rust
impl User {
    pub async fn save(&self, pool: &PgPool) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3",
            self.name,
            self.email,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
```

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("User not found: {0}")]
    NotFound(i32),

    #[error("Duplicate entry: {0}")]
    Duplicate(String),
}

// Convert database constraint violations
impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DbError::NotFound(0),
            _ => DbError::Sqlx(err),
        }
    }
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_create_user(pool: PgPool) {
        let user = create_user(&pool, "Test", "test@example.com").await.unwrap();
        assert_eq!(user.name, "Test");
    }
}
```

## Challenges

1. **Build a Blog API** - CRUD operations with Diesel
2. **Implement Caching** - Redis layer over PostgreSQL
3. **Full-Text Search** - PostgreSQL full-text search
4. **Sharding** - Distribute data across databases
5. **Event Sourcing** - Event log with projections
6. **Graph Queries** - Recursive CTEs or graph database

## Performance Tips

1. **Use Connection Pools** - Reuse connections
2. **Batch Operations** - Reduce round trips
3. **Index Strategically** - Analyze query plans
4. **Prepared Statements** - Reuse query plans
5. **Async All The Way** - Don't block database threads
6. **Monitor Slow Queries** - Use EXPLAIN ANALYZE

## Resources

- [SQLx Documentation](https://docs.rs/sqlx/)
- [Diesel Guide](https://diesel.rs/guides/)
- [PostgreSQL Rust Driver](https://docs.rs/tokio-postgres/)
- [MongoDB Rust Driver](https://www.mongodb.com/docs/drivers/rust/)
- [Database Performance](https://use-the-index-luke.com/)

## Next Steps

After completing this section:
- Build a complete REST API with database
- Implement complex queries and joins
- Learn about database replication
- Explore time-series databases (TimescaleDB)

---

**Estimated Time**: 14-18 hours
**Difficulty**: ★★★★☆ (Advanced)
**Prerequisites**: Rust basics, SQL knowledge, async/await
