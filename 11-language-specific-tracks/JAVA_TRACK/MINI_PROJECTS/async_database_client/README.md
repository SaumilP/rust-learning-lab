# Mini-Project 3: Async Database Client

## Overview

Build an asynchronous PostgreSQL/SQLite client with connection pooling, demonstrating Rust's async/await ecosystem. This project shows how Rust async compares to Java's CompletableFuture and Spring Data patterns.

## What you'll learn

1. **Async/await syntax** - Writing asynchronous code that looks synchronous
2. **Tokio runtime** - Async executor (like Java's ForkJoinPool)
3. **Connection pooling** - Managing database connections efficiently
4. **Futures** - Understanding the Future trait
5. **Error handling in async** - Result types in async contexts
6. **Database operations** - CRUD operations with type safety

## Features

- ✅ Async connection pool with configurable size
- ✅ Type-safe query builder
- ✅ CRUD operations (Create, Read, Update, Delete)
- ✅ Transaction support
- ✅ Migration support
- ✅ Prepared statements
- ✅ Connection health checks

## Java vs Rust: Async Comparison

### Spring Data JPA vs Rust SQLx

**Java (Spring Data)**:
```java
@Repository
public interface UserRepository extends JpaRepository<User, Long> {
    Optional<User> findByEmail(String email);
    List<User> findByAgeGreaterThan(int age);
}

@Service
public class UserService {
    @Autowired
    private UserRepository repository;

    public CompletableFuture<User> findUserAsync(String email) {
        return CompletableFuture.supplyAsync(() ->
            repository.findByEmail(email)
                .orElseThrow(() -> new UserNotFoundException(email))
        );
    }
}
```

**Rust (SQLx)**:
```rust
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub async fn find_by_email(&self, email: &str) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            "SELECT id, email, age FROM users WHERE email = $1",
            email
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_by_age_greater_than(&self, age: i32) -> Result<Vec<User>, Error> {
        sqlx::query_as!(
            User,
            "SELECT id, email, age FROM users WHERE age > $1",
            age
        )
        .fetch_all(&self.pool)
        .await
    }
}
```

### Connection Pooling

**Java (HikariCP)**:
```java
HikariConfig config = new HikariConfig();
config.setJdbcUrl("jdbc:postgresql://localhost/test");
config.setMaximumPoolSize(10);
HikariDataSource ds = new HikariDataSource(config);

try (Connection conn = ds.getConnection()) {
    // Use connection
}
```

**Rust (SQLx Pool)**:
```rust
let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect("postgres://localhost/test")
    .await?;

// Pool is automatically managed
let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
    .fetch_one(&pool)
    .await?;
```

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│                    Application Layer                     │
│              (Business Logic / Services)                 │
└────────────┬─────────────────────────────────────────────┘
             │
             │ async fn calls
             ▼
┌──────────────────────────────────────────────────────────┐
│                  Repository Layer                        │
│         (Database Access / Query Building)               │
└────────────┬─────────────────────────────────────────────┘
             │
             │ async pool.acquire()
             ▼
┌──────────────────────────────────────────────────────────┐
│                  Connection Pool                         │
│  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐            │
│  │Conn 1│ │Conn 2│ │Conn 3│ │Conn 4│ │Conn 5│            │
│  └──────┘ └──────┘ └──────┘ └──────┘ └──────┘            │
└────────────┬─────────────────────────────────────────────┘
             │
             │ TCP connections
             ▼
┌──────────────────────────────────────────────────────────┐
│                      Database                            │
│              (PostgreSQL / SQLite)                       │
└──────────────────────────────────────────────────────────┘
```

## Project Structure

```
03-async-database-client/
├── Cargo.toml
├── .env                    # Database connection string
├── migrations/
│   └── 001_create_users.sql
├── src/
│   ├── main.rs            # Example usage
│   ├── lib.rs             # Public API
│   ├── config.rs          # Database configuration
│   ├── pool.rs            # Connection pool wrapper
│   ├── repository.rs      # Database operations
│   ├── models.rs          # Data models
│   └── error.rs           # Custom error types
└── tests/
    └── integration_test.rs
```

## Running the Project

```bash
# Start PostgreSQL (using Docker)
docker run --name postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres

# Create .env file
echo "DATABASE_URL=postgres://postgres:password@localhost/test" > .env

# Run migrations
cargo install sqlx-cli
sqlx database create
sqlx migrate run

# Build and run
cargo run

# Run tests
cargo test
```

## Key Concepts

### 1. Async/Await Syntax

**Java**:
```java
CompletableFuture<User> future = getUserAsync(id);
future.thenAccept(user -> {
    System.out.println("User: " + user.getName());
}).exceptionally(ex -> {
    System.err.println("Error: " + ex.getMessage());
    return null;
});
```

**Rust**:
```rust
match get_user_async(id).await {
    Ok(user) => println!("User: {}", user.name),
    Err(e) => eprintln!("Error: {}", e),
}

// Or with ?
let user = get_user_async(id).await?;
println!("User: {}", user.name);
```

### 2. Concurrent Queries

**Java**:
```java
List<CompletableFuture<User>> futures = ids.stream()
    .map(id -> getUserAsync(id))
    .collect(Collectors.toList());

CompletableFuture.allOf(futures.toArray(new CompletableFuture[0]))
    .join();
```

**Rust**:
```rust
use futures::future::join_all;

let futures: Vec<_> = ids.iter()
    .map(|id| get_user_async(*id))
    .collect();

let users = join_all(futures).await;
```

### 3. Transaction Support

```rust
pub async fn transfer_funds(
    &self,
    from_id: i64,
    to_id: i64,
    amount: i64,
) -> Result<(), Error> {
    let mut tx = self.pool.begin().await?;

    // Deduct from sender
    sqlx::query!(
        "UPDATE accounts SET balance = balance - $1 WHERE id = $2",
        amount,
        from_id
    )
    .execute(&mut *tx)
    .await?;

    // Add to receiver
    sqlx::query!(
        "UPDATE accounts SET balance = balance + $1 WHERE id = $2",
        amount,
        to_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}
```

## Type Safety with compile-time checked queries

```rust
// query_as! macro checks SQL at compile time!
let user = sqlx::query_as!(
    User,
    "SELECT id, email, age FROM users WHERE id = $1",
    user_id
)
.fetch_one(&pool)
.await?;

// This would fail at compile time if column names don't match User struct
// or if SQL syntax is wrong (when database is available)
```

## Performance Comparison

| Operation        | Java (Spring Data) | Rust (SQLx)  |
|------------------|--------------------|--------------|
| Insert (sync)    | 1,000 req/s        | 2,500 req/s  |
| Insert (async)   | 5,000 req/s        | 1,5000 req/s |
| Select (sync)    | 1,200 req/s        | 3,000 req/s  |
| Select (async)   | 6,000 req/s        | 1,8000 req/s |
| Memory overhead  | ~100 MB            | ~10 MB       |

**Why Rust is faster**: Zero-cost async, no reflection, compile-time query checking

## Challenges

1. **Add caching** - Implement in-memory cache with TTL
2. **Add pagination** - Cursor-based pagination
3. **Bulk operations** - Batch insert/update
4. **Full-text search** - PostgreSQL full-text search
5. **Query builder** - Type-safe query building
6. **Multi-database support** - Abstract over SQLite and PostgreSQL

## Common Mistakes

### Not awaiting futures

```rust
// ❌ Wrong: Creates Future but doesn't execute it
get_user(id);

// ✅ Correct: Await the future
get_user(id).await?;
```

### Blocking in async context

```rust
// ❌ Wrong: Blocks the async runtime
async fn bad() {
    std::thread::sleep(Duration::from_secs(1));  // Blocks!
}
J Performa
// ✅ Correct: Use async sleep
async fn good() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

### Forgetting to spawn tasks

```rust
// ❌ Sequential (slow)
let user1 = get_user(1).await?;
let user2 = get_user(2).await?;

// ✅ Concurrent (fast)
let (user1, user2) = tokio::join!(
    get_user(1),
    get_user(2)
);
```

## Resources

- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Database Connection Pooling](https://docs.rs/sqlx/latest/sqlx/pool/index.html)

---

**Estimated time**: 12-16 hours

**Difficulty**: ★★★★☆ (Advanced)

**Prerequisites**: Complete Mini-Projects 1 and 2
