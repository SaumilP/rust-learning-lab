use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Row;
use std::env;

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("✅ Connected to database");

    // Create table
    create_table(&pool).await?;

    // Insert users
    insert_user(&pool, "Alice", "alice@example.com").await?;
    insert_user(&pool, "Bob", "bob@example.com").await?;
    insert_user(&pool, "Carol", "carol@example.com").await?;

    println!("\n📋 All users:");
    list_users(&pool).await?;

    // Find by email
    println!("\n🔍 Finding Alice:");
    if let Some(user) = find_by_email(&pool, "alice@example.com").await? {
        println!("   Found: {} ({})", user.name, user.email);
    }

    // Update
    update_user_name(&pool, 1, "Alice Smith").await?;

    // Count
    let count = count_users(&pool).await?;
    println!("\n📊 Total users: {}", count);

    // Delete
    delete_user(&pool, 2).await?;
    println!("\n🗑️  Deleted user with id=2");

    println!("\n📋 Final user list:");
    list_users(&pool).await?;

    Ok(())
}

async fn create_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    println!("✅ Table created/verified");
    Ok(())
}

async fn insert_user(pool: &PgPool, name: &str, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
    )
    .bind(name)
    .bind(email)
    .fetch_one(pool)
    .await?;

    println!("✅ Created user: {} ({})", user.name, user.email);
    Ok(user)
}

async fn list_users(pool: &PgPool) -> Result<(), sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users ORDER BY id")
        .fetch_all(pool)
        .await?;

    for user in users {
        println!("   [{}] {} <{}>", user.id, user.name, user.email);
    }

    Ok(())
}

async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

async fn update_user_name(pool: &PgPool, id: i32, new_name: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET name = $1 WHERE id = $2")
        .bind(new_name)
        .bind(id)
        .execute(pool)
        .await?;

    println!("✅ Updated user {}", id);
    Ok(())
}

async fn count_users(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM users")
        .fetch_one(pool)
        .await?;

    let count: i64 = row.get("count");
    Ok(count)
}

async fn delete_user(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
